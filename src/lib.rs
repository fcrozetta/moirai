pub mod config_loader;
pub mod plugin_manager;
// pub mod models;
// pub mod events;
// pub mod plugin;
// pub mod engine;
// pub mod system_plugin;
// pub mod converter_plugin;

// #[cfg(test)]
// mod system_plugin_tests;
// #[cfg(test)]
// mod converter_plugin_tests;

// use pyo3::prelude::*;
// use std::sync::Arc;

// use tokio::runtime::Runtime;
// use crate::models::{Job, Node, NodeStatus, RetryPolicy, ControlPortDefinition, Position, Edge, Metadata};
// use crate::events::EventBus;
// use crate::plugin::PluginRegistry;
// use crate::engine::{Engine, JobStatus};

// /// A Python module implemented in Rust.
// #[pymodule]
// fn moirai(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     // Create Python wrapper for the Job structure
//     #[pyo3::prelude::pyclass]
//     struct PyJob {
//         inner: Job,
//     }

//     #[pymethods]
//     impl PyJob {
//         #[new]
//         fn new(
//             job_id: String, 
//             name: String, 
//             description: String,
//             timeout: u32,
//             retries: u32,
//             callback_url: String
//         ) -> Self {
//             // Create empty metadata
//             let metadata = Metadata {
//                 created_at: chrono::Utc::now().to_rfc3339(),
//                 created_by: "python-binding".to_string(),
//                 queued_at: chrono::Utc::now().to_rfc3339(),
//                 started_at: None,
//                 finished_at: None,
//             };
            
//             PyJob {
//                 inner: Job {
//                     job_id,
//                     name,
//                     description,
//                     timeout,
//                     retries, 
//                     callback_url,
//                     parameters: serde_json::json!({}),
//                     nodes: Vec::new(),
//                     edges: Vec::new(),
//                     metadata,
//                 }
//             }
//         }
        
//         // Add a node to the job
//         fn add_node(
//             &mut self, 
//             id: String, 
//             name: String, 
//             action: String, 
//             plugin: String,
//             version: String,
//             x: f32,
//             y: f32,
//             timeout: u32
//         ) -> PyResult<()> {
//             // Create default control ports
//             let trigger = ControlPortDefinition {
//                 id: "trigger".to_string(),
//                 name: "Trigger".to_string(),
//                 description: Some("Triggers the node".to_string()),
//                 required: true,
//                 multiple: false,
//             };
            
//             let success_exit = ControlPortDefinition {
//                 id: "success".to_string(),
//                 name: "Success".to_string(),
//                 description: Some("Success exit".to_string()),
//                 required: false,
//                 multiple: true,
//             };
            
//             let failure_exit = ControlPortDefinition {
//                 id: "failure".to_string(),
//                 name: "Failure".to_string(),
//                 description: Some("Failure exit".to_string()),
//                 required: false,
//                 multiple: true,
//             };
            
//             // Create node with default values
//             let node = Node {
//                 id,
//                 name,
//                 action,
//                 plugin,
//                 version,
//                 trigger,
//                 success_exit,
//                 failure_exit,
//                 inputs: Vec::new(),
//                 outputs: Vec::new(),
//                 timeout,
//                 retry_policy: RetryPolicy {
//                     max_retries: 0,
//                     delay: 0,
//                     backoff_factor: 1.0,
//                     retry_on: Vec::new(),
//                 },
//                 static_parameters: serde_json::json!({}),
//                 position: Position { x, y },
//                 logs: Vec::new(),
//                 status: NodeStatus::Pending,
//                 error: None,
//                 metadata: self.inner.metadata.clone(),
//             };
            
//             self.inner.nodes.push(node);
//             Ok(())
//         }
        
//         // Add an edge between nodes
//         fn add_edge(
//             &mut self, 
//             from_node: String, 
//             from_port: String, 
//             to_node: String, 
//             to_port: String,
//             condition: Option<String>
//         ) -> PyResult<()> {
//             // Verify nodes exist
//             let from_exists = self.inner.nodes.iter().any(|n| n.id == from_node);
//             let to_exists = self.inner.nodes.iter().any(|n| n.id == to_node);
            
//             if !from_exists || !to_exists {
//                 return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
//                     format!("Node '{}' or '{}' not found", from_node, to_node)
//                 ));
//             }
            
//             let edge = Edge {
//                 from: from_node,
//                 from_port,
//                 to: to_node,
//                 to_port,
//                 condition,
//             };
            
//             self.inner.edges.push(edge);
//             Ok(())
//         }
        
//         // Validate the job structure
//         fn validate(&self) -> PyResult<bool> {
//             match self.inner.validate() {
//                 Ok(_) => Ok(true),
//                 Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e)),
//             }
//         }
        
//         // Get the raw job JSON as a string for debugging
//         fn to_json(&self) -> PyResult<String> {
//             match serde_json::to_string_pretty(&self.inner) {
//                 Ok(json) => Ok(json),
//                 Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
//             }
//         }
//     }
    
//     // Create Python wrapper for the Engine
//     #[pyo3::prelude::pyclass]
//     struct PyEngine {
//         engine: Engine,
//         runtime: Runtime,
//     }
    
//     #[pymethods]
//     impl PyEngine {
//         #[new]
//         fn new() -> PyResult<Self> {
//             // Create Tokio runtime for async operations
//             let runtime = match Runtime::new() {
//                 Ok(rt) => rt,
//                 Err(e) => return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
//                     format!("Failed to create Tokio runtime: {}", e)
//                 )),
//             };
            
//             // Create EventBus, PluginRegistry and Engine in runtime context
//             let event_bus = Arc::new(EventBus::new(1000)); // Use capacity of 1000 for events
//             let plugin_registry = Arc::new(tokio::sync::RwLock::new(PluginRegistry::new()));
            
//             let engine = Engine::new(plugin_registry, event_bus);
            
//             Ok(PyEngine { engine, runtime })
//         }
        
//         // Submit a job for execution
//         fn submit_job(&self, job: &PyJob) -> PyResult<String> {
//             let job_inner = job.inner.clone();
            
//             // Run in tokio runtime context
//             match self.runtime.block_on(self.engine.submit_job(job_inner)) {
//                 Ok(job_id) => Ok(job_id),
//                 Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e)),
//             }
//         }
        
//         // Get job status
//         fn get_job_status(&self, job_id: &str) -> PyResult<Option<String>> {
//             // Run in tokio runtime context
//             let status = self.runtime.block_on(self.engine.get_job_status(job_id));
            
//             Ok(status.map(|s| match s {
//                 JobStatus::Created => "created".to_string(),
//                 JobStatus::Queued => "queued".to_string(),
//                 JobStatus::Running => "running".to_string(),
//                 JobStatus::Success => "success".to_string(),
//                 JobStatus::Failure => "failure".to_string(),
//                 JobStatus::Cancelled => "cancelled".to_string(),
//             }))
//         }
        
//         // Cancel a job
//         fn cancel_job(&self, job_id: &str) -> PyResult<()> {
//             // Run in tokio runtime context
//             match self.runtime.block_on(self.engine.cancel_job(job_id)) {
//                 Ok(_) => Ok(()),
//                 Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e)),
//             }
//         }
//     }
    
//     // Register classes with module
//     m.add_class::<PyJob>()?;
//     m.add_class::<PyEngine>()?;
    
//     Ok(())
// }
