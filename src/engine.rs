use crate::{
    config_loader::{load_config, Config, ConfigError},
    plugin_manager::{PluginError, PluginManager},
    specs::{EdgeDefinition, NodeInstance},
    workflow_validator::{validate_workflow_file, WorkflowDefinition, WorkflowError},
};

use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

pub struct Engine {
    config: Arc<Config>,
    plugins: Arc<Mutex<PluginManager>>,

    queue: Arc<Mutex<Vec<String>>>, // Queue of pending workflow paths
    workers: Vec<JoinHandle<()>>,

    // Channel for emitting events back to the user
    // TODO: Learn How this works
    event_tx: mpsc::UnboundedSender<EngineEvent>,

    // keep cancellation channels per workflow
    // TODO: Learn this as well
    cancels: Arc<Mutex<HashMap<String, oneshot::Sender<()>>>>,
}

impl Engine {
    /// Creates the engine, load config and plugins
    /// Do *not* start workers yet
    pub fn new(
        config_path: impl AsRef<Path>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<EngineEvent>), EngineError> {

        // load COnfig
        let config = load_config(config_path)?;
        // load plugins
        let mut pm = PluginManager::new();
        // TODO: I added the unwrap... pretty sure this is wrong
        pm.load_from_paths(&config.plugins)?;

        // create event channel
        let (tx,rx) = mpsc::unbounded_channel();
        Ok((
            Engine {
                config: Arc::new(config),
                plugins: Arc::new(Mutex::new(pm)),
                queue: Arc::new(Mutex::new(Vec::new())),
                workers: Vec::new(),
                event_tx: tx,
                cancels:Arc::new(Mutex::new(Default::default())),
            },
            rx,
        ))
    }

    /// Start a fixed set of worker threads that will process the queue
    pub fn start(&mut self, worker_count:usize) {
        for _ in 0..worker_count {
            let queue = Arc::clone(&self.queue);
            let plugins = Arc::clone(&self.plugins);
            let config = Arc::clone(&self.config);
            let event_tx = self.event_tx.clone();
            let cancels = Arc::clone(&self.cancels);

            let handle = thread::spawn(move || {
                loop {
                    // TODO: What is going on here?
                    let workflow_path = {
                        let mut q = queue.lock().unwrap();
                        q.pop()
                    };
                    if let Some(path) = workflow_path {
                        // TODO: This should be an UUID
                        let wf_id = path.clone();
                        
                        // Create cancel channel
                        let (cancel_tx, mut cancel_rx) = oneshot::channel();
                        cancels.lock().unwrap().insert(wf_id.clone(), cancel_tx);

                        let _ = event_tx.send(EngineEvent::WorkflowStarted { workflow_id: wf_id.clone() });

                        // Check cancel before validation
                        // TODO: I modified this to pass the compilations. May break on execution
                        if let Ok(_) = cancel_rx.try_recv() {
                            let _ = event_tx.send(EngineEvent::WorkflowFinished { workflow_id: wf_id.clone(), is_success: false });
                            continue;
                        };

                        // validate
                        let plugins_lock = plugins.lock().unwrap();
                        match validate_workflow_file(&Path::new(&path), &*config, &*plugins_lock) {
                            Err(e) => {
                                let _ = event_tx.send(EngineEvent::WorkflowFinished { workflow_id: wf_id.clone(), is_success: false });
                                // TODO: Improve logs when workflow fail validation
                                continue;
                            },

                            Ok(definition) => {
                                // TODO: Proceed with execution
                                // Before each node execution, check for cancellation
                                
                                
                            }
                        }
                    } else {
                        thread::sleep(std::time::Duration::from_secs(5));
                    };
                }
            });
            self.workers.push(handle);
        }
    }

    /// Request cancellation of a running workflow
    /// Return true if workflow was found/cancellation sent
    pub fn cancel_workflow(&self, workflow_id: &str) -> bool {
        let mut cancels = self.cancels.lock().unwrap();
        if let Some(tx) = cancels.remove(workflow_id) {
            let _ = tx.send(());
            true
        }else{
            false
        }
    }

    pub fn shutdown(&mut self) {
        let keys: Vec<_> = self.cancels.lock().unwrap().keys().cloned().collect();
        for wf in keys {
            self.cancel_workflow(&wf);
        }

        for handle in self.workers.drain(..) {
            let _ = handle.join();
        }

        let _ = self.event_tx.send(EngineEvent::EngineMessage { level: "info".into(), message: "Engine Shutdown complete".into() });

    }

}

/// Handle to control a running workflow
pub struct WorkflowHandle {
    // TODO: Figure it out what is this XD
    cancel_tx: oneshot::Sender<()>,
}

/// Events emitted by moirai (engine level and node-level)
#[derive(Debug)]
pub enum EngineEvent {
    // Engine lifecycle
    EngineMessage {
        level: String,
        message: String,
    },
    PluginsReloaded,

    // Workflow lifecycle
    WorkflowQueued {
        workflow_path: String,
    },
    WorkflowStarted {
        workflow_id: String,
    },
    WorkflowFinished {
        workflow_id: String,
        is_success: bool,
    },

    // Node level events (just proxying it thru)
    // TODO: Review this later
    NodeLog {
        workflow_id: String,
        node_id: String,
        level: String,
        message: String,
    },
    NodeProgress {
        workflow_id: String,
        node_id: String,
        percentage: usize,
    },
    NodeOutput {
        workflow_id: String,
        node_id: String,
    },
    NodeError {
        workflow_id: String,
        node_id: String,
        error: String,
    },
}

#[derive(Debug, Error)]
pub enum EngineError {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Plugin(#[from] PluginError),

    #[error(transparent)]
    Workflow(#[from] WorkflowError),
}