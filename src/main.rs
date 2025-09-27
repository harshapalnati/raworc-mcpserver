use anyhow::Result;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing_subscriber::{fmt::writer::BoxMakeWriter, EnvFilter};
use raworc_mcp::{Config, RaworcMcpServer};
use std::net::SocketAddr;

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

    // Choose transport: HTTP (Smithery) or STDIO (default)
    match std::env::var("SERVER_MODE").ok().as_deref() {
        Some("http") => run_http().await,
        _ => run_stdio(config).await,
    }
}

async fn run_stdio(config: Config) -> Result<()> {
    let mut server = RaworcMcpServer::new(config)?;
    let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    let mut stdout = tokio::io::stdout();

    while let Some(line) = stdin.next_line().await? {
        if line.trim().is_empty() { continue; }

        let msg: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => { eprintln!("Bad JSON on stdin: {}", e); continue; }
        };
        handle_jsonrpc(&mut server, &mut stdout, msg).await?;
    }
    eprintln!("EOF from client; exiting.");
    Ok(())
}

async fn handle_jsonrpc(server: &mut RaworcMcpServer, stdout: &mut tokio::io::Stdout, msg: Value) -> Result<()> {
    let method = msg.get("method").and_then(Value::as_str);
    let id = msg.get("id").and_then(Value::as_u64);
    match method {
        Some("initialize") => {
            write_json(stdout, json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {"tools": {}},
                    "serverInfo": {"name": "raworc-mcp", "version": env!("CARGO_PKG_VERSION")}
                }
            })).await?;
            write_json(stdout, json!({"jsonrpc":"2.0","method":"notifications/initialized","params":{}})).await?;
        }
        Some("tools/list") => {
            if let Some(id) = id {
                let tools: Value = serde_json::from_str(raworc_mcp::CAPABILITIES).unwrap_or_else(|_| json!({"tools":[]}));
                write_json(stdout, json!({"jsonrpc":"2.0","id":id,"result":tools})).await?;
            }
        }
        Some("tools/call") => {
            let name = msg.pointer("/params/name").and_then(Value::as_str).unwrap_or("");
            let args = msg.pointer("/params/arguments").cloned().unwrap_or_else(|| json!({}));
            match server.handle_tool_call(name, &args).await {
                Ok(response) => {
                    write_json(stdout, json!({"jsonrpc":"2.0","id":id,"result":{"content":response.content}})).await?;
                }
                Err(e) => {
                    write_json(stdout, json!({"jsonrpc":"2.0","id":id,"error":{"code":-32000,"message":e.to_string()}})).await?;
                }
            }
        }
        Some("ping") => {
            if let Some(id) = id { write_json(stdout, json!({"jsonrpc":"2.0","id":id,"result":{"ok":true}})).await?; }
        }
        _ => { eprintln!("unknown/notification: {method:?}"); }
    }
    Ok(())
}

async fn run_http() -> Result<()> {
    use axum::{routing::post, Router};
    use tower_http::cors::{Any, CorsLayer};

    // Build router
    let app = Router::new()
        .route("/mcp", post(mcp_handler))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
            .expose_headers(Any)
        );

    let port: u16 = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8081);
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    eprintln!("HTTP MCP listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}

async fn mcp_handler(
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>, 
    axum::Json(msg): axum::Json<Value>
) -> axum::Json<Value> {
    // Build config from env and allow override from Smithery session config via query params
    // Expected query keys (JSON-dot-notation supported by Smithery):
    //   apiKey, apiUrl, defaultSpace, timeoutSeconds
    let api_url = params.get("apiUrl").cloned()
        .or_else(|| std::env::var("RAWORC_API_URL").ok())
        .unwrap_or_else(|| "https://api.remoteagent.com/api/v0".to_string());
    let auth_token = params.get("apiKey").cloned()
        .or_else(|| std::env::var("RAWORC_AUTH_TOKEN").ok());
    let default_space = params.get("defaultSpace").cloned()
        .or_else(|| std::env::var("RAWORC_DEFAULT_SPACE").ok());
    let timeout_seconds = params.get("timeoutSeconds")
        .and_then(|s| s.parse().ok())
        .or_else(|| std::env::var("RAWORC_TIMEOUT").ok().and_then(|s| s.parse().ok()));

    let config = Config {
        api_url: Some(api_url),
        auth_token,
        username: std::env::var("RAWORC_USERNAME").ok(),
        password: std::env::var("RAWORC_PASSWORD").ok(),
        default_space,
        timeout_seconds,
    };
    let mut server = match RaworcMcpServer::new(config) { Ok(s) => s, Err(e) => {
        return axum::Json(json!({"jsonrpc":"2.0","id":msg.get("id"),"error":{"code":-32000,"message":e.to_string()}}));
    }};

    // Emulate the same protocol as STDIO in a single request/response
    let method = msg.get("method").and_then(Value::as_str);
    let id = msg.get("id").cloned();
    let reply = match method {
        Some("initialize") => json!({
            "jsonrpc":"2.0","id":id,
            "result":{
                "protocolVersion":"2024-11-05",
                "capabilities": {"tools": {}},
                "serverInfo": {"name":"raworc-mcp","version":env!("CARGO_PKG_VERSION")}
            }
        }),
        Some("tools/list") => {
            let tools: Value = serde_json::from_str(raworc_mcp::CAPABILITIES).unwrap_or_else(|_| json!({"tools":[]}));
            json!({"jsonrpc":"2.0","id":id,"result":tools})
        }
        Some("tools/call") => {
            let name = msg.pointer("/params/name").and_then(Value::as_str).unwrap_or("");
            let args = msg.pointer("/params/arguments").cloned().unwrap_or_else(|| json!({}));
            match server.handle_tool_call(name, &args).await {
                Ok(response) => json!({"jsonrpc":"2.0","id":id,"result":{"content":response.content}}),
                Err(e) => json!({"jsonrpc":"2.0","id":id,"error":{"code":-32000,"message":e.to_string()}}),
            }
        }
        Some("ping") => json!({"jsonrpc":"2.0","id":id,"result":{"ok":true}}),
        _ => json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"Method not found"}}),
    };
    axum::Json(reply)
}
async fn write_json(stdout: &mut tokio::io::Stdout, v: Value) -> Result<()> {
    let line = serde_json::to_string(&v)?;
    stdout.write_all(line.as_bytes()).await?;
    stdout.write_all(b"\n").await?;
    stdout.flush().await?;
    Ok(())
}
