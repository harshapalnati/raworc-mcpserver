//! Raworc Cloud API client
//! - Default base URL: https://api.remoteagent.com/api/v0
//! - Space-scoped routes for sessions/agents/secrets/builds
//! - Uniform Bearer auth + small 401 -> re-auth -> retry safeguard

use crate::error::{ApiErrorResponse, RaworcError, RaworcResult};
use crate::models::*;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

/// Raworc API client
pub struct RaworcClient {
    http: Client,
    base_url: Url,
    /// If set, used for Authorization: Bearer <token>
    auth_token: Option<String>,
    /// Default space used when a method allows `space: Option<&str>`
    default_space: Option<String>,
    /// Optional username/password for auto re-auth
    username: Option<String>,
    password: Option<String>,
    /// per-request timeout (seconds)
    timeout: u64,
}

impl RaworcClient {
    /// Create a new client from your config.
    ///
    /// Expected `crate::Config` fields:
    /// - api_url (string) e.g., "https://api.remoteagent.com/api/v0"
    /// - auth_token (optional)
    /// - default_space (optional)
    /// - username/password (optional; used for authenticate() and 401 retry)
    /// - timeout_seconds (optional; default 30)
    pub fn new(config: &crate::Config) -> RaworcResult<Self> {
        // Default to cloud API if not provided
        let base_url = Url::parse(
            config
                .api_url
                .as_ref()
                .map_or("https://api.remoteagent.com/api/v0", |v| v),
        )
        .map_err(|e| RaworcError::ConfigError(format!("Invalid API URL: {}", e)))?;

        let timeout = config.timeout_seconds.unwrap_or(30);
        let http = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .map_err(|e| RaworcError::ConfigError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            http,
            base_url,
            auth_token: config.auth_token.clone(),
            default_space: config.default_space.clone(),
            username: config.username.clone(),
            password: config.password.clone(),
            timeout,
        })
    }

    /// Manually set/replace the bearer token (useful if you persist it)
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.auth_token = Some(token.into());
    }

    /// Authenticate with username and password; stores the token internally.
    pub async fn authenticate(&mut self, username: &str, password: &str) -> RaworcResult<()> {
        #[derive(Serialize)]
        struct AuthRequest {
            user: String,
            pass: String,
        }
        #[derive(Deserialize)]
        struct AuthResponseWire {
            token: String,
        }

        let req = AuthRequest {
            user: username.to_string(),
            pass: password.to_string(),
        };

        let auth: AuthResponseWire = self.post_json("auth/login", &req).await?;
        self.auth_token = Some(auth.token);
        Ok(())
    }

    /// Get current user info (auth required)
    pub async fn get_user_info(&self) -> RaworcResult<UserInfo> {
        self.get_json("auth/me").await
    }

    /// Health (often public)
    pub async fn health_check(&self) -> RaworcResult<String> {
        let res = self.http.get(self.build_url("health")).send().await?;
        Ok(res.text().await.unwrap_or_default())
    }

    /// Version (public)
    pub async fn get_version(&self) -> RaworcResult<VersionResponse> {
        self.get_json("version").await
    }

    /* ------------------------- Spaces (org/global) ------------------------- */

    pub async fn list_spaces(&self) -> RaworcResult<Vec<Space>> {
        self.get_json("spaces").await
    }

    pub async fn create_space(&self, name: &str, description: Option<&str>) -> RaworcResult<Space> {
        let req = CreateSpaceRequest {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
        };
        self.post_json("spaces", &req).await
    }

    pub async fn get_space(&self, name: &str) -> RaworcResult<Space> {
        self.get_json(&format!("spaces/{}", name)).await
    }

    pub async fn update_space(&self, name: &str, description: Option<&str>) -> RaworcResult<Space> {
        let req = UpdateSpaceRequest {
            description: description.map(|s| s.to_string()),
        };
        self.put_json(&format!("spaces/{}", name), &req).await
    }

    pub async fn delete_space(&self, name: &str) -> RaworcResult<()> {
        self.delete_req(&format!("spaces/{}", name)).await
    }

    /* ----------------------- Sessions (space-scoped) ----------------------- */

    pub async fn list_sessions(&self, space: Option<&str>) -> RaworcResult<Vec<Session>> {
        let sp = self.space(space);
        self.get_json(&format!("spaces/{}/sessions", sp)).await
    }

    pub async fn create_session(
        &self,
        space: Option<&str>,
        metadata: Option<HashMap<String, Value>>,
    ) -> RaworcResult<Session> {
        let sp = self.space(space);
        let req = CreateSessionRequest {
            // Space is implied by path in cloud API; keep None in body
            space: None,
            metadata,
        };
        self.post_json(&format!("spaces/{}/sessions", sp), &req).await
    }

    pub async fn get_session(&self, space: Option<&str>, session_id: &str) -> RaworcResult<Session> {
        let sp = self.space(space);
        self.get_json(&format!("spaces/{}/sessions/{}", sp, session_id))
            .await
    }

    pub async fn update_session(
        &self,
        space: Option<&str>,
        session_id: &str,
        request: &UpdateSessionRequest,
    ) -> RaworcResult<Session> {
        let sp = self.space(space);
        self.put_json(&format!("spaces/{}/sessions/{}", sp, session_id), request)
            .await
    }

    pub async fn update_session_state(
        &self,
        space: Option<&str>,
        session_id: &str,
        state: SessionState,
    ) -> RaworcResult<()> {
        let sp = self.space(space);
        let req = UpdateSessionStateRequest { state };
        self.put_json::<_, ()>(&format!("spaces/{}/sessions/{}/state", sp, session_id), &req)
            .await
    }

    pub async fn pause_session(&self, space: Option<&str>, session_id: &str) -> RaworcResult<()> {
        let sp = self.space(space);
        self.post_json::<_, ()>(&format!("spaces/{}/sessions/{}/pause", sp, session_id), &())
            .await
    }

    pub async fn resume_session(&self, space: Option<&str>, session_id: &str) -> RaworcResult<()> {
        let sp = self.space(space);
        self.post_json::<_, ()>(&format!("spaces/{}/sessions/{}/resume", sp, session_id), &())
            .await
    }

    pub async fn terminate_session(&self, space: Option<&str>, session_id: &str) -> RaworcResult<()> {
        let sp = self.space(space);
        self.delete_req(&format!("spaces/{}/sessions/{}", sp, session_id))
            .await
    }

    /* ----------------------- Messages (space+session) ---------------------- */

    pub async fn get_messages(
        &self,
        space: Option<&str>,
        session_id: &str,
        limit: Option<u64>,
    ) -> RaworcResult<Vec<Message>> {
        let sp = self.space(space);
        let mut path = format!("spaces/{}/sessions/{}/messages", sp, session_id);
        if let Some(n) = limit {
            path.push_str(&format!("?limit={}", n));
        }
        self.get_json(&path).await
    }

    pub async fn send_message(
        &self,
        space: Option<&str>,
        session_id: &str,
        content: &str,
    ) -> RaworcResult<Message> {
        let sp = self.space(space);
        let req = CreateMessageRequest {
            content: content.to_string(),
        };
        self.post_json(&format!("spaces/{}/sessions/{}/messages", sp, session_id), &req)
            .await
    }

    pub async fn get_message_count(
        &self,
        space: Option<&str>,
        session_id: &str,
    ) -> RaworcResult<MessageCount> {
        let sp = self.space(space);
        self.get_json(&format!("spaces/{}/sessions/{}/messages/count", sp, session_id))
            .await
    }

    pub async fn clear_messages(&self, space: Option<&str>, session_id: &str) -> RaworcResult<()> {
        let sp = self.space(space);
        self.delete_req(&format!("spaces/{}/sessions/{}/messages", sp, session_id))
            .await
    }

    /* ------------------------- Sessions (global) ------------------------- */

    pub async fn list_all_sessions(&self) -> RaworcResult<Vec<Session>> {
        self.get_json("sessions").await
    }

    pub async fn create_global_session(&self, request: &CreateSessionRequest) -> RaworcResult<Session> {
        self.post_json("sessions", request).await
    }

    pub async fn get_global_session(&self, session_id: &str) -> RaworcResult<Session> {
        self.get_json(&format!("sessions/{}", session_id)).await
    }

    pub async fn update_global_session(&self, session_id: &str, request: &UpdateSessionRequest) -> RaworcResult<Session> {
        self.put_json(&format!("sessions/{}", session_id), request).await
    }

    pub async fn update_global_session_state(&self, session_id: &str, request: &UpdateSessionStateRequest) -> RaworcResult<()> {
        self.put_json::<_, ()>(&format!("sessions/{}/state", session_id), request).await
    }

    pub async fn close_session(&self, session_id: &str) -> RaworcResult<()> {
        self.post_json::<_, ()>(&format!("sessions/{}/close", session_id), &()).await
    }

    pub async fn restore_session(&self, session_id: &str) -> RaworcResult<()> {
        self.post_json::<_, ()>(&format!("sessions/{}/restore", session_id), &()).await
    }

    pub async fn remix_session(&self, session_id: &str, request: &CreateSessionRequest) -> RaworcResult<Session> {
        self.post_json(&format!("sessions/{}/remix", session_id), request).await
    }

    pub async fn delete_global_session(&self, session_id: &str) -> RaworcResult<()> {
        self.delete_req(&format!("sessions/{}", session_id)).await
    }

    /* ------------------------- Session Messages (global) ------------------------- */

    pub async fn get_global_messages(&self, session_id: &str, limit: Option<u64>) -> RaworcResult<Vec<Message>> {
        let mut path = format!("sessions/{}/messages", session_id);
        if let Some(n) = limit {
            path.push_str(&format!("?limit={}", n));
        }
        self.get_json(&path).await
    }

    pub async fn send_global_message(&self, session_id: &str, request: &CreateMessageRequest) -> RaworcResult<Message> {
        self.post_json(&format!("sessions/{}/messages", session_id), request).await
    }

    pub async fn get_global_message_count(&self, session_id: &str) -> RaworcResult<MessageCount> {
        self.get_json(&format!("sessions/{}/messages/count", session_id)).await
    }

    pub async fn clear_global_messages(&self, session_id: &str) -> RaworcResult<()> {
        self.delete_req(&format!("sessions/{}/messages", session_id)).await
    }

    /* ------------------------- Agents (space-scoped) ----------------------- */

    pub async fn list_agents(&self, space: Option<&str>) -> RaworcResult<Vec<Agent>> {
        let sp = self.space(space);
        self.get_json(&format!("spaces/{}/agents", sp)).await
    }

    pub async fn create_agent(
        &self,
        space: &str,
        request: &CreateAgentRequest,
    ) -> RaworcResult<Agent> {
        self.post_json(&format!("spaces/{}/agents", space), request)
            .await
    }

    pub async fn get_agent(&self, space: &str, agent_name: &str) -> RaworcResult<Agent> {
        self.get_json(&format!("spaces/{}/agents/{}", space, agent_name))
            .await
    }

    pub async fn update_agent(
        &self,
        space: &str,
        agent_name: &str,
        request: &UpdateAgentRequest,
    ) -> RaworcResult<Agent> {
        self.put_json(&format!("spaces/{}/agents/{}", space, agent_name), request)
            .await
    }

    pub async fn delete_agent(&self, space: &str, agent_name: &str) -> RaworcResult<()> {
        self.delete_req(&format!("spaces/{}/agents/{}", space, agent_name))
            .await
    }

    pub async fn get_agent_logs(&self, space: &str, agent_name: &str) -> RaworcResult<String> {
        let res = self
            .http
            .get(self.build_url(&format!("spaces/{}/agents/{}/logs", space, agent_name)))
            .headers(self.build_headers())
            .send()
            .await?;
        if !res.status().is_success() {
            return self.map_error_text(res).await;
        }
        Ok(res.text().await.unwrap_or_default())
    }

    /* ------------------------- Secrets (space-scoped) ---------------------- */

    pub async fn list_secrets(&self, space: Option<&str>) -> RaworcResult<Vec<Secret>> {
        let sp = self.space(space);
        self.get_json(&format!("spaces/{}/secrets", sp)).await
    }

    pub async fn get_secret(&self, space: &str, key: &str) -> RaworcResult<Secret> {
        self.get_json(&format!("spaces/{}/secrets/{}", space, key))
            .await
    }

    pub async fn set_secret(&self, space: &str, key: &str, value: &str) -> RaworcResult<Secret> {
        let req = CreateSecretRequest {
            value: value.to_string(),
        };
        self.post_json(&format!("spaces/{}/secrets/{}", space, key), &req)
            .await
    }

    pub async fn update_secret(&self, space: &str, key: &str, value: &str) -> RaworcResult<Secret> {
        let req = UpdateSecretRequest {
            value: value.to_string(),
        };
        self.put_json(&format!("spaces/{}/secrets/{}", space, key), &req)
            .await
    }

    pub async fn delete_secret(&self, space: &str, key: &str) -> RaworcResult<()> {
        self.delete_req(&format!("spaces/{}/secrets/{}", space, key))
            .await
    }

    /* --------------------------- Builds (space) ---------------------------- */

    pub async fn create_build(&self, space: &str, req: &CreateBuildRequest) -> RaworcResult<Build> {
        self.post_json(&format!("spaces/{}/build", space), req).await
    }

    pub async fn get_latest_build(&self, space: &str) -> RaworcResult<Build> {
        self.get_json(&format!("spaces/{}/build/latest", space)).await
    }

    pub async fn get_build(&self, space: &str, build_id: &str) -> RaworcResult<Build> {
        self.get_json(&format!("spaces/{}/build/{}", space, build_id))
            .await
    }

    /* ----------------------------- Internals -------------------------------- */

    fn space<'a>(&'a self, space: Option<&'a str>) -> &'a str {
        space.unwrap_or_else(|| self.default_space.as_deref().unwrap_or("default"))
    }

    fn build_url(&self, path: &str) -> Url {
        // Ensure base_url ends with `/` for proper join
        let mut base = self.base_url.clone();
        if !base.path().ends_with('/') {
            base.set_path(&format!("{}/", base.path()));
        }
        // Strip any leading slash so Url::join keeps `/api/v0`
        let path = path.trim_start_matches('/');
        base.join(path).unwrap_or_else(|_| self.base_url.clone())
    }

    fn build_headers(&self) -> header::HeaderMap {
        let mut h = header::HeaderMap::new();
        h.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));
        h.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        if let Some(token) = &self.auth_token {
            if let Ok(v) = header::HeaderValue::from_str(&format!("Bearer {}", token)) {
                h.insert(header::AUTHORIZATION, v);
            }
        }
        h
    }

    async fn get_json<T>(&self, path: &str) -> RaworcResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        self.with_retry(|| async {
            let res = self
                .http
                .get(self.build_url(path))
                .headers(self.build_headers())
                .send()
                .await?;
            self.handle_json(res).await
        })
        .await
    }

    async fn post_json<B, T>(&self, path: &str, body: &B) -> RaworcResult<T>
    where
        B: Serialize + ?Sized,
        T: for<'de> serde::Deserialize<'de>,
    {
        self.with_retry(|| async {
            let res = self
                .http
                .post(self.build_url(path))
                .headers(self.build_headers())
                .json(body)
                .send()
                .await?;
            self.handle_json(res).await
        })
        .await
    }

    async fn put_json<B, T>(&self, path: &str, body: &B) -> RaworcResult<T>
    where
        B: Serialize + ?Sized,
        T: for<'de> serde::Deserialize<'de>,
    {
        self.with_retry(|| async {
            let res = self
                .http
                .put(self.build_url(path))
                .headers(self.build_headers())
                .json(body)
                .send()
                .await?;
            self.handle_json(res).await
        })
        .await
    }

    async fn patch_json<B, T>(&self, path: &str, body: &B) -> RaworcResult<T>
    where
        B: Serialize + ?Sized,
        T: for<'de> serde::Deserialize<'de>,
    {
        self.with_retry(|| async {
            let res = self
                .http
                .patch(self.build_url(path))
                .headers(self.build_headers())
                .json(body)
                .send()
                .await?;
            self.handle_json(res).await
        })
        .await
    }

    async fn delete_req(&self, path: &str) -> RaworcResult<()> {
        self.with_retry(|| async {
            let res = self
                .http
                .delete(self.build_url(path))
                .headers(self.build_headers())
                .send()
                .await?;
            if res.status().is_success() {
                Ok(())
            } else {
                self.map_error_text(res).await
            }
        })
        .await
    }

    async fn handle_json<T>(&self, res: reqwest::Response) -> RaworcResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        if res.status().is_success() {
            Ok(res.json::<T>().await?)
        } else {
            self.map_error_text(res).await
        }
    }

    async fn map_error_text<T>(&self, res: reqwest::Response) -> RaworcResult<T> {
        let status = res.status();
        let text = res.text().await.unwrap_or_else(|_| "Unknown error".into());

        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(RaworcError::not_found(&text));
        }
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(RaworcError::auth_error(&text));
        }

        if let Ok(api) = serde_json::from_str::<ApiErrorResponse>(&text) {
            return Err(RaworcError::api_error(status.as_u16(), api.error.message));
        }

        Err(RaworcError::api_error(status.as_u16(), text))
    }

    /// Tiny helper: on 401, try one re-auth (if username/password present), then retry once.
    async fn with_retry<F, Fut, T>(&self, f: F) -> RaworcResult<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = RaworcResult<T>>,
        T: Sized,
    {
        match f().await {
            Ok(v) => Ok(v),
            Err(e) if matches!(e, RaworcError::AuthError(_)) => {
                if let (Some(u), Some(p)) = (&self.username, &self.password) {
                    let token = Self::login_once(&self.http, self.base_url.clone(), u, p, self.timeout).await?;
                    let _ = token; // available if you want to persist externally
                    f().await
                } else {
                    Err(e)
                }
            }
            Err(e) => Err(e),
        }
    }

    async fn login_once(
        http: &Client,
        base_url: Url,
        username: &str,
        password: &str,
        _timeout: u64,
    ) -> RaworcResult<String> {
        #[derive(Serialize)]
        struct AuthRequest {
            user: String,
            pass: String,
        }
        #[derive(Deserialize)]
        struct AuthResponseWire {
            token: String,
        }

        let mut base = base_url.clone();
        if !base.path().ends_with('/') {
            base.set_path(&format!("{}/", base.path()));
        }
        // NOTE: no leading slash here so `/api/v0` is preserved
        let url = base.join("auth/login").unwrap();

        let res = http
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&AuthRequest {
                user: username.to_string(),
                pass: password.to_string(),
            })
            .send()
            .await?;

        if res.status().is_success() {
            let r = res.json::<AuthResponseWire>().await?;
            Ok(r.token)
        } else {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            if let Ok(api) = serde_json::from_str::<ApiErrorResponse>(&text) {
                Err(RaworcError::api_error(status.as_u16(), api.error.message))
            } else {
                Err(RaworcError::api_error(status.as_u16(), text))
            }
        }
    }
}
