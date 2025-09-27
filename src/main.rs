use anyhow::Result;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing_subscriber::{fmt::writer::BoxMakeWriter, EnvFilter};
use raworc_mcp::{Config, RaworcMcpServer};

#[tokio::main]
async fn main() -> Result<()> {
    // Configurable structured logging
    let env_filter = std::env::var("RUST_LOG")
        .ok()
        .or_else(|| std::env::var("LOG_LEVEL").ok())
        .unwrap_or_else(|| "info".to_string());
    let use_json = std::env::var("LOG_FORMAT")
        .map(|v| v.eq_ignore_ascii_case("json"))
        .unwrap_or(false);
    if use_json {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new(env_filter))
            .json()
            .with_writer(BoxMakeWriter::new(std::io::stderr))
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new(env_filter))
            .with_writer(BoxMakeWriter::new(std::io::stderr))
            .init();
    }

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

    // Surface a helpful warning if no credentials are configured
    if config.auth_token.is_none() && (config.username.is_none() || config.password.is_none()) {
        tracing::warn!("No RAWORC_AUTH_TOKEN or RAWORC_USERNAME/RAWORC_PASSWORD set; authenticated tools may fail");
    }

    // Create MCP server
    let mut server = RaworcMcpServer::new(config)?;
    let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    let mut stdout = tokio::io::stdout();

    while let Some(line) = stdin.next_line().await? {
        if line.trim().is_empty() { continue; }

        let msg: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => { eprintln!("Bad JSON on stdin: {}", e); continue; }
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
                if let Some(id) = id {
                    let tools: Value = serde_json::from_str(raworc_mcp::CAPABILITIES)
                        .unwrap_or_else(|_| json!({"tools": []}));
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
