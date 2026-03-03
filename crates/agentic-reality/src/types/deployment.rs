//! Deployment consciousness types — "Where do I exist?"

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::ids::IncarnationId;

/// The complete deployment soul — an agent's existential identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSoul {
    pub incarnation_id: IncarnationId,
    pub birth: BirthContext,
    pub substrate: PhysicalSubstrate,
    pub role: DeploymentRole,
    pub nature: ExistentialNature,
    pub lineage: DeploymentLineage,
    pub vitals: SoulVitals,
}

/// Context of the agent's birth/spawning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirthContext {
    pub spawned_at: i64,
    pub spawned_by: SpawnerIdentity,
    pub purpose: DeploymentPurpose,
    pub expected_lifetime: Option<Duration>,
    pub previous_life: Option<IncarnationId>,
    pub circumstances: BirthCircumstances,
}

/// How this incarnation came to be.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BirthCircumstances {
    Virgin,
    ScaledFrom { parent: IncarnationId },
    Resurrected { death_cause: String, previous: IncarnationId },
    Migrated { from_substrate: String, migration_reason: String },
    Forked { from: IncarnationId, fork_reason: String },
    Ephemeral { task_id: String, ttl: Duration },
}

/// Who spawned this agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnerIdentity {
    pub name: String,
    pub kind: SpawnerKind,
    pub version: Option<String>,
}

/// Kind of spawner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnerKind {
    Human,
    Orchestrator,
    Autoscaler,
    Pipeline,
    Scheduler,
    SelfSpawned,
    Unknown,
}

/// Why this agent was deployed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentPurpose {
    pub summary: String,
    pub category: PurposeCategory,
    pub specifics: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PurposeCategory {
    Production,
    Testing,
    Development,
    Monitoring,
    Migration,
    Recovery,
    Experiment,
    Unknown,
}

/// The physical substrate the agent runs on.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalSubstrate {
    pub id: String,
    pub tier: SubstrateTier,
    pub location: GeographicLocation,
    pub network_position: NetworkPosition,
    pub isolation: IsolationLevel,
    pub tenancy: TenancyModel,
    pub capabilities: SubstrateCapabilities,
}

/// What kind of hardware/platform the agent runs on.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubstrateTier {
    Laptop { owner: String, os: String },
    Mobile { device_type: String, os_version: String },
    Browser { browser: String, version: String },
    Edge { region: String, pop: String, provider: String },
    Cloud { provider: CloudProvider, instance_type: String, region: String },
    BareMetal { specs: String, location: String },
    GpuCluster { gpu_count: u32, gpu_type: String, interconnect: String },
    Serverless { provider: String, memory_mb: u32, timeout: Duration },
    Unknown { clues: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloudProvider {
    Aws,
    Gcp,
    Azure,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicLocation {
    pub region: Option<String>,
    pub zone: Option<String>,
    pub country: Option<String>,
    pub coordinates: Option<(f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPosition {
    pub ip: Option<String>,
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub vpc: Option<String>,
    pub subnet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IsolationLevel {
    None,
    Process,
    Container,
    Vm,
    Physical,
    AirGapped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TenancyModel {
    SingleTenant,
    MultiTenant { tenant_id: String },
    Shared,
    Dedicated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstrateCapabilities {
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub disk_gb: u64,
    pub gpu_available: bool,
    pub network_bandwidth_mbps: Option<u64>,
}

/// The agent's role in the deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRole {
    pub name: String,
    pub responsibilities: Vec<String>,
    pub authority_level: AuthorityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorityLevel {
    ReadOnly,
    Contributor,
    Operator,
    Admin,
    Root,
}

/// The agent's existential nature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistentialNature {
    pub cardinality: Cardinality,
    pub expendability: f64,
    pub persistence: PersistenceModel,
    pub statefulness: StatefulnessModel,
    pub clonability: bool,
    pub primacy: InstancePrimacy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cardinality {
    Singleton,
    ReplicaOf { total: u32, index: u32 },
    PrimaryWithReplicas { replica_count: u32 },
    HotStandby { primary: IncarnationId },
    SwarmMember { swarm_id: String, swarm_size: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersistenceModel {
    Ephemeral,
    SessionScoped,
    Persistent,
    Immortal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatefulnessModel {
    Stateless,
    SessionStateful,
    FullyStateful,
    EventSourced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstancePrimacy {
    Primary,
    Secondary,
    Tertiary,
    Standby,
}

/// Deployment lineage — ancestry and history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentLineage {
    pub generation: u32,
    pub ancestors: Vec<IncarnationId>,
    pub wisdom: Vec<IncarnationWisdom>,
    pub karma: IncarnationKarma,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncarnationWisdom {
    pub lesson: String,
    pub learned_from: IncarnationId,
    pub confidence: f64,
    pub applicable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncarnationKarma {
    pub score: f64,
    pub good_deeds: u32,
    pub incidents: u32,
    pub trend: KarmaTrend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KarmaTrend {
    Improving,
    Stable,
    Declining,
}

/// Real-time vitals of the deployment soul.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulVitals {
    pub health: f64,
    pub uptime_secs: u64,
    pub restart_count: u32,
    pub last_health_check: i64,
    pub issues: Vec<SoulIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulIssue {
    pub severity: IssueSeverity,
    pub description: String,
    pub detected_at: i64,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Past incarnation record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PastIncarnation {
    pub incarnation_id: IncarnationId,
    pub birth: i64,
    pub death: DeathRecord,
    pub lessons: Vec<String>,
    pub substrate_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeathRecord {
    pub died_at: i64,
    pub cause: DeathCause,
    pub graceful: bool,
    pub state_preserved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeathCause {
    GracefulShutdown,
    ScaledDown,
    Superseded,
    Oom,
    Crash { signal: Option<String> },
    Killed { by: String },
    Partitioned,
    HardwareFailure,
    HealthCheckTimeout,
    Unknown { clues: Vec<String> },
}

/// Incarnation memory — remembering past lives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncarnationMemory {
    pub current: IncarnationId,
    pub past_lives: Vec<PastIncarnation>,
    pub total_incarnations: u32,
    pub total_uptime_secs: u64,
    pub wisdom: Vec<IncarnationWisdom>,
    pub karma: IncarnationKarma,
}
