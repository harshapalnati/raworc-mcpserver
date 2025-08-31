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
            "name": "get_version",
            "description": "Get API version",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "list_service_accounts",
            "description": "List all service accounts",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "create_service_account",
            "description": "Create a new service account",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user": {
                        "type": "string",
                        "description": "Username for the service account"
                    },
                    "pass": {
                        "type": "string",
                        "description": "Password for the service account"
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name (optional)"
                    },
                    "description": {
                        "type": "string",
                        "description": "Description of the service account"
                    }
                },
                "required": ["user", "pass"]
            }
        },
        {
            "name": "get_service_account",
            "description": "Get a specific service account",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Service account ID"
                    }
                },
                "required": ["id"]
            }
        },
        {
            "name": "update_service_account",
            "description": "Update a service account",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Service account ID"
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name"
                    },
                    "description": {
                        "type": "string",
                        "description": "Description"
                    },
                    "active": {
                        "type": "boolean",
                        "description": "Whether the account is active"
                    }
                },
                "required": ["id"]
            }
        },
        {
            "name": "delete_service_account",
            "description": "Delete a service account",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Service account ID"
                    }
                },
                "required": ["id"]
            }
        },
        {
            "name": "update_service_account_password",
            "description": "Update service account password",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Service account ID"
                    },
                    "current_password": {
                        "type": "string",
                        "description": "Current password"
                    },
                    "new_password": {
                        "type": "string",
                        "description": "New password"
                    }
                },
                "required": ["id", "current_password", "new_password"]
            }
        },
        {
            "name": "list_roles",
            "description": "List all roles",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "create_role",
            "description": "Create a new role",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Role ID"
                    },
                    "description": {
                        "type": "string",
                        "description": "Role description"
                    },
                    "rules": {
                        "type": "array",
                        "description": "Role rules",
                        "items": {
                            "type": "object",
                            "properties": {
                                "resources": {
                                    "type": "array",
                                    "items": {"type": "string"}
                                },
                                "verbs": {
                                    "type": "array",
                                    "items": {"type": "string"}
                                },
                                "scope": {
                                    "type": "string"
                                }
                            }
                        }
                    }
                },
                "required": ["id", "rules"]
            }
        },
        {
            "name": "get_role",
            "description": "Get a specific role",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Role ID"
                    }
                },
                "required": ["id"]
            }
        },
        {
            "name": "delete_role",
            "description": "Delete a role",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Role ID"
                    }
                },
                "required": ["id"]
            }
        },
        {
            "name": "list_role_bindings",
            "description": "List all role bindings",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "create_role_binding",
            "description": "Create a new role binding",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "subject": {
                        "type": "string",
                        "description": "Subject (user/service account)"
                    },
                    "role_ref": {
                        "type": "string",
                        "description": "Role reference"
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name (optional)"
                    }
                },
                "required": ["subject", "role_ref"]
            }
        },
        {
            "name": "get_role_binding",
            "description": "Get a specific role binding",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Role binding ID"
                    }
                },
                "required": ["id"]
            }
        },
        {
            "name": "delete_role_binding",
            "description": "Delete a role binding",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string",
                        "description": "Role binding ID"
                    }
                },
                "required": ["id"]
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
            "name": "create_space",
            "description": "Create a new space",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Space name"
                    },
                    "description": {
                        "type": "string",
                        "description": "Space description"
                    },
                    "settings": {
                        "type": "object",
                        "description": "Space settings"
                    }
                },
                "required": ["name"]
            }
        },
        {
            "name": "get_space",
            "description": "Get a specific space",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Space name"
                    }
                },
                "required": ["name"]
            }
        },
        {
            "name": "update_space",
            "description": "Update a space",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Space name"
                    },
                    "description": {
                        "type": "string",
                        "description": "Space description"
                    },
                    "settings": {
                        "type": "object",
                        "description": "Space settings"
                    }
                },
                "required": ["name"]
            }
        },
        {
            "name": "delete_space",
            "description": "Delete a space",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Space name"
                    }
                },
                "required": ["name"]
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
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name (optional)"
                    }
                },
                "required": ["session_id"]
            }
        },
        {
            "name": "update_session",
            "description": "Update session details",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "Session ID"
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name (optional)"
                    },
                    "metadata": {
                        "type": "object",
                        "description": "Session metadata"
                    }
                },
                "required": ["session_id"]
            }
        },
        {
            "name": "update_session_state",
            "description": "Update session state",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "Session ID"
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name (optional)"
                    },
                    "state": {
                        "type": "string",
                        "description": "New session state",
                        "enum": ["INIT", "RUNNING", "PAUSED", "SUSPENDED", "TERMINATED", "IDLE", "CLOSED"]
                    }
                },
                "required": ["session_id", "state"]
            }
        },
        {
            "name": "close_session",
            "description": "Close a session",
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
            "name": "restore_session",
            "description": "Restore a closed session",
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
            "name": "remix_session",
            "description": "Fork a session",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "Session ID to fork"
                    },
                    "space": {
                        "type": "string",
                        "description": "Target space for the new session"
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
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name (optional)"
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
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name (optional)"
                    }
                },
                "required": ["session_id"]
            }
        },
        {
            "name": "get_message_count",
            "description": "Get message count for a session",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "Session ID"
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name (optional)"
                    }
                },
                "required": ["session_id"]
            }
        },
        {
            "name": "clear_messages",
            "description": "Clear all messages from a session",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "Session ID"
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name (optional)"
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
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name (optional)"
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
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name (optional)"
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
                    },
                    "space": {
                        "type": "string",
                        "description": "Space name (optional)"
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
            "name": "create_agent",
            "description": "Create a new agent",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "space": {
                        "type": "string",
                        "description": "Space name"
                    },
                    "name": {
                        "type": "string",
                        "description": "Agent name"
                    },
                    "description": {
                        "type": "string",
                        "description": "Agent description"
                    },
                    "purpose": {
                        "type": "string",
                        "description": "Agent purpose"
                    },
                    "source_repo": {
                        "type": "string",
                        "description": "Source repository"
                    },
                    "source_branch": {
                        "type": "string",
                        "description": "Source branch"
                    }
                },
                "required": ["space", "name"]
            }
        },
        {
            "name": "get_agent",
            "description": "Get a specific agent",
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
            "name": "update_agent",
            "description": "Update an agent",
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
                    },
                    "description": {
                        "type": "string",
                        "description": "Agent description"
                    },
                    "purpose": {
                        "type": "string",
                        "description": "Agent purpose"
                    }
                },
                "required": ["space", "agent_name"]
            }
        },
        {
            "name": "delete_agent",
            "description": "Delete an agent",
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
            "name": "update_agent_status",
            "description": "Update agent status",
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
                    },
                    "status": {
                        "type": "string",
                        "description": "New agent status",
                        "enum": ["active", "inactive", "running", "stopped", "error"]
                    }
                },
                "required": ["space", "agent_name", "status"]
            }
        },
        {
            "name": "deploy_agent",
            "description": "Deploy an agent",
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
            "name": "stop_agent",
            "description": "Stop an agent",
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
            "name": "list_running_agents",
            "description": "List running agents in a space",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "space": {
                        "type": "string",
                        "description": "Space name"
                    }
                },
                "required": ["space"]
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
            "name": "create_secret",
            "description": "Create a new secret",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "space": {
                        "type": "string",
                        "description": "Space name"
                    },
                    "key_name": {
                        "type": "string",
                        "description": "Secret key name"
                    },
                    "value": {
                        "type": "string",
                        "description": "Secret value"
                    },
                    "description": {
                        "type": "string",
                        "description": "Secret description"
                    }
                },
                "required": ["space", "key_name", "value"]
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
            "name": "update_secret",
            "description": "Update a secret value",
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
                        "description": "New secret value"
                    },
                    "description": {
                        "type": "string",
                        "description": "Secret description"
                    }
                },
                "required": ["space", "key"]
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
            "name": "create_build",
            "description": "Trigger a space build",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "space": {
                        "type": "string",
                        "description": "Space name"
                    },
                    "dockerfile": {
                        "type": "string",
                        "description": "Dockerfile content"
                    },
                    "context": {
                        "type": "string",
                        "description": "Build context"
                    }
                },
                "required": ["space"]
            }
        },
        {
            "name": "get_latest_build",
            "description": "Get latest build status",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "space": {
                        "type": "string",
                        "description": "Space name"
                    }
                },
                "required": ["space"]
            }
        },
        {
            "name": "get_build",
            "description": "Get specific build status",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "space": {
                        "type": "string",
                        "description": "Space name"
                    },
                    "build_id": {
                        "type": "string",
                        "description": "Build ID"
                    }
                },
                "required": ["space", "build_id"]
            }
        }
    ]
}"#;
