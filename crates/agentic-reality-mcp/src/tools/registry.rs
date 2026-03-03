//! Tool registry — 15 MCP tools.

use crate::session::SessionManager;
use crate::types::error::{McpError, McpResult, ToolCallResult, ToolDefinition};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Total number of MCP tools.
pub const MCP_TOOL_COUNT: usize = 15;

/// Tool registry for all reality MCP tools.
pub struct ToolRegistry;

impl ToolRegistry {
    /// List all available tools.
    pub fn list_tools() -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                name: "reality_deployment".into(),
                description: "Manage agent deployment soul, birth context, substrate, and incarnation identity".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string", "description": "Operation to perform" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_memory".into(),
                description: "Access incarnation memory across past lives, wisdom, and karma".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_environment".into(),
                description: "Sense and interact with the operational environment medium".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_resource".into(),
                description: "Monitor resource body including memory, CPU, network, and storage".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_capability".into(),
                description: "Discover and query agent capabilities in the current context".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_layer".into(),
                description: "Manage reality layers and detect simulation vs physical contexts".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_anchor".into(),
                description: "Create and verify reality anchors for ground truth verification".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_hallucination".into(),
                description: "Detect and manage hallucination risks and unverified claims".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_topology".into(),
                description: "Map deployment topology including upstream, downstream, and siblings".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_temporal".into(),
                description: "Ground temporal awareness with causality tracking and timelines".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_stakes".into(),
                description: "Assess consequence awareness, risk fields, and blast radius".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_coherence".into(),
                description: "Maintain coherence across reality domains and manage transitions".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_context".into(),
                description: "Get a unified context summary across all reality domains".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_ground".into(),
                description: "Perform grounding operations to verify claims against reality anchors".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
            ToolDefinition {
                name: "reality_workspace".into(),
                description: "Manage reality workspace files and persistence".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": { "type": "string" }
                    },
                    "required": ["operation"]
                }),
            },
        ]
    }

    /// Call a tool by name.
    pub async fn call(
        name: &str,
        arguments: Option<Value>,
        session: &Arc<Mutex<SessionManager>>,
    ) -> McpResult<Value> {
        let args = arguments.unwrap_or(Value::Object(serde_json::Map::new()));
        let operation = args
            .get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("get");

        let mut session_guard = session.lock().await;

        let result = match name {
            "reality_deployment" => handle_deployment(operation, &args, &mut session_guard),
            "reality_memory" => handle_memory(operation, &args, &mut session_guard),
            "reality_environment" => handle_environment(operation, &args, &mut session_guard),
            "reality_resource" => handle_resource(operation, &args, &mut session_guard),
            "reality_capability" => handle_capability(operation, &args, &mut session_guard),
            "reality_layer" => handle_layer(operation, &args, &mut session_guard),
            "reality_anchor" => handle_anchor(operation, &args, &mut session_guard),
            "reality_hallucination" => handle_hallucination(operation, &args, &mut session_guard),
            "reality_topology" => handle_topology(operation, &args, &mut session_guard),
            "reality_temporal" => handle_temporal(operation, &args, &mut session_guard),
            "reality_stakes" => handle_stakes(operation, &args, &mut session_guard),
            "reality_coherence" => handle_coherence(operation, &args, &mut session_guard),
            "reality_context" => handle_context(operation, &args, &mut session_guard),
            "reality_ground" => handle_ground(operation, &args, &mut session_guard),
            "reality_workspace" => handle_workspace(operation, &args, &mut session_guard),
            _ => {
                return Err(McpError::ToolNotFound {
                    tool: name.to_string(),
                })
            }
        };

        match result {
            Ok(r) => Ok(r.to_value()),
            Err(e) => Ok(ToolCallResult::error(e.to_string()).to_value()),
        }
    }
}

fn handle_deployment(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "get" => {
            let reader = session.engine.reader();
            match reader.get_soul() {
                Ok(soul) => {
                    let json = serde_json::to_string_pretty(soul).map_err(|e| {
                        McpError::InternalError {
                            message: e.to_string(),
                        }
                    })?;
                    Ok(ToolCallResult::success(json))
                }
                Err(e) => Ok(ToolCallResult::error(e.to_string())),
            }
        }
        "summary" => {
            let reader = session.engine.reader();
            let summary = reader.get_context_summary();
            let json =
                serde_json::to_string_pretty(&summary).map_err(|e| McpError::InternalError {
                    message: e.to_string(),
                })?;
            Ok(ToolCallResult::success(json))
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_memory(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "get" => {
            let reader = session.engine.reader();
            match reader.get_incarnation_memory() {
                Ok(mem) => {
                    let json =
                        serde_json::to_string_pretty(mem).map_err(|e| McpError::InternalError {
                            message: e.to_string(),
                        })?;
                    Ok(ToolCallResult::success(json))
                }
                Err(e) => Ok(ToolCallResult::error(e.to_string())),
            }
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_environment(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "get" => {
            let reader = session.engine.reader();
            match reader.get_environment() {
                Ok(env) => {
                    let json =
                        serde_json::to_string_pretty(env).map_err(|e| McpError::InternalError {
                            message: e.to_string(),
                        })?;
                    Ok(ToolCallResult::success(json))
                }
                Err(e) => Ok(ToolCallResult::error(e.to_string())),
            }
        }
        "mood" => {
            let reader = session.engine.reader();
            match reader.get_mood() {
                Ok(mood) => Ok(ToolCallResult::success(mood.to_string())),
                Err(e) => Ok(ToolCallResult::error(e.to_string())),
            }
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_resource(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "get" => {
            let reader = session.engine.reader();
            match reader.get_body() {
                Ok(body) => {
                    let json = serde_json::to_string_pretty(body).map_err(|e| {
                        McpError::InternalError {
                            message: e.to_string(),
                        }
                    })?;
                    Ok(ToolCallResult::success(json))
                }
                Err(e) => Ok(ToolCallResult::error(e.to_string())),
            }
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_capability(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "list" => {
            let reader = session.engine.reader();
            match reader.get_capabilities() {
                Ok(caps) => {
                    let json = serde_json::to_string_pretty(caps).map_err(|e| {
                        McpError::InternalError {
                            message: e.to_string(),
                        }
                    })?;
                    Ok(ToolCallResult::success(json))
                }
                Err(e) => Ok(ToolCallResult::error(e.to_string())),
            }
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_layer(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "get" => {
            let reader = session.engine.reader();
            match reader.get_reality_layers() {
                Ok(layers) => {
                    let json = serde_json::to_string_pretty(layers).map_err(|e| {
                        McpError::InternalError {
                            message: e.to_string(),
                        }
                    })?;
                    Ok(ToolCallResult::success(json))
                }
                Err(e) => Ok(ToolCallResult::error(e.to_string())),
            }
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_anchor(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "list" => {
            let reader = session.engine.reader();
            let anchors = reader.get_anchors();
            let json =
                serde_json::to_string_pretty(anchors).map_err(|e| McpError::InternalError {
                    message: e.to_string(),
                })?;
            Ok(ToolCallResult::success(json))
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_hallucination(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "get" => {
            let reader = session.engine.reader();
            match reader.get_hallucination_state() {
                Ok(state) => {
                    let json = serde_json::to_string_pretty(state).map_err(|e| {
                        McpError::InternalError {
                            message: e.to_string(),
                        }
                    })?;
                    Ok(ToolCallResult::success(json))
                }
                Err(e) => Ok(ToolCallResult::error(e.to_string())),
            }
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_topology(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "get" => {
            let reader = session.engine.reader();
            match reader.get_topology_map() {
                Ok(topo) => {
                    let json = serde_json::to_string_pretty(topo).map_err(|e| {
                        McpError::InternalError {
                            message: e.to_string(),
                        }
                    })?;
                    Ok(ToolCallResult::success(json))
                }
                Err(e) => Ok(ToolCallResult::error(e.to_string())),
            }
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_temporal(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "get" => {
            let reader = session.engine.reader();
            match reader.get_grounded_time() {
                Ok(time) => {
                    let json = serde_json::to_string_pretty(time).map_err(|e| {
                        McpError::InternalError {
                            message: e.to_string(),
                        }
                    })?;
                    Ok(ToolCallResult::success(json))
                }
                Err(e) => Ok(ToolCallResult::error(e.to_string())),
            }
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_stakes(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "get" => {
            let reader = session.engine.reader();
            match reader.get_stakes_level() {
                Ok(level) => Ok(ToolCallResult::success(level.to_string())),
                Err(e) => Ok(ToolCallResult::error(e.to_string())),
            }
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_coherence(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "get" => {
            let reader = session.engine.reader();
            match reader.get_coherence_state() {
                Ok(state) => {
                    let json = serde_json::to_string_pretty(state).map_err(|e| {
                        McpError::InternalError {
                            message: e.to_string(),
                        }
                    })?;
                    Ok(ToolCallResult::success(json))
                }
                Err(e) => Ok(ToolCallResult::error(e.to_string())),
            }
        }
        "check" => {
            let reader = session.engine.reader();
            let coherent = reader.is_coherent();
            Ok(ToolCallResult::success(format!("coherent: {}", coherent)))
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_context(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "summary" | "get" => {
            let reader = session.engine.reader();
            let summary = reader.get_context_summary();
            let json =
                serde_json::to_string_pretty(&summary).map_err(|e| McpError::InternalError {
                    message: e.to_string(),
                })?;
            Ok(ToolCallResult::success(json))
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_ground(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "status" | "get" => {
            let reader = session.engine.reader();
            match reader.get_grounding_status() {
                Some(status) => {
                    let json = serde_json::to_string_pretty(status).map_err(|e| {
                        McpError::InternalError {
                            message: e.to_string(),
                        }
                    })?;
                    Ok(ToolCallResult::success(json))
                }
                None => Ok(ToolCallResult::error("grounding not initialized".into())),
            }
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}

fn handle_workspace(
    operation: &str,
    _args: &Value,
    session: &mut SessionManager,
) -> Result<ToolCallResult, McpError> {
    match operation {
        "info" | "get" => {
            let info = serde_json::json!({
                "initialized": session.engine.is_initialized(),
                "dirty": session.engine.is_dirty(),
                "data_path": session.data_path,
            });
            Ok(ToolCallResult::success(
                serde_json::to_string_pretty(&info).map_err(|e| McpError::InternalError {
                    message: e.to_string(),
                })?,
            ))
        }
        _ => Ok(ToolCallResult::error(format!(
            "unknown operation: {}",
            operation
        ))),
    }
}
