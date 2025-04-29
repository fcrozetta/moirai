use std::collections::HashMap;
use serde_json::Value;
use crate::plugin::ActionDefinition;
use crate::models::{ParameterDefinition, ParameterType};

/// Definition for the "string" system node
pub fn definition() -> ActionDefinition {
    ActionDefinition {
        name: "string".to_string(),
        description: "Create or manipulate a string value".to_string(),
        entrypoint: "internal:string".to_string(),
        inputs: Vec::new(),
        outputs: vec![
            ParameterDefinition {
                id: "output".to_string(),
                name: "Output".to_string(),
                param_type: ParameterType::String,
                description: Some("The resulting string value".to_string()),
                required: true,
                multiple: false,
                default: None,
            },
        ],
    }
}

/// Execute the "string" system action
pub async fn execute(
    inputs: HashMap<String, Value>
) -> Result<HashMap<String, Value>, String> {
    let mut outputs = HashMap::new();
    if let Some(val) = inputs.get("value").cloned() {
        outputs.insert("output".to_string(), val);
    }
    Ok(outputs)
}