use std::collections::HashMap;
use serde_json::Value;
use crate::plugin::ActionDefinition;
use crate::models::{ParameterDefinition, ParameterType};

/// Definition for the "boolean" system node
pub fn definition() -> ActionDefinition {
    ActionDefinition {
        name: "boolean".to_string(),
        description: "Create or manipulate a boolean value".to_string(),
        entrypoint: "internal:boolean".to_string(),
        inputs: vec![
            ParameterDefinition {
                id: "value".to_string(),
                name: "Value".to_string(),
                param_type: ParameterType::Boolean,
                description: Some("Boolean value to output".to_string()),
                required: true,
                multiple: false,
                default: Some(Value::Bool(false)),
            },
        ],
        outputs: vec![
            ParameterDefinition {
                id: "output".to_string(),
                name: "Output".to_string(),
                param_type: ParameterType::Boolean,
                description: Some("The resulting boolean value".to_string()),
                required: true,
                multiple: false,
                default: None,
            },
        ],
    }
}

/// Execute the "boolean" system action
pub async fn execute(
    inputs: HashMap<String, Value>
) -> Result<HashMap<String, Value>, String> {
    let mut outputs = HashMap::new();
    let value = inputs.get("value").cloned().unwrap_or(Value::Bool(false));

    let bool_value = match value {
        Value::Bool(b) => Value::Bool(b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Bool(i != 0)
            } else if let Some(f) = n.as_f64() {
                Value::Bool(f != 0.0)
            } else {
                Value::Bool(false)
            }
        }
        Value::String(s) => Value::Bool(s.to_lowercase() == "true" || !s.is_empty()),
        Value::Null => Value::Bool(false),
        Value::Array(a) => Value::Bool(!a.is_empty()),
        Value::Object(o) => Value::Bool(!o.is_empty()),
    };

    outputs.insert("output".to_string(), bool_value);
    Ok(outputs)
}