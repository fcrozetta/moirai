use crate::specs::EdgeDefinition;
use crate::config_loader::Config;
use crate::plugin_manager::PluginManager;
use serde::Deserialize;
use std::{collections::{HashMap, HashSet}, fs::{read_to_string}, path::Path};
use thiserror::Error;



/// Top level workflow definition loaded from JSON
#[derive(Debug, Deserialize)]
pub struct WorkflowDefinition {
    pub version: String,
    pub name: String,
    pub metadata: Option<WorkflowMetadata>,
    pub nodes: Vec<NodeInstance>,
    pub edges: Vec<EdgeDefinition>,
}

/// Optional metadata in the workflow
#[derive(Debug, Deserialize)]
pub struct WorkflowMetadata {
    pub description:Option<String>,
    pub author: Option<String>,
    pub created: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// A single node instance in the workflow
#[derive(Debug, Deserialize)]
pub struct NodeInstance {
    pub id: String,
    pub node_fqdn: String,

    /// Only action nodes have inputs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<HashMap<String, serde_json::Value>>,

    /// Only primitive nodes have a literal value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// Validates a workflow against loaded plugins and node/type specs
pub fn validate_workflow_file(
    path:&Path,
    config: &Config,
    plugins: &PluginManager
) -> Result<WorkflowDefinition, WorkflowError> {

    // *1. Structural check
    validate_structure(path).map_err(WorkflowError::Structure)?;

    // *2.Parse json into definition
    let s = read_to_string(path)?;
    let wf: WorkflowDefinition = serde_json::from_str(&s)?;

    // TODO: I think 1.0 should not be fixed here...
    // *3. Version
    if wf.version != "1.0" {
        return Err(WorkflowError::Validation(format!("Unsupported workflow version: {}", wf.version)));
    }

    // *4. Unique IDs
    let mut ids= HashSet::new();
    for node in &wf.nodes {
        if !ids.insert(&node.id){
            return Err(WorkflowError::Validation(format!("Duplicate node Id: {}", node.id)));
        }
    }

    // *5. Edge references
    for edge in &wf.edges {
        if !ids.contains(&edge.from_node){
            return Err(WorkflowError::Validation(format!("Edge from unknown node: {}", edge.from_node)));
        }

        if !ids.contains(&edge.to_node) {
            return Err(WorkflowError::Validation(format!("Edge to unknown node: {}", edge.to_node)));
        }
    }

    // *6. FQDN exist in plugins (aka: plugin is installed)
    for node in &wf.nodes {
        let parts:Vec<&str> = node.node_fqdn.split(":").collect();
        if parts.len() != 2 {
            return Err(WorkflowError::Validation(format!("Invalid FQDN: {}", node.node_fqdn)));
        }

        let namespace = parts[0];
        if namespace != "sys" {
            plugins.get(namespace)
                .ok_or_else(|| WorkflowError::Validation(format!("Unknown plugin namespace: {}", namespace)))?;
        }
        // TODO: Verify node_fqdn in plugin.manifest.nodes?
    }

    // *7. Additional checks
    // TODO: primitive nodes with value must be of correct type
    // TODO: Edges wiring ensures only data to data and flow to flow edges

    // ALL GOOD!!
    Ok(wf)

}



/// Light structural validation:
/// Checks wiring and basic shapes,
/// but *not* NodeSpecs or types
pub fn validate_structure(path: &Path) -> Result<(), StructureError> {
    // *1. Read and parse JSON
    let s = std::fs::read_to_string(path)?;
    let def: WorkflowDefinition = serde_json::from_str(&s)?;

    // // *2. Version check
    // if def.version != "1.0" {
    //     return Err(StructureError::Version(def.version.clone()));
    // }

    // *3. Exactly one sys:start
    let start_count = def.nodes
        .iter()
        .filter(|n| n.node_fqdn == "sys:start")
        .count();
    if start_count != 1 {
        return Err(StructureError::StartNodeCount(start_count));
    }

    // *4. Build set of declared IDs
    let ids: HashSet<&String> = def.nodes.iter().map(|n| &n.id).collect();

    // *5. Validate each edge
    for edge in &def.edges {
        if !ids.contains(&edge.from_node) {
            return Err(StructureError::MissingNode(edge.from_node.clone()));
        }
        if !ids.contains(&edge.to_node) {
            return Err(StructureError::MissingNode(edge.to_node.clone()));
        }

        if edge.from_output.trim().is_empty() || edge.to_input.trim().is_empty() {
            return Err(StructureError::EmptyPort);
        }
    }
    Ok(())
}

// Errors produced by light (structural) validator
#[derive(Debug, Error)]
pub enum StructureError {
    #[error("I/O error reading workflow file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse workflow JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Unsupported workflow version: `{0}`, expected `1.0`")]
    Version(String),

    #[error("Expected exactly one `sys:start` node, found: `{0}`")]
    StartNodeCount(usize),

    #[error("Edge references unknown node `{0}`")]
    MissingNode(String),

    #[error("Edge has an empty port name (from_output or to_input)")]
    EmptyPort,
}

// Errors during workflow validation
#[derive(Debug, Error)]
pub enum WorkflowError {

    #[error(transparent)]
    Structure(#[from] StructureError),

    #[error("I/O error reading workflow JSON: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse workflow JSON {0}")]
    Json(#[from] serde_json::Error),

    #[error("Validation error: '{0}'")]
    Validation(String),

    #[error("No `sys:start` node found in workflow")]
    MissingStartNode,

    #[error("Could Not Execute the Node")]
    Execution,

    // TODO: add errors for Plugin-aware errors, ports not found, type mismatch, etc.
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::config_loader::{Config,EngineConfig};
    use crate::plugin_manager::PluginManager;
    use std::{collections::HashMap,io::Write};
    use tempfile::NamedTempFile;

    fn make_config() -> Config {
        Config {
            version: "1.0.0".into(),
            engine: EngineConfig { 
                engine_version: "0.1.0".into(), 
                workflow: None, 
                concurrency: None, 
                timeout_seconds: None 
            },
            plugins: HashMap::new()
        }
    }

    #[test]
    fn struct_missing_start() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"{{
                "version": "1.0",
                "name": "wf",
                "nodes": [
                    {{ "id": "n1", "node_fqdn": "sys:log", "inputs": {{}} }}
                ],
                "edges": []
            }}"#
        ).unwrap();

        let err = validate_structure(file.path()).unwrap_err();
        match err {
            StructureError::StartNodeCount(cnt) if cnt == 0 => {},
            _=> panic!("Expected StartNodeCount(0), got {:?}", err),
        }
    }

    #[test]
    fn struct_missing_node_id_in_edge() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"{{
                "version":"1.0",
                "name":"wf",
                "nodes":[
                    {{ "id":"n1","node_fqdn":"sys:start" }}
                ],
                "edges":[
                    {{ "from_node":"nX","from_output":"__success__","to_node":"n1","to_input":"__input__" }}
                ]
            }}"#
        ).unwrap();

        let err = validate_structure(file.path()).unwrap_err();
        match err {
            StructureError::MissingNode(ref id) if id == "nX" => {},
            _ => panic!("expected MissingNode(\"nX\"), got {:?}", err),
        }
    }

    #[test]
    fn struct_empty_port_name() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"{{
                "version":"1.0",
                "name":"wf",
                "nodes":[
                    {{ "id":"n1","node_fqdn":"sys:start" }}
                ],
                "edges":[
                    {{ "from_node":"n1","from_output":"   ","to_node":"n1","to_input":"__input__" }}
                ]
            }}"#
        ).unwrap();

        let err = validate_structure(file.path()).unwrap_err();
        match err {
            StructureError::EmptyPort => {},
            _ => panic!("expected EmptyPort, got {:?}", err),
        }
    }

    // ----------------- Heavy validator tests -------------

    #[test]
    fn heavy_version_mismatch() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"{{
                "version":"2.0",
                "name":"wf",
                "nodes":[
                    {{ "id":"n1","node_fqdn":"sys:start" }}
                ],
                "edges":[]
            }}"#
        ).unwrap();

        let cfg = make_config();
        let pm  = PluginManager::new();
        let err = validate_workflow_file(file.path(), &cfg, &pm).unwrap_err();
        match err {
            WorkflowError::Validation(ref msg) if msg.contains("Unsupported workflow version: 2.0") => {},
            _ => panic!("Expected version mismatch error, got {:?}", err),
        }
    }

    #[test]
    fn heavy_duplicate_node_ids() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"{{
                "version":"1.0",
                "name":"wf",
                "nodes":[
                    {{ "id":"n0","node_fqdn":"sys:start" }},
                    {{ "id":"n1","node_fqdn":"sys:log" }},
                    {{ "id":"n1","node_fqdn":"sys:log" }}
                ],
                "edges":[]
            }}"#
        ).unwrap();

        let cfg = make_config();
        let pm  = PluginManager::new();
        let err = validate_workflow_file(file.path(), &cfg, &pm).unwrap_err();
        match err {
            WorkflowError::Validation(ref msg) if msg.contains("Duplicate node Id: n1") => {},
            _ => panic!("Expected duplicate node id error, got {:?}", err),
        }
    }

    #[test]
    fn heavy_unknown_edge_reference() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"{{
                "version":"1.0",
                "name":"wf",
                "nodes":[
                    {{ "id":"n1","node_fqdn":"sys:start" }}
                ],
                "edges":[
                    {{ "from_node":"n1","from_output":"__success__","to_node":"n2","to_input":"__input__" }}
                ]
            }}"#
        ).unwrap();

        let cfg = make_config();
        let pm  = PluginManager::new();
        let err = validate_workflow_file(file.path(), &cfg, &pm).unwrap_err();
        match err {
            WorkflowError::Structure(StructureError::MissingNode(ref id)) if id == "n2" => {},
            _ => panic!("Expected missing node error for 'n2', got {:?}", err),
        }
    }

    #[test]
    fn heavy_invalid_fqdn_format() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"{{
                "version":"1.0",
                "name":"wf",
                "nodes":[
                    {{ "id":"n1","node_fqdn":"sys:start" }},
                    {{ "id":"n2","node_fqdn":"invalidFQDN" }}
                ],
                "edges":[]
            }}"#
        ).unwrap();

        let cfg = make_config();
        let pm  = PluginManager::new();
        let err = validate_workflow_file(file.path(), &cfg, &pm).unwrap_err();
        match err {
            WorkflowError::Validation(ref msg) if msg.contains("Invalid FQDN: invalidFQDN") => {},
            _ => panic!("Expected invalid FQDN error, got {:?}", err),
        }
    }

    #[test]
    fn heavy_unknown_plugin_namespace() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"{{
                "version":"1.0",
                "name":"wf",
                "nodes":[
                    {{ "id":"n1","node_fqdn":"sys:start" }},
                    {{ "id":"n2","node_fqdn":"foo:bar" }}
                ],
                "edges":[]
            }}"#
        ).unwrap();

        let cfg = make_config();
        let pm  = PluginManager::new();
        let err = validate_workflow_file(file.path(), &cfg, &pm).unwrap_err();
        match err {
            WorkflowError::Validation(ref msg) if msg.contains("Unknown plugin namespace: foo") => {},
            _ => panic!("Expected unknown plugin namespace error, got {:?}", err),
        }
    }
}