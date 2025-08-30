use raworc_mcp::Config;

#[test]
fn test_config_defaults() {
    let config = Config {
        api_url: Some("https://api.remoteagent.com/api/v0".to_string()),
        auth_token: Some("test-token".to_string()),
        username: None,
        password: None,
        default_space: Some("default".to_string()),
        timeout_seconds: Some(30),
    };

    assert_eq!(config.api_url, Some("https://api.remoteagent.com/api/v0".to_string()));
    assert_eq!(config.auth_token, Some("test-token".to_string()));
    assert_eq!(config.default_space, Some("default".to_string()));
    assert_eq!(config.timeout_seconds, Some(30));
}

#[test]
fn test_capabilities_parsing() {
    let capabilities = raworc_mcp::CAPABILITIES;
    let parsed: serde_json::Value = serde_json::from_str(capabilities).unwrap();
    assert!(parsed.is_object());
    
    let tools = parsed.get("tools").unwrap();
    assert!(tools.is_array());
    
    let tools_array = tools.as_array().unwrap();
    assert!(!tools_array.is_empty());
    
    // Check for specific tools
    let tool_names: Vec<String> = tools_array
        .iter()
        .filter_map(|tool| tool.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()))
        .collect();
    
    assert!(tool_names.contains(&"list_sessions".to_string()));
    assert!(tool_names.contains(&"create_session".to_string()));
    assert!(tool_names.contains(&"send_message".to_string()));
    assert!(tool_names.contains(&"health_check".to_string()));
}
