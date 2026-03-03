//! MCP request types — typed wrappers for known MCP method requests.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::capabilities::ClientCapabilities;

/// Parameters for the `initialize` request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeRequest {
    /// Protocol version requested by the client.
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// Client capabilities.
    #[serde(default)]
    pub capabilities: ClientCapabilities,
    /// Client info.
    #[serde(rename = "clientInfo", default)]
    pub client_info: Option<ClientInfo>,
}

/// Client information sent during initialization.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
}

/// Parameters for the `tools/list` request (typically empty).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolsListRequest {
    /// Optional cursor for pagination.
    #[serde(default)]
    pub cursor: Option<String>,
}

/// Parameters for the `tools/call` request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsCallRequest {
    /// Name of the tool to invoke.
    pub name: String,
    /// Arguments to pass to the tool.
    #[serde(default)]
    pub arguments: Value,
}

/// Parameters for the `resources/list` request.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourcesListRequest {
    /// Optional cursor for pagination.
    #[serde(default)]
    pub cursor: Option<String>,
}

/// Parameters for the `prompts/list` request.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PromptsListRequest {
    /// Optional cursor for pagination.
    #[serde(default)]
    pub cursor: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_initialize_request_deser() {
        let val = json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": { "name": "test-client", "version": "1.0" }
        });
        let req: Result<InitializeRequest, _> = serde_json::from_value(val);
        assert!(req.is_ok());
        let req = req.unwrap_or_else(|_| panic!("deser failed"));
        assert_eq!(req.protocol_version, "2024-11-05");
    }

    #[test]
    fn test_tools_call_request_deser() {
        let val = json!({
            "name": "reality_deployment",
            "arguments": { "operation": "get" }
        });
        let req: Result<ToolsCallRequest, _> = serde_json::from_value(val);
        assert!(req.is_ok());
    }

    #[test]
    fn test_tools_list_request_default() {
        let req = ToolsListRequest::default();
        assert!(req.cursor.is_none());
    }

    #[test]
    fn test_resources_list_request_default() {
        let req = ResourcesListRequest::default();
        assert!(req.cursor.is_none());
    }

    #[test]
    fn test_prompts_list_request_default() {
        let req = PromptsListRequest::default();
        assert!(req.cursor.is_none());
    }
}
