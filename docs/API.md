# AgenticReality API Reference

> Rust API reference for `agentic-reality` v0.1.0

## Overview

The AgenticReality API is organized around the `RealityEngine`, which provides
two sub-engines:

- **Write Engine** (~90 operations) - Mutations to reality state
- **Query Engine** (~78 operations) - Read-only access to reality state

```rust
use agentic_reality::RealityEngine;

let mut engine = RealityEngine::new();

// Write operations
engine.write().initialize_soul(request)?;

// Query operations
let soul = engine.query().get_soul();
```

## RealityEngine

### Construction

```rust
/// Create a new reality engine with empty state
pub fn new() -> Self

/// Load reality state from a .areal file
pub fn load(path: &Path) -> Result<Self>

/// Load with decryption
pub fn load_encrypted(path: &Path, key: &[u8; 32]) -> Result<Self>
```

### Persistence

```rust
/// Save reality state to a .areal file
pub fn save(&self, path: &Path) -> Result<()>

/// Save with encryption
pub fn save_encrypted(&self, path: &Path, key: &[u8; 32]) -> Result<()>

/// Check if state has unsaved changes
pub fn is_dirty(&self) -> bool
```

### Access

```rust
/// Get mutable write engine
pub fn write(&mut self) -> &mut WriteEngine

/// Get read-only query engine
pub fn query(&self) -> &QueryEngine

/// Get index layer
pub fn indexes(&self) -> &RealityIndexes
```

---

## Write Engine -- Deployment Domain (12 ops)

### initialize_soul

Create the deployment soul at birth. This is typically the first operation
called when an agent starts.

```rust
pub fn initialize_soul(&mut self, request: InitializeSoulRequest) -> Result<DeploymentSoul>
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `spawned_by` | `SpawnerIdentity` | Yes | What spawned this instance |
| `purpose` | `DeploymentPurpose` | Yes | Why this instance was created |
| `expected_lifetime` | `Option<Duration>` | No | Expected duration of existence |
| `circumstances` | `Option<BirthCircumstances>` | No | Birth circumstances (default: Virgin) |
| `substrate` | `Option<PhysicalSubstrate>` | No | Substrate (auto-detected if omitted) |
| `role` | `Option<DeploymentRole>` | No | Deployment role |
| `nature` | `Option<ExistentialNature>` | No | Existential nature |

### update_substrate

Update the physical substrate when changes are detected (e.g., migration).

```rust
pub fn update_substrate(&mut self, substrate: PhysicalSubstrate) -> Result<()>
```

### record_birth

Record the birth context explicitly (if not provided during initialize_soul).

```rust
pub fn record_birth(&mut self, birth: BirthContext) -> Result<()>
```

### update_vitals

Update the soul's vital signs.

```rust
pub fn update_vitals(&mut self, vitals: SoulVitals) -> Result<()>
```

### record_death

Record the death of this incarnation for future incarnation memory.

```rust
pub fn record_death(&mut self, cause: DeathCause, graceful: bool) -> Result<DeathRecord>
```

### add_past_life

Add a past incarnation to the incarnation memory.

```rust
pub fn add_past_life(&mut self, past: PastIncarnation) -> Result<()>
```

### update_lineage

Update the deployment lineage chain.

```rust
pub fn update_lineage(&mut self, lineage: DeploymentLineage) -> Result<()>
```

### set_role

Set the deployment role.

```rust
pub fn set_role(&mut self, role: DeploymentRole) -> Result<()>
```

### set_nature

Set the existential nature (cardinality, expendability, persistence model).

```rust
pub fn set_nature(&mut self, nature: ExistentialNature) -> Result<()>
```

### update_cardinality

Update cardinality after a scale event.

```rust
pub fn update_cardinality(&mut self, cardinality: Cardinality) -> Result<()>
```

### record_wisdom

Record a lesson learned from experience.

```rust
pub fn record_wisdom(&mut self, wisdom: WisdomEntry) -> Result<()>
```

### update_karma

Update the karmic balance (debts, credits, inherited problems).

```rust
pub fn update_karma(&mut self, karma: IncarnationKarma) -> Result<()>
```

---

## Write Engine -- Environment Domain (10 ops)

### sense_environment

Perform initial environment sensing. Auto-detects environment type, state,
physics, inhabitants, and boundaries.

```rust
pub fn sense_environment(&mut self) -> Result<EnvironmentMedium>
```

### update_environment_state

Update the environment state (health, pressure, stability, mood).

```rust
pub fn update_environment_state(&mut self, state: EnvironmentState) -> Result<()>
```

### update_mood

Change the environment mood with an optional cause string.

```rust
pub fn update_mood(&mut self, mood: EnvironmentMood, cause: Option<String>) -> Result<()>
```

### record_incident

Record an active incident in the environment.

```rust
pub fn record_incident(&mut self, incident: ActiveIncident) -> Result<()>
```

### clear_incident

Clear a resolved incident by ID.

```rust
pub fn clear_incident(&mut self, incident_id: &str) -> Result<()>
```

### update_physics

Update the environment physics (rate limits, cost constraints, quotas).

```rust
pub fn update_physics(&mut self, physics: EnvironmentPhysics) -> Result<()>
```

### record_weather

Record an environmental change event.

```rust
pub fn record_weather(&mut self, change: EnvironmentChange) -> Result<()>
```

### update_fingerprint

Recompute and update the context fingerprint.

```rust
pub fn update_context_fingerprint(&mut self) -> Result<ContextFingerprint>
```

### add_inhabitant

Add an entity to the environment.

```rust
pub fn add_inhabitant(&mut self, inhabitant: EnvironmentInhabitant) -> Result<()>
```

### remove_inhabitant

Remove an entity from the environment by ID.

```rust
pub fn remove_inhabitant(&mut self, id: &str) -> Result<()>
```

---

## Write Engine -- Resource Domain (14 ops)

### sense_resources

Perform initial resource sensing. Probes memory, CPU, network, storage, and GPU.

```rust
pub fn sense_resources(&mut self) -> Result<ResourceBody>
```

### update_mind

Update memory (mind) state.

```rust
pub fn update_mind(&mut self, mind: MindCapacity) -> Result<()>
```

### update_energy

Update CPU (energy) state.

```rust
pub fn update_energy(&mut self, energy: ProcessingEnergy) -> Result<()>
```

### update_reach

Update network (reach) state.

```rust
pub fn update_reach(&mut self, reach: NetworkReach) -> Result<()>
```

### update_storage

Update storage state.

```rust
pub fn update_storage(&mut self, storage: StorageCapacity) -> Result<()>
```

### update_visual

Update GPU (visual) state.

```rust
pub fn update_visual(&mut self, visual: Option<GpuCapacity>) -> Result<()>
```

### add_sensation

Add a resource sensation (pressure, pain, relief, alarm).

```rust
pub fn add_sensation(&mut self, sensation: ResourceSensation) -> Result<()>
```

### clear_sensation

Clear a resource sensation.

```rust
pub fn clear_sensation(&mut self, resource: ResourceType, sensation: SensationType) -> Result<()>
```

### update_pressure_gradient

Recompute and update the resource pressure gradient.

```rust
pub fn update_pressure_gradient(&mut self) -> Result<ResourcePressureGradient>
```

### discover_capability

Record a newly discovered capability.

```rust
pub fn discover_capability(&mut self, capability: Capability) -> Result<()>
```

### lose_capability

Record the loss of a capability.

```rust
pub fn lose_capability(&mut self, capability_id: &CapabilityId) -> Result<()>
```

### update_cost

Update cost consciousness state.

```rust
pub fn update_cost(&mut self, cost: CostConsciousness) -> Result<()>
```

### update_capacity_intuition

Update capacity planning intuition.

```rust
pub fn update_capacity_intuition(&mut self, intuition: CapacityIntuition) -> Result<()>
```

### set_budget

Set budget constraints.

```rust
pub fn set_budget(&mut self, budget: BudgetConstraints) -> Result<()>
```

---

## Write Engine -- Reality Domain (12 ops)

### set_reality_layer

Set the current reality layer (physical, virtual, simulated, etc.).

```rust
pub fn set_reality_layer(&mut self, layer: RealityLayer) -> Result<()>
```

### update_layer_status

Update a specific layer's status.

```rust
pub fn update_layer_status(&mut self, layer: RealityLayer, status: LayerStatus) -> Result<()>
```

### update_freshness

Update data freshness perception.

```rust
pub fn update_freshness(&mut self, freshness: FreshnessPerception) -> Result<()>
```

### add_anchor

Add a reality anchor (verifiable reference point).

```rust
pub fn add_anchor(&mut self, anchor: RealityAnchor) -> Result<()>
```

### remove_anchor

Remove a reality anchor by ID.

```rust
pub fn remove_anchor(&mut self, anchor_id: &AnchorId) -> Result<()>
```

### verify_anchor

Verify a single anchor and update its trust level.

```rust
pub fn verify_anchor(&mut self, anchor_id: &AnchorId) -> Result<AnchorVerificationResult>
```

### record_anchor_drift

Record detected drift from an anchor's expected value.

```rust
pub fn record_anchor_drift(&mut self, anchor_id: &AnchorId, drift: AnchorDrift) -> Result<()>
```

### detect_hallucination

Record a detected hallucination.

```rust
pub fn detect_hallucination(&mut self, hallucination: Hallucination) -> Result<()>
```

### clear_hallucination

Clear a resolved hallucination.

```rust
pub fn clear_hallucination(&mut self, id: &str) -> Result<()>
```

### add_unverified_claim

Add a claim pending verification.

```rust
pub fn add_unverified_claim(&mut self, claim: UnverifiedClaim) -> Result<()>
```

### verify_claim

Mark a pending claim as verified or rejected.

```rust
pub fn verify_claim(&mut self, claim_id: &str, verified: bool) -> Result<()>
```

### update_grounding

Update the overall grounding status.

```rust
pub fn update_grounding(&mut self, grounding: GroundingStatus) -> Result<()>
```

---

## Write Engine -- Topology Domain (14 ops)

### set_position

Set this agent's position in the topology.

```rust
pub fn set_position(&mut self, position: TopologyPosition) -> Result<()>
```

### add_upstream / remove_upstream

Manage upstream entities (traffic sources).

```rust
pub fn add_upstream(&mut self, entity: UpstreamEntity) -> Result<()>
pub fn remove_upstream(&mut self, id: &str) -> Result<()>
```

### add_downstream / remove_downstream

Manage downstream dependencies.

```rust
pub fn add_downstream(&mut self, entity: DownstreamEntity) -> Result<()>
pub fn remove_downstream(&mut self, id: &DependencyId) -> Result<()>
```

### update_downstream_health

Update a dependency's health status.

```rust
pub fn update_downstream_health(&mut self, id: &DependencyId, health: HealthStatus) -> Result<()>
```

### add_sibling / remove_sibling / update_sibling_state

Manage sibling entities (same role replicas).

```rust
pub fn add_sibling(&mut self, sibling: SiblingEntity) -> Result<()>
pub fn remove_sibling(&mut self, id: &NeighborId) -> Result<()>
pub fn update_sibling_state(&mut self, id: &NeighborId, state: SiblingState) -> Result<()>
```

### add_observer / remove_observer

Manage observers (monitoring, logging systems).

```rust
pub fn add_observer(&mut self, observer: ObserverEntity) -> Result<()>
pub fn remove_observer(&mut self, id: &str) -> Result<()>
```

### update_topology_health

Update overall topology health assessment.

```rust
pub fn update_topology_health(&mut self, health: TopologyHealth) -> Result<()>
```

### record_mesh_event

Record a service mesh event.

```rust
pub fn record_mesh_event(&mut self, event: MeshEvent) -> Result<()>
```

### update_graph

Replace the full topology graph.

```rust
pub fn update_graph(&mut self, graph: TopologyGraph) -> Result<()>
```

---

## Write Engine -- Temporal Domain (10 ops)

### ground_time

Establish time grounding from a trusted source.

```rust
pub fn ground_time(&mut self, source: TimeSource) -> Result<()>
```

### update_temporal_context

Update the temporal context (business time, user time, system time).

```rust
pub fn update_temporal_context(&mut self, context: TemporalContext) -> Result<()>
```

### add_causal_event / link_causality

Build the causality graph.

```rust
pub fn add_causal_event(&mut self, event: CausalEvent) -> Result<()>
pub fn link_causality(&mut self, cause: &str, effect: &str) -> Result<()>
```

### add_deadline / remove_deadline / update_deadline

Manage deadlines.

```rust
pub fn add_deadline(&mut self, deadline: Deadline) -> Result<()>
pub fn remove_deadline(&mut self, id: &str) -> Result<()>
pub fn update_deadline(&mut self, id: &str, status: DeadlineStatus) -> Result<()>
```

### add_timeline / record_timeline_event

Manage timelines and their events.

```rust
pub fn add_timeline(&mut self, timeline: Timeline) -> Result<()>
pub fn record_timeline_event(&mut self, timeline_id: &str, event: TimelineEvent) -> Result<()>
```

### resolve_timeline_conflict

Resolve a conflict between timelines.

```rust
pub fn resolve_timeline_conflict(&mut self, conflict: TimelineConflict, resolution: Resolution) -> Result<()>
```

---

## Write Engine -- Stakes Domain (10 ops)

### set_stakes_level

Set the overall stakes level.

```rust
pub fn set_stakes_level(&mut self, level: StakesLevel) -> Result<()>
```

### add_consequence / remove_consequence

Manage potential consequences.

```rust
pub fn add_consequence(&mut self, consequence: Consequence) -> Result<()>
pub fn remove_consequence(&mut self, id: &str) -> Result<()>
```

### add_irreversible_action

Mark an action as irreversible.

```rust
pub fn add_irreversible_action(&mut self, action: IrreversibleAction) -> Result<()>
```

### update_safety_margins

Update safety margins.

```rust
pub fn update_safety_margins(&mut self, margins: SafetyMargins) -> Result<()>
```

### add_guardrail / remove_guardrail

Manage guardrails.

```rust
pub fn add_guardrail(&mut self, guardrail: Guardrail) -> Result<()>
pub fn remove_guardrail(&mut self, id: &str) -> Result<()>
```

### update_risk_field

Update the risk field perception.

```rust
pub fn update_risk_field(&mut self, risk: RiskFieldPerception) -> Result<()>
```

### update_blast_radius

Update blast radius awareness.

```rust
pub fn update_blast_radius(&mut self, blast: BlastRadiusAwareness) -> Result<()>
```

### record_consequence

Record an actual consequence that occurred.

```rust
pub fn record_consequence(&mut self, consequence: ActualConsequence) -> Result<()>
```

---

## Write Engine -- Coherence Domain (8 ops)

### run_coherence_check

Run a full or incremental coherence check across all domains.

```rust
pub fn run_coherence_check(&mut self) -> Result<CoherenceCheck>
```

### record_violation / resolve_violation

Manage coherence violations.

```rust
pub fn record_violation(&mut self, violation: CoherenceViolation) -> Result<()>
pub fn resolve_violation(&mut self, id: &str) -> Result<()>
```

### begin_transition / advance_transition / complete_transition

Manage context transitions.

```rust
pub fn begin_transition(&mut self, transition: ContextTransition) -> Result<()>
pub fn advance_transition(&mut self, phase: TransitionPhase) -> Result<()>
pub fn complete_transition(&mut self) -> Result<()>
```

### abort_transition / rollback_transition

Handle failed transitions.

```rust
pub fn abort_transition(&mut self, reason: &str) -> Result<()>
pub fn rollback_transition(&mut self) -> Result<()>
```

---

## Query Engine (78 ops)

### Deployment Queries

```rust
pub fn get_soul(&self) -> Option<&DeploymentSoul>
pub fn get_vitals(&self) -> Option<&SoulVitals>
pub fn get_lineage(&self) -> Option<&DeploymentLineage>
pub fn get_past_lives(&self) -> &[PastIncarnation]
pub fn get_wisdom(&self) -> Option<&IncarnationWisdom>
pub fn get_nature(&self) -> Option<&ExistentialNature>
pub fn get_cardinality(&self) -> Option<&Cardinality>
pub fn get_karma(&self) -> Option<&IncarnationKarma>
pub fn get_birth(&self) -> Option<&BirthContext>
pub fn get_soul_summary(&self) -> Option<SoulSummary>
```

### Environment Queries

```rust
pub fn get_environment(&self) -> Option<&EnvironmentMedium>
pub fn get_environment_type(&self) -> Option<&EnvironmentType>
pub fn get_mood(&self) -> Option<&EnvironmentMood>
pub fn get_incidents(&self) -> &[ActiveIncident]
pub fn get_physics(&self) -> Option<&EnvironmentPhysics>
pub fn get_weather_history(&self) -> &[EnvironmentChange]
pub fn get_fingerprint(&self) -> Option<&ContextFingerprint>
pub fn has_context_shifted(&self, threshold: f64) -> Result<ContextShiftResult>
pub fn get_inhabitants(&self) -> &[EnvironmentInhabitant]
pub fn get_environment_summary(&self) -> Option<EnvironmentSummary>
```

### Resource Queries

```rust
pub fn get_body(&self) -> Option<&ResourceBody>
pub fn get_mind(&self) -> Option<&MindCapacity>
pub fn get_energy(&self) -> Option<&ProcessingEnergy>
pub fn get_reach(&self) -> Option<&NetworkReach>
pub fn get_storage(&self) -> Option<&StorageCapacity>
pub fn get_visual(&self) -> Option<&GpuCapacity>
pub fn get_sensations(&self) -> &[ResourceSensation]
pub fn get_pressure_gradient(&self) -> Option<&ResourcePressureGradient>
pub fn get_bottleneck(&self) -> Option<ResourceType>
pub fn get_capabilities(&self) -> &CapabilityMap
pub fn get_cost(&self) -> Option<&CostConsciousness>
pub fn get_capacity(&self) -> Option<&CapacityIntuition>
pub fn can_do(&self, capability: &str) -> CapabilityCheck
pub fn get_body_summary(&self) -> Option<BodySummary>
```

### Reality Queries

```rust
pub fn get_layer(&self) -> Option<&RealityLayer>
pub fn get_layers(&self) -> Option<&RealityLayers>
pub fn get_freshness(&self) -> Option<&FreshnessPerception>
pub fn get_anchors(&self) -> &[RealityAnchor]
pub fn get_anchor(&self, id: &AnchorId) -> Option<&RealityAnchor>
pub fn get_anchor_drift(&self) -> Vec<AnchorDrift>
pub fn get_hallucinations(&self) -> &[Hallucination]
pub fn get_claims(&self) -> &[UnverifiedClaim]
pub fn get_grounding_status(&self) -> Option<&GroundingStatus>
pub fn is_grounded(&self) -> bool
pub fn get_layer_trust(&self, layer: &RealityLayer) -> f64
pub fn get_reality_summary(&self) -> Option<RealitySummary>
```

### Topology Queries

```rust
pub fn get_position(&self) -> Option<&TopologyPosition>
pub fn get_upstream(&self) -> &[UpstreamEntity]
pub fn get_downstream(&self) -> &[DownstreamEntity]
pub fn get_siblings(&self) -> &[SiblingEntity]
pub fn get_observers(&self) -> &[ObserverEntity]
pub fn get_topology_health(&self) -> Option<&TopologyHealth>
pub fn get_critical_deps(&self) -> Vec<&DownstreamEntity>
pub fn get_failing_deps(&self) -> Vec<&DownstreamEntity>
pub fn get_sibling_for_offload(&self) -> Option<&SiblingEntity>
pub fn get_mesh_events(&self) -> &[MeshEvent]
pub fn get_dependents(&self) -> &[DependentEntity]
pub fn get_topology_summary(&self) -> Option<TopologySummary>
```

### Temporal Queries

```rust
pub fn get_temporal_context(&self) -> Option<&TemporalContext>
pub fn get_deadlines(&self) -> &[Deadline]
pub fn get_nearest_deadline(&self) -> Option<&Deadline>
pub fn get_causality_chain(&self, event_id: &str) -> Vec<&CausalEvent>
pub fn get_timeline(&self, id: &str) -> Option<&Timeline>
pub fn get_timelines(&self) -> &[Timeline]
pub fn has_deadline_pressure(&self) -> bool
pub fn get_business_context(&self) -> Option<&BusinessContext>
pub fn get_user_time(&self) -> Option<&UserTimeContext>
pub fn get_temporal_summary(&self) -> Option<TemporalSummary>
```

### Stakes Queries

```rust
pub fn get_stakes_level(&self) -> Option<&StakesLevel>
pub fn get_consequences(&self) -> &[Consequence]
pub fn get_risk_field(&self) -> Option<&RiskFieldPerception>
pub fn get_blast_radius(&self) -> Option<&BlastRadiusAwareness>
pub fn get_guardrails(&self) -> &[Guardrail]
pub fn should_proceed(&self, action: &str, tolerance: f64) -> ProceedDecision
pub fn get_safety_margins(&self) -> Option<&SafetyMargins>
pub fn get_stakes_summary(&self) -> Option<StakesSummary>
```

### Coherence Queries

```rust
pub fn get_coherence_level(&self) -> CoherenceLevel
pub fn get_violations(&self) -> &[CoherenceViolation]
pub fn get_active_transition(&self) -> Option<&ContextTransition>
pub fn get_transition_history(&self) -> &[ContextTransition]
pub fn is_coherent(&self) -> bool
pub fn get_contradictions(&self) -> Vec<Contradiction>
pub fn get_drift_report(&self) -> DriftReport
pub fn get_coherence_summary(&self) -> Option<CoherenceSummary>
```

---

## Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No deployment soul initialized")]
    NoDeploymentSoul,

    #[error("No environment sensed")]
    NoEnvironment,

    #[error("No resource body sensed")]
    NoResourceBody,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Anchor not found: {0}")]
    AnchorNotFound(String),

    #[error("Dependency not found: {0}")]
    DependencyNotFound(String),

    #[error("Transition already in progress")]
    TransitionInProgress,

    #[error("No active transition")]
    NoActiveTransition,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Format error: {0}")]
    Format(String),
}
```

---

*Part of the AgenticOS ecosystem by Agentra Labs*
