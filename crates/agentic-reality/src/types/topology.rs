//! Topology awareness types — "What surrounds me?"

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ids::{DependencyId, IncarnationId, NeighborId, ObserverId, ServiceId};
use super::resource::LatencyStats;

/// Complete deployment topology map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentTopologyMap {
    pub self_position: TopologyPosition,
    pub upstream: Vec<UpstreamEntity>,
    pub downstream: Vec<DownstreamEntity>,
    pub siblings: Vec<SiblingEntity>,
    pub dependents: Vec<DependentEntity>,
    pub observers: Vec<ObserverEntity>,
    pub full_graph: Option<TopologyGraph>,
    pub topology_health: TopologyHealth,
}

/// The agent's position in the topology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyPosition {
    pub layer: StackLayer,
    pub edge_distance: u32,
    pub core_distance: u32,
    pub criticality: f64,
    pub redundancy: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StackLayer {
    Edge,
    Gateway,
    Application,
    Service,
    Data,
    Infrastructure,
}

impl std::fmt::Display for StackLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Edge => write!(f, "edge"),
            Self::Gateway => write!(f, "gateway"),
            Self::Application => write!(f, "application"),
            Self::Service => write!(f, "service"),
            Self::Data => write!(f, "data"),
            Self::Infrastructure => write!(f, "infrastructure"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamEntity {
    pub id: ServiceId,
    pub name: String,
    pub health: HealthStatus,
    pub latency: LatencyStats,
    pub traffic_share: f64,
}

/// A downstream dependency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownstreamEntity {
    pub id: DependencyId,
    pub service: ServiceId,
    pub entity_type: DownstreamType,
    pub health: HealthStatus,
    pub latency: LatencyStats,
    pub criticality: DependencyCriticality,
    pub fallback: Option<String>,
    pub connection: ConnectionState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownstreamType {
    Database,
    Cache,
    Queue,
    Service,
    ExternalApi,
    Storage,
    SearchIndex,
    MlModel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Healthy => write!(f, "healthy"),
            Self::Degraded => write!(f, "degraded"),
            Self::Unhealthy => write!(f, "unhealthy"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyCriticality {
    Essential,
    Important,
    Useful,
    Optional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionState {
    Connected,
    Connecting,
    Disconnected,
    Failed,
    CircuitOpen,
}

/// A sibling instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiblingEntity {
    pub incarnation: IncarnationId,
    pub neighbor_id: NeighborId,
    pub health: HealthStatus,
    pub load: LoadLevel,
    pub network_distance_ms: f64,
    pub offload_capable: bool,
    pub sync_status: SyncStatus,
    pub last_contact: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadLevel {
    Idle,
    Light,
    Moderate,
    Heavy,
    Overloaded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    InSync,
    Lagging,
    Diverged,
    Unknown,
}

/// An entity that depends on this agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependentEntity {
    pub id: ServiceId,
    pub name: String,
    pub criticality: DependencyCriticality,
    pub last_request: Option<i64>,
}

/// An entity observing this agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObserverEntity {
    pub id: ObserverId,
    pub observer_type: ObserverType,
    pub observing: Vec<String>,
    pub frequency_secs: u64,
    pub healthy: bool,
    pub trust: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObserverType {
    Metrics,
    Logging,
    Tracing,
    Apm,
    Security,
    Audit,
    Custom,
    Human,
}

/// Full topology graph (optional).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyGraph {
    pub nodes: Vec<TopologyNode>,
    pub edges: Vec<TopologyEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyNode {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub health: HealthStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyEdge {
    pub from: String,
    pub to: String,
    pub edge_type: String,
    pub weight: f64,
}

/// Health of the overall topology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyHealth {
    pub score: f64,
    pub weak_links: Vec<String>,
    pub single_points_of_failure: Vec<String>,
    pub redundancy_score: f64,
}

/// Service mesh perception.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshPerception {
    pub visible_services: Vec<VisibleService>,
    pub relationships: Vec<ServiceRelationship>,
    pub mesh_health: MeshHealth,
    pub traffic_patterns: Vec<TrafficPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibleService {
    pub id: ServiceId,
    pub name: String,
    pub version: String,
    pub health: HealthStatus,
    pub endpoints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRelationship {
    pub from: ServiceId,
    pub to: ServiceId,
    pub relationship_type: RelationshipType,
    pub latency: LatencyStats,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipType {
    Calls,
    CalledBy,
    Publishes,
    Subscribes,
    Replicates,
    Caches,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshHealth {
    pub score: f64,
    pub partitions: u32,
    pub unhealthy_services: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficPattern {
    pub pattern_type: TrafficPatternType,
    pub description: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrafficPatternType {
    Normal,
    Spike,
    Decline,
    Oscillating,
    Migrating,
}

/// Neighbor awareness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeighborAwareness {
    pub neighbors: Vec<Neighbor>,
    pub cooperation_opportunities: Vec<CooperationOpportunity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neighbor {
    pub id: NeighborId,
    pub name: String,
    pub neighbor_type: NeighborType,
    pub health: HealthStatus,
    pub distance_ms: f64,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeighborType {
    Replica,
    Complementary,
    Competing,
    Supporting,
    Monitoring,
    Gateway,
    Proxy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooperationOpportunity {
    pub with: NeighborId,
    pub cooperation_type: CooperationType,
    pub benefit: String,
    pub feasibility: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CooperationType {
    LoadBalancing,
    DataSharing,
    Failover,
    Pipeline,
    Aggregation,
    Consensus,
}

/// Dependency awareness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAwareness {
    pub tree: DependencyTree,
    pub critical_path: Vec<DependencyId>,
    pub health_summary: HashMap<DependencyId, HealthStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyTree {
    pub root: DependencyId,
    pub children: Vec<DependencyTree>,
    pub service: ServiceId,
    pub criticality: DependencyCriticality,
}
