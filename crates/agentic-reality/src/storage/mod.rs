//! Domain stores — 8 stores for the 7 reality domains + indexes.

use serde::{Deserialize, Serialize};

use crate::types::coherence::*;
use crate::types::deployment::*;
use crate::types::environment::*;
use crate::types::reality::*;
use crate::types::resource::*;
use crate::types::stakes::*;
use crate::types::temporal::*;
use crate::types::topology::*;

/// Store for deployment consciousness domain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeploymentStore {
    pub soul: Option<DeploymentSoul>,
    pub incarnation_memory: Option<IncarnationMemory>,
}

impl DeploymentStore {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Store for environment sensing domain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnvironmentStore {
    pub medium: Option<EnvironmentMedium>,
    pub fingerprint: Option<ContextFingerprint>,
}

impl EnvironmentStore {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Store for resource proprioception domain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceStore {
    pub body: Option<ResourceBody>,
    pub pressure_gradient: Option<ResourcePressureGradient>,
    pub cost: Option<CostConsciousness>,
    pub capabilities: Option<CapabilityMap>,
    pub capacity_intuition: Option<CapacityIntuition>,
}

impl ResourceStore {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Store for reality physics domain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RealityStore {
    pub layers: Option<RealityLayers>,
    pub freshness: Option<FreshnessPerception>,
    pub anchors: Vec<RealityAnchor>,
    pub anchor_drifts: Vec<AnchorDrift>,
    pub hallucination_state: Option<HallucinationState>,
}

impl RealityStore {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Store for topology awareness domain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopologyStore {
    pub topology: Option<DeploymentTopologyMap>,
    pub mesh: Option<ServiceMeshPerception>,
    pub neighbors: Option<NeighborAwareness>,
    pub dependencies: Option<DependencyAwareness>,
}

impl TopologyStore {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Store for temporal grounding domain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemporalStore {
    pub awareness: Option<TemporalAwareness>,
    pub causality: Option<CausalityGraph>,
    pub timelines: Vec<Timeline>,
}

impl TemporalStore {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Store for stakes perception domain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StakesStore {
    pub consequences: Option<ConsequenceAwareness>,
    pub risk_field: Option<RiskFieldPerception>,
    pub blast_radius: Option<BlastRadiusAwareness>,
}

impl StakesStore {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Store for coherence maintenance domain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CoherenceStore {
    pub state: Option<CoherenceState>,
    pub transitions: Option<TransitionState>,
}

impl CoherenceStore {
    pub fn new() -> Self {
        Self::default()
    }
}
