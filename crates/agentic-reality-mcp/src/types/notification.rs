//! MCP notification types — progress, log, and cancellation notifications.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Progress notification parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressNotification {
    /// Token identifying the operation.
    #[serde(rename = "progressToken")]
    pub progress_token: String,
    /// Current progress value.
    pub progress: f64,
    /// Total expected value (if known).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    /// Human-readable description of current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Log notification parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogNotification {
    /// Log level: "debug", "info", "warning", "error".
    pub level: LogLevel,
    /// The log message.
    pub message: String,
    /// Optional structured data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    /// Optional logger name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger: Option<String>,
}

/// Log levels for MCP log notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warning => write!(f, "warning"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

/// Cancellation notification parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelledNotification {
    /// The request ID that is being cancelled.
    #[serde(rename = "requestId")]
    pub request_id: Value,
    /// Optional human-readable reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_progress_notification() {
        let n = ProgressNotification {
            progress_token: "tok-1".into(),
            progress: 50.0,
            total: Some(100.0),
            message: Some("halfway".into()),
        };
        let json = serde_json::to_value(&n);
        assert!(json.is_ok());
    }

    #[test]
    fn test_log_notification() {
        let n = LogNotification {
            level: LogLevel::Info,
            message: "server started".into(),
            data: None,
            logger: Some("mcp".into()),
        };
        let json = serde_json::to_value(&n);
        assert!(json.is_ok());
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(format!("{}", LogLevel::Debug), "debug");
        assert_eq!(format!("{}", LogLevel::Warning), "warning");
    }

    #[test]
    fn test_cancelled_notification() {
        let n = CancelledNotification {
            request_id: json!(42),
            reason: Some("timeout".into()),
        };
        let json = serde_json::to_value(&n);
        assert!(json.is_ok());
    }

    #[test]
    fn test_log_level_serde_roundtrip() {
        let level = LogLevel::Warning;
        let s = serde_json::to_string(&level);
        assert!(s.is_ok());
        let parsed: Result<LogLevel, _> = serde_json::from_str(s.as_ref().map(|v| v.as_str()).unwrap_or(""));
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap_or(LogLevel::Debug), LogLevel::Warning);
    }
}
