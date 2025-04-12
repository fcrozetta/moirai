use chrono::Utc;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc; // Import Arc for shared ownership
use tokio::sync::RwLock; // Replaced std::sync::RwLock with tokio::sync::RwLock

use crate::events::{EventBus, EventType};
use crate::models::{Node};
use crate::plugin::{PluginExecutor, PluginRegistry};
use crate::system_plugin;

// Handle individal nodes
pub struct NodeExecutor {
    event_bus: Arc<EventBus>,
    plugin_registry: Arc<RwLock<PluginRegistry>>,
    plugin_executor: PluginExecutor,
}

impl NodeExecutor {
    pub fn new(event_bus: Arc<EventBus>, plugin_registry: Arc<RwLock<PluginRegistry>>) -> Self {
        Self {
            event_bus,
            plugin_registry,
            plugin_executor: PluginExecutor::new(),
        }
    }

    // Execute a node with given inputs
    pub async fn execute_node(
        &self,
        job_id: &str,
        node: &Node,
        inputs: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, String> {
        // Log the start of node process
        let _ = self.event_bus.publish(EventType::LogEmitted {
            job_id: job_id.to_string(),
            node_id: node.id.to_string(),
            level: "info".to_string(),
            message: format!("Executing node: {}", node.name),
            timestamp: Utc::now(),
        });

        // special cases for system nodes
        //? I believe this should be handled differently
        if system_plugin::is_system_node(&node.plugin, &node.action) {
            return system_plugin::execute_system_action(&node.action, inputs).await;
        }

        let registry = self.plugin_registry.read().await; // Changed to async read
        let plugins = registry.find_plugins(&node.plugin);

        if plugins.is_empty() {
            return Err(format!("Plugin not found: {}", node.plugin));
        }

        let plugin = plugins
            .iter()
            .find(|p| p.version == node.version)
            .or_else(|| plugins.first())
            .ok_or_else(|| format!("Plugin not found: {}", node.plugin))?;

        let timeout_secs = node.timeout;

        let result = self
            .plugin_executor
            .execute_action(
                plugin, 
                &node.action, inputs, 
                timeout_secs
            ).await;

        match &result {
            Ok(outputs) => {
                let _ = self.event_bus.publish(EventType::LogEmitted { 
                    job_id: job_id.to_string(), 
                    node_id: node.id.clone(), 
                    level: "info".to_string(), message: format!("Node executed successfully with {} outputs", outputs.len()), 
                    timestamp: Utc::now() 
                });
            },
            Err(e) => {
                let _ = self.event_bus.publish(EventType::LogEmitted { 
                    job_id: job_id.to_string(), 
                    node_id: node.id.clone(), 
                    level: "error".to_string(), message: format!("Node executed failed {}", e), 
                    timestamp: Utc::now() 
                });
            }
        }
        result
    }
}
