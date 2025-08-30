pub mod client;
pub mod error;
pub mod mcp;
pub mod models;

pub use client::RaworcClient;
pub use error::{RaworcError, RaworcResult};
pub use mcp::RaworcMcpServer;

/// Configuration for the Raworc client
#[derive(Debug, Clone)]
pub struct Config {
    pub api_url: Option<String>,
    pub auth_token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub default_space: Option<String>,
    pub timeout_seconds: Option<u64>,
}

impl Config {
    /// Create a new configuration
    pub fn new(api_url: String) -> Self {
        Self {
            api_url: Some(api_url),
            auth_token: None,
            username: None,
            password: None,
            default_space: None,
            timeout_seconds: None,
        }
    }

    /// Set authentication token
    pub fn with_auth_token(mut self, token: String) -> Self {
        self.auth_token = Some(token);
        self
    }

    /// Set username and password for authentication
    pub fn with_credentials(mut self, username: String, password: String) -> Self {
        self.username = Some(username);
        self.password = Some(password);
        self
    }

    /// Set default space
    pub fn with_default_space(mut self, space: String) -> Self {
        self.default_space = Some(space);
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout_seconds = Some(timeout);
        self
    }
}

/// MCP capabilities constant
pub const CAPABILITIES: &str = r#"{
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
}"#;
