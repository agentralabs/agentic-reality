//! Phase 04: MCP hardening tests — zero unwrap verification.

use agentic_reality_mcp::session::SessionManager;
use agentic_reality_mcp::tools::registry::ToolRegistry;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

fn session() -> Arc<Mutex<SessionManager>> {
    Arc::new(Mutex::new(SessionManager::new()))
}

#[tokio::test]
async fn test_null_operation_handled() {
    let s = session();
    let result =
        ToolRegistry::call("reality_deployment", Some(json!({"operation": null})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_numeric_operation_handled() {
    let s = session();
    let result = ToolRegistry::call("reality_deployment", Some(json!({"operation": 42})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_empty_string_operation() {
    let s = session();
    let result = ToolRegistry::call("reality_deployment", Some(json!({"operation": ""})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_very_long_operation_name() {
    let s = session();
    let long_op = "a".repeat(10000);
    let result = ToolRegistry::call(
        "reality_deployment",
        Some(json!({"operation": long_op})),
        &s,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_nested_json_arguments() {
    let s = session();
    let result = ToolRegistry::call(
        "reality_deployment",
        Some(json!({"operation": "get", "extra": {"nested": true}})),
        &s,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_array_arguments() {
    let s = session();
    let result = ToolRegistry::call(
        "reality_deployment",
        Some(json!({"operation": "get", "items": [1,2,3]})),
        &s,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_all_tools_with_empty_operation() {
    let s = session();
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        let result = ToolRegistry::call(&tool.name, Some(json!({"operation": ""})), &s).await;
        assert!(
            result.is_ok(),
            "Tool '{}' panicked with empty operation",
            tool.name
        );
    }
}

#[tokio::test]
async fn test_all_tools_with_null_args() {
    let s = session();
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        let result = ToolRegistry::call(&tool.name, None, &s).await;
        assert!(
            result.is_ok(),
            "Tool '{}' panicked with null args",
            tool.name
        );
    }
}

#[tokio::test]
async fn test_unicode_operation() {
    let s = session();
    let result =
        ToolRegistry::call("reality_deployment", Some(json!({"operation": "获取"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_special_chars_operation() {
    let s = session();
    let result = ToolRegistry::call(
        "reality_deployment",
        Some(json!({"operation": "get; DROP TABLE users;--"})),
        &s,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_concurrent_tool_calls() {
    let s = session();
    let mut handles = vec![];
    for _ in 0..10 {
        let s_clone = s.clone();
        handles.push(tokio::spawn(async move {
            ToolRegistry::call(
                "reality_context",
                Some(json!({"operation": "summary"})),
                &s_clone,
            )
            .await
        }));
    }
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_tool_result_never_panics() {
    let s = session();
    let malicious_inputs = vec![
        json!(null),
        json!("string"),
        json!(42),
        json!([1, 2, 3]),
        json!({"operation": "get"}),
        json!({"operation": null}),
        json!({}),
    ];
    for input in malicious_inputs {
        // Even with bad input shapes, should not panic
        let _ = ToolRegistry::call("reality_context", Some(input), &s).await;
    }
}
