//! Session management for the MCP server.

use agentic_reality::engine::RealityEngine;

/// Manages the reality engine session.
pub struct SessionManager {
    pub engine: RealityEngine,
    pub data_path: Option<String>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            engine: RealityEngine::new(),
            data_path: None,
        }
    }

    pub fn with_path(path: String) -> Self {
        Self {
            engine: RealityEngine::new(),
            data_path: Some(path),
        }
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
