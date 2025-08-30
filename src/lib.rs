pub mod client;
pub mod error;
pub mod mcp;
pub mod models;
pub mod server;

pub use client::RaworcClient;
pub use error::{RaworcError, RaworcResult};
pub use mcp::RaworcMcpServer;
pub use models::*;
pub use server::run_server;

use serde::{Deserialize, Serialize};

/// MCP Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Raworc API base URL
    pub api_url: String,
    /// Authentication token
    pub auth_token: Option<String>,
    /// Username for authentication
    pub username: Option<String>,
    /// Password for authentication
    pub password: Option<String>,
    /// Default space to use
    pub default_space: Option<String>,
    /// Request timeout in seconds
    pub timeout_seconds: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: "http://raworc.remoteagent.com:9000/api/v0".to_string(),
            auth_token: None,
            username: None,
            password: None,
            default_space: Some("default".to_string()),
            timeout_seconds: Some(30),
        }
    }
}

/// MCP Server capabilities
pub const CAPABILITIES: &str = r#"{
  "tools": [
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
      "description": "Get session details by ID",
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
            "type": "integer",
            "description": "Maximum number of messages to return"
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
      "description": "Resume a paused session",
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
      "name": "list_spaces",
      "description": "List all spaces",
      "inputSchema": {
        "type": "object",
        "properties": {}
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
      "description": "Get logs for a specific agent",
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
      "description": "Get a specific secret",
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
      "name": "health_check",
      "description": "Check Raworc API health",
      "inputSchema": {
        "type": "object",
        "properties": {}
      }
    },
    {
      "name": "get_version",
      "description": "Get Raworc API version",
      "inputSchema": {
        "type": "object",
        "properties": {}
      }
    }
  ]
}"#;
