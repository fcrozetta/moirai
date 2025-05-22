use serde::Deserialize;
use std::collections::HashMap;

/// Data type definitions,
/// matching `types/<type_fqdn>.json`
#[derive(Debug, Deserialize)]
pub struct DataType {
    pub type_fqdn: String,
    pub version: String,
    pub metadata: TypeMetadata,
    pub properties: TypeProperties
}

#[derive(Debug, Deserialize)]
pub struct TypeMetadata {
    pub display_name: String,
    pub description: String,
    pub author: Option<String>,
    pub created: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct TypeProperties {
    pub kind: String,
    pub format: String,
    pub constraints: serde_json::Value,
}

/// Node specification
/// matchin `nodes/<node_fqdn>.json`
#[derive(Debug, Deserialize, Clone)]
pub struct NodeSpec {
    #[serde(rename = "type")]
    pub node_type: String,       // primitive | action | flow
    pub node_fqdn: String,
    pub plugin_version: String,
    pub node_version: String,

    // Example: for python plugins, which class to dispatch to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,

    

    pub inputs: Vec<IOField>,
    pub outputs: Vec<IOField>,

    pub metadata: NodeMetadata,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IOField {
    pub name: String,
    pub r#type: String,
    pub multiple: bool,
    pub required: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NodeMetadata {
    pub display_name: String,
    pub description: String,

    pub author: Option<String>,
    pub created: Option<String>,
    pub tags: Option<Vec<String>>,
    pub icon: Option<String>,
}

// Taken from `workflow_validator`, a node _instance_ in a workflow
#[derive(Debug, Deserialize)]
pub struct NodeInstance {
    pub id: String,
    pub node_fqdn: String,
    #[serde(flatten)]
    pub params: HashMap<String, serde_json::Value>, // e.g. { "value": "hello" } for primitives
}

/// Taken from `workflow_validator`, an edge connecting two instances
#[derive(Clone, Debug, Deserialize)]
pub struct EdgeDefinition {
    pub from_node: String,
    pub from_output: String,
    pub to_node: String,
    pub to_input: String,
}