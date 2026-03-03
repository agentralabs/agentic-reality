//! Phase 01: Type system tests.

use agentic_reality::types::ids::*;
use agentic_reality::types::deployment::*;
use agentic_reality::types::environment::*;
use agentic_reality::types::resource::*;
use agentic_reality::types::reality::*;
use agentic_reality::types::topology::*;
use agentic_reality::types::stakes::*;
use agentic_reality::types::coherence::*;
use agentic_reality::types::error::*;

// === ID Tests ===

#[test]
fn test_incarnation_id_new_unique() {
    let id1 = IncarnationId::new();
    let id2 = IncarnationId::new();
    assert_ne!(id1, id2);
}

#[test]
fn test_incarnation_id_from_context_deterministic() {
    let id1 = IncarnationId::from_context("prod-us-east-1");
    let id2 = IncarnationId::from_context("prod-us-east-1");
    assert_eq!(id1, id2);
}

#[test]
fn test_incarnation_id_from_context_different() {
    let id1 = IncarnationId::from_context("prod-us-east-1");
    let id2 = IncarnationId::from_context("staging-eu-west-1");
    assert_ne!(id1, id2);
}

#[test]
fn test_incarnation_id_display() {
    let id = IncarnationId::new();
    let display = format!("{}", id);
    assert!(!display.is_empty());
    assert!(display.contains('-')); // UUID format
}

#[test]
fn test_context_id_new() {
    let id = ContextId::new();
    assert_ne!(id, ContextId::new());
}

#[test]
fn test_anchor_id_new() {
    let id = AnchorId::new();
    assert_ne!(id, AnchorId::new());
}

#[test]
fn test_timeline_id_new() {
    let id = TimelineId::new();
    assert_ne!(id, TimelineId::new());
}

#[test]
fn test_transition_id_new() {
    let id = TransitionId::new();
    assert_ne!(id, TransitionId::new());
}

#[test]
fn test_event_id_new() {
    let id = EventId::new();
    assert_ne!(id, EventId::new());
}

#[test]
fn test_service_id() {
    let id = ServiceId::new("my-service");
    assert_eq!(id.to_string(), "my-service");
}

#[test]
fn test_dependency_id_new() {
    let id = DependencyId::new();
    assert_ne!(id, DependencyId::new());
}

#[test]
fn test_neighbor_id() {
    let id = NeighborId::new("neighbor-1");
    assert_eq!(id.to_string(), "neighbor-1");
}

#[test]
fn test_observer_id() {
    let id = ObserverId::new("observer-1");
    assert_eq!(id.to_string(), "observer-1");
}

#[test]
fn test_snapshot_id_new() {
    let id = SnapshotId::new();
    assert_ne!(id, SnapshotId::new());
}

// === Deployment Type Tests ===

#[test]
fn test_birth_circumstances_virgin() {
    let bc = BirthCircumstances::Virgin;
    assert!(matches!(bc, BirthCircumstances::Virgin));
}

#[test]
fn test_substrate_tier_cloud() {
    let tier = SubstrateTier::Cloud {
        provider: CloudProvider::Aws,
        instance_type: "t3.large".into(),
        region: "us-east-1".into(),
    };
    assert!(matches!(tier, SubstrateTier::Cloud { .. }));
}

#[test]
fn test_cardinality_singleton() {
    let c = Cardinality::Singleton;
    assert!(matches!(c, Cardinality::Singleton));
}

#[test]
fn test_cardinality_replica() {
    let c = Cardinality::ReplicaOf { total: 3, index: 1 };
    if let Cardinality::ReplicaOf { total, index } = c {
        assert_eq!(total, 3);
        assert_eq!(index, 1);
    }
}

#[test]
fn test_death_cause_variants() {
    assert!(matches!(DeathCause::GracefulShutdown, DeathCause::GracefulShutdown));
    assert!(matches!(DeathCause::Oom, DeathCause::Oom));
    assert!(matches!(DeathCause::Partitioned, DeathCause::Partitioned));
}

#[test]
fn test_karma_trend() {
    let k = IncarnationKarma { score: 0.8, good_deeds: 10, incidents: 2, trend: KarmaTrend::Improving };
    assert_eq!(k.score, 0.8);
    assert!(matches!(k.trend, KarmaTrend::Improving));
}

// === Environment Type Tests ===

#[test]
fn test_environment_mood_display() {
    assert_eq!(EnvironmentMood::Calm.to_string(), "calm");
    assert_eq!(EnvironmentMood::Crisis.to_string(), "crisis");
    assert_eq!(EnvironmentMood::Recovering.to_string(), "recovering");
    assert_eq!(EnvironmentMood::Dying.to_string(), "dying");
}

#[test]
fn test_environment_type_production() {
    let env_type = EnvironmentType::Production {
        tier: "primary".into(),
        region: "us-east-1".into(),
        criticality: 0.95,
    };
    assert!(matches!(env_type, EnvironmentType::Production { .. }));
}

#[test]
fn test_context_stability_variants() {
    assert_ne!(ContextStability::Stable, ContextStability::Volatile);
    assert_eq!(ContextStability::Stable, ContextStability::Stable);
}

// === Resource Type Tests ===

#[test]
fn test_mind_feeling_display() {
    assert_eq!(MindFeeling::Clear.to_string(), "clear");
    assert_eq!(MindFeeling::Drowning.to_string(), "drowning");
}

#[test]
fn test_energy_feeling_display() {
    assert_eq!(EnergyFeeling::Vigorous.to_string(), "vigorous");
    assert_eq!(EnergyFeeling::Depleted.to_string(), "depleted");
}

#[test]
fn test_resource_type_display() {
    assert_eq!(ResourceType::Memory.to_string(), "memory");
    assert_eq!(ResourceType::Cpu.to_string(), "cpu");
    assert_eq!(ResourceType::Network.to_string(), "network");
}

#[test]
fn test_cost_feeling_variants() {
    assert!(matches!(CostFeeling::Comfortable, CostFeeling::Comfortable));
    assert!(matches!(CostFeeling::Panicked, CostFeeling::Panicked));
}

#[test]
fn test_capability_category_variants() {
    assert!(matches!(CapabilityCategory::Compute, CapabilityCategory::Compute));
    assert!(matches!(CapabilityCategory::MachineLearning, CapabilityCategory::MachineLearning));
}

// === Reality Type Tests ===

#[test]
fn test_reality_layer_variants() {
    let layer = RealityLayer::Physical { substrate: "bare-metal".into(), certainty: 0.99 };
    assert!(matches!(layer, RealityLayer::Physical { .. }));
}

#[test]
fn test_simulation_fidelity() {
    assert_ne!(SimulationFidelity::Perfect, SimulationFidelity::Stub);
}

#[test]
fn test_freshness_level_live() {
    let level = FreshnessLevel::Live { latency_ms: 5 };
    assert!(matches!(level, FreshnessLevel::Live { .. }));
}

#[test]
fn test_hallucination_type_variants() {
    assert!(matches!(HallucinationType::FactualError, HallucinationType::FactualError));
    assert!(matches!(HallucinationType::Confabulation, HallucinationType::Confabulation));
}

#[test]
fn test_anchor_type_time() {
    let anchor = AnchorType::Time { source: "ntp".into() };
    assert!(matches!(anchor, AnchorType::Time { .. }));
}

// === Topology Type Tests ===

#[test]
fn test_stack_layer_display() {
    assert_eq!(StackLayer::Edge.to_string(), "edge");
    assert_eq!(StackLayer::Data.to_string(), "data");
    assert_eq!(StackLayer::Infrastructure.to_string(), "infrastructure");
}

#[test]
fn test_health_status_display() {
    assert_eq!(HealthStatus::Healthy.to_string(), "healthy");
    assert_eq!(HealthStatus::Degraded.to_string(), "degraded");
}

#[test]
fn test_downstream_type_variants() {
    assert!(matches!(DownstreamType::Database, DownstreamType::Database));
    assert!(matches!(DownstreamType::MlModel, DownstreamType::MlModel));
}

#[test]
fn test_observer_type_variants() {
    assert!(matches!(ObserverType::Metrics, ObserverType::Metrics));
    assert!(matches!(ObserverType::Human, ObserverType::Human));
}

// === Stakes Type Tests ===

#[test]
fn test_stakes_level_display() {
    let level = StakesLevel::Minimal { can_experiment: true };
    assert_eq!(level.to_string(), "minimal");
    let level = StakesLevel::Critical { multiple_approvals: true, audit_required: true, no_risk_tolerance: true };
    assert_eq!(level.to_string(), "critical");
}

#[test]
fn test_severity_variants() {
    assert!(matches!(Severity::Negligible, Severity::Negligible));
    assert!(matches!(Severity::Catastrophic, Severity::Catastrophic));
}

#[test]
fn test_reversibility_variants() {
    assert!(matches!(Reversibility::Easy, Reversibility::Easy));
    assert!(matches!(Reversibility::Irreversible, Reversibility::Irreversible));
}

#[test]
fn test_risk_category_display() {
    assert_eq!(RiskCategory::DataRisk.to_string(), "data");
    assert_eq!(RiskCategory::SecurityRisk.to_string(), "security");
    assert_eq!(RiskCategory::UserHarmRisk.to_string(), "user_harm");
}

// === Coherence Type Tests ===

#[test]
fn test_coherence_level_display() {
    let level = CoherenceLevel::Full { confidence: 0.99 };
    assert_eq!(level.to_string(), "full");
}

#[test]
fn test_coherence_check_type_display() {
    assert_eq!(CoherenceCheckType::TimeConsistency.to_string(), "time");
    assert_eq!(CoherenceCheckType::CausalConsistency.to_string(), "causal");
}

#[test]
fn test_transition_type_display() {
    assert_eq!(TransitionType::EnvironmentChange.to_string(), "environment_change");
    assert_eq!(TransitionType::Failover.to_string(), "failover");
}

#[test]
fn test_transition_phase_display() {
    assert_eq!(TransitionPhase::Planning.to_string(), "planning");
    assert_eq!(TransitionPhase::RollingBack.to_string(), "rolling_back");
}

// === Error Type Tests ===

#[test]
fn test_reality_error_display() {
    let err = RealityError::InvalidMagic;
    assert_eq!(err.to_string(), "Invalid magic bytes");
}

#[test]
fn test_reality_error_not_found() {
    let err = RealityError::NotFound("test".into());
    assert!(err.to_string().contains("test"));
}

#[test]
fn test_reality_error_validation() {
    let err = RealityError::Validation("bad input".into());
    assert!(err.to_string().contains("bad input"));
}

// === Serialization Tests ===

#[test]
fn test_incarnation_id_serialization() {
    let id = IncarnationId::new();
    let json = serde_json::to_string(&id).unwrap();
    let deserialized: IncarnationId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, deserialized);
}

#[test]
fn test_environment_mood_serialization() {
    let mood = EnvironmentMood::Crisis;
    let json = serde_json::to_string(&mood).unwrap();
    let deserialized: EnvironmentMood = serde_json::from_str(&json).unwrap();
    assert_eq!(mood, deserialized);
}

#[test]
fn test_stakes_level_serialization() {
    let level = StakesLevel::High { caution_required: true, approval_needed: true };
    let json = serde_json::to_string(&level).unwrap();
    let deserialized: StakesLevel = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.to_string(), "high");
}
