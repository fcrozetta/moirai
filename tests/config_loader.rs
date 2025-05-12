use std::io::Write;
use tempfile::NamedTempFile;
use moirai::config_loader::{load_config,ConfigError};

#[test]
fn test_load_valid_config() -> Result<(),Box<dyn std::error::Error>> {
    // create temp file with valid config
    let mut file = NamedTempFile::new()?;
    write!(
        file,
        r#"
version = "1.0.0"

[engine]
engine_version = "0.1.0"
workflow = "workflow.json"

concurrency = 4
timeout_seconds = 60

[plugins]

[plugins.sys]
path = "./plugins/sys"
"#
    )?;

    // Load and verify
    let cfg = load_config(file.path())?;
    assert_eq!(cfg.version, "1.0.0");
    assert_eq!(cfg.engine.engine_version, "0.1.0");
    assert_eq!(cfg.engine.workflow.as_deref(), Some("workflow.json"));
    assert_eq!(cfg.engine.concurrency, Some(4));
    assert_eq!(cfg.engine.timeout_seconds, Some(60));
    assert!(cfg.plugins.contains_key("sys"));

    let sys = cfg.plugins.get("sys").unwrap();
    assert_eq!(sys.path.as_deref(),Some("./plugins/sys"));
    assert!(sys.git.is_none());
    Ok(())
}

#[test]
fn test_load_nonexisten_file() {
    let result = load_config("does_not_exist_toml");
    assert!(matches!(result.unwrap_err(),ConfigError::Io(_)));
}

#[test]
fn test_load_malformed_toml() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    write!(file,"Not a valid toml");

    let err = load_config(file.path()).unwrap_err();
    assert!(matches!(err,ConfigError::Toml(_)));

    Ok(())
}