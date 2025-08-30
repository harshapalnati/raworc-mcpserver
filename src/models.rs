use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Session state enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SessionState {
    Init,
    Running,
    Paused,
    Suspended,
    Terminated,
}

/// Session model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub space: String,
    pub created_by: String,
    pub state: SessionState,
    pub container_id: Option<String>,
    pub persistent_volume_id: Option<String>,
    pub parent_session_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub terminated_at: Option<DateTime<Utc>>,
    pub termination_reason: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Create session request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub space: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Update session request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSessionRequest {
    pub space: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Update session state request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSessionStateRequest {
    pub state: SessionState,
}

/// Message role enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}

/// Message model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub session_id: String,
    pub role: MessageRole,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

/// Create message request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    pub content: String,
}

/// Message count response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCount {
    pub count: u64,
}

/// Space model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create space request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpaceRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Update space request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpaceRequest {
    pub description: Option<String>,
}

/// Agent model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub name: String,
    pub description: Option<String>,
    pub image: String,
    pub command: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
    pub resources: Option<AgentResources>,
    pub status: AgentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Agent status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Stopped,
    Running,
    Error,
}

/// Agent resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResources {
    pub cpu: Option<String>,
    pub memory: Option<String>,
    pub gpu: Option<String>,
}

/// Create agent request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
    pub description: Option<String>,
    pub image: String,
    pub command: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
    pub resources: Option<AgentResources>,
}

/// Update agent request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentRequest {
    pub description: Option<String>,
    pub image: Option<String>,
    pub command: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
    pub resources: Option<AgentResources>,
}

/// Update agent status request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentStatusRequest {
    pub status: AgentStatus,
}

/// Secret model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub key: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create secret request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSecretRequest {
    pub value: String,
}

/// Update secret request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSecretRequest {
    pub value: String,
}

/// Service account model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAccount {
    pub id: String,
    pub user: String,
    pub space: Option<String>,
    pub description: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

/// Create service account request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServiceAccountRequest {
    pub user: String,
    pub pass: String,
    pub space: Option<String>,
    pub description: Option<String>,
}

/// Update service account request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateServiceAccountRequest {
    pub space: Option<String>,
    pub description: Option<String>,
    pub active: Option<bool>,
}

/// Update password request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

/// Role model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub description: Option<String>,
    pub rules: Vec<RoleRule>,
    pub created_at: DateTime<Utc>,
}

/// Role rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleRule {
    pub resources: Vec<String>,
    pub verbs: Vec<String>,
    pub scope: String,
}

/// Create role request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub id: String,
    pub description: Option<String>,
    pub rules: Vec<RoleRule>,
}

/// Role binding model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleBinding {
    pub id: String,
    pub subject: String,
    pub role_ref: String,
    pub space: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create role binding request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleBindingRequest {
    pub subject: String,
    pub role_ref: String,
    pub space: Option<String>,
}

/// Authentication request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub user: String,
    pub pass: String,
}

/// Authentication response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
    pub expires_at: DateTime<Utc>,
}

/// User info response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user: String,
    pub namespace: Option<String>,
    pub r#type: String,
}

/// Version response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionResponse {
    pub version: String,
    pub api: String,
}

/// Build model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub id: String,
    pub space: String,
    pub status: BuildStatus,
    pub image: Option<String>,
    pub logs: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Build status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BuildStatus {
    Pending,
    Building,
    Completed,
    Failed,
}

/// Create build request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBuildRequest {
    pub dockerfile: String,
    pub context: Option<String>,
}

/// MCP Tool call request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRequest {
    pub name: String,
    pub arguments: serde_json::Value,
}

/// MCP Tool call response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResponse {
    pub content: Vec<ToolCallContent>,
}

/// MCP Tool call content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
    pub image_url: Option<ImageUrl>,
}

/// MCP Image URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
    pub detail: Option<String>,
}
