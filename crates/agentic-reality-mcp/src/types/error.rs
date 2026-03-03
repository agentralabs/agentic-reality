//! MCP error types — two-tier error handling.

use serde_json::Value;

/// MCP-specific errors with JSON-RPC error codes.
#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("Method not found: {method}")]
    MethodNotFound { method: String },

    #[error("Invalid params: {message}")]
    InvalidParams { message: String },

    #[error("Tool not found: {tool}")]
    ToolNotFound { tool: String },

    #[error("Tool execution error: {message}")]
    ToolExecutionError { message: String },

    #[error("Internal error: {message}")]
    InternalError { message: String },
}

impl McpError {
    /// JSON-RPC error code.
    pub fn code(&self) -> i32 {
        match self {
            McpError::MethodNotFound { .. } => -32601,
            McpError::InvalidParams { .. } => -32602,
            McpError::ToolNotFound { .. } => -32803,
            McpError::ToolExecutionError { .. } => -32000,
            McpError::InternalError { .. } => -32603,
        }
    }

    /// Convert a RealityError into an MCP tool execution error.
    pub fn from_reality(e: agentic_reality::types::error::RealityError) -> Self {
        McpError::ToolExecutionError { message: e.to_string() }
    }
}

/// MCP tool call result.
pub type McpResult<T> = Result<T, McpError>;

/// Tool definition for MCP tools/list.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// Tool call result.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolCallResult {
    pub content: Vec<ToolContent>,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

impl ToolCallResult {
    pub fn success(text: String) -> Self {
        Self {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text,
            }],
            is_error: None,
        }
    }

    pub fn error(text: String) -> Self {
        Self {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text,
            }],
            is_error: Some(true),
        }
    }

    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}
