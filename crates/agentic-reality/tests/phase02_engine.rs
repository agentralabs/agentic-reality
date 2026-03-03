//! Phase 02: Engine write and query tests.

use agentic_reality::engine::RealityEngine;
use agentic_reality::types::deployment::*;
use agentic_reality::types::environment::{self, *};
use agentic_reality::types::resource::*;
use agentic_reality::types::reality::*;
use agentic_reality::types::topology::*;
use agentic_reality::types::temporal::*;
use agentic_reality::types::stakes::*;
use agentic_reality::types::coherence::*;
use agentic_reality::types::ids::*;
use std::collections::HashMap;

fn make_soul() -> DeploymentSoul {
    DeploymentSoul {
        incarnation_id: IncarnationId::new(),
        birth: BirthContext {
            spawned_at: 1000,
            spawned_by: SpawnerIdentity { name: "test".into(), kind: SpawnerKind::Human, version: None },
            purpose: DeploymentPurpose { summary: "testing".into(), category: PurposeCategory::Testing, specifics: HashMap::new() },
            expected_lifetime: None,
            previous_life: None,
            circumstances: BirthCircumstances::Virgin,
        },
        substrate: PhysicalSubstrate {
            id: "test-substrate".into(),
            tier: SubstrateTier::Laptop { owner: "dev".into(), os: "macos".into() },
            location: GeographicLocation { region: None, zone: None, country: None, coordinates: None },
            network_position: NetworkPosition { ip: None, hostname: None, port: None, vpc: None, subnet: None },
            isolation: IsolationLevel::Process,
            tenancy: TenancyModel::SingleTenant,
            capabilities: SubstrateCapabilities { cpu_cores: 8, memory_mb: 16384, disk_gb: 512, gpu_available: false, network_bandwidth_mbps: Some(1000) },
        },
        role: DeploymentRole { name: "test-agent".into(), responsibilities: vec![], authority_level: AuthorityLevel::Contributor },
        nature: ExistentialNature {
            cardinality: Cardinality::Singleton,
            expendability: 0.5,
            persistence: PersistenceModel::SessionScoped,
            statefulness: StatefulnessModel::FullyStateful,
            clonability: true,
            primacy: InstancePrimacy::Primary,
        },
        lineage: DeploymentLineage {
            generation: 1,
            ancestors: vec![],
            wisdom: vec![],
            karma: IncarnationKarma { score: 0.0, good_deeds: 0, incidents: 0, trend: KarmaTrend::Stable },
        },
        vitals: SoulVitals {
            health: 1.0,
            uptime_secs: 0,
            restart_count: 0,
            last_health_check: 1000,
            issues: vec![],
        },
    }
}

// === Deployment Write Tests ===

#[test]
fn test_initialize_soul() {
    let mut engine = RealityEngine::new();
    let soul = make_soul();
    let id = engine.writer().initialize_soul(soul).unwrap();
    assert!(engine.is_initialized());
    assert_eq!(engine.incarnation_id().unwrap(), id);
}

#[test]
fn test_initialize_soul_twice_fails() {
    let mut engine = RealityEngine::new();
    engine.writer().initialize_soul(make_soul()).unwrap();
    let result = engine.writer().initialize_soul(make_soul());
    assert!(result.is_err());
}

#[test]
fn test_update_vitals() {
    let mut engine = RealityEngine::new();
    engine.writer().initialize_soul(make_soul()).unwrap();
    let vitals = SoulVitals { health: 0.9, uptime_secs: 100, restart_count: 1, last_health_check: 2000, issues: vec![] };
    engine.writer().update_vitals(vitals).unwrap();
    let reader = engine.reader();
    let got = reader.get_vitals().unwrap();
    assert_eq!(got.uptime_secs, 100);
}

#[test]
fn test_update_vitals_without_soul_fails() {
    let mut engine = RealityEngine::new();
    let vitals = SoulVitals { health: 1.0, uptime_secs: 0, restart_count: 0, last_health_check: 0, issues: vec![] };
    assert!(engine.writer().update_vitals(vitals).is_err());
}

#[test]
fn test_set_role() {
    let mut engine = RealityEngine::new();
    engine.writer().initialize_soul(make_soul()).unwrap();
    let role = DeploymentRole { name: "primary".into(), responsibilities: vec!["serve".into()], authority_level: AuthorityLevel::Admin };
    engine.writer().set_role(role).unwrap();
    let reader = engine.reader();
    assert_eq!(reader.get_soul().unwrap().role.name, "primary");
}

#[test]
fn test_update_cardinality() {
    let mut engine = RealityEngine::new();
    engine.writer().initialize_soul(make_soul()).unwrap();
    engine.writer().update_cardinality(Cardinality::ReplicaOf { total: 3, index: 0 }).unwrap();
    let reader = engine.reader();
    let nature = reader.get_nature().unwrap();
    assert!(matches!(nature.cardinality, Cardinality::ReplicaOf { total: 3, index: 0 }));
}

#[test]
fn test_record_wisdom() {
    let mut engine = RealityEngine::new();
    engine.writer().initialize_soul(make_soul()).unwrap();
    let wisdom = IncarnationWisdom { lesson: "always check health".into(), learned_from: IncarnationId::new(), confidence: 0.9, applicable: true };
    engine.writer().record_wisdom(wisdom).unwrap();
    let reader = engine.reader();
    let w = reader.get_wisdom().unwrap();
    assert_eq!(w.len(), 1);
    assert_eq!(w[0].lesson, "always check health");
}

// === Environment Write/Query Tests ===

#[test]
fn test_sense_environment() {
    let mut engine = RealityEngine::new();
    let medium = EnvironmentMedium {
        environment_type: EnvironmentType::Development { developer: "dev".into(), local: true },
        current_state: EnvironmentState {
            health: EnvironmentHealth { score: 1.0, components: HashMap::new() },
            pressure: EnvironmentPressure { level: 0.1, source: "none".into(), trend: environment::PressureTrend::Stable },
            stability: StabilityAssessment { score: 0.99, window_secs: 300, volatility: 0.01 },
            incidents: vec![], degradations: vec![],
            mood: EnvironmentMood::Calm, last_sensed: 1000,
        },
        properties: EnvironmentProperties { name: "dev".into(), version: None, config: HashMap::new(), labels: HashMap::new() },
        physics: EnvironmentPhysics { rate_limits: vec![], cost_constraints: vec![], time_constraints: vec![], quotas: vec![], permissions: vec![], forbidden_actions: vec![], compliance: vec![] },
        inhabitants: vec!["agent-1".into()], boundaries: vec![], weather_history: vec![],
    };
    engine.writer().sense_environment(medium).unwrap();
    let reader = engine.reader();
    let env = reader.get_environment().unwrap();
    assert!(matches!(env.environment_type, EnvironmentType::Development { .. }));
}

#[test]
fn test_update_mood() {
    let mut engine = RealityEngine::new();
    let medium = EnvironmentMedium {
        environment_type: EnvironmentType::Development { developer: "dev".into(), local: true },
        current_state: EnvironmentState {
            health: EnvironmentHealth { score: 1.0, components: HashMap::new() },
            pressure: EnvironmentPressure { level: 0.0, source: "".into(), trend: environment::PressureTrend::Stable },
            stability: StabilityAssessment { score: 1.0, window_secs: 300, volatility: 0.0 },
            incidents: vec![], degradations: vec![], mood: EnvironmentMood::Calm, last_sensed: 1000,
        },
        properties: EnvironmentProperties { name: "dev".into(), version: None, config: HashMap::new(), labels: HashMap::new() },
        physics: EnvironmentPhysics { rate_limits: vec![], cost_constraints: vec![], time_constraints: vec![], quotas: vec![], permissions: vec![], forbidden_actions: vec![], compliance: vec![] },
        inhabitants: vec![], boundaries: vec![], weather_history: vec![],
    };
    engine.writer().sense_environment(medium).unwrap();
    engine.writer().update_mood(EnvironmentMood::Stressed).unwrap();
    assert_eq!(engine.reader().get_mood().unwrap(), EnvironmentMood::Stressed);
}

#[test]
fn test_record_and_clear_incident() {
    let mut engine = RealityEngine::new();
    let medium = EnvironmentMedium {
        environment_type: EnvironmentType::Production { tier: "primary".into(), region: "us".into(), criticality: 0.9 },
        current_state: EnvironmentState {
            health: EnvironmentHealth { score: 0.8, components: HashMap::new() },
            pressure: EnvironmentPressure { level: 0.5, source: "traffic".into(), trend: environment::PressureTrend::Rising },
            stability: StabilityAssessment { score: 0.8, window_secs: 300, volatility: 0.1 },
            incidents: vec![], degradations: vec![], mood: EnvironmentMood::Busy, last_sensed: 1000,
        },
        properties: EnvironmentProperties { name: "prod".into(), version: None, config: HashMap::new(), labels: HashMap::new() },
        physics: EnvironmentPhysics { rate_limits: vec![], cost_constraints: vec![], time_constraints: vec![], quotas: vec![], permissions: vec![], forbidden_actions: vec![], compliance: vec![] },
        inhabitants: vec![], boundaries: vec![], weather_history: vec![],
    };
    engine.writer().sense_environment(medium).unwrap();
    engine.writer().record_incident(ActiveIncident { id: "inc-1".into(), severity: "high".into(), summary: "CPU spike".into(), started_at: 2000, acknowledged: false }).unwrap();
    let reader = engine.reader();
    assert_eq!(reader.get_incidents().unwrap().len(), 1);
    drop(reader);
    engine.writer().clear_incident("inc-1").unwrap();
    let reader = engine.reader();
    assert_eq!(reader.get_incidents().unwrap().len(), 0);
}

#[test]
fn test_add_remove_inhabitant() {
    let mut engine = RealityEngine::new();
    let medium = EnvironmentMedium {
        environment_type: EnvironmentType::Development { developer: "dev".into(), local: true },
        current_state: EnvironmentState {
            health: EnvironmentHealth { score: 1.0, components: HashMap::new() },
            pressure: EnvironmentPressure { level: 0.0, source: "".into(), trend: environment::PressureTrend::Stable },
            stability: StabilityAssessment { score: 1.0, window_secs: 300, volatility: 0.0 },
            incidents: vec![], degradations: vec![], mood: EnvironmentMood::Calm, last_sensed: 1000,
        },
        properties: EnvironmentProperties { name: "dev".into(), version: None, config: HashMap::new(), labels: HashMap::new() },
        physics: EnvironmentPhysics { rate_limits: vec![], cost_constraints: vec![], time_constraints: vec![], quotas: vec![], permissions: vec![], forbidden_actions: vec![], compliance: vec![] },
        inhabitants: vec![], boundaries: vec![], weather_history: vec![],
    };
    engine.writer().sense_environment(medium).unwrap();
    engine.writer().add_inhabitant("agent-2".into()).unwrap();
    let reader = engine.reader();
    assert_eq!(reader.get_environment().unwrap().inhabitants.len(), 1);
    drop(reader);
    engine.writer().remove_inhabitant("agent-2").unwrap();
    let reader = engine.reader();
    assert_eq!(reader.get_environment().unwrap().inhabitants.len(), 0);
}

// === Resource Write/Query Tests ===

#[test]
fn test_sense_resources() {
    let mut engine = RealityEngine::new();
    let body = make_resource_body();
    engine.writer().sense_resources(body).unwrap();
    let reader = engine.reader();
    let mind = reader.get_mind().unwrap();
    assert_eq!(mind.feeling, MindFeeling::Clear);
}

#[test]
fn test_update_mind() {
    let mut engine = RealityEngine::new();
    engine.writer().sense_resources(make_resource_body()).unwrap();
    let mind = MindCapacity {
        total_bytes: 16_000_000_000, used_bytes: 14_000_000_000, available_bytes: 2_000_000_000,
        feeling: MindFeeling::Strained, pressure: MemoryPressure::High, largest_free_bytes: 500_000_000,
        fragmentation: 0.3, swap: None,
    };
    engine.writer().update_mind(mind).unwrap();
    let reader = engine.reader();
    assert_eq!(reader.get_mind().unwrap().feeling, MindFeeling::Strained);
}

#[test]
fn test_discover_capability() {
    let mut engine = RealityEngine::new();
    let cap = Capability { name: "gpu-inference".into(), category: CapabilityCategory::MachineLearning, available: true, constraints: vec![], confidence: 0.95 };
    engine.writer().discover_capability(cap).unwrap();
    assert!(engine.reader().can_do("gpu-inference"));
    assert!(!engine.reader().can_do("teleportation"));
}

#[test]
fn test_lose_capability() {
    let mut engine = RealityEngine::new();
    engine.writer().discover_capability(Capability { name: "feature-x".into(), category: CapabilityCategory::Tool, available: true, constraints: vec![], confidence: 1.0 }).unwrap();
    assert!(engine.reader().can_do("feature-x"));
    engine.writer().lose_capability("feature-x").unwrap();
    assert!(!engine.reader().can_do("feature-x"));
}

#[test]
fn test_add_sensation() {
    let mut engine = RealityEngine::new();
    engine.writer().sense_resources(make_resource_body()).unwrap();
    let sensation = ResourceSensation { resource: ResourceType::Memory, sensation: SensationType::Pressure, intensity: 0.7, started: 1000, trend: SensationTrend::Worsening };
    engine.writer().add_sensation(sensation).unwrap();
    let reader = engine.reader();
    assert_eq!(reader.get_sensations().unwrap().len(), 1);
}

#[test]
fn test_clear_sensation() {
    let mut engine = RealityEngine::new();
    engine.writer().sense_resources(make_resource_body()).unwrap();
    engine.writer().add_sensation(ResourceSensation { resource: ResourceType::Cpu, sensation: SensationType::Pain, intensity: 0.9, started: 1000, trend: SensationTrend::Stable }).unwrap();
    engine.writer().clear_sensation(ResourceType::Cpu).unwrap();
    let reader = engine.reader();
    let sensations = reader.get_sensations().unwrap();
    assert!(sensations.iter().all(|s| s.resource != ResourceType::Cpu));
}

// === Reality Write/Query Tests ===

#[test]
fn test_add_and_get_anchor() {
    let mut engine = RealityEngine::new();
    let anchor = RealityAnchor {
        id: AnchorId::new(),
        anchor_type: AnchorType::Time { source: "ntp".into() },
        verification: VerificationMethod::Direct,
        last_value: AnchorValue { value: "2024-01-01T00:00:00Z".into(), verified_at: 1000, confidence: 0.99 },
        trust: 0.99,
        frequency_secs: 60,
        dependents: vec![],
    };
    let id = engine.writer().add_anchor(anchor).unwrap();
    let reader = engine.reader();
    let got = reader.get_anchor(&id).unwrap();
    assert_eq!(got.trust, 0.99);
}

#[test]
fn test_remove_anchor() {
    let mut engine = RealityEngine::new();
    let id = engine.writer().add_anchor(RealityAnchor {
        id: AnchorId::new(), anchor_type: AnchorType::Time { source: "ntp".into() },
        verification: VerificationMethod::Direct, last_value: AnchorValue { value: "now".into(), verified_at: 1000, confidence: 1.0 },
        trust: 1.0, frequency_secs: 60, dependents: vec![],
    }).unwrap();
    assert_eq!(engine.reader().get_anchors().len(), 1);
    engine.writer().remove_anchor(&id).unwrap();
    assert_eq!(engine.reader().get_anchors().len(), 0);
}

#[test]
fn test_detect_hallucination() {
    let mut engine = RealityEngine::new();
    engine.writer().detect_hallucination(DetectedHallucination {
        id: "h-1".into(), hallucination_type: HallucinationType::FactualError,
        claim: "2+2=5".into(), evidence: "math".into(),
        detection_method: DetectionMethod::AnchorVerification,
        detected_at: 1000, resolved: false,
    }).unwrap();
    let reader = engine.reader();
    let state = reader.get_hallucination_state().unwrap();
    assert_eq!(state.detected.len(), 1);
}

// === Topology Write/Query Tests ===

#[test]
fn test_set_position_and_get() {
    let mut engine = RealityEngine::new();
    let pos = TopologyPosition { layer: StackLayer::Application, edge_distance: 2, core_distance: 1, criticality: 0.8, redundancy: 3 };
    engine.writer().set_position(pos).unwrap();
    let reader = engine.reader();
    let got = reader.get_position().unwrap();
    assert!(matches!(got.layer, StackLayer::Application));
    assert_eq!(got.redundancy, 3);
}

#[test]
fn test_add_downstream() {
    let mut engine = RealityEngine::new();
    let pos = TopologyPosition { layer: StackLayer::Service, edge_distance: 1, core_distance: 1, criticality: 0.5, redundancy: 1 };
    engine.writer().set_position(pos).unwrap();
    let dep_id = engine.writer().add_downstream(DownstreamEntity {
        id: DependencyId::new(), service: ServiceId::new("postgres"),
        entity_type: DownstreamType::Database, health: HealthStatus::Healthy,
        latency: agentic_reality::types::resource::LatencyStats { p50_ms: 1.0, p95_ms: 5.0, p99_ms: 10.0, max_ms: 50.0 },
        criticality: DependencyCriticality::Essential, fallback: None, connection: ConnectionState::Connected,
    }).unwrap();
    let reader = engine.reader();
    let deps = reader.get_downstream().unwrap();
    assert_eq!(deps.len(), 1);
    let dep = reader.get_dependency(&dep_id).unwrap();
    assert_eq!(dep.service.0, "postgres");
}

// === Temporal Write/Query Tests ===

#[test]
fn test_add_causal_event() {
    let mut engine = RealityEngine::new();
    let evt = CausalEvent {
        id: EventId::new(), event_type: CausalEventType::Action,
        description: "deployed v2".into(), timestamp: 1000,
        causes: vec![], effects: vec![], metadata: HashMap::new(),
    };
    let id = engine.writer().add_causal_event(evt).unwrap();
    let reader = engine.reader();
    let graph = reader.get_causality_graph().unwrap();
    assert_eq!(graph.events.len(), 1);
    assert_eq!(graph.events[0].id, id);
}

#[test]
fn test_link_causality() {
    let mut engine = RealityEngine::new();
    let cause_id = engine.writer().add_causal_event(CausalEvent { id: EventId::new(), event_type: CausalEventType::Action, description: "cause".into(), timestamp: 1000, causes: vec![], effects: vec![], metadata: HashMap::new() }).unwrap();
    let effect_id = engine.writer().add_causal_event(CausalEvent { id: EventId::new(), event_type: CausalEventType::Observation, description: "effect".into(), timestamp: 2000, causes: vec![], effects: vec![], metadata: HashMap::new() }).unwrap();
    engine.writer().link_causality(&cause_id, &effect_id).unwrap();
    let reader = engine.reader();
    let chain = reader.get_causal_chain(&effect_id).unwrap();
    assert!(chain.len() >= 1);
}

// === Stakes Write/Query Tests ===

#[test]
fn test_set_stakes_level() {
    let mut engine = RealityEngine::new();
    engine.writer().set_stakes_level(StakesLevel::High { caution_required: true, approval_needed: true }).unwrap();
    let reader = engine.reader();
    let level = reader.get_stakes_level().unwrap();
    assert_eq!(level.to_string(), "high");
}

#[test]
fn test_add_consequence() {
    let mut engine = RealityEngine::new();
    engine.writer().set_stakes_level(StakesLevel::Medium { review_recommended: true }).unwrap();
    engine.writer().add_consequence(Consequence { effect: "data loss".into(), probability: 0.1, severity: Severity::Major, reversibility: Reversibility::Irreversible, affected: vec!["users".into()], latency_secs: Some(0) }).unwrap();
    let reader = engine.reader();
    let consequences = reader.get_consequences().unwrap();
    assert_eq!(consequences.len(), 1);
}

#[test]
fn test_is_irreversible() {
    let mut engine = RealityEngine::new();
    engine.writer().set_stakes_level(StakesLevel::Critical { multiple_approvals: true, audit_required: true, no_risk_tolerance: true }).unwrap();
    engine.writer().add_irreversible_action(IrreversibleAction { action: "delete-database".into(), consequences: vec!["data loss".into()], safeguards: vec!["backup".into()], requires_approval: true }).unwrap();
    assert!(engine.reader().is_irreversible("delete-database"));
    assert!(!engine.reader().is_irreversible("read-data"));
}

#[test]
fn test_add_guardrail() {
    let mut engine = RealityEngine::new();
    engine.writer().set_stakes_level(StakesLevel::Minimal { can_experiment: true }).unwrap();
    engine.writer().add_guardrail(Guardrail { name: "max-writes".into(), description: "limit writes per second".into(), active: true, triggered_count: 0 }).unwrap();
    let reader = engine.reader();
    let guardrails = reader.get_guardrails().unwrap();
    assert_eq!(guardrails.len(), 1);
}

// === Coherence Write/Query Tests ===

#[test]
fn test_run_coherence_check() {
    let mut engine = RealityEngine::new();
    engine.writer().run_coherence_check(CoherenceCheck { check_type: CoherenceCheckType::TimeConsistency, passed: true, details: "ok".into(), timestamp: 1000, duration_ms: 5 }).unwrap();
    let reader = engine.reader();
    let checks = reader.get_coherence_checks().unwrap();
    assert_eq!(checks.len(), 1);
    assert!(engine.reader().is_coherent());
}

#[test]
fn test_record_violation() {
    let mut engine = RealityEngine::new();
    engine.writer().record_violation(CoherenceViolation { violation_type: CoherenceCheckType::StateConsistency, detected: 1000, severity: 0.5, evidence: vec!["stale state".into()], possible_causes: vec!["clock drift".into()], resolution: None, description: "state mismatch".into() }).unwrap();
    assert!(!engine.reader().is_coherent());
}

#[test]
fn test_transition_lifecycle() {
    let mut engine = RealityEngine::new();
    let transition = PendingTransition {
        id: TransitionId::new(), from: ContextId::new(), to_context: "staging".into(),
        transition_type: TransitionType::EnvironmentChange, phase: TransitionPhase::Planning,
        carry_over: vec!["state".into()], started: 1000,
    };
    let tid = engine.writer().begin_transition(transition).unwrap();
    engine.writer().advance_transition(&tid, TransitionPhase::Executing).unwrap();
    let reader = engine.reader();
    let pending = reader.get_pending_transitions().unwrap();
    assert_eq!(pending.len(), 1);
    assert!(matches!(pending[0].phase, TransitionPhase::Executing));
    drop(reader);
    engine.writer().complete_transition(&tid).unwrap();
    let reader = engine.reader();
    let pending = reader.get_pending_transitions().unwrap();
    assert_eq!(pending.len(), 0);
    let history = reader.get_transition_history().unwrap();
    assert_eq!(history.len(), 1);
    assert!(history[0].success);
}

// === Context Summary Test ===

#[test]
fn test_context_summary() {
    let mut engine = RealityEngine::new();
    let summary = engine.reader().get_context_summary();
    assert!(summary.incarnation_id.is_none());
    engine.writer().initialize_soul(make_soul()).unwrap();
    let summary = engine.reader().get_context_summary();
    assert!(summary.incarnation_id.is_some());
}

// === Dirty Flag Tests ===

#[test]
fn test_dirty_flag() {
    let mut engine = RealityEngine::new();
    assert!(!engine.is_dirty());
    engine.writer().initialize_soul(make_soul()).unwrap();
    assert!(engine.is_dirty());
    engine.mark_clean();
    assert!(!engine.is_dirty());
}

// Helper

fn make_resource_body() -> ResourceBody {
    ResourceBody {
        mind: MindCapacity { total_bytes: 16_000_000_000, used_bytes: 4_000_000_000, available_bytes: 12_000_000_000, feeling: MindFeeling::Clear, pressure: MemoryPressure::Low, largest_free_bytes: 10_000_000_000, fragmentation: 0.05, swap: None },
        energy: ProcessingEnergy { cores: 8, utilization: 0.3, feeling: EnergyFeeling::Steady, load_average: [1.0, 0.8, 0.7], burst_available: true, throttled: false, credits: None, temperature: Some(55.0) },
        reach: NetworkReach { bandwidth_mbps: 1000, utilization: 0.1, latencies: HashMap::new(), feeling: ReachFeeling::Connected, connections: ConnectionStats { active: 10, idle: 5, max: 100, errored: 0 }, stability: 0.99, packet_loss: 0.0, egress_remaining: None },
        storage: StorageCapacity { total_bytes: 500_000_000_000, used_bytes: 100_000_000_000, available_bytes: 400_000_000_000, iops: Some(10000), throughput_mbps: Some(500), latency_ms: Some(0.5) },
        visual: None,
        vitals: BodyVitals { overall_health: 1.0, heartbeat_ms: 100, last_check: 1000, anomalies: vec![] },
        sensations: vec![],
    }
}
