//! Core identifier types for AgenticReality.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a deployment instance (incarnation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IncarnationId(pub Uuid);

impl IncarnationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_context(context: &str) -> Self {
        let namespace = Uuid::NAMESPACE_OID;
        Self(Uuid::new_v5(&namespace, context.as_bytes()))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for IncarnationId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for IncarnationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a reality context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContextId(pub Uuid);

impl ContextId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ContextId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ContextId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a reality anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnchorId(pub Uuid);

impl AnchorId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AnchorId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AnchorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a timeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimelineId(pub Uuid);

impl TimelineId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TimelineId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TimelineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a context transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransitionId(pub Uuid);

impl TransitionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TransitionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TransitionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a causal event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(pub Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Service identifier in topology.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceId(pub String);

impl ServiceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for ServiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Dependency identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DependencyId(pub Uuid);

impl DependencyId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for DependencyId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for DependencyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Neighbor identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NeighborId(pub String);

impl NeighborId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for NeighborId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Observer identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObserverId(pub String);

impl ObserverId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for ObserverId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Snapshot identifier for ghost writer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnapshotId(pub Uuid);

impl SnapshotId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SnapshotId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SnapshotId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incarnation_id_new() {
        let id1 = IncarnationId::new();
        let id2 = IncarnationId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_incarnation_id_from_context() {
        let id1 = IncarnationId::from_context("test-context");
        let id2 = IncarnationId::from_context("test-context");
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_context_id_uniqueness() {
        let id1 = ContextId::new();
        let id2 = ContextId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_service_id_display() {
        let sid = ServiceId::new("my-service");
        assert_eq!(sid.to_string(), "my-service");
    }
}
