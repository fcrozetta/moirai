use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::plugin::{Plugin, PluginRegistry};

// Delegate to individual node modules
mod start;
mod end;
mod string;
mod number;
mod boolean;
mod object;
mod log;

// register the built-in system plugin
pub fn register_system_plugin(registry: &mut PluginRegistry) -> Result<(), String> {
    let mut actions = HashMap::new();
    for def in vec![
        start::definition(),
        end::definition(),
        string::definition(),
        number::definition(),
        boolean::definition(),
        object::definition(),
        log::definition(),
    ] {
        actions.insert(def.name.clone(), def);
    }
    let plugin = Plugin {
        id: "system:system".to_string(),
        name: "system".to_string(),
        version: "1.0.0".to_string(),
        runtime: "internal".to_string(),
        path: PathBuf::from("."),
        actions,
    };
    registry.register_plugin(plugin)
}

/// Execute a system call by delegating to node modules
pub async fn execute_system_action(
    action: &str,
    inputs: HashMap<String, Value>,
) -> Result<HashMap<String, Value>, String> {
    match action {
        "start" => start::execute(inputs).await,
        "end" => end::execute(inputs).await,
        "string" => string::execute(inputs).await,
        "number" => number::execute(inputs).await,
        "boolean" => boolean::execute(inputs).await,
        "object" => object::execute(inputs).await,
        "log" => log::execute(inputs).await,
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
        action == "log"
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
        action == "object"
    )
}

// Check if node is a log node
pub fn is_log_node(plugin: &str, action: &str) -> bool {
    plugin == "system" && action == "log"
}