//! 26 inventions — the core innovations of AgenticReality.

// Tier 1: Deployment Consciousness
pub mod context_fingerprint;
pub mod deployment_soul;
pub mod environment_sensing;
pub mod incarnation_memory;
pub mod topology_map;

// Tier 2: Resource Proprioception
pub mod capability_discovery;
pub mod capacity_intuition;
pub mod cost_consciousness;
pub mod pressure_gradient;
pub mod resource_body;

// Tier 3: Reality Physics
pub mod freshness;
pub mod hallucination;
pub mod reality_anchors;
pub mod reality_layers;

// Tier 4: Topology Awareness
pub mod dependencies;
pub mod neighbors;
pub mod observers;
pub mod service_mesh;

// Tier 5: Temporal Grounding
pub mod causality;
pub mod temporal_awareness;
pub mod timeline_coherence;

// Tier 6: Stakes Perception
pub mod blast_radius;
pub mod consequences;
pub mod risk_field;

// Tier 7: Coherence Maintenance
pub mod coherence_engine;
pub mod transitions;

/// Total number of inventions.
pub const INVENTION_COUNT: usize = 26;

/// Get all invention names.
pub fn all_invention_names() -> Vec<&'static str> {
    vec![
        "Deployment Soul",
        "Environment Sensing",
        "Incarnation Memory",
        "Context Fingerprint",
        "Deployment Topology Map",
        "Resource Body Schema",
        "Capability Discovery",
        "Resource Pressure Gradient",
        "Cost Consciousness",
        "Capacity Planning Intuition",
        "Reality Layers",
        "Freshness Perception",
        "Reality Anchors",
        "Hallucination Detection",
        "Service Mesh Perception",
        "Neighbor Awareness",
        "Dependency Awareness",
        "Observer Awareness",
        "Temporal Awareness",
        "Causality Tracking",
        "Timeline Coherence",
        "Consequence Awareness",
        "Risk Field Perception",
        "Blast Radius Awareness",
        "Reality Coherence Engine",
        "Context Transition Manager",
    ]
}
