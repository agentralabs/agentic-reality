//! Request validation for MCP operations.

use crate::types::error::McpError;

/// Validates MCP requests before execution.
pub struct RequestValidator;

impl RequestValidator {
    /// Validate a tool call — checks tool name and required fields.
    pub fn validate_tool_call(
        name: Option<&str>,
        arguments: &serde_json::Value,
    ) -> Result<(String, serde_json::Value), McpError> {
        let tool_name = match name {
            Some(n) if !n.is_empty() => n.to_string(),
            Some(_) => {
                return Err(McpError::InvalidParams {
                    message: "tool name must not be empty".into(),
                });
            }
            None => {
                return Err(McpError::InvalidParams {
                    message: "missing tool name".into(),
                });
            }
        };

        // Validate tool name characters — must be ASCII alphanumeric or underscore
        if !tool_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(McpError::InvalidParams {
                message: format!("invalid tool name: {}", tool_name),
            });
        }

        // Ensure arguments is an object (or null which we treat as empty object)
        let args = if arguments.is_null() || arguments.is_object() {
            if arguments.is_null() {
                serde_json::Value::Object(serde_json::Map::new())
            } else {
                arguments.clone()
            }
        } else {
            return Err(McpError::InvalidParams {
                message: "arguments must be an object or null".into(),
            });
        };

        Ok((tool_name, args))
    }

    /// Validate a resource read — checks URI format.
    pub fn validate_resource_read(uri: Option<&str>) -> Result<String, McpError> {
        let resource_uri = match uri {
            Some(u) if !u.is_empty() => u.to_string(),
            Some(_) => {
                return Err(McpError::InvalidParams {
                    message: "resource URI must not be empty".into(),
                });
            }
            None => {
                return Err(McpError::InvalidParams {
                    message: "missing resource URI".into(),
                });
            }
        };

        // Basic URI validation — must contain ://
        if !resource_uri.contains("://") {
            return Err(McpError::InvalidParams {
                message: format!("invalid resource URI: {}", resource_uri),
            });
        }

        Ok(resource_uri)
    }

    /// Validate an operation string (used by tool dispatchers).
    pub fn validate_operation(operation: &str) -> Result<&str, McpError> {
        if operation.is_empty() {
            return Err(McpError::InvalidParams {
                message: "operation must not be empty".into(),
            });
        }
        if operation.len() > 256 {
            return Err(McpError::InvalidParams {
                message: "operation name too long".into(),
            });
        }
        Ok(operation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_tool_call_valid() {
        let result = RequestValidator::validate_tool_call(
            Some("reality_deployment"),
            &json!({"operation": "get"}),
        );
        assert!(result.is_ok());
        let (name, _args) = result.unwrap_or_else(|_| panic!("expected ok"));
        assert_eq!(name, "reality_deployment");
    }

    #[test]
    fn test_validate_tool_call_missing_name() {
        let result = RequestValidator::validate_tool_call(None, &json!({}));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_tool_call_empty_name() {
        let result = RequestValidator::validate_tool_call(Some(""), &json!({}));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_tool_call_invalid_chars() {
        let result = RequestValidator::validate_tool_call(Some("tool; DROP"), &json!({}));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_tool_call_null_args() {
        let result = RequestValidator::validate_tool_call(Some("test_tool"), &json!(null));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_tool_call_non_object_args() {
        let result = RequestValidator::validate_tool_call(Some("test_tool"), &json!("bad"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_resource_read_valid() {
        let result = RequestValidator::validate_resource_read(Some("reality://deployment/soul"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_resource_read_missing() {
        let result = RequestValidator::validate_resource_read(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_resource_read_no_scheme() {
        let result = RequestValidator::validate_resource_read(Some("just-a-path"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_operation_valid() {
        let result = RequestValidator::validate_operation("get");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_operation_empty() {
        let result = RequestValidator::validate_operation("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_operation_too_long() {
        let long = "a".repeat(300);
        let result = RequestValidator::validate_operation(&long);
        assert!(result.is_err());
    }
}
