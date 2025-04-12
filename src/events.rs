use tokio::sync::broadcast;
use chrono::{DateTime, Utc};
use crate::models::{NodeStatus};

// Define our event types
#[derive(Debug, Clone)]
pub enum EventType{
    JobStatusChanged{
        job_id: String,
        old_status: String,
        new_status: String,
        timestamp: DateTime<Utc>,
    },
    NodeStatusChanged {
        job_id: String,
        node_id: String,
        old_status: NodeStatus,
        new_status: NodeStatus,
        timestamp: DateTime<Utc>,
    },
    LogEmitted {
        job_id: String,
        node_id: String,
        level: String,
        message: String,
        timestamp: DateTime<Utc>,
    }
}

pub struct EventBus {
    sender: broadcast::Sender<EventType>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        EventBus { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<EventType> {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: EventType) -> Result<usize, broadcast::error::SendError<EventType>> {
        self.sender.send(event)
    }
}