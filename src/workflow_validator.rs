use crate::config_loader::Config;
use crate::plugin_manager::PluginManager;
use serde::Deserialize;
use std::{collections::{HashMap, HashSet}, fs, path::Path};
use thiserror::Error;

// Errors during workflow validation
#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error("Failed to read workflow file '{0}': {1}")]
    ReadFile(String, #[source] std::io::Error),

    #[error("Failed to parse workflow JSON '{0}': {1}")]
    ParseJson(String, #[source] serde_json::Error),

    #[error("Validation error'{0}'")]
    Validation(String),
}

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

    // TODO: Maybe this have to change if new primitive types are added?
    /// Only primitive nodes have a literal value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// A directed connection between nodes
#[derive(Debug, Deserialize)]
pub struct EdgeDefinition {
    pub from_node: String,
    pub from_output: String,
    pub to_node: String,
    pub to_input: String,
}

/// Validates a workflow against loaded plugins and node/type specs
pub fn validate_workflow_file<P: AsRef<Path>>(
    path:P,
    config: &Config,
    plugins: &PluginManager
) -> Result<WorkflowDefinition, WorkflowError> {
    let path_str = path.as_ref().display().to_string();
    let json_str = fs::read_to_string(&path)
        .map_err(|e| WorkflowError::ReadFile(path_str.clone(), e))?;
    let wf: WorkflowDefinition = serde_json::from_str(&json_str)
        .map_err(|e| WorkflowError::ParseJson(path_str.clone(), e))?;

    // TODO: I think 1.0 should not be fixed here...
    // * 1. Version
    if wf.version != "1.0" {
        return Err(WorkflowError::Validation(format!("Unsupported workflow version: {}", wf.version)));
    }

    // * 2. Unique IDs
    let mut ids= HashSet::new();
    for node in &wf.nodes {
        if !ids.insert(&node.id){
            return Err(WorkflowError::Validation(format!("Duplicate node Id: {}", node.id)));
        }
    }

    // * 3. Edge references
    for edge in &wf.edges {
        if !ids.contains(&edge.from_node){
            return Err(WorkflowError::Validation(format!("Edge from unknown node: {}", edge.from_node)));
        }

        if !ids.contains(&edge.to_node) {
            return Err(WorkflowError::Validation(format!("Edge to unknown node: {}", edge.to_node)));
        }
    }

    // * 4. FQDN exist in plugins (aka: plugin is installed)
    for node in &wf.nodes {
        let parts:Vec<&str> = node.node_fqdn.split(":").collect();
        if parts.len() != 2 {
            return Err(WorkflowError::Validation(format!("Invalid FQDN: {}", node.node_fqdn)));
        }

        let namespace = parts[0];
        let plugin = plugins.get(namespace)
            .ok_or_else(|| WorkflowError::Validation(format!("Unknown plugin namespace: {}", namespace)))?;
        // TODO: Verify node_fqdn in plugin.manifest.nodes?
    }

    // * 5. Additional checks
    // TODO: primitive nodes with value must be of correct type
    // TODO: Edges wiring ensures only data to data and flow to flow edges

    // ALL GOOD!!
    Ok(wf)

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::config_loader::{Config,EngineConfig, PluginSource};
    use crate::plugin_manager::PluginManager;
    use std::collections::HashMap;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn make_config() -> Config {
        Config {
            version: "1.0.0".to_string(),
            engine: EngineConfig { engine_version: "0.1.0".to_string(), workflow: None, concurrency: None, timeout_seconds: None },
            plugins: HashMap::new()
        }
    }

    #[test]
    fn version_mismatch() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"{{
                "version": "2.0",
                "name": "wf",
                    "nodes": [],
                    "edges": []
            }}"#
        ).unwrap();

        let cfg = make_config();
        let pm = PluginManager::new();
        let err = validate_workflow_file(file.path(), &cfg, &pm).unwrap_err();
        assert!(
            format!("{}",err).contains("Unsupported workflow version: 2.0"),
            "got error: {}",
            err
        )
    }

    #[test]
    fn duplicate_node_ids() {
        let mut file = NamedTempFile::new().unwrap();

        write!(
            file,
            r#"{{
                "version": "1.0",
                "name": "wf",
                "nodes": [
                    {{ "id": "n1", "node_fqdn": "sys:start" }},
                    {{ "id": "n1", "node_fqdn": "sys:start" }}
                ],
                "edges": []
            }}"#,
        ).unwrap();

        let cfg = make_config();
        let pm = PluginManager::new();
        let err = validate_workflow_file(file.path(), &cfg, &pm).unwrap_err();
        assert!(
            format!("{}",err).contains("Duplicate node Id: n1"),
            "got error: {}",
            err
        );
    }

    #[test]
    fn unknown_edge_reference() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"{{
                "version": "1.0",
                "name": "wf",
                "nodes": [
                    {{ "id": "n1", "node_fqdn": "sys:start" }}
                ],
                "edges": [
                    {{ "from_node": "n1", "from_output": "on_success", "to_node": "n2", "to_input": "__flow__" }}
                ]
            }}"#
        ).unwrap();

        let cfg = make_config();
        let pm = PluginManager::new();
        let err = validate_workflow_file(file.path(), &cfg, &pm).unwrap_err();

        assert!(
            format!("{}",err).contains("Edge to unknown node: n2"),
            "got error: {}",
            err
        );
    }

    #[test]
    fn invalid_fqdn_format() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"{{
                "version": "1.0",
                "name": "wf",
                "nodes": [
                    {{ "id": "n1", "node_fqdn": "invalidFQDN" }}
                ],
                "edges": []
            }}"#
        ).unwrap();

        let cfg = make_config();
        let pm = PluginManager::new();
        let err = validate_workflow_file(file.path(), &cfg, &pm).unwrap_err();

        assert!(
            format!("{}",err).contains("Invalid FQDN: invalidFQDN"),
            "got error: {}",
            err
        );
    }

    #[test]
    fn unknown_plugin_namespace() {
        // valid id and fqdn format, but no "foo" plugin loaded
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"{{
                "version": "1.0",
                "name": "wf",
                "nodes": [
                    {{ "id": "n1", "node_fqdn": "foo:bar" }}
                ],
                "edges": []
            }}"#
        ).unwrap();

        let cfg = make_config();
        let pm = PluginManager::new();
        let err = validate_workflow_file(file.path(), &cfg, &pm).unwrap_err();
        assert!(
            format!("{}", err).contains("Unknown plugin namespace: foo"),
            "got error: {}",
            err
        );
    }

}