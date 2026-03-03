//! Protocol handler for MCP JSON-RPC messages.

use crate::session::SessionManager;
use crate::tools::ToolRegistry;
use crate::types::error::McpError;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

/// MCP protocol handler.
pub struct ProtocolHandler {
    session: Arc<Mutex<SessionManager>>,
}

impl ProtocolHandler {
    pub fn new(session: Arc<Mutex<SessionManager>>) -> Self {
        Self { session }
    }

    /// Handle an incoming JSON-RPC request.
    pub async fn handle_request(&self, request: Value) -> Value {
        let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let id = request.get("id").cloned().unwrap_or(Value::Null);
        let params = request
            .get("params")
            .cloned()
            .unwrap_or(Value::Object(Default::default()));

        let result = match method {
            "initialize" => self.handle_initialize(&params).await,
            "tools/list" => self.handle_list_tools().await,
            "tools/call" => self.handle_tool_call(&params).await,
            "resources/list" => self.handle_list_resources().await,
            "prompts/list" => self.handle_list_prompts().await,
            _ => Err(McpError::MethodNotFound {
                method: method.to_string(),
            }),
        };

        match result {
            Ok(value) => serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": value,
            }),
            Err(e) => serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": e.code(),
                    "message": e.to_string(),
                },
            }),
        }
    }

    async fn handle_initialize(&self, _params: &Value) -> Result<Value, McpError> {
        Ok(serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": { "listChanged": false },
                "resources": { "subscribe": false, "listChanged": false },
                "prompts": { "listChanged": false },
            },
            "serverInfo": {
                "name": "agentic-reality",
                "version": env!("CARGO_PKG_VERSION"),
            },
        }))
    }

    async fn handle_list_tools(&self) -> Result<Value, McpError> {
        let tools = ToolRegistry::list_tools();
        Ok(serde_json::json!({ "tools": tools }))
    }

    async fn handle_tool_call(&self, params: &Value) -> Result<Value, McpError> {
        let name =
            params
                .get("name")
                .and_then(|n| n.as_str())
                .ok_or_else(|| McpError::InvalidParams {
                    message: "missing tool name".into(),
                })?;
        let arguments = params.get("arguments").cloned();
        ToolRegistry::call(name, arguments, &self.session).await
    }

    async fn handle_list_resources(&self) -> Result<Value, McpError> {
        Ok(serde_json::json!({ "resources": [] }))
    }

    async fn handle_list_prompts(&self) -> Result<Value, McpError> {
        Ok(serde_json::json!({ "prompts": [] }))
    }
}
