use std::collections::HashMap;
use serde_json::{Value, Map};
use crate::plugin::ActionDefinition;
use crate::models::{ParameterDefinition, ParameterType};

/// Definition for the "object" system node
pub fn definition() -> ActionDefinition {
    ActionDefinition {
        name: "object".to_string(),
        description: "Create or manipulate a JSON object".to_string(),
        entrypoint: "internal:object".to_string(),
        inputs: vec![
            ParameterDefinition {
                id: "value".to_string(),
                name: "Value".to_string(),
                param_type: ParameterType::Object,
                description: Some("Object value to output".to_string()),
                required: true,
                multiple: false,
                default: Some(Value::Object(Map::new())),
            },
        ],
        outputs: vec![
            ParameterDefinition {
                id: "output".to_string(),
                name: "Output".to_string(),
                param_type: ParameterType::Object,
                description: Some("The resulting object value".to_string()),
                required: true,
                multiple: false,
                default: None,
            },
        ],
    }
}

/// Execute the "object" system action
pub async fn execute(
    inputs: HashMap<String, Value>
) -> Result<HashMap<String, Value>, String> {
    let mut outputs = HashMap::new();
    let value = inputs.get("value").cloned().unwrap_or(Value::Object(Map::new()));
    let obj_value = match value {
        Value::Object(o) => Value::Object(o),
        _ => Value::Object(Map::new()),
    };
    outputs.insert("output".to_string(), obj_value);
    Ok(outputs)
}