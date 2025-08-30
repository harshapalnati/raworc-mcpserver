use anyhow::Result;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use raworc_mcp::{Config, RaworcMcpServer};

#[tokio::main]
async fn main() -> Result<()> {
    // STDERR-only logging
    tracing_subscriber::fmt()
        .with_writer(BoxMakeWriter::new(std::io::stderr))
        .init();

    // Create configuration from environment variables
    let config = Config {
        api_url: Some(std::env::var("RAWORC_API_URL")
            .unwrap_or_else(|_| "https://api.remoteagent.com/api/v0".to_string())),
        auth_token: std::env::var("RAWORC_AUTH_TOKEN").ok(),
        username: std::env::var("RAWORC_USERNAME").ok(),
        password: std::env::var("RAWORC_PASSWORD").ok(),
        default_space: std::env::var("RAWORC_DEFAULT_SPACE").ok(),
        timeout_seconds: std::env::var("RAWORC_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok()),
    };

    // Create MCP server
    let mut server = RaworcMcpServer::new(config)?;
    let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    let mut stdout = tokio::io::stdout();

    while let Some(line) = stdin.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        let msg: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Bad JSON on stdin: {}", e);
                continue;
            }
        };

        let method = msg.get("method").and_then(Value::as_str);
        let id = msg.get("id").and_then(Value::as_u64);

        match method {
            Some("initialize") => {
                write_json(&mut stdout, json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": { "tools": {} },
                        "serverInfo": {
                            "name": "raworc-mcp",
                            "version": env!("CARGO_PKG_VERSION")
                        }
                    }
                })).await?;
                
                write_json(&mut stdout, json!({
                    "jsonrpc": "2.0",
                    "method": "notifications/initialized",
                    "params": {}
                })).await?;
            }
            
            Some("tools/list") => {
                let tools = json!({
                    "tools": [
                        {
                            "name": "health_check",
                            "description": "Check Raworc API health",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        },
                        {
                            "name": "list_spaces",
                            "description": "List all spaces",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        },
                        {
                            "name": "list_sessions",
                            "description": "List all sessions in a space",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "space": {
                                        "type": "string",
                                        "description": "Space name (optional, uses default if not provided)"
                                    }
                                }
                            }
                        },
                        {
                            "name": "create_session",
                            "description": "Create a new session",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "space": {
                                        "type": "string",
                                        "description": "Space name (optional, uses default if not provided)"
                                    },
                                    "metadata": {
                                        "type": "object",
                                        "description": "Additional metadata for the session"
                                    }
                                }
                            }
                        },
                        {
                            "name": "get_session",
                            "description": "Get session details",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "session_id": {
                                        "type": "string",
                                        "description": "Session ID"
                                    }
                                },
                                "required": ["session_id"]
                            }
                        },
                        {
                            "name": "send_message",
                            "description": "Send a message to a session",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "session_id": {
                                        "type": "string",
                                        "description": "Session ID"
                                    },
                                    "content": {
                                        "type": "string",
                                        "description": "Message content"
                                    }
                                },
                                "required": ["session_id", "content"]
                            }
                        },
                        {
                            "name": "get_messages",
                            "description": "Get messages from a session",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "session_id": {
                                        "type": "string",
                                        "description": "Session ID"
                                    },
                                    "limit": {
                                        "type": "number",
                                        "description": "Maximum number of messages to retrieve"
                                    }
                                },
                                "required": ["session_id"]
                            }
                        },
                        {
                            "name": "pause_session",
                            "description": "Pause a session",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "session_id": {
                                        "type": "string",
                                        "description": "Session ID"
                                    }
                                },
                                "required": ["session_id"]
                            }
                        },
                        {
                            "name": "resume_session",
                            "description": "Resume a session",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "session_id": {
                                        "type": "string",
                                        "description": "Session ID"
                                    }
                                },
                                "required": ["session_id"]
                            }
                        },
                        {
                            "name": "terminate_session",
                            "description": "Terminate a session",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "session_id": {
                                        "type": "string",
                                        "description": "Session ID"
                                    }
                                },
                                "required": ["session_id"]
                            }
                        },
                        {
                            "name": "list_agents",
                            "description": "List agents in a space",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "space": {
                                        "type": "string",
                                        "description": "Space name (optional, uses default if not provided)"
                                    }
                                }
                            }
                        },
                        {
                            "name": "get_agent_logs",
                            "description": "Get logs for an agent",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "space": {
                                        "type": "string",
                                        "description": "Space name"
                                    },
                                    "agent_name": {
                                        "type": "string",
                                        "description": "Agent name"
                                    }
                                },
                                "required": ["space", "agent_name"]
                            }
                        },
                        {
                            "name": "list_secrets",
                            "description": "List secrets in a space",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "space": {
                                        "type": "string",
                                        "description": "Space name (optional, uses default if not provided)"
                                    }
                                }
                            }
                        },
                        {
                            "name": "get_secret",
                            "description": "Get a secret value",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "space": {
                                        "type": "string",
                                        "description": "Space name"
                                    },
                                    "key": {
                                        "type": "string",
                                        "description": "Secret key"
                                    }
                                },
                                "required": ["space", "key"]
                            }
                        },
                        {
                            "name": "set_secret",
                            "description": "Set a secret value",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "space": {
                                        "type": "string",
                                        "description": "Space name"
                                    },
                                    "key": {
                                        "type": "string",
                                        "description": "Secret key"
                                    },
                                    "value": {
                                        "type": "string",
                                        "description": "Secret value"
                                    }
                                },
                                "required": ["space", "key", "value"]
                            }
                        },
                        {
                            "name": "delete_secret",
                            "description": "Delete a secret",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "space": {
                                        "type": "string",
                                        "description": "Space name"
                                    },
                                    "key": {
                                        "type": "string",
                                        "description": "Secret key"
                                    }
                                },
                                "required": ["space", "key"]
                            }
                        },
                        {
                            "name": "get_version",
                            "description": "Get API version",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        }
                    ]
                });
                if let Some(id) = id {
                    write_json(&mut stdout, json!({"jsonrpc":"2.0","id":id,"result":tools})).await?;
                }
            }
            Some("tools/call") => {
                let name = msg.pointer("/params/name").and_then(Value::as_str).unwrap_or("");
                let args = msg.pointer("/params/arguments").cloned().unwrap_or_else(|| json!({}));
                
                match server.handle_tool_call(name, &args).await {
                    Ok(response) => {
                        write_json(&mut stdout, json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": {
                                "content": response.content
                            }
                        })).await?;
                    }
                    Err(e) => {
                        write_json(&mut stdout, json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "error": {
                                "code": -32000,
                                "message": e.to_string()
                            }
                        })).await?;
                    }
                }
            }
            Some("ping") => {
                if let Some(id) = id {
                    write_json(&mut stdout, json!({"jsonrpc":"2.0","id":id,"result":{"ok":true}})).await?;
                }
            }
            _ => { eprintln!("unknown/notification: {method:?}"); }
        }
    }
    eprintln!("EOF from client; exiting."); // only when Claude closes us
    Ok(())
}

async fn write_json(stdout: &mut tokio::io::Stdout, v: Value) -> Result<()> {
    let line = serde_json::to_string(&v)?;
    stdout.write_all(line.as_bytes()).await?;
    stdout.write_all(b"\n").await?;
    stdout.flush().await?;
    Ok(())
}

