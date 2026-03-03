//! Validation module — strict input validation with zero silent fallbacks.

use crate::types::error::{RealityError, RealityResult};
use serde_json::Value;

/// MCP-specific validator for tool inputs.
pub struct McpValidator;

impl McpValidator {
    /// Require a string field from JSON params.
    pub fn require_string(params: &Value, field: &str) -> RealityResult<String> {
        params
            .get(field)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| RealityError::MissingField(field.to_string()))
    }

    /// Require an optional string field.
    pub fn optional_string(params: &Value, field: &str) -> Option<String> {
        params
            .get(field)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Require a u64 field.
    pub fn require_u64(params: &Value, field: &str) -> RealityResult<u64> {
        params
            .get(field)
            .and_then(|v| v.as_u64())
            .ok_or_else(|| RealityError::MissingField(field.to_string()))
    }

    /// Require a f64 field.
    pub fn require_f64(params: &Value, field: &str) -> RealityResult<f64> {
        params
            .get(field)
            .and_then(|v| v.as_f64())
            .ok_or_else(|| RealityError::MissingField(field.to_string()))
    }

    /// Optional f64 field.
    pub fn optional_f64(params: &Value, field: &str) -> Option<f64> {
        params.get(field).and_then(|v| v.as_f64())
    }

    /// Require a boolean field.
    pub fn require_bool(params: &Value, field: &str) -> RealityResult<bool> {
        params
            .get(field)
            .and_then(|v| v.as_bool())
            .ok_or_else(|| RealityError::MissingField(field.to_string()))
    }

    /// Optional boolean field.
    pub fn optional_bool(params: &Value, field: &str) -> Option<bool> {
        params.get(field).and_then(|v| v.as_bool())
    }

    /// Validate an operation string against allowed values.
    pub fn validate_operation(operation: &str, allowed: &[&str]) -> RealityResult<()> {
        if allowed.contains(&operation) {
            Ok(())
        } else {
            Err(RealityError::InvalidOperation(format!(
                "unknown operation '{}', expected one of: {}",
                operation,
                allowed.join(", ")
            )))
        }
    }

    /// Validate a string is non-empty.
    pub fn validate_non_empty(value: &str, field: &str) -> RealityResult<()> {
        if value.trim().is_empty() {
            Err(RealityError::InvalidParameter {
                field: field.to_string(),
                reason: "must not be empty".to_string(),
            })
        } else {
            Ok(())
        }
    }

    /// Validate a float is in range [0.0, 1.0].
    pub fn validate_probability(value: f64, field: &str) -> RealityResult<()> {
        if !(0.0..=1.0).contains(&value) {
            Err(RealityError::InvalidParameter {
                field: field.to_string(),
                reason: format!("must be between 0.0 and 1.0, got {}", value),
            })
        } else {
            Ok(())
        }
    }
}
