use crate::specs::{DataType, NodeSpec};
use crate::config_loader::PluginSource;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

/// Errors that can occur while loading plugins.
#[derive(Debug, Error)]
pub enum PluginError {
    #[error("I/O error loading plugin: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse plugin manifest JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Plugin FQDN mismatch: expected `{expected}`, found `{found}`")]
    FqdnMismatch { expected: String, found: String },
}


// Representation of plugin manifest (plugin.json)
#[derive(Debug, Deserialize)]
pub struct PluginManifest {
    // Unique namespace
    pub plugin_fqdn: String,
    pub version: String,
    // Author is optional
    pub author: Option<String>,
    // Created timestamp is optional
    pub created: Option<String>,

    // Plugin metadata
    #[serde(default)]
    pub metadata: Option<PluginMetadata>,

    // Relative paths to type definitions
    pub types: Vec<PathBuf>,
    pub nodes: Vec<PathBuf>,
}

// Additional metadata fields
#[derive(Debug, Deserialize)]
pub struct PluginMetadata {
    pub description: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
}

/// In-memmory representation of a loaded plugin
pub struct Plugin {
    /// Manifest data
    pub manifest: PluginManifest,

    /// Absolute paths to loaded type definition files
    pub types: HashMap<String, DataType>,
    ///Absolute paths to loaded node definition files
    pub nodes: HashMap<String, NodeSpec>,

    /// Root directory of the plugin
    pub root: PathBuf,
}

impl Plugin {
    pub fn load_types_and_nodes(&mut self) -> Result<(), PluginError> {
        for type_path in &self.manifest.types {
            let full_path = self.root.join(type_path);
            let json = std::fs::read_to_string(&full_path)?;
            let dt: DataType = serde_json::from_str(&json)?;
            self.types.insert(dt.type_fqdn.clone(), dt);
        }

        for node_path in &self.manifest.nodes {
            let full_path = self.root.join(node_path);
            let json = std::fs::read_to_string(&full_path)?;
            let dt: NodeSpec = serde_json::from_str(&json)?;
            self.nodes.insert(dt.node_fqdn.clone(), dt);
        }

        Ok(())
    }
}

/// Manages discovery and loading of Moirai plugins
pub struct PluginManager {
    /// Map namespace -> plugin
    pub plugins: HashMap<String, Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    /// Loads plugins declared in plugin_specs
    pub fn load_from_paths(
        &mut self,
        plugin_specs: &HashMap<String, PluginSource>,
    ) -> Result<(), PluginError> {
        // TODO: Check if this needs refactoring (After it works)
        for (namespace, spec) in plugin_specs {
            // ! Only handle local plugins for now
            let root = if let Some(path_str) = &spec.path {
                PathBuf::from(path_str)
            } else {
                // TODO: Add support for git based
                continue;
            };
            
            
            // TODO: Checck if this is correct
            let manifest_file = root.join("plugin.json");

            // Read manifest
            let manifest_str = fs::read_to_string(&manifest_file)?;

            // Parse manifest
            let manifest: PluginManifest = serde_json::from_str(&manifest_str)?;
                
            // Validate FQDN
            if manifest.plugin_fqdn != *namespace {
                return Err(PluginError::FqdnMismatch {
                    expected: namespace.clone(),
                    found: manifest.plugin_fqdn.clone(),
                });
            }

            // Build the plugin and load its specs
            let mut plugin = Plugin {
                root: root.clone(),
                manifest,
                types: HashMap::new(),
                nodes: HashMap::new(),
            };

            plugin.load_types_and_nodes()?;
            self.plugins.insert(namespace.clone(), plugin);
        }
        Ok(())
    }

    /// Retrieve a loaded plugin by namespace
    pub fn get(&self, namespace: &str) -> Option<&Plugin> {
        self.plugins.get(namespace)
    }

    /// List all loaded plugin FQDNs
    pub fn list_namespaces(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }
}
