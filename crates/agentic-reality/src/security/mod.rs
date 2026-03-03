//! Security module — auth, authz, encryption, audit logging.

use crate::types::error::{RealityError, RealityResult};
use std::collections::HashMap;

/// Authentication manager.
pub struct AuthManager {
    token: Option<String>,
    sessions: HashMap<String, SessionBinding>,
    failed_attempts: u32,
    rate_limit: u32,
}

impl AuthManager {
    pub fn new() -> Self {
        let token = std::env::var("AGENTIC_AUTH_TOKEN").ok();
        Self {
            token,
            sessions: HashMap::new(),
            failed_attempts: 0,
            rate_limit: 100,
        }
    }

    pub fn authenticate(&mut self, provided_token: &str) -> RealityResult<String> {
        if self.failed_attempts >= self.rate_limit {
            return Err(RealityError::Authentication("rate limited".into()));
        }
        match &self.token {
            Some(expected) => {
                if constant_time_eq::constant_time_eq(expected.as_bytes(), provided_token.as_bytes()) {
                    self.failed_attempts = 0;
                    let session_id = uuid::Uuid::new_v4().to_string();
                    self.sessions.insert(session_id.clone(), SessionBinding {
                        session_id: session_id.clone(),
                        created_at: chrono::Utc::now().timestamp(),
                        last_activity: chrono::Utc::now().timestamp(),
                        permissions: Permissions::default(),
                    });
                    Ok(session_id)
                } else {
                    self.failed_attempts += 1;
                    Err(RealityError::Authentication("invalid token".into()))
                }
            }
            None => {
                // No token configured — auth disabled
                Ok("anonymous".to_string())
            }
        }
    }

    pub fn is_auth_required(&self) -> bool {
        self.token.is_some()
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Session binding.
#[derive(Debug, Clone)]
pub struct SessionBinding {
    pub session_id: String,
    pub created_at: i64,
    pub last_activity: i64,
    pub permissions: Permissions,
}

/// Per-domain read/write permissions.
#[derive(Debug, Clone)]
pub struct Permissions {
    pub deployment_read: bool,
    pub deployment_write: bool,
    pub environment_read: bool,
    pub environment_write: bool,
    pub resource_read: bool,
    pub resource_write: bool,
    pub reality_read: bool,
    pub reality_write: bool,
    pub topology_read: bool,
    pub topology_write: bool,
    pub temporal_read: bool,
    pub temporal_write: bool,
    pub stakes_read: bool,
    pub stakes_write: bool,
    pub admin: bool,
}

impl Default for Permissions {
    fn default() -> Self {
        Self {
            deployment_read: true, deployment_write: true,
            environment_read: true, environment_write: true,
            resource_read: true, resource_write: true,
            reality_read: true, reality_write: true,
            topology_read: true, topology_write: true,
            temporal_read: true, temporal_write: true,
            stakes_read: true, stakes_write: true,
            admin: false,
        }
    }
}

/// Audit logger.
pub struct AuditLogger {
    log_path: Option<String>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            log_path: std::env::var("AGENTIC_AUDIT_LOG").ok(),
        }
    }

    pub fn log_operation(&self, operation: &str, session: &str, success: bool) {
        if let Some(path) = &self.log_path {
            let entry = serde_json::json!({
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "operation": operation,
                "session": session,
                "success": success,
            });
            if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
                use std::io::Write;
                let _ = writeln!(file, "{}", entry);
            }
        }
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}
