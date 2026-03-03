//! 26 inventions — the core innovations of AgenticReality.

// Tier 1: Deployment Consciousness
pub mod deployment_soul;
pub mod environment_sensing;
pub mod incarnation_memory;
pub mod context_fingerprint;
pub mod topology_map;

// Tier 2: Resource Proprioception
pub mod resource_body;
pub mod capability_discovery;
pub mod pressure_gradient;
pub mod cost_consciousness;
pub mod capacity_intuition;

// Tier 3: Reality Physics
pub mod reality_layers;
pub mod freshness;
pub mod reality_anchors;
pub mod hallucination;

// Tier 4: Topology Awareness
pub mod service_mesh;
pub mod neighbors;
pub mod dependencies;
pub mod observers;

// Tier 5: Temporal Grounding
pub mod temporal_awareness;
pub mod causality;
pub mod timeline_coherence;

// Tier 6: Stakes Perception
pub mod consequences;
pub mod risk_field;
pub mod blast_radius;

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
