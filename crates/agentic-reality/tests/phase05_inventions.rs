//! Phase 05: Invention module tests.

use agentic_reality::inventions;

#[test]
fn test_invention_count() {
    assert_eq!(inventions::INVENTION_COUNT, 26);
}

#[test]
fn test_all_invention_names() {
    let names = inventions::all_invention_names();
    assert_eq!(names.len(), 26);
}

#[test]
fn test_invention_names_contain_deployment_soul() {
    let names = inventions::all_invention_names();
    assert!(names.contains(&"Deployment Soul"));
}

#[test]
fn test_invention_names_contain_hallucination() {
    let names = inventions::all_invention_names();
    assert!(names.contains(&"Hallucination Detection"));
}

#[test]
fn test_invention_names_contain_context_transition() {
    let names = inventions::all_invention_names();
    assert!(names.contains(&"Context Transition Manager"));
}

#[test]
fn test_invention_names_unique() {
    let names = inventions::all_invention_names();
    let unique: std::collections::HashSet<_> = names.iter().collect();
    assert_eq!(unique.len(), names.len());
}

#[test]
fn test_tier1_inventions() {
    let names = inventions::all_invention_names();
    assert!(names.contains(&"Deployment Soul"));
    assert!(names.contains(&"Environment Sensing"));
    assert!(names.contains(&"Incarnation Memory"));
    assert!(names.contains(&"Context Fingerprint"));
    assert!(names.contains(&"Deployment Topology Map"));
}

#[test]
fn test_tier2_inventions() {
    let names = inventions::all_invention_names();
    assert!(names.contains(&"Resource Body Schema"));
    assert!(names.contains(&"Capability Discovery"));
    assert!(names.contains(&"Resource Pressure Gradient"));
    assert!(names.contains(&"Cost Consciousness"));
    assert!(names.contains(&"Capacity Planning Intuition"));
}

#[test]
fn test_tier3_inventions() {
    let names = inventions::all_invention_names();
    assert!(names.contains(&"Reality Layers"));
    assert!(names.contains(&"Freshness Perception"));
    assert!(names.contains(&"Reality Anchors"));
    assert!(names.contains(&"Hallucination Detection"));
}

#[test]
fn test_tier4_inventions() {
    let names = inventions::all_invention_names();
    assert!(names.contains(&"Service Mesh Perception"));
    assert!(names.contains(&"Neighbor Awareness"));
    assert!(names.contains(&"Dependency Awareness"));
    assert!(names.contains(&"Observer Awareness"));
}

#[test]
fn test_tier5_inventions() {
    let names = inventions::all_invention_names();
    assert!(names.contains(&"Temporal Awareness"));
    assert!(names.contains(&"Causality Tracking"));
    assert!(names.contains(&"Timeline Coherence"));
}

#[test]
fn test_tier6_inventions() {
    let names = inventions::all_invention_names();
    assert!(names.contains(&"Consequence Awareness"));
    assert!(names.contains(&"Risk Field Perception"));
    assert!(names.contains(&"Blast Radius Awareness"));
}

#[test]
fn test_tier7_inventions() {
    let names = inventions::all_invention_names();
    assert!(names.contains(&"Reality Coherence Engine"));
    assert!(names.contains(&"Context Transition Manager"));
}
