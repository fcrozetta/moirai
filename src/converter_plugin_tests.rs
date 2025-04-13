// filepath: /Users/fcrozetta/projects/moirai/src/converter_plugin_tests.rs
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::{json, Value};
    use crate::converter_plugin;
    use crate::plugin::{PluginRegistry, Plugin};
    use crate::models::ParameterType;

    async fn execute_action(action: &str, inputs: HashMap<String, Value>) -> Result<HashMap<String, Value>, String> {
        converter_plugin::execute_converter_action(action, inputs).await
    }

    #[tokio::test]
    async fn test_number_to_string_converter() {
        // Test with integer
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(42));
        
        let result = execute_action("number_to_string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("42")));
        
        // Test with float
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(3.14159));
        
        let result = execute_action("number_to_string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("3.14159")));
        
        // Test with negative number
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(-999));
        
        let result = execute_action("number_to_string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("-999")));
    }

    #[tokio::test]
    async fn test_boolean_to_string_converter() {
        // Test with true value, default format
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(true));
        
        let result = execute_action("boolean_to_string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("true")));
        
        // Test with false value, default format
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(false));
        
        let result = execute_action("boolean_to_string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("false")));
        
        // Test with yes/no format
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(true));
        inputs.insert("format".to_string(), json!("yes/no"));
        
        let result = execute_action("boolean_to_string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("yes")));
        
        // Test with 1/0 format
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(false));
        inputs.insert("format".to_string(), json!("1/0"));
        
        let result = execute_action("boolean_to_string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("0")));
        
        // Test with on/off format
        let mut inputs = HashMap::new();
        inputs.insert("value".to_string(), json!(true));
        inputs.insert("format".to_string(), json!("on/off"));
        
        let result = execute_action("boolean_to_string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("on")));
    }

    #[tokio::test]
    async fn test_array_to_string_converter() {
        // Test with array of strings
        let mut inputs = HashMap::new();
        inputs.insert("array".to_string(), json!(["apple", "banana", "cherry"]));
        inputs.insert("separator".to_string(), json!(", "));
        
        let result = execute_action("array_to_string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("apple, banana, cherry")));
        
        // Test with array of mixed types
        let mut inputs = HashMap::new();
        inputs.insert("array".to_string(), json!([42, true, "hello", null]));
        inputs.insert("separator".to_string(), json!(" | "));
        
        let result = execute_action("array_to_string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("42 | true | hello | null")));
        
        // Test with empty array
        let mut inputs = HashMap::new();
        inputs.insert("array".to_string(), json!([]));
        
        let result = execute_action("array_to_string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("")));
        
        // Test with custom separator
        let mut inputs = HashMap::new();
        inputs.insert("array".to_string(), json!([1, 2, 3, 4, 5]));
        inputs.insert("separator".to_string(), json!("--"));
        
        let result = execute_action("array_to_string", inputs).await.unwrap();
        assert_eq!(result.get("output"), Some(&json!("1--2--3--4--5")));
    }

    #[tokio::test]
    async fn test_object_to_string_converter() {
        // Test with simple object, no pretty print
        let mut inputs = HashMap::new();
        inputs.insert("object".to_string(), json!({"name": "John", "age": 30}));
        inputs.insert("pretty".to_string(), json!(false));
        
        let result = execute_action("object_to_string", inputs).await.unwrap();
        let output = result.get("output").unwrap().as_str().unwrap();
        assert!(output.contains("John"));
        assert!(output.contains("30"));
        assert!(!output.contains("\n"));  // No newlines without pretty print
        
        // Test with pretty print
        let mut inputs = HashMap::new();
        inputs.insert("object".to_string(), json!({"name": "John", "age": 30}));
        inputs.insert("pretty".to_string(), json!(true));
        
        let result = execute_action("object_to_string", inputs).await.unwrap();
        let output = result.get("output").unwrap().as_str().unwrap();
        assert!(output.contains("John"));
        assert!(output.contains("30"));
        assert!(output.contains("\n"));  // Pretty print should have newlines
        
        // Test with nested object
        let mut inputs = HashMap::new();
        inputs.insert("object".to_string(), json!({
            "person": {
                "name": "Alice",
                "details": {
                    "age": 28,
                    "active": true
                }
            }
        }));
        
        let result = execute_action("object_to_string", inputs).await.unwrap();
        let output = result.get("output").unwrap().as_str().unwrap();
        assert!(output.contains("Alice"));
        assert!(output.contains("28"));
    }

    #[tokio::test]
    async fn test_register_converter_plugin() {
        let mut registry = PluginRegistry::new();
        let result = converter_plugin::register_converter_plugin(&mut registry);
        assert!(result.is_ok());
        
        // Verify that the plugin was registered with all the expected actions
        let plugins = registry.find_plugins("converter");
        assert!(!plugins.is_empty(), "Converter plugin should be registered");
        
        let plugin = &plugins[0];
        
        assert!(plugin.actions.contains_key("number_to_string"));
        assert!(plugin.actions.contains_key("boolean_to_string"));
        assert!(plugin.actions.contains_key("array_to_string"));
        assert!(plugin.actions.contains_key("object_to_string"));
        
        // Check parameter types for number_to_string converter
        let number_action = plugin.actions.get("number_to_string").unwrap();
        assert_eq!(number_action.inputs.len(), 2); // value and format
        assert_eq!(number_action.inputs[0].param_type, ParameterType::Number);
        
        // Check parameter types for boolean_to_string converter
        let boolean_action = plugin.actions.get("boolean_to_string").unwrap();
        assert_eq!(boolean_action.inputs.len(), 2); // value and format
        assert_eq!(boolean_action.inputs[0].param_type, ParameterType::Boolean);
    }

    #[tokio::test]
    async fn test_converter_node_check() {
        assert!(converter_plugin::is_converter_node("converter", "number_to_string"));
        assert!(converter_plugin::is_converter_node("converter", "boolean_to_string"));
        assert!(converter_plugin::is_converter_node("converter", "array_to_string"));
        assert!(converter_plugin::is_converter_node("converter", "object_to_string"));
        assert!(!converter_plugin::is_converter_node("converter", "unknown_action"));
        assert!(!converter_plugin::is_converter_node("system", "number_to_string"));
    }
}