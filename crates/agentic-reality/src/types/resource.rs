//! Resource proprioception types — "What do I have?"

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The agent's resource body — all resources it can feel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBody {
    pub mind: MindCapacity,
    pub energy: ProcessingEnergy,
    pub reach: NetworkReach,
    pub storage: StorageCapacity,
    pub visual: Option<GpuCapacity>,
    pub vitals: BodyVitals,
    pub sensations: Vec<ResourceSensation>,
}

/// Memory as mind capacity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindCapacity {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub feeling: MindFeeling,
    pub pressure: MemoryPressure,
    pub largest_free_bytes: u64,
    pub fragmentation: f64,
    pub swap: Option<SwapUsage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MindFeeling {
    Clear,
    Active,
    Crowded,
    Strained,
    Overwhelmed,
    Drowning,
}

impl std::fmt::Display for MindFeeling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clear => write!(f, "clear"),
            Self::Active => write!(f, "active"),
            Self::Crowded => write!(f, "crowded"),
            Self::Strained => write!(f, "strained"),
            Self::Overwhelmed => write!(f, "overwhelmed"),
            Self::Drowning => write!(f, "drowning"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryPressure {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapUsage {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub active: bool,
}

/// CPU as processing energy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingEnergy {
    pub cores: u32,
    pub utilization: f64,
    pub feeling: EnergyFeeling,
    pub load_average: [f64; 3],
    pub burst_available: bool,
    pub throttled: bool,
    pub credits: Option<CpuCredits>,
    pub temperature: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergyFeeling {
    Vigorous,
    Steady,
    Busy,
    Strained,
    Depleted,
    Constrained,
}

impl std::fmt::Display for EnergyFeeling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vigorous => write!(f, "vigorous"),
            Self::Steady => write!(f, "steady"),
            Self::Busy => write!(f, "busy"),
            Self::Strained => write!(f, "strained"),
            Self::Depleted => write!(f, "depleted"),
            Self::Constrained => write!(f, "constrained"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuCredits {
    pub remaining: f64,
    pub earning_rate: f64,
    pub depleted: bool,
}

/// Network as reach.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkReach {
    pub bandwidth_mbps: u64,
    pub utilization: f64,
    pub latencies: HashMap<String, LatencyStats>,
    pub feeling: ReachFeeling,
    pub connections: ConnectionStats,
    pub stability: f64,
    pub packet_loss: f64,
    pub egress_remaining: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReachFeeling {
    Connected,
    Normal,
    Sluggish,
    Constrained,
    Isolated,
    Partitioned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyStats {
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub max_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStats {
    pub active: u32,
    pub idle: u32,
    pub max: u32,
    pub errored: u32,
}

/// Disk as storage capacity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapacity {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub iops: Option<u64>,
    pub throughput_mbps: Option<u64>,
    pub latency_ms: Option<f64>,
}

/// GPU capacity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuCapacity {
    pub count: u32,
    pub gpu_type: String,
    pub memory_total_bytes: u64,
    pub memory_used_bytes: u64,
    pub utilization: f64,
    pub temperature: Option<f64>,
}

/// Vital signs of the resource body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyVitals {
    pub overall_health: f64,
    pub heartbeat_ms: u64,
    pub last_check: i64,
    pub anomalies: Vec<String>,
}

/// A sensation felt in a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSensation {
    pub resource: ResourceType,
    pub sensation: SensationType,
    pub intensity: f64,
    pub started: i64,
    pub trend: SensationTrend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Memory,
    Cpu,
    Network,
    Storage,
    Gpu,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Memory => write!(f, "memory"),
            Self::Cpu => write!(f, "cpu"),
            Self::Network => write!(f, "network"),
            Self::Storage => write!(f, "storage"),
            Self::Gpu => write!(f, "gpu"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensationType {
    Comfort,
    Pressure,
    Pain,
    Relief,
    Alarm,
    Numbness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensationTrend {
    Improving,
    Stable,
    Worsening,
}

/// Pressure gradient across resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePressureGradient {
    pub bottleneck: Option<ResourceType>,
    pub pressures: HashMap<ResourceType, Pressure>,
    pub flow: PressureFlow,
    pub building: Vec<ResourceType>,
    pub releasing: Vec<ResourceType>,
    pub predicted_bottleneck: Option<PredictedBottleneck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pressure {
    pub level: f64,
    pub trend: PressureTrend,
    pub source: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PressureTrend {
    Rising,
    Stable,
    Falling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PressureFlow {
    Balanced,
    Shifting,
    Cascading,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedBottleneck {
    pub resource: ResourceType,
    pub estimated_secs: u64,
    pub confidence: f64,
}

/// Cost consciousness — feeling operational costs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostConsciousness {
    pub burn_rate: Cost,
    pub session_cost: Cost,
    pub budget: Option<BudgetConstraints>,
    pub breakdown: HashMap<String, Cost>,
    pub feeling: CostFeeling,
    pub projections: Vec<CostProjection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cost {
    pub monetary: f64,
    pub carbon: Option<f64>,
    pub opportunity: Option<f64>,
    pub reputation: Option<f64>,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConstraints {
    pub total: f64,
    pub remaining: f64,
    pub period: String,
    pub hard_limit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CostFeeling {
    Comfortable,
    Mindful,
    Concerned,
    Anxious,
    Panicked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostProjection {
    pub period: String,
    pub projected_cost: f64,
    pub confidence: f64,
}

/// Capability map — what the agent can do.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMap {
    pub capabilities: Vec<Capability>,
    pub discovered_at: i64,
    pub stale_after_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub category: CapabilityCategory,
    pub available: bool,
    pub constraints: Vec<CapabilityConstraint>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapabilityCategory {
    Compute,
    Storage,
    Network,
    ExternalApi,
    SisterBridge,
    Security,
    MachineLearning,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityConstraint {
    pub constraint_type: ConstraintType,
    pub description: String,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintType {
    RateLimit,
    Quota,
    TimeWindow,
    CostLimit,
    Permission,
    Dependency,
}

/// Capacity planning intuition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityIntuition {
    pub adequacy: CapacityAdequacy,
    pub intuitions: Vec<ResourceIntuition>,
    pub patterns: Vec<CapacityPattern>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapacityAdequacy {
    Abundant,
    Sufficient,
    Tight,
    Insufficient,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceIntuition {
    pub resource: ResourceType,
    pub current_adequacy: CapacityAdequacy,
    pub predicted_adequacy: CapacityAdequacy,
    pub horizon_secs: u64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityPattern {
    pub pattern_type: PatternType,
    pub resource: ResourceType,
    pub description: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternType {
    DailySpike,
    WeeklyPattern,
    GrowthTrend,
    DecayTrend,
    BurstPattern,
    SeasonalPattern,
    Anomaly,
}
