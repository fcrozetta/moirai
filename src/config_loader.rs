use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: String,

    pub engine: EngineConfig,

    pub plugins: HashMap<String,PluginSource>,
}

#[derive(Debug, Deserialize)]
pub struct EngineConfig {
    pub engine_version: String,
    pub workflow: Option<String>,
    pub concurrency: Option<usize>,
    pub timeout_seconds: Option<u64>,
}

#[derive(Debug,Deserialize)]
pub struct PluginSource {
    pub path: Option<String>,
    pub git: Option<String>,
    pub rev: Option<String>,
}


// Configuration errors
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse config TOML: {0}")]
    Toml(#[from] toml::de::Error)
}

/// Load and parse moirai config file from the given TOML file path
/// 
/// # Errors
/// 
/// - Returns `ConfigError::Io` if the file cannot be read
/// - Returns `ConfigError::Toml` if parsing fails
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config,ConfigError> {
    let toml_str = fs::read_to_string(path)?;
    let cfg = toml::from_str(&toml_str)?;
    Ok(cfg)
}

