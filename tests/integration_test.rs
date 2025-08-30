use raworc_mcp::{Config, RaworcMcpServer};

#[tokio::test]
async fn test_mcp_server_creation() {
    let config = Config {
        api_url: "http://raworc.remoteagent.com:9000/api/v0".to_string(),
        auth_token: None,
        username: None,
        password: None,
        default_space: Some("test".to_string()),
        timeout_seconds: Some(30),
    };

    let server = RaworcMcpServer::new(config);
    assert!(server.is_ok());
}

#[tokio::test]
async fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.api_url, "http://raworc.remoteagent.com:9000/api/v0");
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
