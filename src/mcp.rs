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

    // Service Accounts
    async fn handle_list_service_accounts(&self, _arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let accounts = self.client.list_service_accounts().await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&accounts)?))
    }

    async fn handle_create_service_account(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let user = arguments.get("user").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("user is required"))?;
        let pass = arguments.get("pass").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("pass is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str()).map(|s| s.to_string());
        let description = arguments.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        let request = CreateServiceAccountRequest {
            user: user.to_string(),
            pass: pass.to_string(),
            space,
            description,
        };
        let account = self.client.create_service_account(&request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&account)?))
    }

    async fn handle_get_service_account(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let id = arguments.get("id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("id is required"))?;
        let account = self.client.get_service_account(id).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&account)?))
    }

    async fn handle_update_service_account(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let id = arguments.get("id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("id is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str()).map(|s| s.to_string());
        let description = arguments.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        let active = arguments.get("active").and_then(|v| v.as_bool());
        
        let request = UpdateServiceAccountRequest {
            space,
            description,
            active,
        };
        let account = self.client.update_service_account(id, &request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&account)?))
    }

    async fn handle_delete_service_account(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let id = arguments.get("id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("id is required"))?;
        self.client.delete_service_account(id).await?;
        Ok(Self::text_content("Service account deleted successfully"))
    }

    async fn handle_update_service_account_password(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let id = arguments.get("id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("id is required"))?;
        let current_password = arguments.get("current_password").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("current_password is required"))?;
        let new_password = arguments.get("new_password").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("new_password is required"))?;
        
        let request = UpdatePasswordRequest {
            current_password: current_password.to_string(),
            new_password: new_password.to_string(),
        };
        self.client.update_service_account_password(id, &request).await?;
        Ok(Self::text_content("Password updated successfully"))
    }

    // Roles
    async fn handle_list_roles(&self, _arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let roles = self.client.list_roles().await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&roles)?))
    }

    async fn handle_create_role(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let id = arguments.get("id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("id is required"))?;
        let description = arguments.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        let rules_value = arguments.get("rules")
            .ok_or_else(|| RaworcError::validation_error("rules is required"))?;
        let rules: Vec<RoleRule> = serde_json::from_value(rules_value.clone())?;
        
        let request = CreateRoleRequest {
            id: id.to_string(),
            description,
            rules,
        };
        let role = self.client.create_role(&request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&role)?))
    }

    async fn handle_get_role(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let id = arguments.get("id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("id is required"))?;
        let role = self.client.get_role(id).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&role)?))
    }

    async fn handle_delete_role(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let id = arguments.get("id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("id is required"))?;
        self.client.delete_role(id).await?;
        Ok(Self::text_content("Role deleted successfully"))
    }

    // Role Bindings
    async fn handle_list_role_bindings(&self, _arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let bindings = self.client.list_role_bindings().await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&bindings)?))
    }

    async fn handle_create_role_binding(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let subject = arguments.get("subject").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("subject is required"))?;
        let role_ref = arguments.get("role_ref").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("role_ref is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        let request = CreateRoleBindingRequest {
            subject: subject.to_string(),
            role_ref: role_ref.to_string(),
            space,
        };
        let binding = self.client.create_role_binding(&request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&binding)?))
    }

    async fn handle_get_role_binding(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let id = arguments.get("id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("id is required"))?;
        let binding = self.client.get_role_binding(id).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&binding)?))
    }

    async fn handle_delete_role_binding(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let id = arguments.get("id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("id is required"))?;
        self.client.delete_role_binding(id).await?;
        Ok(Self::text_content("Role binding deleted successfully"))
    }

    // Additional space methods
    async fn handle_create_space(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let description = arguments.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        let settings = arguments.get("settings").and_then(|v| v.as_object())
            .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<HashMap<String, Value>>());
        
        let request = CreateSpaceRequest {
            name: name.to_string(),
            description,
            settings,
        };
        let space = self.client.create_space(&request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&space)?))
    }

    async fn handle_get_space(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let space = self.client.get_space(name).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&space)?))
    }

    async fn handle_update_space(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let description = arguments.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        let settings = arguments.get("settings").and_then(|v| v.as_object())
            .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<HashMap<String, Value>>());
        
        let request = UpdateSpaceRequest {
            description,
            settings,
        };
        let space = self.client.update_space(name, &request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&space)?))
    }

    async fn handle_delete_space(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("name is required"))?;
        self.client.delete_space(name).await?;
        Ok(Self::text_content("Space deleted successfully"))
    }

    // Additional session methods
    async fn handle_update_session(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments.get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str());
        let metadata = arguments.get("metadata").and_then(|v| v.as_object())
            .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<HashMap<String, Value>>());
        
        let request = UpdateSessionRequest {
            space: space.map(|s| s.to_string()),
            metadata,
        };
        let session = self.client.update_session(space, session_id, &request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&session)?))
    }

    async fn handle_update_session_state(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments.get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str());
        let state_str = arguments.get("state").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("state is required"))?;
        
        let state = match state_str {
            "INIT" => SessionState::Init,
            "RUNNING" => SessionState::Running,
            "PAUSED" => SessionState::Paused,
            "SUSPENDED" => SessionState::Suspended,
            "TERMINATED" => SessionState::Terminated,
            "IDLE" => SessionState::Idle,
            "CLOSED" => SessionState::Closed,
            _ => return Err(RaworcError::validation_error("Invalid session state")),
        };
        
        self.client.update_session_state(space, session_id, state).await?;
        Ok(Self::text_content("Session state updated successfully"))
    }

    async fn handle_close_session(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments.get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        self.client.close_session(session_id).await?;
        Ok(Self::text_content("Session closed successfully"))
    }

    async fn handle_restore_session(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments.get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        self.client.restore_session(session_id).await?;
        Ok(Self::text_content("Session restored successfully"))
    }

    async fn handle_remix_session(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments.get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        let request = CreateSessionRequest {
            space,
            metadata: None,
        };
        let session = self.client.remix_session(session_id, &request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&session)?))
    }

    // Additional message methods
    async fn handle_get_message_count(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments.get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str());
        let count = self.client.get_message_count(space, session_id).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&count)?))
    }

    async fn handle_clear_messages(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let session_id = arguments.get("session_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("session_id is required"))?;
        let space = arguments.get("space").and_then(|v| v.as_str());
        self.client.clear_messages(space, session_id).await?;
        Ok(Self::text_content("Messages cleared successfully"))
    }

    // Additional agent methods
    async fn handle_create_agent(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let name = arguments.get("name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let description = arguments.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        let purpose = arguments.get("purpose").and_then(|v| v.as_str()).map(|s| s.to_string());
        let source_repo = arguments.get("source_repo").and_then(|v| v.as_str()).map(|s| s.to_string());
        let source_branch = arguments.get("source_branch").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        let request = CreateAgentRequest {
            name: name.to_string(),
            description,
            purpose,
            source_repo,
            source_branch,
            image: None,
            command: None,
            env: None,
            resources: None,
        };
        let agent = self.client.create_agent(space, &request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&agent)?))
    }

    async fn handle_get_agent(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let agent_name = arguments.get("agent_name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("agent_name is required"))?;
        let agent = self.client.get_agent(space, agent_name).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&agent)?))
    }

    async fn handle_update_agent(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let agent_name = arguments.get("agent_name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("agent_name is required"))?;
        let description = arguments.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        let purpose = arguments.get("purpose").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        let request = UpdateAgentRequest {
            description,
            purpose,
            source_repo: None,
            source_branch: None,
            image: None,
            command: None,
            env: None,
            resources: None,
        };
        let agent = self.client.update_agent(space, agent_name, &request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&agent)?))
    }

    async fn handle_delete_agent(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let agent_name = arguments.get("agent_name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("agent_name is required"))?;
        self.client.delete_agent(space, agent_name).await?;
        Ok(Self::text_content("Agent deleted successfully"))
    }

    async fn handle_update_agent_status(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let agent_name = arguments.get("agent_name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("agent_name is required"))?;
        let status_str = arguments.get("status").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("status is required"))?;
        
        let status = match status_str {
            "active" => AgentStatus::Active,
            "inactive" => AgentStatus::Inactive,
            "running" => AgentStatus::Running,
            "stopped" => AgentStatus::Stopped,
            "error" => AgentStatus::Error,
            _ => return Err(RaworcError::validation_error("Invalid agent status")),
        };
        
        let request = UpdateAgentStatusRequest { status };
        self.client.update_agent_status(space, agent_name, &request).await?;
        Ok(Self::text_content("Agent status updated successfully"))
    }

    async fn handle_deploy_agent(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let agent_name = arguments.get("agent_name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("agent_name is required"))?;
        self.client.deploy_agent(space, agent_name).await?;
        Ok(Self::text_content("Agent deployed successfully"))
    }

    async fn handle_stop_agent(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let agent_name = arguments.get("agent_name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("agent_name is required"))?;
        self.client.stop_agent(space, agent_name).await?;
        Ok(Self::text_content("Agent stopped successfully"))
    }

    async fn handle_list_running_agents(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let agents = self.client.list_running_agents(space).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&agents)?))
    }

    // Additional secret methods
    async fn handle_create_secret(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let key_name = arguments.get("key_name").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("key_name is required"))?;
        let value = arguments.get("value").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("value is required"))?;
        let description = arguments.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        let request = CreateSecretRequest {
            key_name: key_name.to_string(),
            value: value.to_string(),
            description,
        };
        let secret = self.client.create_secret(space, &request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&secret)?))
    }

    async fn handle_update_secret(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let key = arguments.get("key").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("key is required"))?;
        let value = arguments.get("value").and_then(|v| v.as_str()).map(|s| s.to_string());
        let description = arguments.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        let request = UpdateSecretRequest {
            value,
            description,
        };
        let secret = self.client.update_secret(space, key, &request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&secret)?))
    }

    // Build methods
    async fn handle_create_build(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let dockerfile = arguments.get("dockerfile").and_then(|v| v.as_str()).map(|s| s.to_string());
        let context = arguments.get("context").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        let request = CreateBuildRequest {
            dockerfile,
            context,
        };
        let build = self.client.create_build(space, &request).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&build)?))
    }

    async fn handle_get_latest_build(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let build = self.client.get_latest_build(space).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&build)?))
    }

    async fn handle_get_build(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let space = arguments.get("space").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("space is required"))?;
        let build_id = arguments.get("build_id").and_then(|v| v.as_str())
            .ok_or_else(|| RaworcError::validation_error("build_id is required"))?;
        let build = self.client.get_build(space, build_id).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&build)?))
    }
}
