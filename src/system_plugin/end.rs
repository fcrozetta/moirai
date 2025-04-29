use std::collections::HashMap;
use serde_json::Value;
use crate::plugin::ActionDefinition;

/// Definition for the "end" system node
pub fn definition() -> ActionDefinition {
    ActionDefinition {
        name: "end".to_string(),
        description: "Workflow end node".to_string(),
        entrypoint: "internal:end".to_string(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    }
}

/// Execute the "end" system action (no-op)
pub async fn execute(
    inputs: HashMap<String, Value>
) -> Result<HashMap<String, Value>, String> {
    Ok(inputs)
}