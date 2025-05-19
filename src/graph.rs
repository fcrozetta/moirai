use crate::{
    engine::{EngineError, EngineEvent},
    plugin_manager::PluginManager,
    specs::{EdgeDefinition,NodeSpec},
    workflow_validator::{WorkflowDefinition, WorkflowError}
};

use serde_json::Value;
use tokio::sync::{mpsc, oneshot};
use std::{
    collections::HashMap
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status { NotStarted, Running, Success, Failed }

#[derive(Clone, Debug)]
pub struct NodeState {
    /// Static spec from plugin
    pub spec: NodeSpec,
    
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

        // ! [ Mocked ]
        // TODO: replace this stub with real node runner that streams NDJSON events
        // Here we simply mark success with no outputs:
        {
            let state = self.nodes.get_mut(node_id).unwrap();
            state.status = Status::Success;
            state.outputs = Some(HashMap::new());
        }
        let _ = event_tx.send(EngineEvent::NodeLog {
            workflow_id: workflow_id.to_string(),
            node_id: node_id.to_string(),
            level: "info".into(),
            message: format!("Finished {} with Success", node_id),
        });
        // ! [ End Mocked ]

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