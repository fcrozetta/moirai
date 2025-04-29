use std::collections::HashMap;
use serde_json::Value;
use crate::plugin::ActionDefinition;

/// Definition for the "start" system node
pub fn definition() -> ActionDefinition {
    ActionDefinition {
        name: "start".to_string(),
        description: "Workflow entrypoint".to_string(),
        entrypoint: "internal:start".to_string(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    }
}

/// Execute the "start" system action (no-op)
pub async fn execute(
    inputs: HashMap<String, Value>
) -> Result<HashMap<String, Value>, String> {
    Ok(inputs)
}