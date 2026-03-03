//! Phase 08: Stress and edge case tests.

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
            spawned_at: 1000, spawned_by: SpawnerIdentity { name: "stress".into(), kind: SpawnerKind::Human, version: None },
            purpose: DeploymentPurpose { summary: "stress test".into(), category: PurposeCategory::Testing, specifics: HashMap::new() },
            expected_lifetime: None, previous_life: None, circumstances: BirthCircumstances::Virgin,
        },
        substrate: PhysicalSubstrate {
            id: "stress".into(), tier: SubstrateTier::Cloud { provider: CloudProvider::Aws, instance_type: "c5.xlarge".into(), region: "us-east-1".into() },
            location: GeographicLocation { region: Some("us-east-1".into()), zone: Some("a".into()), country: Some("US".into()), coordinates: None },
            network_position: NetworkPosition { ip: Some("10.0.0.1".into()), hostname: Some("stress-test".into()), port: Some(8080), vpc: None, subnet: None },
            isolation: IsolationLevel::Container, tenancy: TenancyModel::MultiTenant { tenant_id: "t-1".into() },
            capabilities: SubstrateCapabilities { cpu_cores: 4, memory_mb: 8192, disk_gb: 100, gpu_available: false, network_bandwidth_mbps: Some(1000) },
        },
        role: DeploymentRole { name: "stress-agent".into(), responsibilities: vec!["test".into()], authority_level: AuthorityLevel::Operator },
        nature: ExistentialNature { cardinality: Cardinality::ReplicaOf { total: 3, index: 0 }, expendability: 0.3, persistence: PersistenceModel::Persistent, statefulness: StatefulnessModel::EventSourced, clonability: true, primacy: InstancePrimacy::Primary },
        lineage: DeploymentLineage { generation: 3, ancestors: vec![IncarnationId::new(), IncarnationId::new()], wisdom: vec![], karma: IncarnationKarma { score: 0.7, good_deeds: 15, incidents: 3, trend: KarmaTrend::Improving } },
        vitals: SoulVitals { health: 0.95, uptime_secs: 86400, restart_count: 2, last_health_check: 1000, issues: vec![] },
    }
}

// === Stress: Many anchors ===

#[test]
fn test_many_anchors() {
    let mut engine = RealityEngine::new();
    for i in 0..100 {
        engine.writer().add_anchor(RealityAnchor {
            id: AnchorId::new(),
            anchor_type: AnchorType::State { source: format!("source-{}", i) },
            verification: VerificationMethod::Direct,
            last_value: AnchorValue { value: format!("val-{}", i), verified_at: 1000 + i, confidence: 0.9 },
            trust: 0.8, frequency_secs: 60, dependents: vec![],
        }).unwrap();
    }
    assert_eq!(engine.reader().get_anchors().len(), 100);
}

// === Stress: Many events ===

#[test]
fn test_many_causal_events() {
    let mut engine = RealityEngine::new();
    let mut last_id = None;
    for i in 0..50 {
        let id = engine.writer().add_causal_event(CausalEvent {
            id: EventId::new(), event_type: CausalEventType::Action,
            description: format!("event-{}", i), timestamp: 1000 + i,
            causes: last_id.map(|id| vec![id]).unwrap_or_default(),
            effects: vec![], metadata: HashMap::new(),
        }).unwrap();
        last_id = Some(id);
    }
    let reader = engine.reader();
    let graph = reader.get_causality_graph().unwrap();
    assert_eq!(graph.events.len(), 50);
}

// === Stress: Many downstream dependencies ===

#[test]
fn test_many_downstream() {
    let mut engine = RealityEngine::new();
    let pos = TopologyPosition { layer: StackLayer::Application, edge_distance: 1, core_distance: 2, criticality: 0.9, redundancy: 2 };
    engine.writer().set_position(pos).unwrap();
    for i in 0..20 {
        engine.writer().add_downstream(DownstreamEntity {
            id: DependencyId::new(), service: ServiceId::new(format!("svc-{}", i)),
            entity_type: if i % 2 == 0 { DownstreamType::Database } else { DownstreamType::Service },
            health: HealthStatus::Healthy,
            latency: LatencyStats { p50_ms: 1.0, p95_ms: 5.0, p99_ms: 10.0, max_ms: 50.0 },
            criticality: DependencyCriticality::Important, fallback: None, connection: ConnectionState::Connected,
        }).unwrap();
    }
    let reader = engine.reader();
    let deps = reader.get_downstream().unwrap();
    assert_eq!(deps.len(), 20);
}

// === Stress: Many coherence violations ===

#[test]
fn test_many_violations() {
    let mut engine = RealityEngine::new();
    for i in 0..10 {
        engine.writer().record_violation(CoherenceViolation {
            violation_type: CoherenceCheckType::StateConsistency,
            detected: 1000 + i, severity: 0.3, evidence: vec![format!("evidence-{}", i)],
            possible_causes: vec!["drift".into()], resolution: None, description: format!("violation-{}", i),
        }).unwrap();
    }
    assert!(!engine.reader().is_coherent());
    let reader = engine.reader();
    let violations = reader.get_violations().unwrap();
    assert_eq!(violations.len(), 10);
}

// === Edge case: Empty operations on uninitialized stores ===

#[test]
fn test_get_soul_uninitialized() {
    let engine = RealityEngine::new();
    assert!(engine.reader().get_soul().is_err());
}

#[test]
fn test_get_environment_uninitialized() {
    let engine = RealityEngine::new();
    assert!(engine.reader().get_environment().is_err());
}

#[test]
fn test_get_body_uninitialized() {
    let engine = RealityEngine::new();
    assert!(engine.reader().get_body().is_err());
}

#[test]
fn test_get_topology_uninitialized() {
    let engine = RealityEngine::new();
    let reader = engine.reader();
    assert!(reader.get_topology_map().is_err());
}

#[test]
fn test_get_temporal_uninitialized() {
    let engine = RealityEngine::new();
    let reader = engine.reader();
    assert!(reader.get_grounded_time().is_err());
}

#[test]
fn test_get_stakes_uninitialized() {
    let engine = RealityEngine::new();
    let reader = engine.reader();
    assert!(reader.get_stakes_level().is_err());
}

#[test]
fn test_get_coherence_uninitialized() {
    let engine = RealityEngine::new();
    let reader = engine.reader();
    assert!(reader.get_coherence_state().is_err());
}

#[test]
fn test_has_context_shifted_uninitialized() {
    let engine = RealityEngine::new();
    assert!(!engine.reader().has_context_shifted());
}

#[test]
fn test_is_coherent_uninitialized() {
    let engine = RealityEngine::new();
    assert!(engine.reader().is_coherent());
}

#[test]
fn test_can_do_uninitialized() {
    let engine = RealityEngine::new();
    assert!(!engine.reader().can_do("anything"));
}

#[test]
fn test_is_irreversible_uninitialized() {
    let engine = RealityEngine::new();
    assert!(!engine.reader().is_irreversible("anything"));
}

#[test]
fn test_get_bottleneck_uninitialized() {
    let engine = RealityEngine::new();
    assert!(engine.reader().get_bottleneck().is_none());
}

#[test]
fn test_get_hallucination_risk_uninitialized() {
    let engine = RealityEngine::new();
    assert!(engine.reader().get_hallucination_risk().is_none());
}

#[test]
fn test_get_grounding_status_uninitialized() {
    let engine = RealityEngine::new();
    assert!(engine.reader().get_grounding_status().is_none());
}

// === Edge case: Remove non-existent ===

#[test]
fn test_remove_nonexistent_anchor() {
    let mut engine = RealityEngine::new();
    let fake_id = AnchorId::new();
    assert!(engine.writer().remove_anchor(&fake_id).is_err());
}

// === Roundtrip: Full engine state ===

#[test]
fn test_full_state_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("full.areal");

    let mut engine = RealityEngine::new();
    engine.writer().initialize_soul(make_soul()).unwrap();
    engine.writer().sense_environment(EnvironmentMedium {
        environment_type: EnvironmentType::Production { tier: "primary".into(), region: "us-east-1".into(), criticality: 0.99 },
        current_state: EnvironmentState {
            health: EnvironmentHealth { score: 0.95, components: HashMap::new() },
            pressure: EnvironmentPressure { level: 0.3, source: "traffic".into(), trend: environment::PressureTrend::Stable },
            stability: StabilityAssessment { score: 0.9, window_secs: 3600, volatility: 0.05 },
            incidents: vec![], degradations: vec![], mood: EnvironmentMood::Busy, last_sensed: 1000,
        },
        properties: EnvironmentProperties { name: "production".into(), version: Some("1.2.3".into()), config: HashMap::new(), labels: HashMap::new() },
        physics: EnvironmentPhysics { rate_limits: vec![], cost_constraints: vec![], time_constraints: vec![], quotas: vec![], permissions: vec![], forbidden_actions: vec![], compliance: vec![] },
        inhabitants: vec!["agent-1".into(), "agent-2".into()], boundaries: vec![], weather_history: vec![],
    }).unwrap();
    engine.writer().set_stakes_level(StakesLevel::High { caution_required: true, approval_needed: true }).unwrap();
    engine.writer().add_anchor(RealityAnchor {
        id: AnchorId::new(), anchor_type: AnchorType::Time { source: "ntp".into() },
        verification: VerificationMethod::Direct, last_value: AnchorValue { value: "now".into(), verified_at: 1000, confidence: 0.99 },
        trust: 0.99, frequency_secs: 60, dependents: vec![],
    }).unwrap();

    agentic_reality::format::ArealWriter::save(&engine, &path).unwrap();
    let loaded = agentic_reality::format::ArealReader::load(&path).unwrap();

    assert!(loaded.is_initialized());
    assert_eq!(loaded.reader().get_mood().unwrap(), EnvironmentMood::Busy);
    assert_eq!(loaded.reader().get_stakes_level().unwrap().to_string(), "high");
    assert_eq!(loaded.reader().get_anchors().len(), 1);
}

// === Multiple engine instances ===

#[test]
fn test_multiple_engines() {
    let mut e1 = RealityEngine::new();
    let mut e2 = RealityEngine::new();
    e1.writer().initialize_soul(make_soul()).unwrap();
    e2.writer().set_stakes_level(StakesLevel::Minimal { can_experiment: true }).unwrap();
    assert!(e1.is_initialized());
    assert!(!e2.is_initialized());
}

// === Timelines ===

#[test]
fn test_unified_timeline_empty() {
    let engine = RealityEngine::new();
    let unified = engine.reader().get_unified_timeline();
    assert_eq!(unified.coherence_score, 1.0);
    assert!(unified.merged_events.is_empty());
}

#[test]
fn test_multiple_timelines() {
    let mut engine = RealityEngine::new();
    let t1 = engine.writer().add_timeline(Timeline {
        id: TimelineId::new(), name: "main".into(), events: vec![], conflicts: vec![], coherent: true,
    }).unwrap();
    let t2 = engine.writer().add_timeline(Timeline {
        id: TimelineId::new(), name: "branch".into(), events: vec![], conflicts: vec![], coherent: true,
    }).unwrap();
    let reader = engine.reader();
    let timelines = reader.get_timelines();
    assert_eq!(timelines.len(), 2);
    drop(reader);
    let unified = engine.reader().get_unified_timeline();
    assert_eq!(unified.timelines.len(), 2);
    assert_eq!(unified.coherence_score, 1.0);
}

// === Deadline tests ===

#[test]
fn test_add_deadline() {
    let mut engine = RealityEngine::new();
    engine.writer().ground_time(TemporalAwareness {
        wall_clock: 1000, monotonic_ns: 0, time_source: TimeSource::System,
        time_confidence: 1.0, drift: None,
        context: TemporalContext { session_start: 1000, session_duration_secs: 0, request_count: 0, deadlines: vec![], time_budget: None, causal_position: None },
    }).unwrap();
    engine.writer().add_deadline(Deadline { id: "d-1".into(), name: "deploy".into(), due_at: 2000, priority: DeadlinePriority::High, remaining_secs: 1000, met: None }).unwrap();
    let reader = engine.reader();
    let deadlines = reader.get_deadlines().unwrap();
    assert_eq!(deadlines.len(), 1);
    assert_eq!(deadlines[0].name, "deploy");
}

#[test]
fn test_remove_deadline() {
    let mut engine = RealityEngine::new();
    engine.writer().ground_time(TemporalAwareness {
        wall_clock: 1000, monotonic_ns: 0, time_source: TimeSource::System,
        time_confidence: 1.0, drift: None,
        context: TemporalContext { session_start: 1000, session_duration_secs: 0, request_count: 0, deadlines: vec![], time_budget: None, causal_position: None },
    }).unwrap();
    engine.writer().add_deadline(Deadline { id: "d-1".into(), name: "deploy".into(), due_at: 2000, priority: DeadlinePriority::High, remaining_secs: 1000, met: None }).unwrap();
    engine.writer().remove_deadline("d-1").unwrap();
    let reader = engine.reader();
    assert_eq!(reader.get_deadlines().unwrap().len(), 0);
}

// === Transition abort ===

#[test]
fn test_abort_transition() {
    let mut engine = RealityEngine::new();
    let tid = engine.writer().begin_transition(PendingTransition {
        id: TransitionId::new(), from: ContextId::new(), to_context: "new".into(),
        transition_type: TransitionType::Migration, phase: TransitionPhase::Planning,
        carry_over: vec![], started: 1000,
    }).unwrap();
    engine.writer().abort_transition(&tid).unwrap();
    let reader = engine.reader();
    let history = reader.get_transition_history().unwrap();
    assert_eq!(history.len(), 1);
    assert!(!history[0].success);
}

// === Update substrate ===

#[test]
fn test_update_substrate() {
    let mut engine = RealityEngine::new();
    engine.writer().initialize_soul(make_soul()).unwrap();
    engine.writer().update_substrate(PhysicalSubstrate {
        id: "new".into(), tier: SubstrateTier::GpuCluster { gpu_count: 8, gpu_type: "A100".into(), interconnect: "NVLink".into() },
        location: GeographicLocation { region: Some("us-west-2".into()), zone: None, country: None, coordinates: None },
        network_position: NetworkPosition { ip: None, hostname: None, port: None, vpc: None, subnet: None },
        isolation: IsolationLevel::Physical, tenancy: TenancyModel::Dedicated,
        capabilities: SubstrateCapabilities { cpu_cores: 96, memory_mb: 786432, disk_gb: 10000, gpu_available: true, network_bandwidth_mbps: Some(100000) },
    }).unwrap();
    let reader = engine.reader();
    let substrate = reader.get_substrate().unwrap();
    assert!(matches!(substrate.tier, SubstrateTier::GpuCluster { .. }));
}

// === Incarnation memory ===

#[test]
fn test_add_past_life() {
    let mut engine = RealityEngine::new();
    engine.writer().initialize_soul(make_soul()).unwrap();
    engine.writer().add_past_life(PastIncarnation {
        incarnation_id: IncarnationId::new(), birth: 500,
        death: DeathRecord { died_at: 999, cause: DeathCause::GracefulShutdown, graceful: true, state_preserved: true },
        lessons: vec!["always save state".into()], substrate_tier: "Laptop".into(),
    }).unwrap();
    let reader = engine.reader();
    let mem = reader.get_incarnation_memory().unwrap();
    assert_eq!(mem.past_lives.len(), 1);
}

// === Update cost ===

#[test]
fn test_update_cost() {
    let mut engine = RealityEngine::new();
    engine.writer().update_cost(CostConsciousness {
        burn_rate: Cost { monetary: 0.05, carbon: None, opportunity: None, reputation: None, currency: "USD".into() },
        session_cost: Cost { monetary: 0.10, carbon: None, opportunity: None, reputation: None, currency: "USD".into() },
        budget: None, breakdown: HashMap::new(),
        feeling: CostFeeling::Mindful, projections: vec![],
    }).unwrap();
    let reader = engine.reader();
    let cost = reader.get_cost().unwrap();
    assert!(matches!(cost.feeling, CostFeeling::Mindful));
}

// === Risk field ===

#[test]
fn test_risk_field() {
    let mut engine = RealityEngine::new();
    let mut risk_map = HashMap::new();
    risk_map.insert(RiskCategory::DataRisk, RiskLevel { score: 0.3, trend: RiskTrend::Stable, mitigations: vec!["encryption".into()] });
    risk_map.insert(RiskCategory::SecurityRisk, RiskLevel { score: 0.5, trend: RiskTrend::Increasing, mitigations: vec![] });
    engine.writer().update_risk_field(RiskFieldPerception {
        overall_risk: 0.4, risk_map, hotspots: vec![], safe_zones: vec!["internal".into()],
        gradients: vec![], forecast: vec![],
    }).unwrap();
    let reader = engine.reader();
    let field = reader.get_risk_field().unwrap();
    assert!((field.overall_risk - 0.4).abs() < f64::EPSILON);
    let data_risk = reader.get_risk_for(&RiskCategory::DataRisk);
    assert!(data_risk.is_some());
}

// === Blast radius ===

#[test]
fn test_blast_radius() {
    let mut engine = RealityEngine::new();
    engine.writer().update_blast_radius(BlastRadiusAwareness {
        my_blast_radius: BlastRadius { affected_services: vec!["api".into()], affected_users: EstimatedImpact { count: 1000, percentage: 0.1, confidence: 0.8 }, affected_data: vec![], geographic_scope: vec!["us".into()] },
        dependency_blast: vec![], cascade: CascadeAnalysis { max_depth: 3, affected_count: 5, critical_path: vec!["db".into()], containment_possible: true },
        isolation: IsolationAssessment { isolation_score: 0.7, blast_walls: vec![], leaks: vec![] },
        containment: vec![],
    }).unwrap();
    let reader = engine.reader();
    let cascade = reader.get_cascade_analysis().unwrap();
    assert!(cascade.containment_possible);
}
