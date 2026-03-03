//! Server configuration — loading from environment, args, and defaults.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// MCP server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Transport mode: "stdio" or "http".
    pub mode: String,
    /// HTTP port (only used in http mode).
    pub port: u16,
    /// Optional authentication token for HTTP mode.
    pub auth_token: Option<String>,
    /// Data directory for .areal file storage.
    pub data_dir: Option<String>,
    /// Server name advertised in MCP initialize.
    pub server_name: String,
    /// Whether autosave is enabled.
    pub autosave: bool,
    /// Log level: "error", "warn", "info", "debug", "trace".
    pub log_level: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            mode: "stdio".to_string(),
            port: 3010,
            auth_token: None,
            data_dir: None,
            server_name: "agentic-reality".to_string(),
            autosave: true,
            log_level: "info".to_string(),
        }
    }
}

impl ServerConfig {
    /// Resolve the data path for the .areal file.
    ///
    /// Priority:
    /// 1. Explicit data_dir from config
    /// 2. AGENTIC_REALITY_DATA_DIR env var
    /// 3. $HOME/.agentic-reality/
    pub fn resolve_data_path(&self) -> PathBuf {
        resolve_data_path(self.data_dir.as_deref())
    }
}

/// Resolve the data path from an optional explicit dir, env var, or default.
pub fn resolve_data_path(explicit: Option<&str>) -> PathBuf {
    if let Some(dir) = explicit {
        return PathBuf::from(dir);
    }

    if let Ok(env_dir) = std::env::var("AGENTIC_REALITY_DATA_DIR") {
        if !env_dir.is_empty() {
            return PathBuf::from(env_dir);
        }
    }

    // Default: ~/.agentic-reality/
    match home_dir() {
        Some(home) => home.join(".agentic-reality"),
        None => PathBuf::from(".agentic-reality"),
    }
}

/// Load server config from environment variables, merging with defaults.
pub fn load_config() -> ServerConfig {
    let mut config = ServerConfig::default();

    if let Ok(mode) = std::env::var("AGENTIC_REALITY_MODE") {
        if !mode.is_empty() {
            config.mode = mode;
        }
    }

    if let Ok(port_str) = std::env::var("AGENTIC_REALITY_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            config.port = port;
        }
    }

    if let Ok(token) = std::env::var("AGENTIC_REALITY_AUTH_TOKEN") {
        if !token.is_empty() {
            config.auth_token = Some(token);
        }
    }

    if let Ok(dir) = std::env::var("AGENTIC_REALITY_DATA_DIR") {
        if !dir.is_empty() {
            config.data_dir = Some(dir);
        }
    }

    if let Ok(level) = std::env::var("AGENTIC_REALITY_LOG_LEVEL") {
        if !level.is_empty() {
            config.log_level = level;
        }
    }

    if let Ok(autosave) = std::env::var("AGENTIC_REALITY_AUTOSAVE") {
        config.autosave = autosave != "0" && autosave != "false";
    }

    config
}

/// Cross-platform home directory lookup.
fn home_dir() -> Option<PathBuf> {
    #[cfg(unix)]
    {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
    #[cfg(windows)]
    {
        std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .ok()
            .map(PathBuf::from)
    }
    #[cfg(not(any(unix, windows)))]
    {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let c = ServerConfig::default();
        assert_eq!(c.mode, "stdio");
        assert_eq!(c.port, 3010);
        assert!(c.auth_token.is_none());
        assert!(c.autosave);
        assert_eq!(c.server_name, "agentic-reality");
    }

    #[test]
    fn test_resolve_data_path_explicit() {
        let path = resolve_data_path(Some("/tmp/my-data"));
        assert_eq!(path, PathBuf::from("/tmp/my-data"));
    }

    #[test]
    fn test_resolve_data_path_default() {
        let path = resolve_data_path(None);
        // Should end with .agentic-reality
        let path_str = path.to_string_lossy();
        assert!(path_str.ends_with(".agentic-reality") || path_str.contains("agentic-reality"));
    }

    #[test]
    fn test_load_config_returns_defaults() {
        // In test environment, env vars may not be set, so should return defaults
        let c = load_config();
        assert!(!c.mode.is_empty());
        assert!(c.port > 0);
    }

    #[test]
    fn test_config_serialization() {
        let c = ServerConfig::default();
        let json = serde_json::to_string(&c);
        assert!(json.is_ok());
    }
}
