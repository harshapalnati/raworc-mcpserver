use thiserror::Error;
use serde::Deserialize;

/// Custom error type for Raworc MCP operations
#[derive(Error, Debug)]
pub enum RaworcError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("API error: {message}")]
    ApiError {
        status: u16,
        message: String,
    },

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("MCP protocol error: {0}")]
    McpError(String),
}

impl RaworcError {
    pub fn api_error(status: u16, message: String) -> Self {
        Self::ApiError { status, message }
    }

    pub fn not_found(resource: &str) -> Self {
        Self::NotFound(resource.to_string())
    }

    pub fn auth_error(message: &str) -> Self {
        Self::AuthError(message.to_string())
    }

    pub fn validation_error(message: &str) -> Self {
        Self::ValidationError(message.to_string())
    }

    pub fn config_error(message: &str) -> Self {
        Self::ConfigError(message.to_string())
    }

    pub fn timeout_error(message: &str) -> Self {
        Self::TimeoutError(message.to_string())
    }

    pub fn internal_error(message: &str) -> Self {
        Self::InternalError(message.to_string())
    }

    pub fn mcp_error(message: &str) -> Self {
        Self::McpError(message.to_string())
    }
}

/// Result type for Raworc operations
pub type RaworcResult<T> = Result<T, RaworcError>;

/// API error response from Raworc
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
}
