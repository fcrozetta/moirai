use std::collections::HashMap;
use serde_json::Value;
use crate::plugin::ActionDefinition;
use crate::models::{ParameterDefinition, ParameterType};

/// Definition for the "log" system node
pub fn definition() -> ActionDefinition {
    ActionDefinition {
        name: "log".to_string(),
        description: "Log one or more messages".to_string(),
        entrypoint: "internal:log".to_string(),
        inputs: vec![
            ParameterDefinition {
                id: "message".to_string(),
                name: "Message".to_string(),
                param_type: ParameterType::String,
                description: Some("Message to log (can be multiple)".to_string()),
                required: true,
                multiple: true,
                default: Some(Value::String("".to_string())),
            },
            ParameterDefinition {
                id: "level".to_string(),
                name: "Log Level".to_string(),
                param_type: ParameterType::String,
                description: Some("Log level (info, warning, error, debug)".to_string()),
                required: false,
                multiple: false,
                default: Some(Value::String("info".to_string())),
            },
        ],
        outputs: Vec::new(),
    }
}

/// Execute the "log" system action
pub async fn execute(
    inputs: HashMap<String, Value>
) -> Result<HashMap<String, Value>, String> {
    let messages = match inputs.get("message") {
        Some(Value::Array(arr)) => arr.iter()
            .map(|m| match m {
                Value::String(s) => s.clone(),
                other => format!("{:?}", other),
            })
            .collect::<Vec<_>>()
            .join("\n"),
        Some(Value::String(s)) => s.clone(),
        Some(other) => format!("{:?}", other),
        None => String::new(),
    };
    let level = match inputs.get("level") {
        Some(Value::String(s)) => s.clone(),
        _ => "info".to_string(),
    };
    match level.to_lowercase().as_str() {
        "error" => eprintln!("ERROR: {}", messages),
        "warning" | "warn" => println!("WARNING: {}", messages),
        "debug" => println!("DEBUG: {}", messages),
        _ => println!("INFO: {}", messages),
    }
    Ok(HashMap::new())
}