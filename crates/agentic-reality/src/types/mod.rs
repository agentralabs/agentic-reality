//! Core data types for AgenticReality.

pub mod coherence;
pub mod deployment;
pub mod environment;
pub mod error;
pub mod ids;
pub mod reality;
pub mod resource;
pub mod stakes;
pub mod temporal;
pub mod topology;

// Re-export all ID types
pub use ids::{
    AnchorId, ContextId, DependencyId, EventId, IncarnationId, NeighborId, ObserverId, ServiceId,
    SnapshotId, TimelineId, TransitionId,
};

// Re-export error types
pub use error::{RealityError, RealityResult};

/// Current timestamp in microseconds since epoch.
pub fn now_micros() -> u64 {
    chrono::Utc::now().timestamp_micros() as u64
}

/// .areal file magic bytes: "REAL"
pub const AREAL_MAGIC: [u8; 4] = [0x52, 0x45, 0x41, 0x4C];

/// .areal format version
pub const FORMAT_VERSION: u32 = 1;

/// Header size in bytes
pub const HEADER_SIZE: usize = 256;

/// Footer size in bytes
pub const FOOTER_SIZE: usize = 64;

/// Footer magic: "REALEND\0"
pub const FOOTER_MAGIC: [u8; 8] = [0x52, 0x45, 0x41, 0x4C, 0x45, 0x4E, 0x44, 0x00];
