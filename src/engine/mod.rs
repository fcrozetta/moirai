mod job_manager;
mod executor;
mod dependency;

pub use job_manager::{JobManager, JobStatus};
pub use executor::NodeExecutor;

use crate::events::EventBus;
use crate::plugin::PluginRegistry;
use std::sync::Arc;
use tokio::sync::RwLock;

// Main engine to coordinate jobs
pub struct Engine {
    job_manager: Arc<JobManager>,
    node_executor: Arc<NodeExecutor>,
    plugin_registry: Arc<RwLock<PluginRegistry>>,
    event_bus: Arc<EventBus>
}

impl Engine {
    pub fn new(
        plugin_registry: Arc<RwLock<PluginRegistry>>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        let node_executor = Arc::new(NodeExecutor::new(event_bus.clone(), plugin_registry.clone()));
        let job_manager = Arc::new(JobManager::new(
            event_bus.clone(),
            plugin_registry.clone(),
            node_executor.clone(),
        ));

        Self{
            job_manager,
            node_executor,
            plugin_registry,
            event_bus,
        }
    }

    //submit a job for execution
    pub async fn submit_job(&self, job:crate::models::Job) -> Result<String,String> {
        self.job_manager.submit_job(job).await
    }

    // Get status of a job
    pub async fn get_job_status(&self, job_id: &str) -> Option<JobStatus> {
        self.job_manager.get_job_status(job_id).await
    }

    // Cancel a job
    pub async fn cancel_job(&self, job_id: &str) -> Result<(), String> {
        self.job_manager.cancel_job(job_id).await
    }
}