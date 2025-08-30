use crate::error::{RaworcError, RaworcResult, ApiErrorResponse};
use crate::models::*;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

/// Raworc API client
pub struct RaworcClient {
    client: Client,
    base_url: Url,
    auth_token: Option<String>,
    default_space: Option<String>,
}

impl RaworcClient {
    /// Create a new Raworc client
    pub fn new(config: &crate::Config) -> RaworcResult<Self> {
        let base_url = Url::parse(&config.api_url)
            .map_err(|e| RaworcError::ConfigError(format!("Invalid API URL: {}", e)))?;

        let timeout = Duration::from_secs(config.timeout_seconds.unwrap_or(30));
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| RaworcError::ConfigError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            base_url,
            auth_token: config.auth_token.clone(),
            default_space: config.default_space.clone(),
        })
    }

    /// Authenticate with username and password
    pub async fn authenticate(&mut self, username: &str, password: &str) -> RaworcResult<()> {
        let auth_request = AuthRequest {
            user: username.to_string(),
            pass: password.to_string(),
        };

        let response: AuthResponse = self
            .post("/auth/login", &auth_request)
            .await?;

        self.auth_token = Some(response.token);
        Ok(())
    }

    /// Get current user info
    pub async fn get_user_info(&self) -> RaworcResult<UserInfo> {
        self.get("/auth/me").await
    }

    /// Check API health
    pub async fn health_check(&self) -> RaworcResult<String> {
        let response = self.client.get(self.build_url("/health")).send().await?;
        response.text().await.map_err(RaworcError::from)
    }

    /// Get API version
    pub async fn get_version(&self) -> RaworcResult<VersionResponse> {
        self.get("/version").await
    }

    // Session management
    /// List sessions
    pub async fn list_sessions(&self, space: Option<&str>) -> RaworcResult<Vec<Session>> {
        let space = space.unwrap_or_else(|| self.default_space.as_deref().unwrap_or("default"));
        let url = format!("/sessions?space={}", space);
        self.get(&url).await
    }

    /// Create a new session
    pub async fn create_session(
        &self,
        space: Option<&str>,
        metadata: Option<HashMap<String, Value>>,
    ) -> RaworcResult<Session> {
        let space = space.unwrap_or_else(|| self.default_space.as_deref().unwrap_or("default"));
        let request = CreateSessionRequest {
            space: Some(space.to_string()),
            metadata,
        };
        self.post("/sessions", &request).await
    }

    /// Get session by ID
    pub async fn get_session(&self, session_id: &str) -> RaworcResult<Session> {
        let url = format!("/sessions/{}", session_id);
        self.get(&url).await
    }

    /// Update session
    pub async fn update_session(
        &self,
        session_id: &str,
        request: &UpdateSessionRequest,
    ) -> RaworcResult<Session> {
        let url = format!("/sessions/{}", session_id);
        self.put(&url, request).await
    }

    /// Update session state
    pub async fn update_session_state(
        &self,
        session_id: &str,
        state: SessionState,
    ) -> RaworcResult<()> {
        let url = format!("/sessions/{}/state", session_id);
        let request = UpdateSessionStateRequest { state };
        self.put::<_, ()>(&url, &request).await?;
        Ok(())
    }

    /// Pause session
    pub async fn pause_session(&self, session_id: &str) -> RaworcResult<()> {
        let url = format!("/sessions/{}/pause", session_id);
        self.post::<_, ()>(&url, &()).await?;
        Ok(())
    }

    /// Resume session
    pub async fn resume_session(&self, session_id: &str) -> RaworcResult<()> {
        let url = format!("/sessions/{}/resume", session_id);
        self.post::<_, ()>(&url, &()).await?;
        Ok(())
    }

    /// Terminate session
    pub async fn terminate_session(&self, session_id: &str) -> RaworcResult<()> {
        let url = format!("/sessions/{}", session_id);
        self.delete(&url).await?;
        Ok(())
    }

    // Message management
    /// Get messages from session
    pub async fn get_messages(
        &self,
        session_id: &str,
        limit: Option<u64>,
    ) -> RaworcResult<Vec<Message>> {
        let mut url = format!("/sessions/{}/messages", session_id);
        if let Some(limit) = limit {
            url.push_str(&format!("?limit={}", limit));
        }
        self.get(&url).await
    }

    /// Send message to session
    pub async fn send_message(&self, session_id: &str, content: &str) -> RaworcResult<Message> {
        let url = format!("/sessions/{}/messages", session_id);
        let request = CreateMessageRequest {
            content: content.to_string(),
        };
        self.post(&url, &request).await
    }

    /// Get message count
    pub async fn get_message_count(&self, session_id: &str) -> RaworcResult<MessageCount> {
        let url = format!("/sessions/{}/messages/count", session_id);
        self.get(&url).await
    }

    /// Clear session messages
    pub async fn clear_messages(&self, session_id: &str) -> RaworcResult<()> {
        let url = format!("/sessions/{}/messages", session_id);
        self.delete(&url).await?;
        Ok(())
    }

    // Space management
    /// List spaces
    pub async fn list_spaces(&self) -> RaworcResult<Vec<Space>> {
        self.get("/spaces").await
    }

    /// Create space
    pub async fn create_space(&self, name: &str, description: Option<&str>) -> RaworcResult<Space> {
        let request = CreateSpaceRequest {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
        };
        self.post("/spaces", &request).await
    }

    /// Get space by name
    pub async fn get_space(&self, name: &str) -> RaworcResult<Space> {
        let url = format!("/spaces/{}", name);
        self.get(&url).await
    }

    /// Update space
    pub async fn update_space(&self, name: &str, description: Option<&str>) -> RaworcResult<Space> {
        let url = format!("/spaces/{}", name);
        let request = UpdateSpaceRequest {
            description: description.map(|s| s.to_string()),
        };
        self.put(&url, &request).await
    }

    /// Delete space
    pub async fn delete_space(&self, name: &str) -> RaworcResult<()> {
        let url = format!("/spaces/{}", name);
        self.delete(&url).await?;
        Ok(())
    }

    // Agent management
    /// List agents in space
    pub async fn list_agents(&self, space: Option<&str>) -> RaworcResult<Vec<Agent>> {
        let space = space.unwrap_or_else(|| self.default_space.as_deref().unwrap_or("default"));
        let url = format!("/spaces/{}/agents", space);
        self.get(&url).await
    }

    /// Create agent
    pub async fn create_agent(&self, space: &str, request: &CreateAgentRequest) -> RaworcResult<Agent> {
        let url = format!("/spaces/{}/agents", space);
        self.post(&url, request).await
    }

    /// Get agent by name
    pub async fn get_agent(&self, space: &str, agent_name: &str) -> RaworcResult<Agent> {
        let url = format!("/spaces/{}/agents/{}", space, agent_name);
        self.get(&url).await
    }

    /// Update agent
    pub async fn update_agent(
        &self,
        space: &str,
        agent_name: &str,
        request: &UpdateAgentRequest,
    ) -> RaworcResult<Agent> {
        let url = format!("/spaces/{}/agents/{}", space, agent_name);
        self.put(&url, request).await
    }

    /// Delete agent
    pub async fn delete_agent(&self, space: &str, agent_name: &str) -> RaworcResult<()> {
        let url = format!("/spaces/{}/agents/{}", space, agent_name);
        self.delete(&url).await?;
        Ok(())
    }

    /// Get agent logs
    pub async fn get_agent_logs(&self, space: &str, agent_name: &str) -> RaworcResult<String> {
        let url = format!("/spaces/{}/agents/{}/logs", space, agent_name);
        let response = self.client.get(self.build_url(&url)).send().await?;
        response.text().await.map_err(RaworcError::from)
    }

    // Secret management
    /// List secrets in space
    pub async fn list_secrets(&self, space: Option<&str>) -> RaworcResult<Vec<Secret>> {
        let space = space.unwrap_or_else(|| self.default_space.as_deref().unwrap_or("default"));
        let url = format!("/spaces/{}/secrets", space);
        self.get(&url).await
    }

    /// Get secret by key
    pub async fn get_secret(&self, space: &str, key: &str) -> RaworcResult<Secret> {
        let url = format!("/spaces/{}/secrets/{}", space, key);
        self.get(&url).await
    }

    /// Set secret
    pub async fn set_secret(&self, space: &str, key: &str, value: &str) -> RaworcResult<Secret> {
        let url = format!("/spaces/{}/secrets/{}", space, key);
        let request = CreateSecretRequest {
            value: value.to_string(),
        };
        self.post(&url, &request).await
    }

    /// Update secret
    pub async fn update_secret(&self, space: &str, key: &str, value: &str) -> RaworcResult<Secret> {
        let url = format!("/spaces/{}/secrets/{}", space, key);
        let request = UpdateSecretRequest {
            value: value.to_string(),
        };
        self.put(&url, &request).await
    }

    /// Delete secret
    pub async fn delete_secret(&self, space: &str, key: &str) -> RaworcResult<()> {
        let url = format!("/spaces/{}/secrets/{}", space, key);
        self.delete(&url).await?;
        Ok(())
    }

    // Service account management
    /// List service accounts
    pub async fn list_service_accounts(&self) -> RaworcResult<Vec<ServiceAccount>> {
        self.get("/service-accounts").await
    }

    /// Create service account
    pub async fn create_service_account(&self, request: &CreateServiceAccountRequest) -> RaworcResult<ServiceAccount> {
        self.post("/service-accounts", request).await
    }

    /// Get service account by ID
    pub async fn get_service_account(&self, id: &str) -> RaworcResult<ServiceAccount> {
        let url = format!("/service-accounts/{}", id);
        self.get(&url).await
    }

    /// Update service account
    pub async fn update_service_account(
        &self,
        id: &str,
        request: &UpdateServiceAccountRequest,
    ) -> RaworcResult<ServiceAccount> {
        let url = format!("/service-accounts/{}", id);
        self.put(&url, request).await
    }

    /// Delete service account
    pub async fn delete_service_account(&self, id: &str) -> RaworcResult<()> {
        let url = format!("/service-accounts/{}", id);
        self.delete(&url).await?;
        Ok(())
    }

    // Role management
    /// List roles
    pub async fn list_roles(&self) -> RaworcResult<Vec<Role>> {
        self.get("/roles").await
    }

    /// Create role
    pub async fn create_role(&self, request: &CreateRoleRequest) -> RaworcResult<Role> {
        self.post("/roles", request).await
    }

    /// Get role by ID
    pub async fn get_role(&self, id: &str) -> RaworcResult<Role> {
        let url = format!("/roles/{}", id);
        self.get(&url).await
    }

    /// Delete role
    pub async fn delete_role(&self, id: &str) -> RaworcResult<()> {
        let url = format!("/roles/{}", id);
        self.delete(&url).await?;
        Ok(())
    }

    // Role binding management
    /// List role bindings
    pub async fn list_role_bindings(&self) -> RaworcResult<Vec<RoleBinding>> {
        self.get("/role-bindings").await
    }

    /// Create role binding
    pub async fn create_role_binding(&self, request: &CreateRoleBindingRequest) -> RaworcResult<RoleBinding> {
        self.post("/role-bindings", request).await
    }

    /// Get role binding by ID
    pub async fn get_role_binding(&self, id: &str) -> RaworcResult<RoleBinding> {
        let url = format!("/role-bindings/{}", id);
        self.get(&url).await
    }

    /// Delete role binding
    pub async fn delete_role_binding(&self, id: &str) -> RaworcResult<()> {
        let url = format!("/role-bindings/{}", id);
        self.delete(&url).await?;
        Ok(())
    }

    // Build management
    /// Create build
    pub async fn create_build(&self, space: &str, request: &CreateBuildRequest) -> RaworcResult<Build> {
        let url = format!("/spaces/{}/build", space);
        self.post(&url, request).await
    }

    /// Get latest build
    pub async fn get_latest_build(&self, space: &str) -> RaworcResult<Build> {
        let url = format!("/spaces/{}/build/latest", space);
        self.get(&url).await
    }

    /// Get build by ID
    pub async fn get_build(&self, space: &str, build_id: &str) -> RaworcResult<Build> {
        let url = format!("/spaces/{}/build/{}", space, build_id);
        self.get(&url).await
    }

    // Helper methods
    fn build_url(&self, path: &str) -> Url {
        self.base_url.join(path).unwrap_or_else(|_| self.base_url.clone())
    }

    fn build_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        if let Some(token) = &self.auth_token {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", token).parse().unwrap(),
            );
        }
        headers
    }

    async fn get<T>(&self, path: &str) -> RaworcResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = self
            .client
            .get(self.build_url(path))
            .headers(self.build_headers())
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn post<T, U>(&self, path: &str, body: &T) -> RaworcResult<U>
    where
        T: serde::Serialize,
        U: for<'de> serde::Deserialize<'de>,
    {
        let response = self
            .client
            .post(self.build_url(path))
            .headers(self.build_headers())
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn put<T, U>(&self, path: &str, body: &T) -> RaworcResult<U>
    where
        T: serde::Serialize,
        U: for<'de> serde::Deserialize<'de>,
    {
        let response = self
            .client
            .put(self.build_url(path))
            .headers(self.build_headers())
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn delete(&self, path: &str) -> RaworcResult<()> {
        let response = self
            .client
            .delete(self.build_url(path))
            .headers(self.build_headers())
            .send()
            .await?;

        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }
        Ok(())
    }

    async fn handle_response<T>(&self, response: reqwest::Response) -> RaworcResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        if response.status().is_success() {
            response.json::<T>().await.map_err(RaworcError::from)
        } else {
            self.handle_error_response(response).await
        }
    }

    async fn handle_error_response<T>(&self, response: reqwest::Response) -> RaworcResult<T> {
        let status = response.status();
        let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());

        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(RaworcError::not_found(&text));
        }

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(RaworcError::auth_error(&text));
        }

        // Try to parse as API error response
        if let Ok(api_error) = serde_json::from_str::<ApiErrorResponse>(&text) {
            return Err(RaworcError::api_error(
                status.as_u16(),
                api_error.error.message,
            ));
        }

        Err(RaworcError::api_error(status.as_u16(), text))
    }
}
