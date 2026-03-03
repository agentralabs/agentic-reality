//! Phase 01: MCP deployment tool tests.

use agentic_reality_mcp::session::SessionManager;
use agentic_reality_mcp::tools::registry::ToolRegistry;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

fn session() -> Arc<Mutex<SessionManager>> {
    Arc::new(Mutex::new(SessionManager::new()))
}

#[tokio::test]
async fn test_deployment_get_uninitalized() {
    let s = session();
    let result =
        ToolRegistry::call("reality_deployment", Some(json!({"operation": "get"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deployment_summary() {
    let s = session();
    let result = ToolRegistry::call(
        "reality_deployment",
        Some(json!({"operation": "summary"})),
        &s,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deployment_unknown_operation() {
    let s = session();
    let result =
        ToolRegistry::call("reality_deployment", Some(json!({"operation": "fly"})), &s).await;
    assert!(result.is_ok()); // Returns isError in result, not Err
}

#[tokio::test]
async fn test_environment_get() {
    let s = session();
    let result =
        ToolRegistry::call("reality_environment", Some(json!({"operation": "get"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_environment_mood() {
    let s = session();
    let result = ToolRegistry::call(
        "reality_environment",
        Some(json!({"operation": "mood"})),
        &s,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_resource_get() {
    let s = session();
    let result =
        ToolRegistry::call("reality_resource", Some(json!({"operation": "get"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_capability_list() {
    let s = session();
    let result =
        ToolRegistry::call("reality_capability", Some(json!({"operation": "list"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_layer_get() {
    let s = session();
    let result = ToolRegistry::call("reality_layer", Some(json!({"operation": "get"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_anchor_list() {
    let s = session();
    let result = ToolRegistry::call("reality_anchor", Some(json!({"operation": "list"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_hallucination_get() {
    let s = session();
    let result = ToolRegistry::call(
        "reality_hallucination",
        Some(json!({"operation": "get"})),
        &s,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_topology_get() {
    let s = session();
    let result =
        ToolRegistry::call("reality_topology", Some(json!({"operation": "get"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_temporal_get() {
    let s = session();
    let result =
        ToolRegistry::call("reality_temporal", Some(json!({"operation": "get"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stakes_get() {
    let s = session();
    let result = ToolRegistry::call("reality_stakes", Some(json!({"operation": "get"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_coherence_get() {
    let s = session();
    let result =
        ToolRegistry::call("reality_coherence", Some(json!({"operation": "get"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_coherence_check() {
    let s = session();
    let result =
        ToolRegistry::call("reality_coherence", Some(json!({"operation": "check"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_context_summary() {
    let s = session();
    let result =
        ToolRegistry::call("reality_context", Some(json!({"operation": "summary"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_context_get() {
    let s = session();
    let result = ToolRegistry::call("reality_context", Some(json!({"operation": "get"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_ground_status() {
    let s = session();
    let result =
        ToolRegistry::call("reality_ground", Some(json!({"operation": "status"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_workspace_info() {
    let s = session();
    let result =
        ToolRegistry::call("reality_workspace", Some(json!({"operation": "info"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_memory_get() {
    let s = session();
    let result = ToolRegistry::call("reality_memory", Some(json!({"operation": "get"})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_unknown_tool_returns_error() {
    let s = session();
    let result = ToolRegistry::call("nonexistent_tool", Some(json!({})), &s).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_unknown_tool_error_code_32803() {
    let s = session();
    let result = ToolRegistry::call("nonexistent_tool", Some(json!({})), &s).await;
    match result {
        Err(e) => assert_eq!(e.code(), -32803),
        Ok(_) => panic!("Expected error for unknown tool"),
    }
}

#[tokio::test]
async fn test_no_arguments() {
    let s = session();
    let result = ToolRegistry::call("reality_context", None, &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_empty_arguments() {
    let s = session();
    let result = ToolRegistry::call("reality_context", Some(json!({})), &s).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_all_15_tools_callable() {
    let s = session();
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        let result = ToolRegistry::call(&tool.name, Some(json!({"operation": "get"})), &s).await;
        assert!(result.is_ok(), "Tool '{}' failed to execute", tool.name);
    }
}
