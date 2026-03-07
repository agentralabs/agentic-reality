//! AgenticReality — existential grounding system for AI agents.
//!
//! Gives AI agents awareness of deployment context, resources, reality layers,
//! topology, time, stakes, and coherence. Sister #10 "The Ground" in the
//! Agentra Labs ecosystem.

pub mod bridges;
pub mod cache;
#[cfg(feature = "cli")]
pub mod cli;
pub mod engine;
#[cfg(feature = "ffi")]
pub mod ffi;
#[cfg(feature = "format")]
pub mod format;
pub mod index;
pub mod inventions;
pub mod metrics;
pub mod query;
pub mod security;
pub mod storage;
pub mod types;
pub mod validation;

// Re-export commonly used types at the crate root
pub use engine::{QueryEngine, RealityEngine, WriteEngine};
pub use types::{
    AnchorId, ContextId, DependencyId, EventId, IncarnationId, NeighborId, ObserverId,
    RealityError, RealityResult, ServiceId, SnapshotId, TimelineId, TransitionId,
};
