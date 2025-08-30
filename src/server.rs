use crate::error::{RaworcError, RaworcResult};
use crate::mcp::RaworcMcpServer;
use crate::{Config, CAPABILITIES};
use clap::Parser;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader as TokioBufReader};
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

/// Command line arguments for the MCP server
#[derive(Parser, Debug)]
#[command(name = "raworc-mcp")]
#[command(about = "Model Context Protocol server for Raworc")]
pub struct Args {
    /// Raworc API URL
    #[arg(long, default_value = "http://raworc.remoteagent.com:9000/api/v0")]
    pub api_url: String,

    /// Authentication token
    #[arg(long)]
    pub auth_token: Option<String>,

    /// Username for authentication
    #[arg(long)]
    pub username: Option<String>,

    /// Password for authentication
    #[arg(long)]
    pub password: Option<String>,

    /// Default space to use
    #[arg(long, default_value = "default")]
    pub default_space: String,

    /// Request timeout in seconds
    #[arg(long, default_value = "30")]
    pub timeout: u64,

    /// Log level
    #[arg(long, default_value = "info")]
    pub log_level: String,
}

/// MCP Server implementation
pub struct McpServer {
    server: RaworcMcpServer,
    stdin: tokio::io::Stdin,
    stdout: tokio::io::Stdout,
}

impl McpServer {
    /// Create a new MCP server
    pub fn new(config: Config) -> RaworcResult<Self> {
        let server = RaworcMcpServer::new(config)?;
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        Ok(Self {
            server,
            stdin,
            stdout,
        })
    }

    /// Run the MCP server
    pub async fn run(&mut self) -> RaworcResult<()> {
        // Initialize the server
        self.server.initialize().await?;

        // Send initialization message
        self.send_initialize().await?;

        // Main message loop
        let mut line = String::new();
        
        loop {
            line.clear();
            let mut reader = TokioBufReader::new(&mut self.stdin);
            
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    debug!("Received message: {}", line);
                    self.handle_message(line).await?;
                }
                Err(e) => {
                    error!("Failed to read line: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Send initialization message
    async fn send_initialize(&mut self) -> RaworcResult<()> {
        let message = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "clientInfo": {
                    "name": "raworc-mcp",
                    "version": env!("CARGO_PKG_VERSION")
                }
            }
        });

        self.send_message(&message).await?;

        // Send initialized notification
        let initialized = json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized",
            "params": {}
        });

        self.send_message(&initialized).await?;

        // Send tools/list response
        let tools_response = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "result": {
                "tools": serde_json::from_str::<Value>(CAPABILITIES)?
            }
        });

        self.send_message(&tools_response).await?;

        info!("MCP server initialized successfully");
        Ok(())
    }

    /// Handle incoming message
    async fn handle_message(&mut self, line: &str) -> RaworcResult<()> {
        let message: Value = serde_json::from_str(line)
            .map_err(|e| RaworcError::mcp_error(&format!("Failed to parse JSON: {}", e)))?;

        let method = message.get("method").and_then(|v| v.as_str());
        let id = message.get("id").and_then(|v| v.as_u64());

        match method {
            Some("tools/call") => {
                self.handle_tool_call(message, id).await?;
            }
            Some("ping") => {
                self.handle_ping(id).await?;
            }
            _ => {
                warn!("Unknown method: {:?}", method);
            }
        }

        Ok(())
    }

    /// Handle tool call
    async fn handle_tool_call(&mut self, message: Value, id: Option<u64>) -> RaworcResult<()> {
        let params = message.get("params")
            .ok_or_else(|| RaworcError::mcp_error("Missing params in tool call"))?;

        let name = params.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::mcp_error("Missing tool name"))?;

        let arguments = params.get("arguments")
            .unwrap_or(&json!({}))
            .clone();

        let result = self.server.handle_tool_call(name, &arguments).await;

        let response = match result {
            Ok(tool_response) => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": tool_response.content
                    }
                })
            }
            Err(e) => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32000,
                        "message": e.to_string()
                    }
                })
            }
        };

        self.send_message(&response).await?;
        Ok(())
    }

    /// Handle ping
    async fn handle_ping(&mut self, id: Option<u64>) -> RaworcResult<()> {
        let response = json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "pong": true
            }
        });

        self.send_message(&response).await?;
        Ok(())
    }

    /// Send message to client
    async fn send_message(&mut self, message: &Value) -> RaworcResult<()> {
        let message_str = serde_json::to_string(message)
            .map_err(|e| RaworcError::mcp_error(&format!("Failed to serialize message: {}", e)))?;

        debug!("Sending message: {}", message_str);
        
        let message_with_newline = format!("{}\n", message_str);
        self.stdout.write_all(message_with_newline.as_bytes()).await
            .map_err(|e| RaworcError::mcp_error(&format!("Failed to write message: {}", e)))?;

        self.stdout.flush().await
            .map_err(|e| RaworcError::mcp_error(&format!("Failed to flush stdout: {}", e)))?;

        Ok(())
    }
}

/// Run the MCP server
pub async fn run_server() -> RaworcResult<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(format!("raworc_mcp={}", args.log_level)))
        .init();

    info!("Starting Raworc MCP server");

    // Create configuration
    let config = Config {
        api_url: args.api_url,
        auth_token: args.auth_token,
        username: args.username,
        password: args.password,
        default_space: Some(args.default_space),
        timeout_seconds: Some(args.timeout),
    };

    // Create and run MCP server
    let mut server = McpServer::new(config)?;
    server.run().await?;

    info!("MCP server stopped");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_args() {
        let args = Args::try_parse_from(&["raworc-mcp", "--api-url", "http://test.com"]).unwrap();
        assert_eq!(args.api_url, "http://test.com");
    }

    #[test]
    fn test_json_serialization() {
        let message = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "test",
            "params": {}
        });

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }
}
