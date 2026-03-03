//! JSON-RPC message types for the MCP protocol.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A JSON-RPC message identifier — either a number or a string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageId {
    /// Numeric identifier.
    Number(u64),
    /// String identifier.
    Text(String),
}

impl std::fmt::Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageId::Number(n) => write!(f, "{}", n),
            MessageId::Text(s) => write!(f, "{}", s),
        }
    }
}

/// A parsed JSON-RPC message — request, response, or notification.
#[derive(Debug, Clone)]
pub enum JsonRpcMessage {
    /// A request with an id expecting a response.
    Request(JsonRpcRequest),
    /// A response to a prior request.
    Response(JsonRpcResponse),
    /// A notification (no id, no response expected).
    Notification(JsonRpcNotification),
}

/// A JSON-RPC request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: MessageId,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

/// A JSON-RPC response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcErrorBody>,
}

/// Error body inside a JSON-RPC error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcErrorBody {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// A JSON-RPC notification (no id).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

impl JsonRpcMessage {
    /// Parse a raw JSON value into a typed message.
    ///
    /// Returns `None` if the value is not a valid JSON-RPC message.
    pub fn parse(value: &Value) -> Option<Self> {
        let obj = value.as_object()?;
        let _jsonrpc = obj.get("jsonrpc")?.as_str()?;

        if let Some(_id_val) = obj.get("id") {
            // Has an id — could be request or response
            if obj.contains_key("method") {
                // Request
                let req: JsonRpcRequest = serde_json::from_value(value.clone()).ok()?;
                Some(JsonRpcMessage::Request(req))
            } else {
                // Response
                let resp: JsonRpcResponse = serde_json::from_value(value.clone()).ok()?;
                Some(JsonRpcMessage::Response(resp))
            }
        } else if obj.contains_key("method") {
            // Notification
            let notif: JsonRpcNotification = serde_json::from_value(value.clone()).ok()?;
            Some(JsonRpcMessage::Notification(notif))
        } else {
            None
        }
    }
}

impl JsonRpcResponse {
    /// Build a success response.
    pub fn success(id: MessageId, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Build an error response.
    pub fn error(id: MessageId, code: i32, message: String, data: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcErrorBody {
                code,
                message,
                data,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_message_id_number() {
        let id = MessageId::Number(42);
        assert_eq!(format!("{}", id), "42");
    }

    #[test]
    fn test_message_id_string() {
        let id = MessageId::Text("abc".into());
        assert_eq!(format!("{}", id), "abc");
    }

    #[test]
    fn test_parse_request() {
        let val = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {}
        });
        let msg = JsonRpcMessage::parse(&val);
        assert!(matches!(msg, Some(JsonRpcMessage::Request(_))));
    }

    #[test]
    fn test_parse_notification() {
        let val = json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized",
            "params": {}
        });
        let msg = JsonRpcMessage::parse(&val);
        assert!(matches!(msg, Some(JsonRpcMessage::Notification(_))));
    }

    #[test]
    fn test_parse_response() {
        let val = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {"ok": true}
        });
        let msg = JsonRpcMessage::parse(&val);
        assert!(matches!(msg, Some(JsonRpcMessage::Response(_))));
    }

    #[test]
    fn test_parse_invalid() {
        let val = json!("not a message");
        assert!(JsonRpcMessage::parse(&val).is_none());
    }

    #[test]
    fn test_success_response() {
        let resp = JsonRpcResponse::success(MessageId::Number(1), json!({"ok": true}));
        assert_eq!(resp.jsonrpc, "2.0");
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn test_error_response() {
        let resp = JsonRpcResponse::error(MessageId::Number(1), -32601, "not found".into(), None);
        assert!(resp.result.is_none());
        assert!(resp.error.is_some());
        let err = resp.error.as_ref();
        assert!(err.is_some());
        assert_eq!(err.map(|e| e.code), Some(-32601));
    }
}
