//! Phase 03: MCP error handling tests.

use agentic_reality_mcp::types::error::*;

#[test]
fn test_mcp_error_codes() {
    assert_eq!(
        McpError::MethodNotFound { method: "x".into() }.code(),
        -32601
    );
    assert_eq!(
        McpError::InvalidParams {
            message: "x".into()
        }
        .code(),
        -32602
    );
    assert_eq!(McpError::ToolNotFound { tool: "x".into() }.code(), -32803);
    assert_eq!(
        McpError::ToolExecutionError {
            message: "x".into()
        }
        .code(),
        -32000
    );
    assert_eq!(
        McpError::InternalError {
            message: "x".into()
        }
        .code(),
        -32603
    );
}

#[test]
fn test_tool_not_found_code_is_32803() {
    let err = McpError::ToolNotFound {
        tool: "nonexistent".into(),
    };
    assert_eq!(err.code(), -32803);
    assert!(err.to_string().contains("nonexistent"));
}

#[test]
fn test_tool_call_result_success() {
    let result = ToolCallResult::success("hello".into());
    assert!(result.is_error.is_none());
    assert_eq!(result.content.len(), 1);
    assert_eq!(result.content[0].text, "hello");
}

#[test]
fn test_tool_call_result_error() {
    let result = ToolCallResult::error("bad".into());
    assert_eq!(result.is_error, Some(true));
    assert_eq!(result.content[0].text, "bad");
}

#[test]
fn test_tool_call_result_to_value() {
    let result = ToolCallResult::success("data".into());
    let value = result.to_value();
    assert!(value.is_object());
    assert!(value["content"].is_array());
}

#[test]
fn test_tool_definition_serialization() {
    let def = ToolDefinition {
        name: "test_tool".into(),
        description: "Test tool".into(),
        input_schema: serde_json::json!({"type": "object"}),
    };
    let json = serde_json::to_value(&def).unwrap();
    assert_eq!(json["name"], "test_tool");
    assert_eq!(json["inputSchema"]["type"], "object");
}

#[test]
fn test_mcp_error_from_reality() {
    let reality_err = agentic_reality::types::error::RealityError::NotFound("test".into());
    let mcp_err = McpError::from_reality(reality_err);
    assert_eq!(mcp_err.code(), -32000);
    assert!(mcp_err.to_string().contains("test"));
}

#[test]
fn test_tool_content_type() {
    let content = ToolContent {
        content_type: "text".into(),
        text: "hello".into(),
    };
    assert_eq!(content.content_type, "text");
}

#[test]
fn test_error_display() {
    let err = McpError::ToolExecutionError {
        message: "timeout".into(),
    };
    assert!(err.to_string().contains("timeout"));
}

#[test]
fn test_method_not_found_display() {
    let err = McpError::MethodNotFound {
        method: "foo/bar".into(),
    };
    assert!(err.to_string().contains("foo/bar"));
}
