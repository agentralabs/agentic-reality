//! Protocol version negotiation for MCP handshake.

/// Supported MCP protocol versions, newest first.
pub const SUPPORTED_VERSIONS: &[&str] = &["2024-11-05"];

/// Default protocol version to offer.
pub const DEFAULT_VERSION: &str = "2024-11-05";

/// Negotiates protocol version between client and server.
pub struct ProtocolNegotiator {
    supported: Vec<String>,
}

impl ProtocolNegotiator {
    /// Create a negotiator with the standard supported versions.
    pub fn new() -> Self {
        Self {
            supported: SUPPORTED_VERSIONS
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
        }
    }

    /// Negotiate the best protocol version.
    ///
    /// Returns the highest mutually supported version, or `None` if no
    /// compatible version exists.
    pub fn negotiate(&self, client_version: &str) -> Option<String> {
        // If the client requests a version we support, use it directly
        if self.supported.iter().any(|v| v == client_version) {
            return Some(client_version.to_string());
        }
        // Otherwise fall back to default if client sent something unknown
        // This is lenient: we still offer our default
        Some(DEFAULT_VERSION.to_string())
    }

    /// Check whether a specific version is supported.
    pub fn is_supported(&self, version: &str) -> bool {
        self.supported.iter().any(|v| v == version)
    }

    /// Return the list of supported versions.
    pub fn supported_versions(&self) -> &[String] {
        &self.supported
    }
}

impl Default for ProtocolNegotiator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negotiate_known_version() {
        let n = ProtocolNegotiator::new();
        let result = n.negotiate("2024-11-05");
        assert_eq!(result.as_deref(), Some("2024-11-05"));
    }

    #[test]
    fn test_negotiate_unknown_version_falls_back() {
        let n = ProtocolNegotiator::new();
        let result = n.negotiate("2099-01-01");
        assert_eq!(result.as_deref(), Some(DEFAULT_VERSION));
    }

    #[test]
    fn test_is_supported() {
        let n = ProtocolNegotiator::new();
        assert!(n.is_supported("2024-11-05"));
        assert!(!n.is_supported("1999-01-01"));
    }

    #[test]
    fn test_supported_versions_non_empty() {
        let n = ProtocolNegotiator::new();
        assert!(!n.supported_versions().is_empty());
    }

    #[test]
    fn test_default() {
        let n = ProtocolNegotiator::default();
        assert!(!n.supported_versions().is_empty());
    }
}
