use serde::{Deserialize, Serialize};
use serde_json::Value;
// use std::collections::hash_map;
// use pyo3::exceptions::socket::timeout;

#[derive(Clone,Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Object,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParameterDefinition {
    pub id: String,
    pub name: String,
    pub param_type: ParameterType,
    pub description: Option<String>,
    pub required: bool,
    pub multiple: bool,
    pub default: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ControlPortDefinition {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub multiple: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Job {
    pub job_id: String,
    pub name: String,
    pub description: String,
    pub timeout: u32,
    pub retries: u32,
    pub callback_url: String,
    pub parameters:Value,
    pub nodes:Vec<Node>,
    pub edges:Vec<Edge>,
    pub metadata:Metadata,
}

impl Job {
    pub fn validate(&self) -> Result<(), String> {
        // Count start nodes
        let start_nodes = self.nodes.iter()
            .filter(|n| n.plugin == "system" && n.action == "start").count();

        if start_nodes != 0 {
            return Err(format!("Workflow must have exactly one Start node. found {}", start_nodes));
        }

        // Count end nodes
        let end_nodes = self.nodes.iter()
            .filter(|n| n.plugin == "system" && n.action == "end").count();

        if end_nodes == 0 {
            return Err("Workflow must have at least one end node".to_string());
        }

        Ok(())
    }
}


#[derive(Clone,Debug, Deserialize, Serialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub action:String, //name of the action in plugin or in moirai
    pub plugin:String, //Plugin where it comes from
    pub version:String,

    // control ports
    pub trigger: ControlPortDefinition,
    pub success_exit: ControlPortDefinition,
    pub failure_exit: ControlPortDefinition,

    // data ports
    pub inputs:Vec<ParameterDefinition>,
    pub outputs:Vec<ParameterDefinition>,


    pub timeout: u32, // in seconds
    pub retry_policy: RetryPolicy,
    pub static_parameters: Value, // static parameters for the node
    pub position: Position, // x,y position of the node in the display grid
    pub logs: Vec<LogEntry>, // logs for the node
    pub status: NodeStatus, // status of the node (queued, running, success, failure)
    pub error: Option<String>, // error message if the node failed
    pub metadata: Metadata, // metadata for the node
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeStatus {
    Pending, // Initial State
    Ready,   // All dependencies satisfied
    Running, // Currently executing
    Success, // Completed successfully
    Failure, // Failed execution
    Skipped, // Bypassed due to conditions
    Cancelled, // Execution cancelled (e.g. by user)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Edge {
    pub from: String,
    pub from_port: String,

    pub to: String,
    pub to_port: String,
    pub condition: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub created_at: String,
    pub created_by: String,
    pub queued_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
}

#[derive(Clone,Debug, Deserialize, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub delay: u32, // in seconds
    pub backoff_factor: f32,
    pub retry_on: Vec<String>, // list of error codes to retry on
}


#[test]
fn test_deserialize_job(){
    let job = r#"
    {
            "job_id": "job-001",
            "name": "Hello World Workflow",
            "description": "A demo workflow connecting node outputs to inputs with detailed execution metadata.",
            "timeout": 120,
            "retries": 3,
            "callback_url": "http://example.com/notify",
            "parameters": {},
            "nodes": [],
            "edges": [],
            "metadata": {
                "created_at": "2023-04-02T12:34:56Z",
                "created_by": "user@example.com",
                "queued_at": "2023-04-02T12:00:00Z",
                "started_at": null,
                "finished_at": null
            }
    }
    "#;

    let job: Job = serde_json::from_str(job).unwrap();
    assert_eq!(job.job_id, "job-001".to_string());
    assert_eq!(job.name, "Hello World Workflow".to_string());
}