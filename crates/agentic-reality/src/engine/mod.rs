//! Write and Query engines for AgenticReality.

pub mod query;
pub mod write;

pub use query::QueryEngine;
pub use write::WriteEngine;

use crate::index::RealityIndexes;
use crate::storage::*;
use crate::types::ids::IncarnationId;

/// The main reality engine combining write and query capabilities.
pub struct RealityEngine {
    pub deployment_store: DeploymentStore,
    pub environment_store: EnvironmentStore,
    pub resource_store: ResourceStore,
    pub reality_store: RealityStore,
    pub topology_store: TopologyStore,
    pub temporal_store: TemporalStore,
    pub stakes_store: StakesStore,
    pub coherence_store: CoherenceStore,
    pub indexes: RealityIndexes,
    dirty: bool,
}

impl RealityEngine {
    /// Create a new empty reality engine.
    pub fn new() -> Self {
        Self {
            deployment_store: DeploymentStore::new(),
            environment_store: EnvironmentStore::new(),
            resource_store: ResourceStore::new(),
            reality_store: RealityStore::new(),
            topology_store: TopologyStore::new(),
            temporal_store: TemporalStore::new(),
            stakes_store: StakesStore::new(),
            coherence_store: CoherenceStore::new(),
            indexes: RealityIndexes::new(),
            dirty: false,
        }
    }

    /// Get a write engine handle.
    pub fn writer(&mut self) -> WriteEngine<'_> {
        WriteEngine::new(self)
    }

    /// Get a query engine handle.
    pub fn reader(&self) -> QueryEngine<'_> {
        QueryEngine::new(self)
    }

    /// Whether the engine has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Mark the engine as dirty (has unsaved changes).
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Mark the engine as clean (all changes saved).
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Get the current incarnation ID if initialized.
    pub fn incarnation_id(&self) -> Option<IncarnationId> {
        self.deployment_store.soul.as_ref().map(|s| s.incarnation_id)
    }

    /// Check if the engine has been initialized with a deployment soul.
    pub fn is_initialized(&self) -> bool {
        self.deployment_store.soul.is_some()
    }
}

impl Default for RealityEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Context summary — quick overview of reality state.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextSummary {
    pub incarnation_id: Option<IncarnationId>,
    pub substrate_tier: Option<String>,
    pub environment_type: Option<String>,
    pub environment_mood: Option<String>,
    pub resource_bottleneck: Option<String>,
    pub stakes_level: Option<String>,
    pub coherence_level: Option<String>,
    pub uptime_secs: Option<u64>,
}
