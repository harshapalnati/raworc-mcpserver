use crate::client::RaworcClient;
use crate::error::{RaworcError, RaworcResult};
use crate::models::*; // ensure ToolCallContent has #[serde(rename = "type")] on content_type
use crate::Config;
use serde_json::{self, Value};
use std::collections::HashMap;
use tracing::{debug, info};

/// Raworc MCP Server
pub struct RaworcMcpServer {
    client: RaworcClient,
    config: Config,
}

impl RaworcMcpServer {
    /// Create a new MCP server
    pub fn new(config: Config) -> RaworcResult<Self> {
        let client = RaworcClient::new(&config)?;
        Ok(Self { client, config })
    }

    /// Initialize (authenticate lazily if user/pass provided and no token)
    pub async fn initialize(&mut self) -> RaworcResult<()> {
        if self.config.username.is_some()
            && self.config.password.is_some()
            && self.config.auth_token.is_none()
        {
            let username = self.config.username.as_ref().unwrap();
            let password = self.config.password.as_ref().unwrap();
            info!("Authenticating as service account");
            self.client.authenticate(username, password).await?;
            info!("Authentication successful");
        }
        Ok(())
    }

    /// Dispatch a tool call by name
    pub async fn handle_tool_call(
        &mut self,
        name: &str,
        arguments: &Value
    ) -> RaworcResult<ToolCallResponse> {
        debug!("Tool call: {name} args={arguments:?}");

        // Lazy auth only when needed
        self.initialize().await?;

        let content = match name {
            "list_sessions"   => self.handle_list_sessions(arguments).await?,
            "create_session"  => self.handle_create_session(arguments).await?,
            "get_session"     => self.handle_get_session(arguments).await?,
            "send_message"    => self.handle_send_message(arguments).await?,
            "get_messages"    => self.handle_get_messages(arguments).await?,
            "pause_session"   => self.handle_pause_session(arguments).await?,
            "resume_session"  => self.handle_resume_session(arguments).await?,
            "terminate_session" => self.handle_terminate_session(arguments).await?,
            "list_spaces"     => self.handle_list_spaces(arguments).await?,
            "list_agents"     => self.handle_list_agents(arguments).await?,
            "get_agent_logs"  => self.handle_get_agent_logs(arguments).await?,
            "list_secrets"    => self.handle_list_secrets(arguments).await?,
            "get_secret"      => self.handle_get_secret(arguments).await?,
            "set_secret"      => self.handle_set_secret(arguments).await?,
            "delete_secret"   => self.handle_delete_secret(arguments).await?,
            "health_check"    => self.handle_health_check(arguments).await?,
            "get_version"     => self.handle_get_version(arguments).await?,
            _ => return Err(RaworcError::mcp_error(&format!("Unknown tool: {name}")))
        };

        Ok(ToolCallResponse { content })
    }

    // ---------- Helpers ----------
    #[inline]
    fn text_content<S: Into<String>>(s: S) -> Vec<ToolCallContent> {
        vec![ToolCallContent {
            content_type: "text".to_string(),
            text: Some(s.into()),
            image_url: None,
        }]
    }

    // ---------- Tool handlers ----------

    async fn handle_list_sessions(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str());
        let sessions = self.client.list_sessions(space).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&sessions)?))
    }

    async fn handle_create_session(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str());
        let metadata = arguments
            .get("metadata")
            .and_then(|v| v.as_object())
            .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<HashMap<String, Value>>());
        let session = self.client.create_session(space, metadata).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&session)?))
    }

    async fn handle_get_session(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments
            .get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str());
        let session = self.client.get_session(space, session_id).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&session)?))
    }

    async fn handle_send_message(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments
            .get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        let content = arguments
            .get("content").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("content is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str());
        let message = self.client.send_message(space, session_id, content).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&message)?))
    }

    async fn handle_get_messages(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments
            .get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        let limit = arguments.get("limit").and_then(|v| v.as_u64());
        let space = arguments.get("space").and_then(|v| v.as_str());
        let messages = self.client.get_messages(space, session_id, limit).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&messages)?))
    }

    async fn handle_pause_session(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments
            .get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str());
        self.client.pause_session(space, session_id).await?;
        Ok(Self::text_content("Session paused successfully"))
    }

    async fn handle_resume_session(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments
            .get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str());
        self.client.resume_session(space, session_id).await?;
        Ok(Self::text_content("Session resumed successfully"))
    }

    async fn handle_terminate_session(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments
            .get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str());
        self.client.terminate_session(space, session_id).await?;
        Ok(Self::text_content("Session terminated successfully"))
    }

    async fn handle_list_spaces(&self, _arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let spaces = self.client.list_spaces().await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&spaces)?))
    }

    async fn handle_list_agents(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str());
        let agents = self.client.list_agents(space).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&agents)?))
    }

    async fn handle_get_agent_logs(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let agent_name = arguments.get("agent_name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("agent_name is required"))?;
        let logs = self.client.get_agent_logs(space, agent_name).await?;
        Ok(Self::text_content(logs))
    }

    async fn handle_list_secrets(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str());
        let secrets = self.client.list_secrets(space).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&secrets)?))
    }

    async fn handle_get_secret(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let key = arguments.get("key").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("key is required"))?;
        let secret = self.client.get_secret(space, key).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&secret)?))
    }

    async fn handle_set_secret(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let key = arguments.get("key").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("key is required"))?;
        let value = arguments.get("value").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("value is required"))?;
        let secret = self.client.set_secret(space, key, value).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&secret)?))
    }

    async fn handle_delete_secret(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let key = arguments.get("key").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("key is required"))?;
        self.client.delete_secret(space, key).await?;
        Ok(Self::text_content("Secret deleted successfully"))
    }

    async fn handle_health_check(&self, _arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let health = self.client.health_check().await?;
        Ok(Self::text_content(health))
    }

    async fn handle_get_version(&self, _arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let version = self.client.get_version().await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&version)?))
    }
}
