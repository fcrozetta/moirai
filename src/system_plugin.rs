use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::plugin::{ActionDefinition, Plugin, PluginRegistry};

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
            entrypoint: "internal::end".to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
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
        _ => Err(format!("Unknown system action {}", action)),
    }
}

// Check if plugin/action is a system node
//? Why is this needed
pub fn is_system_node(plugin: &str, action: &str) -> bool {
    plugin == "system" && (action == "start" || action == "end")
}

// Check if node is start node
pub fn is_start_node(plugin: &str, action: &str) -> bool {
    plugin == "system" && action =="start"
}

// Check if node is end node
pub fn is_end_node(plugin: &str, action: &str) -> bool {
    plugin == "system" && action =="end"
}