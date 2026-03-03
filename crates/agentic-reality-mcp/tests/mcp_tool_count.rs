//! CRITICAL: MCP tool count verification test.

use agentic_reality_mcp::tools::registry::{ToolRegistry, MCP_TOOL_COUNT};

#[test]
fn test_mcp_tool_count_exactly_15() {
    let tools = ToolRegistry::list_tools();
    assert_eq!(
        tools.len(),
        MCP_TOOL_COUNT,
        "Expected exactly {} MCP tools, got {}",
        MCP_TOOL_COUNT,
        tools.len()
    );
    assert_eq!(tools.len(), 15);
}

#[test]
fn test_all_tools_have_names() {
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        assert!(!tool.name.is_empty(), "Tool has empty name");
    }
}

#[test]
fn test_all_tools_have_descriptions() {
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        assert!(
            !tool.description.is_empty(),
            "Tool '{}' has empty description",
            tool.name
        );
    }
}

#[test]
fn test_tool_names_are_unique() {
    let tools = ToolRegistry::list_tools();
    let names: Vec<_> = tools.iter().map(|t| &t.name).collect();
    let unique: std::collections::HashSet<_> = names.iter().collect();
    assert_eq!(unique.len(), names.len(), "Duplicate tool names found");
}

#[test]
fn test_tool_descriptions_verb_first() {
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        let first_char = tool.description.chars().next().unwrap();
        assert!(
            first_char.is_uppercase(),
            "Tool '{}' description should start with uppercase verb, got '{}'",
            tool.name,
            tool.description
        );
    }
}

#[test]
fn test_tool_descriptions_no_trailing_period() {
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        assert!(
            !tool.description.ends_with('.'),
            "Tool '{}' description should not end with period",
            tool.name
        );
    }
}

#[test]
fn test_all_tool_names_prefixed_reality() {
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        assert!(
            tool.name.starts_with("reality_"),
            "Tool '{}' should be prefixed with 'reality_'",
            tool.name
        );
    }
}

#[test]
fn test_required_tools_present() {
    let tools = ToolRegistry::list_tools();
    let names: Vec<_> = tools.iter().map(|t| t.name.as_str()).collect();
    let required = [
        "reality_deployment",
        "reality_memory",
        "reality_environment",
        "reality_resource",
        "reality_capability",
        "reality_layer",
        "reality_anchor",
        "reality_hallucination",
        "reality_topology",
        "reality_temporal",
        "reality_stakes",
        "reality_coherence",
        "reality_context",
        "reality_ground",
        "reality_workspace",
    ];
    for tool in &required {
        assert!(names.contains(tool), "Required tool '{}' not found", tool);
    }
}

#[test]
fn test_all_tools_have_input_schema() {
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        assert!(
            tool.input_schema.is_object(),
            "Tool '{}' should have object input schema",
            tool.name
        );
    }
}

#[test]
fn test_all_tools_require_operation() {
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        let required = tool.input_schema.get("required").and_then(|r| r.as_array());
        assert!(
            required.is_some(),
            "Tool '{}' should have required fields",
            tool.name
        );
        let required_names: Vec<_> = required
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect();
        assert!(
            required_names.contains(&"operation"),
            "Tool '{}' should require 'operation' field",
            tool.name
        );
    }
}
