//! SSE streaming support for MCP over HTTP.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A server-sent event for SSE transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    /// Event type (e.g., "message", "endpoint").
    #[serde(rename = "event")]
    pub event_type: String,
    /// Event data payload.
    pub data: Value,
    /// Optional event ID for reconnection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl StreamEvent {
    /// Create a new message event.
    pub fn message(data: Value) -> Self {
        Self {
            event_type: "message".to_string(),
            data,
            id: None,
        }
    }

    /// Create an endpoint event (sent at SSE connection start).
    pub fn endpoint(url: &str) -> Self {
        Self {
            event_type: "endpoint".to_string(),
            data: Value::String(url.to_string()),
            id: None,
        }
    }

    /// Format as SSE wire format.
    pub fn to_sse_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("event: {}\n", self.event_type));
        if let Some(ref id) = self.id {
            out.push_str(&format!("id: {}\n", id));
        }
        // Data can be multiline — each line needs "data: " prefix
        let data_str = match serde_json::to_string(&self.data) {
            Ok(s) => s,
            Err(_) => "null".to_string(),
        };
        for line in data_str.lines() {
            out.push_str(&format!("data: {}\n", line));
        }
        out.push('\n');
        out
    }
}

/// SSE stream transport state.
pub struct StreamTransport {
    /// Whether the transport is connected.
    connected: bool,
    /// Event counter for sequential IDs.
    event_counter: u64,
}

impl StreamTransport {
    /// Create a new stream transport.
    pub fn new() -> Self {
        Self {
            connected: false,
            event_counter: 0,
        }
    }

    /// Mark the transport as connected.
    pub fn connect(&mut self) {
        self.connected = true;
    }

    /// Mark the transport as disconnected.
    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    /// Whether the transport is currently connected.
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Generate the next event with a sequential ID.
    pub fn wrap_event(&mut self, mut event: StreamEvent) -> StreamEvent {
        self.event_counter += 1;
        event.id = Some(self.event_counter.to_string());
        event
    }
}

impl Default for StreamTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_stream_event_message() {
        let event = StreamEvent::message(json!({"result": "ok"}));
        assert_eq!(event.event_type, "message");
    }

    #[test]
    fn test_stream_event_endpoint() {
        let event = StreamEvent::endpoint("/mcp/message");
        assert_eq!(event.event_type, "endpoint");
    }

    #[test]
    fn test_sse_format() {
        let event = StreamEvent {
            event_type: "message".into(),
            data: json!({"hello": "world"}),
            id: Some("1".into()),
        };
        let sse = event.to_sse_string();
        assert!(sse.contains("event: message\n"));
        assert!(sse.contains("id: 1\n"));
        assert!(sse.contains("data: "));
        assert!(sse.ends_with("\n\n"));
    }

    #[test]
    fn test_stream_transport_connect() {
        let mut t = StreamTransport::new();
        assert!(!t.is_connected());
        t.connect();
        assert!(t.is_connected());
        t.disconnect();
        assert!(!t.is_connected());
    }

    #[test]
    fn test_stream_transport_wrap_event() {
        let mut t = StreamTransport::new();
        let e1 = t.wrap_event(StreamEvent::message(json!(null)));
        assert_eq!(e1.id.as_deref(), Some("1"));
        let e2 = t.wrap_event(StreamEvent::message(json!(null)));
        assert_eq!(e2.id.as_deref(), Some("2"));
    }
}
