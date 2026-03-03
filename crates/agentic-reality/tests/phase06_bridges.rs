//! Phase 06: Bridge trait tests.

use agentic_reality::bridges::*;

#[test]
fn test_noop_time_bridge() {
    let bridge = NoOpBridges;
    assert!(bridge.link_deadline("a", "d").is_ok());
    assert!(bridge.temporal_context("topic").is_empty());
    assert_eq!(bridge.is_deadline_past("d"), None);
}

#[test]
fn test_noop_contract_bridge() {
    let bridge = NoOpBridges;
    assert!(bridge.check_policy("op", "ctx").unwrap());
    assert!(bridge.record_operation("op", "ctx").is_ok());
}

#[test]
fn test_noop_identity_bridge() {
    let bridge = NoOpBridges;
    assert!(bridge.verify_identity("agent").unwrap());
    assert_eq!(bridge.resolve_identity("agent"), None);
    assert!(bridge.sign_data(b"data").unwrap().is_empty());
}

#[test]
fn test_noop_memory_bridge() {
    let bridge = NoOpBridges;
    assert!(bridge.store_context("k", "v").is_ok());
    assert_eq!(bridge.recall_context("k"), None);
    assert!((bridge.ground_claim("claim").unwrap() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_noop_cognition_bridge() {
    let bridge = NoOpBridges;
    assert!((bridge.assess_coherence("ctx").unwrap() - 1.0).abs() < f64::EPSILON);
    assert_eq!(bridge.suggest_action("ctx"), None);
}

#[test]
fn test_noop_comm_bridge() {
    let bridge = NoOpBridges;
    assert!(bridge.broadcast_state("state").is_ok());
    assert_eq!(bridge.receive_state(), None);
}

#[test]
fn test_noop_codebase_bridge() {
    let bridge = NoOpBridges;
    assert_eq!(bridge.get_context("path"), None);
    assert!(bridge.analyze_impact("change").unwrap().is_empty());
}

#[test]
fn test_noop_vision_bridge() {
    let bridge = NoOpBridges;
    assert!(bridge.capture_state("desc").unwrap().is_empty());
    assert!(bridge.query_visual("q").is_empty());
}

#[test]
fn test_noop_planning_bridge() {
    let bridge = NoOpBridges;
    assert!(bridge.register_constraint("c").is_ok());
    assert_eq!(bridge.get_plan_context(), None);
}

#[test]
fn test_noop_hydra_adapter() {
    let bridge = NoOpBridges;
    assert!(bridge.register_with_hydra().is_ok());
    assert_eq!(bridge.report_health().unwrap(), "healthy");
    assert!(bridge.accept_command("cmd").unwrap().is_empty());
}

#[test]
fn test_noop_ghost_writer() {
    let bridge = NoOpBridges;
    assert!(bridge.snapshot().unwrap().is_empty());
    assert!(bridge.restore(b"data").is_ok());
    assert!(bridge.get_delta(0).unwrap().is_empty());
    assert!(bridge.apply_delta(b"delta").is_ok());
    assert!(bridge.get_checksum().unwrap().is_empty());
    assert_eq!(bridge.get_ghost_hint(), None);
}

#[test]
fn test_bridge_trait_count() {
    // Verify we have 9 bridge traits + NoOpBridges + HydraAdapter + GhostWriter = 12 impls
    // This test verifies the struct compiles with all trait impls
    let _: Box<dyn TimeBridge> = Box::new(NoOpBridges);
    let _: Box<dyn ContractBridge> = Box::new(NoOpBridges);
    let _: Box<dyn IdentityBridge> = Box::new(NoOpBridges);
    let _: Box<dyn MemoryBridge> = Box::new(NoOpBridges);
    let _: Box<dyn CognitionBridge> = Box::new(NoOpBridges);
    let _: Box<dyn CommBridge> = Box::new(NoOpBridges);
    let _: Box<dyn CodebaseBridge> = Box::new(NoOpBridges);
    let _: Box<dyn VisionBridge> = Box::new(NoOpBridges);
    let _: Box<dyn PlanningBridge> = Box::new(NoOpBridges);
    let _: Box<dyn HydraAdapter> = Box::new(NoOpBridges);
    let _: Box<dyn RealityGhostWriter> = Box::new(NoOpBridges);
}
