use std::{collections::HashMap, fs::File, io::Write};
use tempfile::TempDir;

use moirai::config_loader::PluginSource;
use moirai::plugin_manager::{PluginError, PluginManager};

#[test]
fn test_load_local_plugin() -> Result<(), Box<dyn std::error::Error>> {
    // Setup temp dir
    let temp_dir = TempDir::new()?;
    let plugin_dir = temp_dir.path().join("mysys");
    let types_dir = plugin_dir.join("types");
    let nodes_dir = plugin_dir.join("nodes");
    std::fs::create_dir_all(&types_dir)?;
    std::fs::create_dir_all(&nodes_dir)?;

    //write plugin.json manifest
    let manifest_path = plugin_dir.join("plugin.json");
    let mut manifest_file = File::create(&manifest_path)?;
    write!(
        manifest_file,
        r#"
{{
"plugin_fqdn": "mysys",
"version": "1.0.0",
"types": ["types/t1.json"],
"nodes": ["nodes/n1.json"]
}}"#
    )?;

    // 3) Write a minimal types/t1.json
    let mut t1 = File::create(types_dir.join("t1.json"))?;
    write!(
        t1,
        r#"{{
"type_fqdn": "mysys:t1",
"version": "1.0.0",
"metadata": {{
    "display_name": "T1",
    "description": "Test type",
    "author": null,
    "created": null
}},
"properties": {{
    "kind": "primitive",
    "format": "text/plain",
    "constraints": {{}}
    }}
}}
"#
    )?;

    // Write a minimal nodes/n1.json
    let mut n1 = File::create(nodes_dir.join("n1.json"))?;
    write!(
        n1,
        r#"{{
"type": "primitive",
"node_fqdn": "mysys:n1",
"plugin_version": "1.0.0",
"node_version": "1.0.0",
"entrypoint": null,
"inputs": [],
"outputs": [],
"metadata": {{
    "display_name": "N1",
    "description": "Test node",
    "author": null,
    "created": null,
    "tags": null,
    "icon": null
    }}
}}
"#
    )?;
    // Prepare plugin specs pointing to local path
    let mut specs = HashMap::new();
    specs.insert(
        "mysys".to_string(),
        PluginSource {
            path: Some(plugin_dir.to_string_lossy().into()),
            git: None,
            rev: None,
        },
    );

    // load plugins
    let mut manager = PluginManager::new();
    manager.load_from_paths(&specs).unwrap();

    // Vewrify plugin registration
    let plugin = manager
        .get("mysys")
        .expect("Plugin 'mysys' should be loaded");
    assert_eq!(plugin.manifest.plugin_fqdn, "mysys");
    assert_eq!(plugin.manifest.version, "1.0.0");
    assert_eq!(plugin.types.len(), 1);
    assert_eq!(plugin.nodes.len(), 1);
    assert!(plugin.types.contains_key("mysys:t1"));
    assert!(plugin.nodes.contains_key("mysys:n1"));

    Ok(())
}
