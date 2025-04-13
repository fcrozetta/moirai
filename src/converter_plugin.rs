// filepath: /Users/fcrozetta/projects/moirai/src/converter_plugin.rs
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::plugin::{ActionDefinition, Plugin, PluginRegistry};
use crate::models::ParameterDefinition;
use crate::models::ParameterType;

// Register the converter plugin with the specific type converters
pub fn register_converter_plugin(registry: &mut PluginRegistry) -> Result<(), String> {
    let mut actions = HashMap::new();

    // Number to String Converter
    actions.insert(
        "number_to_string".to_string(),
        ActionDefinition {
            name: "number_to_string".to_string(),
            description: "Convert a number to a string".to_string(),
            entrypoint: "internal:number_to_string".to_string(),
            inputs: vec![
                ParameterDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    param_type: ParameterType::Number,
                    description: Some("The number to convert to string".to_string()),
                    required: true,
                    multiple: false,
                    default: Some(Value::Number(serde_json::Number::from(0))),
                },
                ParameterDefinition {
                    id: "format".to_string(),
                    name: "Format".to_string(),
                    param_type: ParameterType::String,
                    description: Some("Optional format specifier (not used yet)".to_string()),
                    required: false,
                    multiple: false,
                    default: None,
                },
            ],
            outputs: vec![
                ParameterDefinition {
                    id: "output".to_string(),
                    name: "Output".to_string(),
                    param_type: ParameterType::String,
                    description: Some("The string representation of the number".to_string()),
                    required: true,
                    multiple: false,
                    default: None,
                },
            ],
        },
    );
    
    // Boolean to String Converter
    actions.insert(
        "boolean_to_string".to_string(),
        ActionDefinition {
            name: "boolean_to_string".to_string(),
            description: "Convert a boolean to a string".to_string(),
            entrypoint: "internal:boolean_to_string".to_string(),
            inputs: vec![
                ParameterDefinition {
                    id: "value".to_string(),
                    name: "Value".to_string(),
                    param_type: ParameterType::Boolean,
                    description: Some("The boolean to convert to string".to_string()),
                    required: true,
                    multiple: false,
                    default: Some(Value::Bool(false)),
                },
                ParameterDefinition {
                    id: "format".to_string(),
                    name: "Format".to_string(),
                    param_type: ParameterType::String,
                    description: Some("Format: 'true/false' (default), 'yes/no', '1/0', 'on/off'".to_string()),
                    required: false,
                    multiple: false,
                    default: Some(Value::String("true/false".to_string())),
                },
            ],
            outputs: vec![
                ParameterDefinition {
                    id: "output".to_string(),
                    name: "Output".to_string(),
                    param_type: ParameterType::String,
                    description: Some("The string representation of the boolean".to_string()),
                    required: true,
                    multiple: false,
                    default: None,
                },
            ],
        },
    );
    
    // Array to String Converter
    actions.insert(
        "array_to_string".to_string(),
        ActionDefinition {
            name: "array_to_string".to_string(),
            description: "Convert an array to a string representation".to_string(),
            entrypoint: "internal:array_to_string".to_string(),
            inputs: vec![
                ParameterDefinition {
                    id: "array".to_string(),
                    name: "Array".to_string(),
                    param_type: ParameterType::Object, // JSON arrays are handled as objects in serde_json
                    description: Some("The array to convert".to_string()),
                    required: true,
                    multiple: false,
                    default: Some(Value::Array(vec![])),
                },
                ParameterDefinition {
                    id: "separator".to_string(),
                    name: "Separator".to_string(),
                    param_type: ParameterType::String,
                    description: Some("Separator between array elements (default: comma)".to_string()),
                    required: false,
                    multiple: false,
                    default: Some(Value::String(", ".to_string())),
                },
            ],
            outputs: vec![
                ParameterDefinition {
                    id: "output".to_string(),
                    name: "Output".to_string(),
                    param_type: ParameterType::String,
                    description: Some("The string representation of the array".to_string()),
                    required: true,
                    multiple: false,
                    default: None,
                },
            ],
        },
    );

    // Object to String Converter
    actions.insert(
        "object_to_string".to_string(),
        ActionDefinition {
            name: "object_to_string".to_string(),
            description: "Convert an object to a string representation".to_string(),
            entrypoint: "internal:object_to_string".to_string(),
            inputs: vec![
                ParameterDefinition {
                    id: "object".to_string(),
                    name: "Object".to_string(),
                    param_type: ParameterType::Object,
                    description: Some("The object to convert".to_string()),
                    required: true,
                    multiple: false,
                    default: Some(Value::Object(serde_json::Map::new())),
                },
                ParameterDefinition {
                    id: "pretty".to_string(),
                    name: "Pretty Print".to_string(),
                    param_type: ParameterType::Boolean,
                    description: Some("Whether to format the output with indentation (default: false)".to_string()),
                    required: false,
                    multiple: false,
                    default: Some(Value::Bool(false)),
                },
            ],
            outputs: vec![
                ParameterDefinition {
                    id: "output".to_string(),
                    name: "Output".to_string(),
                    param_type: ParameterType::String,
                    description: Some("The string representation of the object".to_string()),
                    required: true,
                    multiple: false,
                    default: None,
                },
            ],
        },
    );
    
    // Create Converter plugin
    let plugin = Plugin {
        id: "converter:converter".to_string(),
        name: "converter".to_string(),
        version: "1.0.0".to_string(),
        runtime: "internal".to_string(),
        path: PathBuf::from("."),
        actions,
    };
    registry.register_plugin(plugin)
}

/// Execute a converter call
/// 
/// This function handles the execution of converter plugin actions
pub async fn execute_converter_action(
    action: &str,
    inputs: HashMap<String, Value>,
) -> Result<HashMap<String, Value>, String> {
    match action {
        "number_to_string" => {
            let mut outputs = HashMap::new();
            
            // Extract the number value, with 0 as default
            let number = inputs.get("value").cloned().unwrap_or(Value::Number(serde_json::Number::from(0)));
            
            // Convert to string
            let string_value = match number {
                Value::Number(n) => Value::String(n.to_string()),
                _ => Value::String("0".to_string()), // This shouldn't happen due to parameter type constraints
            };
            
            outputs.insert("output".to_string(), string_value);
            Ok(outputs)
        },
        "boolean_to_string" => {
            let mut outputs = HashMap::new();
            
            // Extract the boolean value, with false as default
            let boolean = inputs.get("value").cloned().unwrap_or(Value::Bool(false));
            
            // Get format
            let format = match inputs.get("format") {
                Some(Value::String(format)) => format.as_str(),
                _ => "true/false",
            };
            
            // Convert to string based on format
            let string_value = match boolean {
                Value::Bool(b) => {
                    match format {
                        "yes/no" => Value::String(if b { "yes".to_string() } else { "no".to_string() }),
                        "1/0" => Value::String(if b { "1".to_string() } else { "0".to_string() }),
                        "on/off" => Value::String(if b { "on".to_string() } else { "off".to_string() }),
                        _ => Value::String(if b { "true".to_string() } else { "false".to_string() }),
                    }
                },
                _ => Value::String("false".to_string()), // This shouldn't happen due to parameter type constraints
            };
            
            outputs.insert("output".to_string(), string_value);
            Ok(outputs)
        },
        "array_to_string" => {
            let mut outputs = HashMap::new();
            
            // Get array
            let array = inputs.get("array").cloned().unwrap_or(Value::Array(vec![]));
            
            // Get separator
            let separator = match inputs.get("separator") {
                Some(Value::String(s)) => s.clone(),
                _ => ", ".to_string(),
            };
            
            // Convert array to string
            let string_value = match array {
                Value::Array(elements) => {
                    let string_elements: Vec<String> = elements.iter()
                        .map(|e| match e {
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            Value::Bool(b) => b.to_string(),
                            Value::Null => "null".to_string(),
                            Value::Array(_) => "[array]".to_string(),
                            Value::Object(_) => "{object}".to_string(),
                        })
                        .collect();
                    
                    Value::String(string_elements.join(&separator))
                },
                _ => Value::String("[not an array]".to_string()),
            };
            
            outputs.insert("output".to_string(), string_value);
            Ok(outputs)
        },
        "object_to_string" => {
            let mut outputs = HashMap::new();
            
            // Get object
            let object = inputs.get("object").cloned().unwrap_or(Value::Object(serde_json::Map::new()));
            
            // Get pretty print option
            let pretty = match inputs.get("pretty") {
                Some(Value::Bool(b)) => *b,
                _ => false,
            };
            
            // Convert object to string
            let string_value = match object {
                Value::Object(obj) => {
                    if pretty {
                        match serde_json::to_string_pretty(&Value::Object(obj)) {
                            Ok(s) => Value::String(s),
                            Err(_) => Value::String("{failed to stringify}".to_string()),
                        }
                    } else {
                        match serde_json::to_string(&Value::Object(obj)) {
                            Ok(s) => Value::String(s),
                            Err(_) => Value::String("{failed to stringify}".to_string()),
                        }
                    }
                },
                _ => Value::String("{not an object}".to_string()),
            };
            
            outputs.insert("output".to_string(), string_value);
            Ok(outputs)
        },
        _ => Err(format!("Unknown converter action {}", action)),
    }
}

// Check if plugin/action is a converter node
pub fn is_converter_node(plugin: &str, action: &str) -> bool {
    plugin == "converter" && (
        action == "number_to_string" ||
        action == "boolean_to_string" ||
        action == "array_to_string" ||
        action == "object_to_string"
    )
}