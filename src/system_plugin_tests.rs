// filepath: /Users/fcrozetta/projects/moirai/src/system_plugin_tests.rs
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::{json, Value};
    use crate::system_plugin;
    use crate::plugin::{PluginRegistry, Plugin};
    use crate::models::ParameterType;

    async fn execute_action(action: &str, inputs: HashMap<String, Value>) -> Result<HashMap<String, Value>, String> {
        system_plugin::execute_system_action(action, inputs).await
    }

    #[tokio::test]
    async fn test_string_primitive() {
        // Test with string input
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!("Hello, World!"));
        
        let result = execute_action("string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("Hello, World!")));
        
        // Test with number input - should convert to string
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(42));
        
        let result = execute_action("string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("42")));
        
        // Test with boolean input - should convert to string
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(true));
        
        let result = execute_action("string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("true")));
        
        // Test with null input - should convert to "null" string
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), Value::Null);
        
        let result = execute_action("string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("null")));
        
        // Test with object input - should convert to "{object}" string
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!({"key": "value"}));
        
        let result = execute_action("string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("{object}")));
    }

    #[tokio::test]
    async fn test_number_primitive() {
        // Test with number input
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(42));
        
        let result = execute_action("number", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(42)));
        
        // Test with float input
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(3.14));
        
        let result = execute_action("number", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(3.14)));
        
        // Test with string input that's a valid number
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!("123"));
        
        let result = execute_action("number", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(123)));
        
        // Test with string input that's not a valid number
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!("not a number"));
        
        let result = execute_action("number", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(0)));
        
        // Test with boolean input - should convert to 0 or 1
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(true));
        
        let result = execute_action("number", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(1)));
        
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(false));
        
        let result = execute_action("number", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(0)));
    }

    #[tokio::test]
    async fn test_boolean_primitive() {
        // Test with boolean input
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(true));
        
        let result = execute_action("boolean", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(true)));
        
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(false));
        
        let result = execute_action("boolean", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(false)));
        
        // Test with number input - 0 should be false, non-zero should be true
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(0));
        
        let result = execute_action("boolean", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(false)));
        
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(1));
        
        let result = execute_action("boolean", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(true)));
        
        // Test with string input - empty should be false, non-empty should be true
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(""));
        
        let result = execute_action("boolean", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(false)));
        
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!("hello"));
        
        let result = execute_action("boolean", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(true)));
        
        // Test with "true"/"false" strings
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!("true"));
        
        let result = execute_action("boolean", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(true)));
        
        // Test with array - empty should be false, non-empty should be true
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!([]));
        
        let result = execute_action("boolean", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(false)));
        
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!([1, 2, 3]));
        
        let result = execute_action("boolean", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!(true)));
    }

    #[tokio::test]
    async fn test_object_primitive() {
        // Test with object input
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!({"name": "moirai", "type": "workflow"}));
        
        let result = execute_action("object", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!({"name": "moirai", "type": "workflow"})));
        
        // Test with non-object input - should return empty object
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!("not an object"));
        
        let result = execute_action("object", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!({})));
    }

    #[tokio::test]
    async fn test_null_primitive() {
        // Test null node - should always return null
        let inputs = HashMap::new();
        
        let result = execute_action("null", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&Value::Null));
    }

    #[tokio::test]
    async fn test_start_end_nodes() {
        // Test start node - should return input as is
        let mut inputs = HashMap::new();
        inputs.insert("test".to_string(), json!("value"));
        
        let result = execute_action("start", inputs.clone()).await.unwrap();
        assert_eq!(result.get("test"), Some(&json!("value")));
        
        // Test end node - should return input as is
        let result = execute_action("end", inputs.clone()).await.unwrap();
        assert_eq!(result.get("test"), Some(&json!("value")));
    }

    #[tokio::test]
    async fn test_register_system_plugin() {
        let mut registry = PluginRegistry::new();
        let result = system_plugin::register_system_plugin(&mut registry);
        assert!(result.is_ok());
        
        // Verify that the plugin was registered with all the expected actions
        let plugins = registry.find_plugins("system");
        assert!(!plugins.is_empty(), "System plugin should be registered");
        
        let plugin = &plugins[0];
        
        assert!(plugin.actions.contains_key("start"));
        assert!(plugin.actions.contains_key("end"));
        assert!(plugin.actions.contains_key("string"));
        assert!(plugin.actions.contains_key("number"));
        assert!(plugin.actions.contains_key("boolean"));
        assert!(plugin.actions.contains_key("object"));
        assert!(plugin.actions.contains_key("null"));
        
        // Check parameter types
        let string_action = plugin.actions.get("string").unwrap();
        assert_eq!(string_action.inputs.len(), 1);
        assert_eq!(string_action.inputs[0].param_type, ParameterType::String);
        
        let number_action = plugin.actions.get("number").unwrap();
        assert_eq!(number_action.inputs.len(), 1);
        assert_eq!(number_action.inputs[0].param_type, ParameterType::Number);
        
        let boolean_action = plugin.actions.get("boolean").unwrap();
        assert_eq!(boolean_action.inputs.len(), 1);
        assert_eq!(boolean_action.inputs[0].param_type, ParameterType::Boolean);
    }

    #[tokio::test]
    async fn test_system_node_checks() {
        assert!(system_plugin::is_system_node("system", "start"));
        assert!(system_plugin::is_system_node("system", "end"));
        assert!(system_plugin::is_system_node("system", "string"));
        assert!(system_plugin::is_system_node("system", "number"));
        assert!(system_plugin::is_system_node("system", "boolean"));
        assert!(system_plugin::is_system_node("system", "object"));
        assert!(system_plugin::is_system_node("system", "null"));
        
        assert!(!system_plugin::is_system_node("other", "start"));
        assert!(!system_plugin::is_system_node("system", "unknown"));
        
        assert!(system_plugin::is_start_node("system", "start"));
        assert!(!system_plugin::is_start_node("system", "end"));
        
        assert!(system_plugin::is_end_node("system", "end"));
        assert!(!system_plugin::is_end_node("system", "start"));
        
        assert!(system_plugin::is_primitive_node("system", "string"));
        assert!(system_plugin::is_primitive_node("system", "number"));
        assert!(system_plugin::is_primitive_node("system", "boolean"));
        assert!(system_plugin::is_primitive_node("system", "object"));
        assert!(system_plugin::is_primitive_node("system", "null"));
        assert!(!system_plugin::is_primitive_node("system", "start"));
        assert!(!system_plugin::is_primitive_node("system", "end"));
    }

    #[tokio::test]
    async fn test_log_node() {
        // Test with single string message
        let mut inputs = HashMap::new();
        inputs.insert("message".to_string(), json!("Hello, world!"));
        inputs.insert("level".to_string(), json!("info"));
        
        let result = execute_action("log", inputs).await.unwrap();
        assert!(result.is_empty()); // Log node returns empty output
        
        // Test with multiple messages as array
        let mut inputs = HashMap::new();
        inputs.insert("message".to_string(), json!(["Line 1", "Line 2", "Line 3"]));
        inputs.insert("level".to_string(), json!("debug"));
        
        let result = execute_action("log", inputs).await.unwrap();
        assert!(result.is_empty()); // Log node returns empty output
        
        // Test with different log levels
        let mut inputs = HashMap::new();
        inputs.insert("message".to_string(), json!("Warning message"));
        inputs.insert("level".to_string(), json!("warning"));
        
        let result = execute_action("log", inputs).await.unwrap();
        assert!(result.is_empty());
        
        let mut inputs = HashMap::new();
        inputs.insert("message".to_string(), json!("Error message"));
        inputs.insert("level".to_string(), json!("error"));
        
        let result = execute_action("log", inputs).await.unwrap();
        assert!(result.is_empty());
        
        // Test with non-string input
        let mut inputs = HashMap::new();
        inputs.insert("message".to_string(), json!(42));
        
        let result = execute_action("log", inputs).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_log_node_check() {
        assert!(system_plugin::is_log_node("system", "log"));
        assert!(!system_plugin::is_log_node("system", "start"));
        assert!(!system_plugin::is_log_node("other", "log"));
    }
}