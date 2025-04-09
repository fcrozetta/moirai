use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::hash_map;
use pyo3::exceptions::socket::timeout;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Object,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParameterDefinition {
    pub id: String,
    pub name: String,
    pub param_type: ParameterType,
    pub description: Option<String>,
    pub required: bool,
    pub multiple: bool,
    pub default: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Job {
    pub job_id: String,
    pub name: String,
    pub description: String,
    pub priority: String,
    pub time: u32,
    pub retries: u32,
    pub callback_url: String,
    pub parameters:Value,
    pub nodes:Vec<Node>,
    pub edges:Vec<Edge>,
    pub metadata:Metadata,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub action:String, //name of the action in plugin or in moirai
    pub plugin:String, //Plugin where it comes from
    pub version:String,
    pub inputs:Vec<ParameterDefinition>,
    pub outputs:Vec<ParameterDefinition>,
    pub timeout: u32,
    pub retry_policy: RetryPolicy,
    pub condition:Option<String>,


}

#[derive(Debug, Deserialize, Serialize)]
pub struct Edge {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEntry {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RetryPolicy {}


#[test]
fn test_deserialize_job(){
    let job = r#"
    {
            "job_id": "job-001",
            "name": "Hello World Workflow",
            "description": "A demo workflow connecting node outputs to inputs with detailed execution metadata.",
            "priority": "high",
            "timeout": 120,
            "retries": 3,
            "callback_url": "http://example.com/notify",
            "status": "queued",
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
    assert_eq!(job.job_id, "job001".to_string());
    assert_eq!(job.name, "Hello World Workflow".to_string());
}