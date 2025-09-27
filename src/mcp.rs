use crate::client::RaworcClient;
use crate::error::{RaworcError, RaworcResult};
use crate::models::*; // ToolCallContent has #[serde(rename = "type")] on content_type
use crate::Config;
use serde_json::{self, Value};
use tracing::{debug, info};

/// Raworc MCP Server aligned to ra-hyp-1 REST API (v0)
pub struct RaworcMcpServer {
    client: RaworcClient,
    config: Config,
}

impl RaworcMcpServer {
    pub fn new(config: Config) -> RaworcResult<Self> {
        let client = RaworcClient::new(&config)?;
        Ok(Self { client, config })
    }

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

    pub async fn handle_tool_call(
        &mut self,
        name: &str,
        arguments: &Value,
    ) -> RaworcResult<ToolCallResponse> {
        debug!("Tool call: {name} args={arguments:?}");
        self.initialize().await?;

        let content = match name {
            // Version
            "get_version" => self.handle_get_version(arguments).await?,

            // Agents
            "agents_list" => self.handle_agents_list(arguments).await?,
            "agent_create" => self.handle_agent_create(arguments).await?,
            "agent_get" => self.handle_agent_get(arguments).await?,
            "agent_update" => self.handle_agent_update(arguments).await?,
            "agent_delete" => self.handle_agent_delete(arguments).await?,
            "agent_state_set" => self.handle_agent_state_set(arguments).await?,
            "agent_busy" => self.handle_agent_busy(arguments).await?,
            "agent_idle" => self.handle_agent_idle(arguments).await?,
            "agent_sleep" => self.handle_agent_sleep(arguments).await?,
            "agent_cancel" => self.handle_agent_cancel(arguments).await?,
            "agent_wake" => self.handle_agent_wake(arguments).await?,
            "agent_runtime" => self.handle_agent_runtime(arguments).await?,
            "agent_remix" => self.handle_agent_remix(arguments).await?,
            "agent_publish" => self.handle_agent_publish(arguments).await?,
            "agent_unpublish" => self.handle_agent_unpublish(arguments).await?,

            // Responses
            "responses_list" => self.handle_responses_list(arguments).await?,
            "response_create" => self.handle_response_create(arguments).await?,
            "response_get" => self.handle_response_get(arguments).await?,
            "response_update" => self.handle_response_update(arguments).await?,
            "responses_count" => self.handle_responses_count(arguments).await?,

            // Files
            "files_list_root" => self.handle_files_list_root(arguments).await?,
            "files_list_path" => self.handle_files_list_path(arguments).await?,
            "files_metadata" => self.handle_files_metadata(arguments).await?,
            "files_read" => self.handle_files_read(arguments).await?,
            "files_delete" => self.handle_files_delete(arguments).await?,

            // Context
            "context_get" => self.handle_context_get(arguments).await?,
            "context_clear" => self.handle_context_clear(arguments).await?,
            "context_compact" => self.handle_context_compact(arguments).await?,

            // Published
            "published_agents_list" => self.handle_published_agents_list(arguments).await?,
            "published_agent_get" => self.handle_published_agent_get(arguments).await?,

            _ => return Err(RaworcError::mcp_error(&format!("Unknown tool: {name}"))),
        };
        Ok(ToolCallResponse { content })
    }

    #[inline]
    fn text_content<S: Into<String>>(s: S) -> Vec<ToolCallContent> {
        vec![ToolCallContent { content_type: "text".to_string(), text: Some(s.into()), image_url: None }]
    }

    // Version
    async fn handle_get_version(&self, _arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let v = self.client.ra_get_version().await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&v)?))
    }

    // Agents
    async fn handle_agents_list(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let q = arguments.get("q").and_then(|v| v.as_str());
        let tags = arguments.get("tags").and_then(|v| v.as_str());
        let state = arguments.get("state").and_then(|v| v.as_str());
        let limit = arguments.get("limit").and_then(|v| v.as_u64()).map(|n| n as u32);
        let page = arguments.get("page").and_then(|v| v.as_u64()).map(|n| n as u32);
        let offset = arguments.get("offset").and_then(|v| v.as_u64()).map(|n| n as u32);
        let res = self.client.agents_list(q, tags, state, limit, page, offset).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_create(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let body = arguments.clone();
        let res = self.client.agent_create(&body).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_get(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let res = self.client.agent_get(name).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_update(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let mut body = arguments.clone();
        if let Some(map) = body.as_object_mut() { map.remove("name"); }
        let res = self.client.agent_update(name, &body).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_delete(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        self.client.agent_delete(name).await?;
        Ok(Self::text_content("Agent deleted"))
    }

    async fn handle_agent_state_set(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let state = arguments.get("state").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("state is required"))?;
        let res = self.client.agent_set_state(name, state).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_busy(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let res = self.client.agent_busy(name).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_idle(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let res = self.client.agent_idle(name).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_sleep(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let delay_seconds = arguments.get("delay_seconds").and_then(|v| v.as_u64()).map(|n| n as u32);
        let note = arguments.get("note").and_then(|v| v.as_str());
        let res = self.client.agent_sleep(name, delay_seconds, note).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_cancel(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let res = self.client.agent_cancel(name).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_wake(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let prompt = arguments.get("prompt").and_then(|v| v.as_str());
        let res = self.client.agent_wake(name, prompt).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_runtime(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let res = self.client.agent_runtime(name).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_remix(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name (parent) is required"))?;
        let mut body = arguments.clone();
        if let Some(map) = body.as_object_mut() { map.remove("name"); }
        let res = self.client.agent_remix(name, &body).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_publish(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let mut body = arguments.clone();
        if let Some(map) = body.as_object_mut() { map.remove("name"); }
        let res = self.client.agent_publish(name, &body).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_agent_unpublish(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let res = self.client.agent_unpublish(name).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    // Responses
    async fn handle_responses_list(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let limit = arguments.get("limit").and_then(|v| v.as_u64()).map(|n| n as u32);
        let offset = arguments.get("offset").and_then(|v| v.as_u64()).map(|n| n as u32);
        let res = self.client.responses_list(agent, limit, offset).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_response_create(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let mut body = arguments.clone();
        if let Some(map) = body.as_object_mut() { map.remove("agent"); }
        let res = self.client.response_create(agent, &body).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_response_get(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let id = arguments.get("id").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("id is required"))?;
        let res = self.client.response_get(agent, id).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_response_update(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let id = arguments.get("id").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("id is required"))?;
        let mut body = arguments.clone();
        if let Some(map) = body.as_object_mut() { map.remove("agent"); map.remove("id"); }
        let res = self.client.response_update(agent, id, &body).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_responses_count(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let res = self.client.responses_count(agent).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    // Files
    async fn handle_files_list_root(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let offset = arguments.get("offset").and_then(|v| v.as_u64()).map(|n| n as u32);
        let limit = arguments.get("limit").and_then(|v| v.as_u64()).map(|n| n as u32);
        let res = self.client.files_list_root(agent, offset, limit).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_files_list_path(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let path = arguments.get("path").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("path is required"))?;
        let offset = arguments.get("offset").and_then(|v| v.as_u64()).map(|n| n as u32);
        let limit = arguments.get("limit").and_then(|v| v.as_u64()).map(|n| n as u32);
        let res = self.client.files_list_path(agent, path, offset, limit).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_files_metadata(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let path = arguments.get("path").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("path is required"))?;
        let res = self.client.files_metadata(agent, path).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_files_read(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let path = arguments.get("path").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("path is required"))?;
        let bytes = self.client.files_read(agent, path).await?;
        let b64 = base64::encode(bytes);
        Ok(Self::text_content(format!("base64: {}", b64)))
    }

    async fn handle_files_delete(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let path = arguments.get("path").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("path is required"))?;
        let res = self.client.files_delete(agent, path).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    // Context
    async fn handle_context_get(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let res = self.client.context_get(agent).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_context_clear(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let res = self.client.context_clear(agent).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_context_compact(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let agent = arguments.get("agent").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("agent is required"))?;
        let res = self.client.context_compact(agent).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    // Published
    async fn handle_published_agents_list(&self, _arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let res = self.client.published_agents().await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }

    async fn handle_published_agent_get(&self, arguments: &Value) -> RaworcResult<Vec<ToolCallContent>> {
        let name = arguments.get("name").and_then(|v| v.as_str()).ok_or_else(|| RaworcError::validation_error("name is required"))?;
        let res = self.client.published_agent_get(name).await?;
        Ok(Self::text_content(serde_json::to_string_pretty(&res)?))
    }
}

