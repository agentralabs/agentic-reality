//! Reality physics types — "What is real?"

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ids::AnchorId;

/// Multiple layers of reality the agent perceives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityLayers {
    pub current_layer: RealityLayer,
    pub layers: Vec<LayerStatus>,
    pub consistency: LayerConsistency,
    pub transitions: Vec<LayerTransition>,
    pub confidence: f64,
}

/// A layer of reality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RealityLayer {
    Physical {
        substrate: String,
        certainty: f64,
    },
    Virtual {
        virtualization: String,
        host: Option<String>,
    },
    Container {
        runtime: String,
        orchestrator: Option<String>,
    },
    Sandbox {
        isolation_type: String,
        restrictions: Vec<String>,
    },
    TestEnvironment {
        test_type: String,
        mocked_components: Vec<String>,
    },
    Simulation {
        fidelity: SimulationFidelity,
        purpose: String,
        simulated_time: Option<i64>,
    },
    Replay {
        source: String,
        timestamp: i64,
    },
    Preview {
        what_would_happen: String,
        commit_possible: bool,
    },
    Unknown {
        clues: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationFidelity {
    Perfect,
    High,
    Medium,
    Low,
    Stub,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerStatus {
    pub layer: RealityLayer,
    pub active: bool,
    pub confidence: f64,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerConsistency {
    pub consistent: bool,
    pub conflicts: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerTransition {
    pub from: String,
    pub to: String,
    pub reason: String,
    pub timestamp: i64,
}

/// Information freshness perception.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreshnessPerception {
    pub overall: FreshnessLevel,
    pub by_source: HashMap<String, SourceFreshness>,
    pub stalest: Option<StaleData>,
    pub requirements: Vec<FreshnessRequirement>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FreshnessLevel {
    Live { latency_ms: u64 },
    Fresh { age_secs: u64 },
    Acceptable { age_secs: u64 },
    Aging { age_secs: u64, concern: String },
    Stale { age_secs: u64, usable: bool },
    Ancient { age_secs: u64, archival: bool },
    Unknown { last_known: Option<i64> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFreshness {
    pub source: String,
    pub level: FreshnessLevel,
    pub last_updated: i64,
    pub update_frequency_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaleData {
    pub source: String,
    pub age_secs: u64,
    pub impact: StaleImpact,
    pub recommendation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StaleImpact {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreshnessRequirement {
    pub source: String,
    pub max_age_secs: u64,
    pub reason: String,
}

/// Reality anchor — a verifiable ground truth point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityAnchor {
    pub id: AnchorId,
    pub anchor_type: AnchorType,
    pub verification: VerificationMethod,
    pub last_value: AnchorValue,
    pub trust: f64,
    pub frequency_secs: u64,
    pub dependents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnchorType {
    Time { source: String },
    Identity { verifier: String },
    Configuration { source: String },
    State { source: String },
    External { api: String, field: String },
    Cryptographic { chain: String },
    Human { verifier: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationMethod {
    Direct,
    Api { endpoint: String },
    Consensus { quorum: u32 },
    Cryptographic { algorithm: String },
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorValue {
    pub value: String,
    pub verified_at: i64,
    pub confidence: f64,
}

/// Drift detected in an anchor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorDrift {
    pub anchor_id: AnchorId,
    pub expected: String,
    pub actual: String,
    pub drift_magnitude: f64,
    pub assessment: DriftAssessment,
    pub detected_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriftAssessment {
    Normal,
    Concerning,
    Significant,
    Critical,
}

/// Hallucination detection state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HallucinationState {
    pub risk_level: HallucinationRisk,
    pub detected: Vec<DetectedHallucination>,
    pub patterns: Vec<HallucinationPattern>,
    pub grounding: GroundingStatus,
    pub pending_verification: Vec<UnverifiedClaim>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HallucinationRisk {
    Low { confidence: f64 },
    Moderate { concerns: Vec<String> },
    Elevated { high_risk_claims: Vec<String> },
    High { indicators: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedHallucination {
    pub id: String,
    pub hallucination_type: HallucinationType,
    pub claim: String,
    pub evidence: String,
    pub detection_method: DetectionMethod,
    pub detected_at: i64,
    pub resolved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HallucinationType {
    FactualError,
    TemporalConfusion,
    ContextBleed,
    Confabulation,
    WishfulThinking,
    PatternOverextension,
    SourceConfusion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DetectionMethod {
    AnchorVerification,
    ConsistencyCheck,
    ExternalValidation,
    PatternAnalysis,
    SelfReport,
    PeerVerification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HallucinationPattern {
    pub pattern: String,
    pub frequency: u32,
    pub trigger: HallucinationTrigger,
    pub mitigation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HallucinationTrigger {
    StaleData,
    AnchorDrift,
    ContextSwitch,
    HighLoad,
    InsufficientGrounding,
    Ambiguity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundingStatus {
    pub grounded: bool,
    pub anchor_count: u32,
    pub verified_count: u32,
    pub last_verification: Option<i64>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnverifiedClaim {
    pub claim: String,
    pub source: String,
    pub submitted_at: i64,
    pub priority: ClaimPriority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimPriority {
    Low,
    Medium,
    High,
    Critical,
}
