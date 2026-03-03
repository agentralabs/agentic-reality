//! Phase 02: MCP protocol handler tests.

use agentic_reality_mcp::protocol::ProtocolHandler;
use agentic_reality_mcp::session::SessionManager;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

fn handler() -> ProtocolHandler {
    let session = Arc::new(Mutex::new(SessionManager::new()));
    ProtocolHandler::new(session)
}

#[tokio::test]
async fn test_initialize() {
    let h = handler();
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {}
    });
    let response = h.handle_request(request).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response["result"]["protocolVersion"].is_string());
    assert!(response["result"]["capabilities"].is_object());
    assert!(response["result"]["serverInfo"]["name"]
        .as_str()
        .unwrap()
        .contains("reality"));
}

#[tokio::test]
async fn test_tools_list() {
    let h = handler();
    let request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    });
    let response = h.handle_request(request).await;
    let tools = response["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 15);
}

#[tokio::test]
async fn test_tools_call() {
    let h = handler();
    let request = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "reality_context",
            "arguments": {"operation": "summary"}
        }
    });
    let response = h.handle_request(request).await;
    assert!(response["result"].is_object());
    assert!(response["error"].is_null());
}

#[tokio::test]
async fn test_tools_call_unknown() {
    let h = handler();
    let request = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "nonexistent",
            "arguments": {}
        }
    });
    let response = h.handle_request(request).await;
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32803);
}

#[tokio::test]
async fn test_unknown_method() {
    let h = handler();
    let request = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "unknown/method",
        "params": {}
    });
    let response = h.handle_request(request).await;
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32601);
}

#[tokio::test]
async fn test_resources_list() {
    let h = handler();
    let request = json!({
        "jsonrpc": "2.0",
        "id": 6,
        "method": "resources/list",
        "params": {}
    });
    let response = h.handle_request(request).await;
    assert!(response["result"]["resources"].is_array());
}

#[tokio::test]
async fn test_prompts_list() {
    let h = handler();
    let request = json!({
        "jsonrpc": "2.0",
        "id": 7,
        "method": "prompts/list",
        "params": {}
    });
    let response = h.handle_request(request).await;
    assert!(response["result"]["prompts"].is_array());
}

#[tokio::test]
async fn test_tools_call_missing_name() {
    let h = handler();
    let request = json!({
        "jsonrpc": "2.0",
        "id": 8,
        "method": "tools/call",
        "params": {
            "arguments": {}
        }
    });
    let response = h.handle_request(request).await;
    assert!(response["error"].is_object());
}

#[tokio::test]
async fn test_response_always_has_id() {
    let h = handler();
    let request = json!({
        "jsonrpc": "2.0",
        "id": 42,
        "method": "initialize",
        "params": {}
    });
    let response = h.handle_request(request).await;
    assert_eq!(response["id"], 42);
}

#[tokio::test]
async fn test_multiple_tool_calls() {
    let h = handler();
    for i in 0..10 {
        let request = json!({
            "jsonrpc": "2.0",
            "id": i,
            "method": "tools/call",
            "params": {
                "name": "reality_context",
                "arguments": {"operation": "summary"}
            }
        });
        let response = h.handle_request(request).await;
        assert!(response["error"].is_null());
    }
}
