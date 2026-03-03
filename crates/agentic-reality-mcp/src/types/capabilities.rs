//! MCP capability types — server and client capabilities for the initialize handshake.

use serde::{Deserialize, Serialize};

/// Server capabilities advertised during initialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    /// Tool-serving capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolCapability>,
    /// Resource-serving capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceCapability>,
    /// Prompt-serving capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptCapability>,
}

impl Default for ServerCapabilities {
    fn default() -> Self {
        Self {
            tools: Some(ToolCapability::default()),
            resources: Some(ResourceCapability::default()),
            prompts: Some(PromptCapability::default()),
        }
    }
}

/// Client capabilities sent during initialization.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientCapabilities {
    /// Whether the client supports sampling.
    #[serde(default)]
    pub sampling: bool,
    /// Whether the client supports roots.
    #[serde(default)]
    pub roots: bool,
}

/// Tool capability descriptor.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolCapability {
    /// Whether the server emits tools/list_changed notifications.
    #[serde(rename = "listChanged", default)]
    pub list_changed: bool,
}

/// Resource capability descriptor.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceCapability {
    /// Whether the client can subscribe to resource changes.
    #[serde(default)]
    pub subscribe: bool,
    /// Whether the server emits resources/list_changed notifications.
    #[serde(rename = "listChanged", default)]
    pub list_changed: bool,
}

/// Prompt capability descriptor.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PromptCapability {
    /// Whether the server emits prompts/list_changed notifications.
    #[serde(rename = "listChanged", default)]
    pub list_changed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_capabilities_default() {
        let caps = ServerCapabilities::default();
        assert!(caps.tools.is_some());
        assert!(caps.resources.is_some());
        assert!(caps.prompts.is_some());
    }

    #[test]
    fn test_server_capabilities_serializes() {
        let caps = ServerCapabilities::default();
        let json = serde_json::to_value(&caps);
        assert!(json.is_ok());
    }

    #[test]
    fn test_client_capabilities_default() {
        let caps = ClientCapabilities::default();
        assert!(!caps.sampling);
        assert!(!caps.roots);
    }
}
