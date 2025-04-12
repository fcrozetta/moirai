use std::collections::HashMap;
use std::path::{Path,PathBuf};
use std::process::Stdio;
use serde::{Deserialize,Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::models::ParameterDefinition;
use serde_json::Value;

use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

//Source types for plugins
#[derive(Debug,Clone)]
pub enum SourceType {
    Git {
        url: String,
        branch: Option<String>,
    },
    LocalPath {
        path: PathBuf,
    },
}

// Plugin source definition
#[derive(Debug,Clone)]
pub struct PluginSource {
    pub id: String,
    pub source_type: SourceType,
    pub alias_prefix: Option<String>,
}

// Action definition from manifest
#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct ActionDefinition {
    pub name: String,
    pub description: String,
    pub entrypoint: String,
    pub inputs: Vec<ParameterDefinition>,
    pub outputs: Vec<ParameterDefinition>,
}

// Plugin manifest
#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub runtime: String,
    pub actions: Vec<ActionDefinition>,
}

// Plugin representation
#[derive(Debug,Clone)]
pub struct Plugin {
    pub id: String,                                 // Unique Plugin ID (source_id:name)
    pub name: String,                               // Original plugin name 
    pub version: String,                            // Plugin version
    pub runtime: String,                            // Runtime (python, node, etc...)
    pub path: PathBuf,                              // Path to plugin
    pub actions: HashMap<String,ActionDefinition>   // Available actions
}

// Plugin registry
pub struct PluginRegistry {
    plugins_by_id: HashMap<String,Plugin>,
    plugins_by_name: HashMap<String,Vec<String>>, // name -> vec of plugin_ids
    sources: Vec<PluginSource>,
}

impl PluginRegistry {
    pub fn new() -> PluginRegistry {
        PluginRegistry {
            plugins_by_id: HashMap::new(),
            plugins_by_name: HashMap::new(),
            sources: Vec::new()
        }
    }

    // Add sources
    pub fn add_source(&mut self, source: PluginSource) -> Result<(),String> {
        // Check for duplicate source IDs
        if self.sources.iter().any(|s| s.id == source.id) {
            return Err(format!("Source with ID '{}' already exists", source.id));
        }
        self.sources.push(source);
        Ok(())
    }

    // Register plugin
    pub fn register_plugin(&mut self, plugin: Plugin) -> Result<(), String> {
        if self.plugins_by_id.contains_key(&plugin.id) {
            return Err(format!("Plugin with ID {} already exists", plugin.id));
        }

        // Add to ID map
        self.plugins_by_id.insert(plugin.id.clone(), plugin.clone());

        let entries = self.plugins_by_name
            .entry(plugin.name.clone())
            .or_insert_with(Vec::new);
        entries.push(plugin.id.clone());

        Ok(())
    }

    // Load plugins from all sources
    pub fn load_plugins(&mut self) -> Result<(),String> {
        for source in &self.sources.clone() {
            self.load_plugins_from_source(source)?;
        }
        Ok(())
        
    }

    // Load plugins from specific source
    fn load_plugins_from_source(&mut self, source: &PluginSource) -> Result<(), String> {
        let base_path = match &source.source_type {
            SourceType::LocalPath { path } => {
                if !path.exists() {
                    return Err(format!("Plugin does not exist: {:?}", path));
                }
                path.clone()
            },
            SourceType::Git { url, branch: _ } => {
                let repo_name = url.split('/').last().unwrap_or("repo")
                    .trim_end_matches(".git");
                PathBuf::from("./plugins").join(repo_name)
            }
        };
        
        // Find all manifest files
        self.discover_plugins(&base_path, source)
    }

    fn discover_plugins(&mut self, base_path:&Path, source: &PluginSource) -> Result<(), String> {
        //? AI says I should look reursively in this. Maybe later
        let manifest_path = base_path.join("manifest.yaml");

        // TODO: refactor this block of code
        if manifest_path.exists() {
            self.load_plugin(&manifest_path, source)?;
        }else {
            // JSON as alternative
            let manifest_path = base_path.join("manifest.json");
            if manifest_path.exists() {
                self.load_plugin(&manifest_path, source)?;
            }else{
                return Err(format!("No manifest found in {:?}",base_path));
            }
        }
        Ok(())
    }

    fn load_plugin(&mut self, manifest_path: &Path, source: &PluginSource) -> Result<(), String> {
        // Read file
        let content = std::fs::read_to_string(manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;

        // Parse manifest
        let manifest: PluginManifest = if manifest_path.extension().unwrap_or_default() == "json" {
            serde_json::from_str(&content)
                    .map_err(|e|format!("Failed to parse manifest JSON: {}",e))?
        } else {
            serde_yaml::from_str(&content)
                .map_err(|e| format!("Failed to parse manifest YAML: {}",e))?
                    
        };

        let plugin_name = if let Some(prefix) = &source.alias_prefix {
            format!("{}{}",prefix,manifest.name)
        } else{
            manifest.name.clone()
        };

        let plugin_id = format!("{}:{}",source.id,plugin_name);

        // Map actions by name
        let mut actions = HashMap::new();
        for action in &manifest.actions {
            actions.insert(action.name.clone(), action.clone());
        }

        // Create plugin
        let plugin = Plugin {
            id: plugin_id.clone(),
            name: manifest.name.clone(),
            version:manifest.version.clone(),
            runtime: manifest.runtime.clone(),
            path: manifest_path.parent().unwrap().to_path_buf(),
            actions,
        };

        // Register Plugin
        self.plugins_by_id.insert(plugin_id.clone(), plugin);

        // Add name to index
        let name_entries = self.plugins_by_name.entry(manifest.name.clone())
            .or_insert_with(Vec::new);
        name_entries.push(plugin_id);

        Ok(())
    }

    pub fn find_plugins(&self, name:&str) -> Vec<&Plugin> {
        match self.plugins_by_name.get(name) {
            Some(ids) =>ids.iter()
                .filter_map(|id| self.plugins_by_id.get(id))
                .collect(),
            None => Vec::new(),
        }
    }

}

// Plugin executor
pub struct PluginExecutor;

impl PluginExecutor {
    pub fn new() -> Self {
        Self
    }

    // Execute action within a plugin
    pub async fn execute_action(
        &self,
        plugin: &Plugin,
        action_name: &str, // Remember: &str is a borrowed string slice (kinda like const char*)
        inputs: HashMap<String,Value>,
        timeout_seconds: u32,
    ) -> Result<HashMap<String,Value>, String> {
        // Special case for system nodes
        if plugin.name == "system" {
            return crate::system_plugin::execute_system_action(action_name, inputs).await;
        }

        // Find action
        let action = plugin.actions.get(action_name)
            .ok_or_else(|| format!("Action '{}' not found in plugin {}", action_name, plugin.name))?;

        // Run action based on runtime
        match plugin.runtime.as_str() {
            "python" | "python3" => self.execute_python(plugin,action,inputs, timeout_seconds).await,
            "node" | "nodejs" => self.execute_node(plugin,action,inputs, timeout_seconds).await,
            "bash" | "sh" => self.execute_bash(plugin,action,inputs, timeout_seconds).await,
            _ => Err(format!("Unsupported runtime: {}", plugin.runtime)),
        }
    }

    async fn execute_python(
        &self,
        plugin: &Plugin,
        action: &ActionDefinition,
        inputs: HashMap<String,Value>,
        timeout_seconds: u32,
    ) -> Result<HashMap<String,Value>, String> {
        
        // Parse entrypoint
        let parts: Vec<&str> = action.entrypoint.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid Python entrypoint format: {}", action.entrypoint));
        }

        let module_path = parts[0];
        let function_name = parts[1];

        // Prepare input JSON
        let input_json = serde_json::to_string(&inputs)
            .map_err(|e| format!("Failed to serialize inputs: {}", e))?;

        // Build python command
        // TODO: Implement using .venv
        let mut cmd = Command::new("python");
        cmd.current_dir(&plugin.path)
            .arg("-c")
            .arg(format!(
                "import json,sys; from {} import {}; result = {} (json.loads(sys.stdin.read()));print(json.dumps(result))",
                module_path, function_name, function_name
            ))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
            
        // Execute with timeout
        let result = self.execute_command_with_timeout(cmd,input_json,timeout_seconds).await?;

        // Parse output JSON
        let outputs: HashMap<String,Value> = serde_json::from_str(&result)
            .map_err(|e| format!("Failed to parse action output: {}",e))?;

        Ok(outputs)
    }

    // Execute Nodejs action
    async fn execute_node(&self,
        plugin: &Plugin,
        action: &ActionDefinition,
        inputs: HashMap<String,Value>,
        timeout_seconds: u32,
    ) -> Result<HashMap<String,Value>, String> {
        
        // Parse entrypoint
        let parts: Vec<&str> = action.entrypoint.split(':').collect();

        if parts.len() != 2{
            return Err(format!("Invalid node entrypoint format: {}", action.entrypoint));
        }

        let module_path = parts[0];
        let function_name = parts[1];

        // Prepare json
        let input_json = serde_json::to_string(&inputs)
            .map_err(|e| format!("Failed to serialize inputs: {}", e))?;

        // build node command
        let mut cmd = Command::new("node");
        cmd.current_dir(&plugin.path)
            .arg("-e")
            .arg(format!(
                "const fs = require('fs'); const {{ {} }} = require('./{}'); process.stdin.on('data', data => {{ const input = JSON.parse(data.toString()); const result = {}(input); console.log(JSON.stringify(result)); }});",
                function_name, module_path, function_name
            ))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let result = self.execute_command_with_timeout(cmd,input_json,timeout_seconds).await?;

        let outputs: HashMap<String,Value> = serde_json::from_str(&result)
            .map_err(|e| format!("Failed to parse action output: {}",e))?;

        Ok(outputs)
    }

    async fn execute_bash(&self,
        plugin: &Plugin,
        action: &ActionDefinition,
        inputs: HashMap<String,Value>,
        timeout_seconds: u32,
    ) -> Result<HashMap<String,Value>, String> {
        
        // Parse entrypoint
        let script_path = plugin.path.join(&action.entrypoint);

        if !script_path.exists() {
            return Err(format!("Script not found: {:?}", script_path));
        }
        
        // Prepare json
        let input_json = serde_json::to_string(&inputs)
            .map_err(|e| format!("Failed to serialize inputs: {}", e))?;

        // build node command
        let mut cmd = Command::new("bash");
        cmd.current_dir(&plugin.path)
            .arg(script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let result = self.execute_command_with_timeout(cmd,input_json,timeout_seconds).await?;

        let outputs: HashMap<String,Value> = serde_json::from_str(&result)
            .map_err(|e| format!("Failed to parse action output: {}",e))?;

        Ok(outputs)
    }

    async fn execute_command_with_timeout(
        &self,
        mut cmd: Command,
        input:String,
        timeout_seconds: u32,
    ) -> Result<String,String> {
        let timeout_duration = Duration::from_secs(timeout_seconds as u64);

        let mut child = cmd.spawn()
            .map_err(|e| format!("Failed to start process: {}",e))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(input.as_bytes())
                .await.map_err(|e| format!("Failed to write to stdin: {}",e))?;
        }


        // Execute with timeout
        let status= match timeout(timeout_duration, child.wait()).await {
            Ok(Ok(status)) => status,
            Ok(Err(e)) => return Err(format!("Process error: {}",e)),
            Err(_) => {
                // Timeout happened
                let _ = child.kill();
                return Err(format!("Process timed out after {} seconds",timeout_seconds));
            }
            
        };

        if !status.success() {
            // Capture stderr
            let error = if let Some(mut stderr) = child.stderr.take() {
                let mut error = String::new();
                let _ = stderr.read_to_string(&mut error);
                if !error.is_empty() {
                    format!("Process failed: {}", error)
                } else {
                    format!("Process failed with exit code: {}", status)
                }
            } else {
                format!("Process failed with exit code: {}", status)
            };

            return Err(error);
        }

        let mut output = String::new();
        if let Some(mut stdout) = child.stdout.take() {
            stdout.read_to_string(&mut output)
                .await.map_err(|e| format!("failed to read stdout: {}", e))?;
        }

        Ok(output.trim().to_string())
    }

}