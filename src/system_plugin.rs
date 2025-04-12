use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::plugin::{ActionDefinition, Plugin, PluginRegistry};
use crate::models::ParameterDefinition;
use crate::models::ParameterType;

// register the built-in system plugin
pub fn register_system_plugin(registry: &mut PluginRegistry) -> Result<(), String> {
    let mut actions = HashMap::new();

    // Start Node
    actions.insert(
        "start".to_string(),
        ActionDefinition {
            name: "start".to_string(),
            description: "Workflow entrypoint".to_string(),
            entrypoint: "internal:start".to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        },
    );

    // End Node
    actions.insert(
        "end".to_string(),
        ActionDefinition {
            name: "end".to_string(),
            description: "Workfow end node".to_string(),
            entrypoint: "internal:end".to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        },
    );

    // String Primitive
    actions.insert(
        "string".to_string(),
        ActionDefinition {
            name: "string".to_string(),
            description: "Create or manipulate a string value".to_string(),
            entrypoint: "internal:string".to_string(),
            inputs: vec![
                ParameterDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    param_type: ParameterType::String,
                    description: Some("String value to output".to_string()),
                    required: true,
                    multiple: false,
                    default: Some(Value::String("".to_string())),
                },
            ],
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
        },
    );

    // Number Primitive
    actions.insert(
        "number".to_string(),
        ActionDefinition {
            name: "number".to_string(),
            description: "Create or manipulate a numeric value".to_string(),
            entrypoint: "internal:number".to_string(),
            inputs: vec![
                ParameterDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    param_type: ParameterType::Number,
                    description: Some("Numeric value to output".to_string()),
                    required: true,
                    multiple: false,
                    default: Some(Value::Number(serde_json::Number::from(0))),
                },
            ],
            outputs: vec![
                ParameterDefinition {
                    id: "output".to_string(),
                    name: "Output".to_string(),
                    param_type: ParameterType::Number,
                    description: Some("The resulting numeric value".to_string()),
                    required: true,
                    multiple: false,
                    default: None,
                },
            ],
        },
    );

    // Boolean Primitive
    actions.insert(
        "boolean".to_string(),
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
        },
    );

    // Object Primitive
    actions.insert(
        "object".to_string(),
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
                    default: Some(Value::Object(serde_json::Map::new())),
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
        },
    );

    // Null Primitive
    actions.insert(
        "null".to_string(),
        ActionDefinition {
            name: "null".to_string(),
            description: "Create a null value".to_string(),
            entrypoint: "internal:null".to_string(),
            inputs: vec![],
            outputs: vec![
                ParameterDefinition {
                    id: "output".to_string(),
                    name: "Output".to_string(),
                    param_type: ParameterType::Object,
                    description: Some("A null value".to_string()),
                    required: true,
                    multiple: false,
                    default: None,
                },
            ],
        },
    );

    // Create System plugin
    let plugin = Plugin {
        id:"system:system".to_string(),
        name: "system".to_string(),
        version:"1.0.0".to_string(),
        runtime: "internal".to_string(),
        path: PathBuf::from("."),
        actions,
    };
    registry.register_plugin(plugin)
}

/// Execute a system call
/// 
/// This function handles the execution of system plugin actions,
/// which are implemented in-process rather than an external command
pub async fn execute_system_action(
    action: &str,
    inputs: HashMap<String, Value>,
) -> Result<HashMap<String, Value>, String> {
    match action {
        "start" => {
            Ok(inputs)
        },
        "end" => {
            Ok(inputs)
        },
        "string" => {
            let mut outputs = HashMap::new();
            let value = inputs.get("value").cloned().unwrap_or(Value::String("".to_string()));
            
            // Convert to string if not already a string
            let string_value = match value {
                Value::String(s) => Value::String(s),
                Value::Number(n) => Value::String(n.to_string()),
                Value::Bool(b) => Value::String(b.to_string()),
                Value::Null => Value::String("null".to_string()),
                Value::Array(_) => Value::String("[array]".to_string()),
                Value::Object(_) => Value::String("{object}".to_string()),
            };
            
            outputs.insert("output".to_string(), string_value);
            Ok(outputs)
        },
        "number" => {
            let mut outputs = HashMap::new();
            let value = inputs.get("value").cloned().unwrap_or(Value::Number(serde_json::Number::from(0)));
            
            // Ensure the value is a number
            let number_value = match value {
                Value::Number(n) => Value::Number(n),
                Value::String(s) => {
                    // Try to parse as number
                    if let Ok(i) = s.parse::<i64>() {
                        Value::Number(serde_json::Number::from(i))
                    } else if let Ok(f) = s.parse::<f64>() {
                        // This might lose precision, but it's the best we can do
                        serde_json::Number::from_f64(f)
                            .map(Value::Number)
                            .unwrap_or(Value::Number(serde_json::Number::from(0)))
                    } else {
                        Value::Number(serde_json::Number::from(0))
                    }
                },
                Value::Bool(b) => Value::Number(serde_json::Number::from(if b { 1 } else { 0 })),
                _ => Value::Number(serde_json::Number::from(0)),
            };
            
            outputs.insert("output".to_string(), number_value);
            Ok(outputs)
        },
        "boolean" => {
            let mut outputs = HashMap::new();
            let value = inputs.get("value").cloned().unwrap_or(Value::Bool(false));
            
            // Ensure the value is a boolean
            let bool_value = match value {
                Value::Bool(b) => Value::Bool(b),
                Value::Number(n) => {
                    // Check if the number is non-zero
                    if let Some(i) = n.as_i64() {
                        Value::Bool(i != 0)
                    } else if let Some(f) = n.as_f64() {
                        Value::Bool(f != 0.0)
                    } else {
                        Value::Bool(false)
                    }
                },
                Value::String(s) => {
                    // "true"/"false" or non-empty check
                    Value::Bool(s.to_lowercase() == "true" || !s.is_empty())
                },
                Value::Null => Value::Bool(false),
                Value::Array(a) => Value::Bool(!a.is_empty()),
                Value::Object(o) => Value::Bool(!o.is_empty()),
            };
            
            outputs.insert("output".to_string(), bool_value);
            Ok(outputs)
        },
        "object" => {
            let mut outputs = HashMap::new();
            let value = inputs.get("value").cloned().unwrap_or(Value::Object(serde_json::Map::new()));
            
            // If it's already an object, use it; otherwise create an empty object
            let obj_value = match value {
                Value::Object(o) => Value::Object(o),
                _ => Value::Object(serde_json::Map::new()),
            };
            
            outputs.insert("output".to_string(), obj_value);
            Ok(outputs)
        },
        "null" => {
            let mut outputs = HashMap::new();
            outputs.insert("output".to_string(), Value::Null);
            Ok(outputs)
        },
        _ => Err(format!("Unknown system action {}", action)),
    }
}

// Check if plugin/action is a system node
pub fn is_system_node(plugin: &str, action: &str) -> bool {
    plugin == "system" && (
        action == "start" || 
        action == "end" ||
        action == "string" ||
        action == "number" ||
        action == "boolean" ||
        action == "object" ||
        action == "null"
    )
}

// Check if node is start node
pub fn is_start_node(plugin: &str, action: &str) -> bool {
    plugin == "system" && action == "start"
}

// Check if node is end node
pub fn is_end_node(plugin: &str, action: &str) -> bool {
    plugin == "system" && action == "end"
}

// Check if node is a primitive
pub fn is_primitive_node(plugin: &str, action: &str) -> bool {
    plugin == "system" && (
        action == "string" ||
        action == "number" ||
        action == "boolean" ||
        action == "object" ||
        action == "null"
    )
}