//! Phase 04: Validation tests.

use agentic_reality::validation::McpValidator;
use serde_json::json;

#[test]
fn test_require_string_present() {
    let params = json!({ "operation": "get" });
    let result = McpValidator::require_string(&params, "operation");
    assert_eq!(result.unwrap(), "get");
}

#[test]
fn test_require_string_missing() {
    let params = json!({});
    let result = McpValidator::require_string(&params, "operation");
    assert!(result.is_err());
}

#[test]
fn test_require_string_wrong_type() {
    let params = json!({ "operation": 42 });
    let result = McpValidator::require_string(&params, "operation");
    assert!(result.is_err());
}

#[test]
fn test_optional_string_present() {
    let params = json!({ "name": "test" });
    assert_eq!(
        McpValidator::optional_string(&params, "name"),
        Some("test".to_string())
    );
}

#[test]
fn test_optional_string_missing() {
    let params = json!({});
    assert_eq!(McpValidator::optional_string(&params, "name"), None);
}

#[test]
fn test_require_u64_present() {
    let params = json!({ "count": 42 });
    assert_eq!(McpValidator::require_u64(&params, "count").unwrap(), 42);
}

#[test]
fn test_require_u64_missing() {
    let params = json!({});
    assert!(McpValidator::require_u64(&params, "count").is_err());
}

#[test]
fn test_require_f64_present() {
    let params = json!({ "score": 0.95 });
    assert!((McpValidator::require_f64(&params, "score").unwrap() - 0.95).abs() < f64::EPSILON);
}

#[test]
fn test_require_bool_present() {
    let params = json!({ "enabled": true });
    assert!(McpValidator::require_bool(&params, "enabled").unwrap());
}

#[test]
fn test_validate_operation_valid() {
    let result = McpValidator::validate_operation("get", &["get", "set", "delete"]);
    assert!(result.is_ok());
}

#[test]
fn test_validate_operation_invalid() {
    let result = McpValidator::validate_operation("fly", &["get", "set", "delete"]);
    assert!(result.is_err());
}

#[test]
fn test_validate_non_empty_valid() {
    assert!(McpValidator::validate_non_empty("hello", "name").is_ok());
}

#[test]
fn test_validate_non_empty_empty() {
    assert!(McpValidator::validate_non_empty("", "name").is_err());
}

#[test]
fn test_validate_non_empty_whitespace() {
    assert!(McpValidator::validate_non_empty("   ", "name").is_err());
}

#[test]
fn test_validate_probability_valid() {
    assert!(McpValidator::validate_probability(0.0, "p").is_ok());
    assert!(McpValidator::validate_probability(0.5, "p").is_ok());
    assert!(McpValidator::validate_probability(1.0, "p").is_ok());
}

#[test]
fn test_validate_probability_too_low() {
    assert!(McpValidator::validate_probability(-0.1, "p").is_err());
}

#[test]
fn test_validate_probability_too_high() {
    assert!(McpValidator::validate_probability(1.1, "p").is_err());
}

#[test]
fn test_optional_f64() {
    let params = json!({ "score": 0.5 });
    assert_eq!(McpValidator::optional_f64(&params, "score"), Some(0.5));
    assert_eq!(McpValidator::optional_f64(&params, "missing"), None);
}

#[test]
fn test_optional_bool() {
    let params = json!({ "flag": false });
    assert_eq!(McpValidator::optional_bool(&params, "flag"), Some(false));
    assert_eq!(McpValidator::optional_bool(&params, "missing"), None);
}
