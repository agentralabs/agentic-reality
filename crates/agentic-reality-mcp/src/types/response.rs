//! MCP response types — typed wrappers for known MCP method responses.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::capabilities::ServerCapabilities;
use super::error::ToolDefinition;

/// Server info returned during initialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

/// Response to the `initialize` request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResponse {
    /// Protocol version selected by the server.
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// Server capabilities.
    pub capabilities: ServerCapabilities,
    /// Server info.
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
}

/// Response to the `tools/list` request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsListResponse {
    pub tools: Vec<ToolDefinition>,
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Response to the `tools/call` request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsCallResponse {
    pub content: Vec<ToolContent>,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

/// Content item in a tool call response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

/// A JSON-RPC error object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Numeric error code.
    pub code: i32,
    /// Human-readable message.
    pub message: String,
    /// Optional structured error data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcError {
    /// Standard parse error.
    pub fn parse_error(msg: String) -> Self {
        Self { code: -32700, message: msg, data: None }
    }

    /// Standard invalid request.
    pub fn invalid_request(msg: String) -> Self {
        Self { code: -32600, message: msg, data: None }
    }

    /// Standard method not found.
    pub fn method_not_found(method: &str) -> Self {
        Self { code: -32601, message: format!("Method not found: {}", method), data: None }
    }

    /// Standard invalid params.
    pub fn invalid_params(msg: String) -> Self {
        Self { code: -32602, message: msg, data: None }
    }

    /// Standard internal error.
    pub fn internal_error(msg: String) -> Self {
        Self { code: -32603, message: msg, data: None }
    }

    /// Tool not found (MCP-specific code).
    pub fn tool_not_found(tool: &str) -> Self {
        Self { code: -32803, message: format!("Tool not found: {}", tool), data: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_rpc_error_codes() {
        assert_eq!(JsonRpcError::parse_error("x".into()).code, -32700);
        assert_eq!(JsonRpcError::invalid_request("x".into()).code, -32600);
        assert_eq!(JsonRpcError::method_not_found("x").code, -32601);
        assert_eq!(JsonRpcError::invalid_params("x".into()).code, -32602);
        assert_eq!(JsonRpcError::internal_error("x".into()).code, -32603);
        assert_eq!(JsonRpcError::tool_not_found("x").code, -32803);
    }

    #[test]
    fn test_initialize_response_serialization() {
        let resp = InitializeResponse {
            protocol_version: "2024-11-05".into(),
            capabilities: ServerCapabilities::default(),
            server_info: ServerInfo {
                name: "agentic-reality".into(),
                version: "0.1.0".into(),
            },
        };
        let json = serde_json::to_value(&resp);
        assert!(json.is_ok());
    }

    #[test]
    fn test_tools_list_response() {
        let resp = ToolsListResponse {
            tools: vec![],
            next_cursor: None,
        };
        let json = serde_json::to_value(&resp);
        assert!(json.is_ok());
    }

    #[test]
    fn test_tools_call_response() {
        let resp = ToolsCallResponse {
            content: vec![ToolContent {
                content_type: "text".into(),
                text: "hello".into(),
            }],
            is_error: None,
        };
        let json = serde_json::to_value(&resp);
        assert!(json.is_ok());
    }
}
