//! Session management for the MCP server.
//!
//! Manages the reality engine lifecycle, persistence via .areal files,
//! and workspace identity derivation.

use std::path::{Path, PathBuf};

use agentic_reality::engine::RealityEngine;
use agentic_reality::format::{ArealReader, ArealWriter};

/// Manages the reality engine session.
pub struct SessionManager {
    /// The reality engine instance.
    pub engine: RealityEngine,
    /// Path to the .areal data file (if persistence is enabled).
    pub data_path: Option<String>,
    /// Workspace identifier derived from the canonical data path.
    pub workspace_id: Option<String>,
    /// Whether autosave is enabled.
    pub autosave: bool,
}

impl SessionManager {
    /// Create a new session with no persistence.
    pub fn new() -> Self {
        Self {
            engine: RealityEngine::new(),
            data_path: None,
            workspace_id: None,
            autosave: false,
        }
    }

    /// Create a new session with a data path for persistence.
    pub fn with_path(path: String) -> Self {
        let workspace_id = derive_workspace_id(&path);
        Self {
            engine: RealityEngine::new(),
            data_path: Some(path),
            workspace_id,
            autosave: false,
        }
    }

    /// Enable or disable autosave.
    pub fn set_autosave(&mut self, enabled: bool) {
        self.autosave = enabled;
    }

    /// Whether the engine has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.engine.is_dirty()
    }

    /// Save the session to disk if a data path is configured.
    ///
    /// Returns `Ok(true)` if data was written, `Ok(false)` if no path is set.
    pub fn save(&mut self) -> Result<bool, SessionError> {
        let path_str = match &self.data_path {
            Some(p) => p.clone(),
            None => return Ok(false),
        };

        let path = PathBuf::from(&path_str);

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| SessionError::Io(e.to_string()))?;
        }

        ArealWriter::save(&self.engine, &path)
            .map_err(|e| SessionError::Save(e.to_string()))?;

        self.engine.mark_clean();
        Ok(true)
    }

    /// Load session data from disk if a data path is configured.
    ///
    /// Returns `Ok(true)` if data was loaded, `Ok(false)` if no path or file missing.
    pub fn load(&mut self) -> Result<bool, SessionError> {
        let path_str = match &self.data_path {
            Some(p) => p.clone(),
            None => return Ok(false),
        };

        let path = PathBuf::from(&path_str);
        if !path.exists() {
            return Ok(false);
        }

        let engine = ArealReader::load(&path)
            .map_err(|e| SessionError::Load(e.to_string()))?;

        self.engine = engine;
        Ok(true)
    }

    /// Save if dirty and autosave is enabled.
    pub fn autosave_if_dirty(&mut self) -> Result<bool, SessionError> {
        if self.autosave && self.is_dirty() {
            self.save()
        } else {
            Ok(false)
        }
    }

    /// Get the workspace identifier.
    pub fn workspace_id(&self) -> Option<&str> {
        self.workspace_id.as_deref()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Derive a workspace ID from a canonical file path.
///
/// Uses a BLAKE3 hash of the path to produce a stable, short identifier.
fn derive_workspace_id(path: &str) -> Option<String> {
    let canonical = match std::fs::canonicalize(Path::new(path)) {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => path.to_string(),
    };
    let hash = blake3::hash(canonical.as_bytes());
    let hex = hash.to_hex();
    Some(hex[..16].to_string())
}

/// Session management errors.
#[derive(Debug)]
pub enum SessionError {
    /// IO error during save or load.
    Io(String),
    /// Error while saving.
    Save(String),
    /// Error while loading.
    Load(String),
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::Io(msg) => write!(f, "session IO error: {}", msg),
            SessionError::Save(msg) => write!(f, "session save error: {}", msg),
            SessionError::Load(msg) => write!(f, "session load error: {}", msg),
        }
    }
}

impl std::error::Error for SessionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_session() {
        let s = SessionManager::new();
        assert!(s.data_path.is_none());
        assert!(s.workspace_id.is_none());
        assert!(!s.autosave);
    }

    #[test]
    fn test_session_with_path() {
        let s = SessionManager::with_path("/tmp/test.areal".into());
        assert_eq!(s.data_path.as_deref(), Some("/tmp/test.areal"));
        assert!(s.workspace_id.is_some());
    }

    #[test]
    fn test_autosave_toggle() {
        let mut s = SessionManager::new();
        assert!(!s.autosave);
        s.set_autosave(true);
        assert!(s.autosave);
    }

    #[test]
    fn test_save_no_path_returns_false() {
        let mut s = SessionManager::new();
        let result = s.save();
        assert!(result.is_ok());
        match result {
            Ok(saved) => assert!(!saved),
            Err(_) => panic!("expected Ok(false)"),
        }
    }

    #[test]
    fn test_load_no_path_returns_false() {
        let mut s = SessionManager::new();
        let result = s.load();
        assert!(result.is_ok());
        match result {
            Ok(loaded) => assert!(!loaded),
            Err(_) => panic!("expected Ok(false)"),
        }
    }

    #[test]
    fn test_workspace_id_derivation() {
        let id = derive_workspace_id("/tmp/test.areal");
        assert!(id.is_some());
        match id {
            Some(ref s) => assert_eq!(s.len(), 16),
            None => panic!("expected Some"),
        }
    }

    #[test]
    fn test_session_error_display() {
        let e = SessionError::Save("disk full".into());
        assert!(e.to_string().contains("disk full"));
    }

    #[test]
    fn test_default() {
        let s = SessionManager::default();
        assert!(s.data_path.is_none());
    }

    #[test]
    fn test_autosave_if_dirty_not_dirty() {
        let mut s = SessionManager::new();
        s.set_autosave(true);
        let result = s.autosave_if_dirty();
        assert!(result.is_ok());
        // Engine is not dirty, so no save
        match result {
            Ok(saved) => assert!(!saved),
            Err(_) => panic!("expected Ok(false)"),
        }
    }
}
