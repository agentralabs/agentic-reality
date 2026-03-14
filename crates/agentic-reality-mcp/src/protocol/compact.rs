//! Compact facade mode — reduces 15 tools to 3 grouped facades.
//!
//! Enabled via `AREALITY_MCP_TOOL_SURFACE=compact` (fallback: `MCP_TOOL_SURFACE`).
//! Operations map to `reality_<operation>` — e.g., "deployment" -> "reality_deployment".

use serde_json::Value;

use crate::types::error::ToolDefinition;

struct FacadeGroup {
    name: &'static str,
    description: &'static str,
    operations: &'static [&'static str],
}

/// 3 compact facades covering all 15 reality tools.
const FACADES: &[FacadeGroup] = &[
    FacadeGroup {
        name: "reality_core",
        description: "Manage agent identity, deployment, memory, environment, resources, and capabilities",
        operations: &[
            "deployment",
            "memory",
            "environment",
            "resource",
            "capability",
        ],
    },
    FacadeGroup {
        name: "reality_grounding",
        description: "Verify reality layers, anchors, hallucination checks, grounding, and coherence",
        operations: &[
            "layer",
            "anchor",
            "hallucination",
            "ground",
            "coherence",
        ],
    },
    FacadeGroup {
        name: "reality_awareness",
        description: "Query topology, temporal state, stakes, context, and workspace",
        operations: &[
            "topology",
            "temporal",
            "stakes",
            "context",
            "workspace",
        ],
    },
];

/// Check whether compact tool mode is enabled via environment variable.
pub fn is_compact_mode() -> bool {
    let val = std::env::var("AREALITY_MCP_TOOL_SURFACE")
        .or_else(|_| std::env::var("MCP_TOOL_SURFACE"))
        .unwrap_or_default();
    val.eq_ignore_ascii_case("compact")
}

/// Build the compact facade tool definitions for tools/list.
pub fn compact_tool_definitions() -> Vec<ToolDefinition> {
    FACADES
        .iter()
        .map(|f| {
            let ops_enum: Vec<Value> = f
                .operations
                .iter()
                .map(|o| Value::String(o.to_string()))
                .collect();

            ToolDefinition {
                name: f.name.to_string(),
                description: f.description.to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": {
                            "type": "string",
                            "enum": ops_enum,
                            "description": "Operation to perform"
                        },
                        "params": {
                            "type": "object",
                            "description": "Parameters for the operation"
                        }
                    },
                    "required": ["operation"]
                }),
            }
        })
        .collect()
}

/// Normalize a compact facade call into the underlying tool name
/// and arguments. Returns `(real_tool_name, arguments)`.
///
/// Mapping: facade "reality_core" + operation "deployment"
///          -> tool "reality_deployment" with params as arguments.
pub fn normalize_compact_call(
    facade_name: &str,
    arguments: &Option<Value>,
) -> Option<(String, Option<Value>)> {
    let facade = FACADES.iter().find(|f| f.name == facade_name)?;

    let args = arguments.as_ref().unwrap_or(&Value::Null);
    let operation = args.get("operation").and_then(|v| v.as_str())?;

    if !facade.operations.contains(&operation) {
        return None;
    }

    let real_name = format!("reality_{}", operation);
    let params = args.get("params").cloned();

    Some((real_name, params))
}

/// Check if a tool name is a compact facade name.
pub fn is_compact_facade(name: &str) -> bool {
    FACADES.iter().any(|f| f.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_definitions_count() {
        let defs = compact_tool_definitions();
        assert_eq!(defs.len(), 3);
        assert_eq!(defs[0].name, "reality_core");
        assert_eq!(defs[1].name, "reality_grounding");
        assert_eq!(defs[2].name, "reality_awareness");
    }

    #[test]
    fn normalize_core_deployment() {
        let args = Some(serde_json::json!({
            "operation": "deployment",
            "params": { "operation": "get", "intent": "summary" }
        }));
        let result = normalize_compact_call("reality_core", &args);
        assert!(result.is_some());
        let (name, params) = result.unwrap();
        assert_eq!(name, "reality_deployment");
        assert!(params.is_some());
        assert_eq!(
            params.unwrap().get("operation").unwrap().as_str().unwrap(),
            "get"
        );
    }

    #[test]
    fn normalize_grounding_hallucination() {
        let args = Some(serde_json::json!({
            "operation": "hallucination",
            "params": { "operation": "check" }
        }));
        let result = normalize_compact_call("reality_grounding", &args);
        assert!(result.is_some());
        let (name, _) = result.unwrap();
        assert_eq!(name, "reality_hallucination");
    }

    #[test]
    fn normalize_awareness_workspace() {
        let args = Some(serde_json::json!({
            "operation": "workspace",
            "params": { "operation": "list" }
        }));
        let result = normalize_compact_call("reality_awareness", &args);
        assert!(result.is_some());
        let (name, _) = result.unwrap();
        assert_eq!(name, "reality_workspace");
    }

    #[test]
    fn normalize_unknown_facade() {
        let args = Some(serde_json::json!({ "operation": "deployment" }));
        assert!(normalize_compact_call("reality_unknown", &args).is_none());
    }

    #[test]
    fn normalize_invalid_operation() {
        let args = Some(serde_json::json!({ "operation": "nonexistent" }));
        assert!(normalize_compact_call("reality_core", &args).is_none());
    }

    #[test]
    fn is_facade_check() {
        assert!(is_compact_facade("reality_core"));
        assert!(is_compact_facade("reality_grounding"));
        assert!(is_compact_facade("reality_awareness"));
        assert!(!is_compact_facade("reality_deployment"));
        assert!(!is_compact_facade("unknown"));
    }

    #[test]
    fn all_15_operations_covered() {
        let all_ops: Vec<&str> = FACADES
            .iter()
            .flat_map(|f| f.operations.iter().copied())
            .collect();
        assert_eq!(all_ops.len(), 15);
        let expected = [
            "deployment", "memory", "environment", "resource", "capability",
            "layer", "anchor", "hallucination", "ground", "coherence",
            "topology", "temporal", "stakes", "context", "workspace",
        ];
        for op in &expected {
            assert!(all_ops.contains(op), "missing operation: {}", op);
        }
    }
}
