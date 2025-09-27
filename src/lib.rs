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

/// MCP capabilities constant (aligned with ra-hyp-1 REST API v0)
pub const CAPABILITIES: &str = r#"{
  "tools": [
    {"name": "get_version", "description": "Get API version", "inputSchema": {"type":"object","properties":{}}},
    {"name": "agents_list", "description": "List/search agents", "inputSchema": {"type":"object","properties":{"q":{"type":"string"},"tags":{"type":"string"},"state":{"type":"string"},"limit":{"type":"number"},"page":{"type":"number"},"offset":{"type":"number"}}}},
    {"name": "agent_create", "description": "Create agent", "inputSchema": {"type":"object","properties":{"name":{"type":"string"},"description":{"type":"string"},"metadata":{"type":"object"},"tags":{"type":"array","items":{"type":"string"}},"secrets":{"type":"object"},"instructions":{"type":"string"},"setup":{"type":"string"},"prompt":{"type":"string"},"idle_timeout_seconds":{"type":"number"},"busy_timeout_seconds":{"type":"number"}},"required":["name"]}},
    {"name": "agent_get", "description": "Get agent by name", "inputSchema": {"type":"object","properties":{"name":{"type":"string"}},"required":["name"]}},
    {"name": "agent_update", "description": "Update agent", "inputSchema": {"type":"object","properties":{"name":{"type":"string"},"metadata":{"type":"object"},"description":{"type":"string"},"tags":{"type":"array","items":{"type":"string"}},"idle_timeout_seconds":{"type":"number"},"busy_timeout_seconds":{"type":"number"}},"required":["name"]}},
    {"name": "agent_delete", "description": "Delete agent", "inputSchema": {"type":"object","properties":{"name":{"type":"string"}},"required":["name"]}},
    {"name": "agent_state_set", "description": "Update agent state", "inputSchema": {"type":"object","properties":{"name":{"type":"string"},"state":{"type":"string"}},"required":["name","state"]}},
    {"name": "agent_busy", "description": "Set agent busy", "inputSchema": {"type":"object","properties":{"name":{"type":"string"}},"required":["name"]}},
    {"name": "agent_idle", "description": "Set agent idle", "inputSchema": {"type":"object","properties":{"name":{"type":"string"}},"required":["name"]}},
    {"name": "agent_sleep", "description": "Schedule sleep", "inputSchema": {"type":"object","properties":{"name":{"type":"string"},"delay_seconds":{"type":"number"},"note":{"type":"string"}},"required":["name"]}},
    {"name": "agent_cancel", "description": "Cancel latest pending/processing response", "inputSchema": {"type":"object","properties":{"name":{"type":"string"}},"required":["name"]}},
    {"name": "agent_wake", "description": "Wake agent (optional prompt)", "inputSchema": {"type":"object","properties":{"name":{"type":"string"},"prompt":{"type":"string"}},"required":["name"]}},
    {"name": "agent_runtime", "description": "Get runtime totals", "inputSchema": {"type":"object","properties":{"name":{"type":"string"}},"required":["name"]}},
    {"name": "agent_remix", "description": "Remix agent", "inputSchema": {"type":"object","properties":{"name":{"type":"string"},"new_name":{"type":"string"},"metadata":{"type":"object"},"code":{"type":"boolean"},"secrets":{"type":"boolean"},"content":{"type":"boolean"},"prompt":{"type":"string"}},"required":["name","new_name"]}},
    {"name": "agent_publish", "description": "Publish agent", "inputSchema": {"type":"object","properties":{"name":{"type":"string"},"code":{"type":"boolean"},"secrets":{"type":"boolean"},"content":{"type":"boolean"}},"required":["name"]}},
    {"name": "agent_unpublish", "description": "Unpublish agent", "inputSchema": {"type":"object","properties":{"name":{"type":"string"}},"required":["name"]}},
    {"name": "responses_list", "description": "List responses for agent", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"},"limit":{"type":"number"},"offset":{"type":"number"}},"required":["agent"]}},
    {"name": "response_create", "description": "Create response", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"},"input":{"type":"object"},"background":{"type":"boolean"}},"required":["agent","input"]}},
    {"name": "response_get", "description": "Get response by id", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"},"id":{"type":"string"}},"required":["agent","id"]}},
    {"name": "response_update", "description": "Update a response", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"},"id":{"type":"string"},"status":{"type":"string"},"input":{"type":"object"},"output":{"type":"object"}},"required":["agent","id"]}},
    {"name": "responses_count", "description": "Get response count", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"}},"required":["agent"]}},
    {"name": "files_list_root", "description": "List /agent root", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"},"offset":{"type":"number"},"limit":{"type":"number"}},"required":["agent"]}},
    {"name": "files_list_path", "description": "List under path", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"},"path":{"type":"string"},"offset":{"type":"number"},"limit":{"type":"number"}},"required":["agent","path"]}},
    {"name": "files_metadata", "description": "File/dir metadata", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"},"path":{"type":"string"}},"required":["agent","path"]}},
    {"name": "files_read", "description": "Read file (base64)", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"},"path":{"type":"string"}},"required":["agent","path"]}},
    {"name": "files_delete", "description": "Delete file/empty dir", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"},"path":{"type":"string"}},"required":["agent","path"]}},
    {"name": "context_get", "description": "Get context usage", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"}},"required":["agent"]}},
    {"name": "context_clear", "description": "Clear context (cutoff)", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"}},"required":["agent"]}},
    {"name": "context_compact", "description": "Compact context via LLM", "inputSchema": {"type":"object","properties":{"agent":{"type":"string"}},"required":["agent"]}},
    {"name": "published_agents_list", "description": "List published agents", "inputSchema": {"type":"object","properties":{}}},
    {"name": "published_agent_get", "description": "Get published agent", "inputSchema": {"type":"object","properties":{"name":{"type":"string"}},"required":["name"]}}
  ]
}"#;

