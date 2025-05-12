use serde::Deserialize;
use std::{collections::HashMap, fs, path::{Path, PathBuf}};
use thiserror::Error;

// Error types for plugin loading
#[derive(Debug, Error)]
pub enum Pluginerror {
    #[error("Failed to read manifest file {path}: {source}")]
    ReadManifest {
        path: String,
        
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse manifest TOML: {path}: {source}")]
    ParseManifest {
        path: String,

        #[source]
        source: toml::de::Error,
    },

    #[error("Manifest validation error:{0}")]
    Validation(String),
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
    pub metadata: Option<PluginMetadata>,

    // Relative paths to type definitions
    pub types: Vec<String>,
    pub nodes: Vec<String>,
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
    pub type_paths: Vec<PathBuf>,
    ///Absolute paths to loaded node definition files 
    pub node_paths: Vec<PathBuf>,

    /// Root directory of the plugin
    pub root: PathBuf,
}

/// Manages discovery and loading of Moirai plugins
pub struct PluginManager {
    /// Map namespace -> plugin
    pub plugins: HashMap<String,Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: HashMap::new() }
    }

    /// Loads plugins from given path
    pub fn load_from_paths<P: AsRef<Path>>( &mut self, plugin_specs: &HashMap<String,super::config_loader::PluginSource>) -> Result<(), Pluginerror> {
        // TODO: Check if this needs refactoring (After it works)
        for (namespace, source) in plugin_specs {
            // ! Only handle local plugins for now
            if let Some(path_str) = &source.path {
                let root = PathBuf::from(path_str);
                // TODO: Checck if this is correct
                let manifest_path = root.join("plugin.json");

                // Read manifest
                let manifest_str = fs::read_to_string(&manifest_path)
                    .map_err(|e| Pluginerror::ReadManifest { path: manifest_path.display().to_string(), source: e })?;

                // Parse manifest
                let manifest: PluginManifest = toml::from_str(&manifest_str)
                    .map_err(|e| Pluginerror::ParseManifest { path: manifest_path.display().to_string(), source: e })?;

                // Validate FQDN
                if manifest.plugin_fqdn != *namespace {
                    return Err(Pluginerror::Validation(format!("PLugin FQDN '{}' does not match the config key '{}'", manifest.plugin_fqdn, namespace)));
                }

                //Resolve types and node paths
                let type_paths = manifest.types.iter()
                    .map(| rel| root.join(rel))
                    .collect();
                let node_paths = manifest.nodes.iter()
                    .map( |rel| root.join(rel))
                    .collect();

                // Register plugin
                self.plugins.insert(namespace.clone(), Plugin { manifest: manifest, type_paths: type_paths, node_paths: node_paths, root: root.clone() });
            }
        // TODO: Handle git sources
        }
        Ok(())
    }

    /// Retrieve a loaded plugin by namespace
    pub fn get(&self, namespace: &str,) -> Option<&Plugin> {
        self.plugins.get(namespace)
    }

    /// List all loaded plugin FQDNs
    pub fn list_namespaces(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

}
