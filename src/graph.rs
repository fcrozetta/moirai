use crate::{
    engine::{EngineError, EngineEvent},
    plugin_manager::{PluginManager, PluginManifest},
    specs::{EdgeDefinition,NodeSpec},
    workflow_validator::{WorkflowDefinition, WorkflowError}
};

use serde_json::Value;
use tokio::sync::{mpsc, oneshot};
use std::{
    collections::HashMap, f32::consts::E, io::{BufRead, BufReader, Stdout, Write}, process::{Command, Stdio}, thread
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status { NotStarted, Running, Success, Failed }

#[derive(Clone, Debug)]
pub struct NodeState {
    /// Static spec from plugin
    pub spec: NodeSpec,

    pub executor: Vec<String>,
    
    /// Pre-filled inputs.
    /// literal inputs or primitive value
    pub inputs: HashMap<String, Value>,

    /// Filled after execution
    pub outputs: Option<HashMap<String, Value>>,

    /// Execution Status
    pub status: Status,
}


/// In-memory graph ready for execution
pub struct Graph {
    /// Map of instance_id -> Nodestate
    /// E.g. {node_id, NodeState}
    pub nodes: HashMap<String, NodeState>,

    /// For each node_id, all data edges feeding into it
    /// (edges where `to_input` != "__input__")
    pub data_edges: HashMap<String, Vec<EdgeDefinition>>,           // E.g. {Node_1, [edges tha feed into Node1]}

    /// For each (from_node, outcome) -> the single next node_id
    /// outcome is "__success__" or "__failure__"
    pub control_edges: HashMap<(String,String), String>,            // E.g. (from_node_id, __success__), Node2
}

impl Graph {
    pub fn from_definition(
        def: &WorkflowDefinition,
        plugins: &PluginManager,
    ) -> Result<Self, EngineError> {
        let mut nodes = HashMap::new();
        
        // *1. Instantiate each node state
        for inst in &def.nodes {
            // Lookup namespace and plugin
            let namespace = inst.node_fqdn.splitn(2,":").next().expect("Malformed fqdn");
            let plugin = plugins.get(namespace).expect("Plugin was validated earlier");

            // Lookup nodeSpec
            let spec: NodeSpec = plugin.nodes
                .get(&inst.node_fqdn)
                .expect("Plugin was validated earlier")
                .clone();

            // Load executor
            let executor = executor_for(&plugin.manifest);

            // seed inputs from workflow instance
            let mut inputs_map = HashMap::new();
            if let Some(ref map) = inst.inputs {
                inputs_map.extend(map.clone());
            }

            if let Some(ref val) = inst.value {
                // Primitives drop thru here
                inputs_map.insert("value".into(), val.clone());
            }

            nodes.insert(
                inst.id.clone(),

                NodeState {
                    spec,
                    executor,
                    inputs: inputs_map,
                    outputs: None,
                    status: Status::NotStarted,
                }
            );
        }

        // Partition edges
        let mut data_edges: HashMap<String, Vec<EdgeDefinition>>  = HashMap::new();
        let mut control_edges: HashMap<(String, String), String> = HashMap::new();
        for edge in &def.edges {
            if edge.to_input == "__input__" {                                               
                // Control flow
                control_edges.insert(
                    (edge.from_node.clone(), edge.from_output.clone()), 
                    edge.to_node.clone()
                );
            } else {
                // Data flow
                data_edges
                    .entry(edge.to_node.clone())
                    .or_default()
                    .push(edge.clone());
            }
        }

        Ok(Graph { nodes, data_edges, control_edges })
    }

    pub fn get_start_id(&mut self) -> Result<&String,EngineError> {
        self
            .nodes
            .iter()
            .find(|(_id,_state)| _state.spec.node_fqdn == "sys:start")
            .map(|(id,_)| id )
            .ok_or_else(|| EngineError::Workflow(WorkflowError::MissingStartNode))
    }

    pub fn execute(
        &mut self,
        workflow_id: String,
        mut cancel_rx: oneshot::Receiver<()>,
        event_tx: mpsc::UnboundedSender<EngineEvent>
    ) {
        // *1. Find start Node
        let start_id = match self.get_start_id() {
            Ok(id) => id.clone(),
            Err(e) => {
                let _ = event_tx.send(EngineEvent::WorkflowFinished {
                    workflow_id: workflow_id.clone(),
                    is_success: false,
                });
                return;
            },
        };
        
        // *2. Kickoff recursion
        self.drive_node(&start_id, &workflow_id, &mut cancel_rx, &event_tx);
        
        // *3. Done - final event
    }

    fn drive_node(
        &mut self,
        node_id: &str,
        workflow_id: &str,
        cancel_rx: &mut oneshot::Receiver<()>,
        event_tx: &mpsc::UnboundedSender<EngineEvent>,
    ) -> Status {
        // *1. Get mutable reference to this node state
        let current_status = match self.nodes.get(node_id) {
            Some(s  ) => s.status,
            None => {
                // Something in the build graph failed. In theory should never occurr
                let _ = event_tx.send(EngineEvent::NodeError {
                    workflow_id: workflow_id.to_string(),
                    node_id: node_id.to_string(),
                    error: format!("Unknown node_id `{}`", node_id),
                });
                return Status::Failed;
            }
        };

        // *2. If the node already ran, just return the status
        if current_status != Status::NotStarted {
            return current_status;
        }

        // *3. Early cancellation check
        if let Ok(_) = cancel_rx.try_recv() {
            // We know the node hasn't run yet, so we can mutably borrow it:
            if let Some(state) = self.nodes.get_mut(node_id){
                state.status = Status::Failed;
            }
            let _ = event_tx.send(EngineEvent::WorkflowFinished {
                workflow_id: workflow_id.to_string(),
                is_success: false,
            });
            return Status::Failed;
        }

        // *4. Resolve all DATA dependencies first
        let edges: Vec<EdgeDefinition> = self.data_edges.get(node_id).cloned().unwrap_or_default();
        for edge in edges {
            // Recurse upstream
            let up_status = self.drive_node(
                &edge.from_node, 
                workflow_id, 
                cancel_rx, 
                event_tx
            );

            if up_status != Status::Success {
                if let Some(state) = self.nodes.get_mut(node_id) {
                    state.status = Status::Failed;
                }
                return Status::Failed;
            }

            // TODO: WTF is happening here?
            // pull value out of upstream outputs
            let val = self
                .nodes
                .get(&edge.from_node)
                .and_then(|up_state| up_state.outputs.as_ref())
                .and_then(|out_map| out_map.get(&edge.from_output))
                .cloned();
            match val {
                Some(v) => {
                    if let Some(state) = self.nodes.get_mut(node_id) {
                        state.inputs.insert(
                            edge.to_input.clone(), 
                            v
                        );
                    }
                },
                None => {
                    let _ = event_tx.send(EngineEvent::NodeError {
                        workflow_id: workflow_id.to_string(),
                        node_id: node_id.to_string(),
                        error: format!(
                            "Upstream node `{}` produced no output `{}`",
                            edge.from_node, edge.from_output
                        ),
                    });
                    if let Some(state) = self.nodes.get_mut(node_id) {
                        state.status = Status::Failed;
                    }
                    return Status::Failed;
                }
            }
        }

        // *5. Execute the node
        {
            // Borrow mutably only for this block
            let state = self.nodes.get_mut(node_id).unwrap();
            state.status = Status::Running;
        }
        let _ = event_tx.send(EngineEvent::NodeLog {
            workflow_id: workflow_id.to_string(),
            node_id: node_id.to_string(),
            level: "info".into(),
            message: format!("Executing {}", node_id),
        });

        
        let state = self.nodes.get(node_id).unwrap();
        let (status, outputs) = run_subprocess_node(
            &state.executor,
            &state.spec,
            &state.inputs,
            cancel_rx,
            &event_tx,
            workflow_id,
            node_id,
        );

        {
            let st = self.nodes.get_mut(node_id).unwrap();
            st.status  = status;
            st.outputs = Some(outputs);
        }
        let _ = event_tx.send(EngineEvent::NodeLog {
            workflow_id: workflow_id.to_string(),
            node_id:     node_id.to_string(),
            level:       "info".into(),
            message:     format!("Finished {} with {:?}", node_id, status),
        });

        // *6. Control flow: Pick the next one (if any)
        let state_status = {
            // Read final status (Copy)
            self.nodes.get(node_id).unwrap().status
        };

        let outcome = if state_status == Status::Success {
            "__succecss__"
        } else {
            "__failure__"
        };

        if let Some(next_id) = self
            .control_edges
            .get(&(node_id.to_string(), outcome.to_string()))
            .cloned() 
        {
            // Recurse into the next node
            let _ = self.drive_node(
                &next_id,
                workflow_id,
                cancel_rx,
                event_tx,
            );
        }

        // *7. Return our final status
        state_status

    }
}

fn run_subprocess_node(
    executor: &[String],
    spec: &NodeSpec,
    inputs: &HashMap<String, Value>,
    cancel_rx: &mut oneshot::Receiver<()>,
    event_tx: &mpsc::UnboundedSender<EngineEvent>,
    workflow_id: &str,
    node_id: &str,
    ) -> (Status, HashMap<String, Value>) {
        // *1. Build the command
        let mut cmd = Command::new(&executor[0]);
        cmd.args(&executor[1..]);

        // If this python and has classname
        if let Some(class_name) = &spec.class {
            cmd.arg(class_name);
        }

        // Set up stdio pipes
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // *2. Spawn child process
        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                let _ = event_tx.send(EngineEvent::NodeError {
                    workflow_id: workflow_id.to_string(),
                    node_id:     node_id.to_string(),
                    error:       format!("Failed to spawn process: {}", e),
                });
                return (Status::Failed, HashMap::new());
            }
        };

        // *3. Write inputs JSON into stdin
        if let Some(mut stdin) = child.stdin.take() {
            // Write the JSON
            if let Err(e) = serde_json::to_writer(&mut stdin, &inputs) {
                let _ = event_tx.send(EngineEvent::NodeError {
                    workflow_id: workflow_id.into(),
                    node_id:     node_id.into(),
                    error:       format!("Failed to serialize inputs: {}", e),
                });
                let _ = child.kill();
                return (Status::Failed, HashMap::new());
            }

            // Flush the writer
            if let Err(e) = stdin.flush() {
                let _ = event_tx.send(EngineEvent::NodeError {
                    workflow_id: workflow_id.into(),
                    node_id:     node_id.into(),
                    error:       format!("Failed to flush stdin: {}", e),
                });
                let _ = child.kill();
                return (Status::Failed, HashMap::new());
            }
        }

        // *4. Spawn a thread to reader stderr -> error logs
        if let Some(stderr) = child.stderr.take() {
            let tx = event_tx.clone();
            let wf = workflow_id.to_string();
            let nid = node_id.to_string();

            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines().filter_map(Result::ok) {
                    let _ = tx.send(EngineEvent::NodeLog {
                        workflow_id: wf.clone(),
                        node_id:     nid.clone(),
                        level:       "error".into(),
                        message:     line,
                    });
                }
            });
        }

        // *5. Read stdout libe by line
        let mut outputs = HashMap::new();
        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line_res in reader.lines() {
                // Check for cancellation
                if let Ok(_) = cancel_rx.try_recv() {
                    let _ = child.kill();
                    return (Status::Failed, outputs);
                }

                let line = match line_res {
                    Ok(l) => l,
                    Err(e) => {
                        let _ = event_tx.send(EngineEvent::NodeError {
                            workflow_id: workflow_id.to_string(),
                            node_id:     node_id.to_string(),
                            error:       format!("Stdout I/O error: {}", e),
                        });
                        continue;
                    } 
                };

                // Try parsing as JSON with field "type"
                if let Ok(val) = serde_json::from_str::<Value>(&line) {
                    if let Some(typ) = val.get("type").and_then(|v| v.as_str()) {
                        match typ {
                            "output" => {
                                if let Some(obj) = val.get("outputs").and_then(| v|  v.as_object()) {
                                    // Overwrite outputs
                                    outputs = obj
                                        .iter()
                                        .map(| (k,v)| (k.clone(),v.clone()))
                                        .collect();
                                    let _ = event_tx.send(EngineEvent::NodeOutput {
                                        workflow_id: workflow_id.to_string(),
                                        node_id:     node_id.to_string(),
                                    });
                                }
                                continue;
                            }
                            "log" => {
                                let level = val.get("level")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("info");
                                let msg = val.get("message")
                                    .and_then(| v| v.as_str())
                                    .unwrap_or("");
                                let _ = event_tx.send(EngineEvent::NodeLog {
                                    workflow_id: workflow_id.to_string(),
                                    node_id:     node_id.to_string(),
                                    level:       level.into(),
                                    message:     msg.into(),
                                });
                                continue;;
                            }
                            _ => { /* Other event types here! */}
                        }
                    }
                }
                // Fallback: plain‐text info log
                let _ = event_tx.send(EngineEvent::NodeLog {
                    workflow_id: workflow_id.to_string(),
                    node_id:     node_id.to_string(),
                    level:       "info".into(),
                    message:     line,
                });
            }
        }
        // *6 wait for the process to exit
        let status = match child.wait() {
            Ok(exit) if exit.success() => Status::Success,
            Ok(_) | Err(_)                         => Status::Failed
        };
    (status, outputs)
}



fn executor_for(man: &PluginManifest) -> Vec<String> {
    if let Some(exec) = &man.executor {
        return exec.clone();
    }
    let lang    = man.runtime.language.as_str();
    let manager = man.runtime.manager.as_str();
    let module  = &man.plugin_fqdn;

    match (lang, manager) {
        ("python", "uv") =>
            vec!["uv".into(), "run".into(), "python".into(), "-m".into(), module.clone()],

        ("python", "poetry") =>
            vec!["poetry".into(), "run".into(), "python".into(), "-m".into(), module.clone()],

            ("bash", _) =>
            vec!["bash".into(), "plugin.sh".into()],

        other =>
            panic!("No default executor for {:?}", other),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin_manager::{PluginManager};
    use crate::config_loader::PluginSource;
    use crate::workflow_validator::{WorkflowDefinition, NodeInstance};
    use crate::specs::EdgeDefinition;
    use serde_json::json;
    use tempfile::TempDir;
    use std::{collections::HashMap, fs};

    fn make_temp_mysys_plugin() -> (TempDir, PluginManager) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("mysys");
        fs::create_dir_all(root.join("nodes")).unwrap();

        // plugin.json
        fs::write(
            root.join("plugin.json"),
            json!({
                "plugin_fqdn": "mysys",
                "version": "1.0.0",
                "metadata": null,
                "types": [],
                "nodes": ["nodes/n1.json", "nodes/n2.json"]
            })
            .to_string(),
        )
        .unwrap();

        // nodes/n1.json and nodes/n2.json (primitive specs)
        for name in &["n1", "n2"] {
            fs::write(
                root.join("nodes").join(format!("{}.json", name)),
                json!({
                    "type": "primitive",
                    "node_fqdn": format!("mysys:{}", name),
                    "plugin_version": "1.0.0",
                    "node_version":   "1.0.0",
                    "entrypoint":     null,
                    "inputs":         [],
                    "outputs":        [],
                    "metadata": {
                        "display_name": name.to_uppercase(),
                        "description": format!("Test node {}", name),
                        "author": null,
                        "created": null,
                        "tags": null,
                        "icon": null
                    }
                })
                .to_string(),
            )
            .unwrap();
        }

        // Load into PluginManager
        let mut pm = PluginManager::new();
        let mut specs = HashMap::new();
        specs.insert(
            "mysys".to_string(),
            PluginSource {
                path: Some(root.to_string_lossy().to_string()),
                git:  None,
                rev:  None,
            },
        );
        pm.load_from_paths(&specs).unwrap();
        (tmp, pm)
    }

    #[test]
    fn graph_from_definition_basic_control_flow() {
        let (_tmp_dir, pm) = make_temp_mysys_plugin();

        // Build a WorkflowDefinition: n1 --__success__--> n2
        let def = WorkflowDefinition {
            version: "1.0".into(),
            name:    "wf".into(),
            metadata: None,
            nodes: vec![
                NodeInstance {
                    id:         "n1".into(),
                    node_fqdn:  "mysys:n1".into(),
                    inputs:     None,
                    value:      Some(json!("hello")),
                },
                NodeInstance {
                    id:         "n2".into(),
                    node_fqdn:  "mysys:n2".into(),
                    inputs:     None,
                    value:      None,
                },
            ],
            edges: vec![
                EdgeDefinition {
                    from_node:   "n1".into(),
                    from_output: "__success__".into(),
                    to_node:     "n2".into(),
                    to_input:    "__input__".into(),
                },
            ],
        };

        // Build the graph
        let graph = Graph::from_definition(&def, &pm).expect("graph build failed");

        // We should have both nodes...
        assert_eq!(graph.nodes.len(), 2);
        assert!(graph.nodes.contains_key("n1"));
        assert!(graph.nodes.contains_key("n2"));

        // ... no data edges ...
        assert!(graph.data_edges.is_empty());

        // ... and exactly one control edge
        let key = ("n1".to_string(), "__success__".to_string());
        assert_eq!(
            graph.control_edges.get(&key).unwrap(),
            "n2"
        );
    }
}
