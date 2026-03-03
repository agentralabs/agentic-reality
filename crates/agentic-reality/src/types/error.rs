//! Error types for AgenticReality.

use thiserror::Error;

/// Result type alias for AgenticReality operations.
pub type RealityResult<T> = Result<T, RealityError>;

/// All errors that can occur in AgenticReality.
#[derive(Error, Debug)]
pub enum RealityError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Invalid magic bytes")]
    InvalidMagic,

    #[error("Version mismatch: expected {expected}, got {got}")]
    VersionMismatch { expected: u32, got: u32 },

    #[error("Checksum mismatch: expected {expected}, got {got}")]
    ChecksumMismatch { expected: String, got: String },

    #[error("Section not found: {0}")]
    SectionNotFound(String),

    #[error("Not initialized: {0}")]
    NotInitialized(String),

    #[error("Already initialized: {0}")]
    AlreadyInitialized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Duplicate: {0}")]
    Duplicate(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Invalid parameter: {field} — {reason}")]
    InvalidParameter { field: String, reason: String },

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Capacity exceeded: {0}")]
    CapacityExceeded(String),

    #[error("Transition error: {0}")]
    TransitionError(String),

    #[error("Coherence violation: {0}")]
    CoherenceViolation(String),

    #[error("Bridge error: {bridge} — {message}")]
    BridgeError { bridge: String, message: String },

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<serde_json::Error> for RealityError {
    fn from(e: serde_json::Error) -> Self {
        RealityError::Serialization(e.to_string())
    }
}

impl From<bincode::Error> for RealityError {
    fn from(e: bincode::Error) -> Self {
        RealityError::Serialization(e.to_string())
    }
}

impl From<uuid::Error> for RealityError {
    fn from(e: uuid::Error) -> Self {
        RealityError::Validation(e.to_string())
    }
}
