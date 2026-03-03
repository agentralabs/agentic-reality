# AgenticReality Sister Integration

> Integration with other Agentra sisters through bridge traits

## Overview

AgenticReality defines 9 bridge traits for integration with other sisters in
the AgenticOS ecosystem. Each bridge has a NoOp default implementation,
allowing AgenticReality to operate standalone without any sister dependencies.

When a sister crate is available, its bridge implementation replaces the NoOp,
enabling cross-sister capabilities.

## Bridge Architecture

```
AgenticReality Engine
    │
    ├── TimeBridge       ──→  AgenticTime     (temporal grounding)
    ├── ContractBridge   ──→  AgenticContract  (stakes constraints)
    ├── IdentityBridge   ──→  AgenticIdentity  (incarnation verification)
    ├── MemoryBridge     ──→  AgenticMemory    (incarnation persistence)
    ├── CognitionBridge  ──→  AgenticCognition (risk modeling)
    ├── CommBridge       ──→  AgenticComm      (neighbor communication)
    ├── CodebaseBridge   ──→  AgenticCodebase  (code context)
    ├── VisionBridge     ──→  AgenticVision    (visual sensing)
    └── HydraBridge      ──→  Hydra            (orchestrator)
```

## Required Dependencies

These sisters are required for full functionality. Without them, certain
features operate in degraded mode.

### TimeBridge (AgenticTime >= 0.1.0)

Provides temporal grounding: verified time sources, deadline management,
freshness calculations, and temporal context enrichment.

```rust
pub trait TimeBridge: Send + Sync {
    /// Get verified current time from authoritative source
    fn verified_now(&self) -> Result<VerifiedTimestamp>;

    /// Calculate data freshness relative to verified time
    fn freshness(&self, data_timestamp: Timestamp) -> Result<FreshnessAssessment>;

    /// Check deadline proximity with verified time
    fn deadline_check(&self, deadline: &Deadline) -> Result<DeadlineProximity>;

    /// Get temporal context enrichment (business hours, seasons, etc.)
    fn temporal_context(&self) -> Result<TemporalEnrichment>;

    /// Verify a time anchor against authoritative source
    fn verify_time_anchor(&self, anchor: &RealityAnchor) -> Result<AnchorVerificationResult>;
}
```

**NoOp behavior:** Uses system clock without verification, freshness
calculations use local timestamps, no temporal enrichment.

### ContractBridge (AgenticContract >= 0.1.0)

Provides stakes constraints: SLA definitions, compliance requirements,
contractual limits on agent behavior.

```rust
pub trait ContractBridge: Send + Sync {
    /// Get SLA constraints for current context
    fn get_sla(&self, context: &EnvironmentType) -> Result<Option<SlaConstraints>>;

    /// Check if an action violates any contract
    fn check_action(&self, action: &str, stakes: &StakesLevel) -> Result<ContractCheck>;

    /// Get compliance requirements for this environment
    fn compliance_requirements(&self) -> Result<Vec<ComplianceRequirement>>;

    /// Report a contract violation
    fn report_violation(&self, violation: ContractViolation) -> Result<()>;

    /// Get cost constraints from active contracts
    fn cost_constraints(&self) -> Result<Option<CostConstraints>>;
}
```

**NoOp behavior:** No SLA constraints, all actions permitted, no compliance
requirements.

### IdentityBridge (AgenticIdentity >= 0.3.0)

Provides incarnation identity verification: cryptographic identity for
deployment souls, trust scoring, lineage verification.

```rust
pub trait IdentityBridge: Send + Sync {
    /// Verify this incarnation's identity
    fn verify_incarnation(&self, soul: &DeploymentSoul) -> Result<IdentityVerification>;

    /// Get trust score for a neighbor
    fn trust_score(&self, neighbor: &SiblingEntity) -> Result<f64>;

    /// Verify deployment lineage chain
    fn verify_lineage(&self, lineage: &DeploymentLineage) -> Result<LineageVerification>;

    /// Sign a death record for incarnation memory
    fn sign_death_record(&self, record: &DeathRecord) -> Result<SignedRecord>;

    /// Verify a past life claim
    fn verify_past_life(&self, claim: &PastIncarnation) -> Result<bool>;
}
```

**NoOp behavior:** All identities unverified, trust score defaults to 0.5,
lineage unverified.

## Optional Dependencies

These sisters enhance functionality but are not required for core operation.

### MemoryBridge (AgenticMemory >= 0.4.0)

Persists incarnation memory across restarts using AgenticMemory's storage.

```rust
pub trait MemoryBridge: Send + Sync {
    /// Save incarnation memory for next life
    fn save_incarnation_memory(&self, memory: &IncarnationMemory) -> Result<()>;

    /// Load incarnation memory from previous life
    fn load_incarnation_memory(&self, incarnation_id: &IncarnationId) -> Result<Option<IncarnationMemory>>;

    /// Save wisdom for long-term retention
    fn save_wisdom(&self, wisdom: &IncarnationWisdom) -> Result<()>;

    /// Search for relevant wisdom from any past life
    fn search_wisdom(&self, query: &str) -> Result<Vec<WisdomEntry>>;

    /// Save death record for future reference
    fn save_death_record(&self, record: &DeathRecord) -> Result<()>;
}
```

**NoOp behavior:** Incarnation memory not persisted between restarts, wisdom
lost on death.

### CognitionBridge (AgenticCognition >= 0.1.0)

Enriches risk perception with cognitive modeling: consequence prediction,
risk assessment, decision support.

```rust
pub trait CognitionBridge: Send + Sync {
    /// Model consequences of an action
    fn model_consequences(&self, action: &str, context: &StakesLevel) -> Result<Vec<Consequence>>;

    /// Assess risk with cognitive analysis
    fn assess_risk(&self, action: &str, risk_field: &RiskFieldPerception) -> Result<RiskAssessment>;

    /// Get decision support for proceed/halt choice
    fn decision_support(&self, action: &str, tolerance: f64) -> Result<DecisionSupport>;

    /// Detect cognitive biases in agent's reasoning
    fn detect_biases(&self, reasoning: &str) -> Result<Vec<CognitiveBias>>;
}
```

**NoOp behavior:** Simple heuristic-based risk assessment, no cognitive
modeling.

### CommBridge (AgenticComm >= 0.1.0)

Enables real-time communication with neighbor agents for coordinated
reality awareness.

```rust
pub trait CommBridge: Send + Sync {
    /// Broadcast reality state to neighbors
    fn broadcast_state(&self, state: &RealitySummary) -> Result<()>;

    /// Receive reality state from a neighbor
    fn receive_state(&self, neighbor: &NeighborId) -> Result<Option<RealitySummary>>;

    /// Coordinate topology awareness across siblings
    fn coordinate_topology(&self, topology: &DeploymentTopologyMap) -> Result<()>;

    /// Alert neighbors to detected issues
    fn alert_neighbors(&self, alert: RealityAlert) -> Result<()>;
}
```

**NoOp behavior:** No inter-agent communication, topology awareness is local
only.

### CodebaseBridge (AgenticCodebase >= 0.1.0)

Provides code context for deployment awareness: what version is deployed,
recent changes, code health.

```rust
pub trait CodebaseBridge: Send + Sync {
    /// Get deployed version info
    fn deployed_version(&self) -> Result<Option<VersionInfo>>;

    /// Get recent changes since last deployment
    fn recent_changes(&self) -> Result<Vec<ChangeRecord>>;

    /// Check code health metrics
    fn code_health(&self) -> Result<Option<CodeHealth>>;
}
```

**NoOp behavior:** Version info from environment variables only, no change
history.

### VisionBridge (AgenticVision >= 0.1.0)

Enables visual environment sensing: dashboard screenshots, monitoring
visualizations, physical environment awareness.

```rust
pub trait VisionBridge: Send + Sync {
    /// Capture visual state of monitoring dashboards
    fn capture_dashboard(&self, dashboard_url: &str) -> Result<Option<DashboardSnapshot>>;

    /// Interpret visual alerts from monitoring UIs
    fn interpret_visual_alerts(&self) -> Result<Vec<VisualAlert>>;
}
```

**NoOp behavior:** No visual sensing capability.

### HydraBridge (Hydra)

The orchestrator bridge enabling Hydra to coordinate reality awareness
across all sisters.

```rust
pub trait HydraBridge: Send + Sync {
    /// Report reality state to Hydra
    fn report_state(&self, state: &RealitySummary) -> Result<()>;

    /// Receive reality directives from Hydra
    fn receive_directives(&self) -> Result<Vec<RealityDirective>>;

    /// Register this agent with the Hydra mesh
    fn register(&self, soul: &DeploymentSoul) -> Result<HydraRegistration>;

    /// Heartbeat to Hydra
    fn heartbeat(&self, vitals: &SoulVitals) -> Result<HydraResponse>;
}
```

**NoOp behavior:** No orchestrator integration, fully autonomous operation.

## Integration Patterns

### Graceful Degradation

All bridges follow the graceful degradation pattern. When a sister is
unavailable, the feature degrades rather than fails:

```rust
// Example: temporal grounding with optional TimeBridge
fn ground_temporal_context(&self) -> Result<TemporalContext> {
    match self.time_bridge.temporal_context() {
        Ok(enriched) => Ok(enriched.into()),
        Err(_) => {
            // Degrade to local-only temporal context
            Ok(TemporalContext::from_system_clock())
        }
    }
}
```

### Bridge Registration

Bridges are registered at engine creation:

```rust
let mut engine = RealityEngine::builder()
    .with_time_bridge(AgenticTimeBridge::new())
    .with_contract_bridge(AgenticContractBridge::new())
    .with_identity_bridge(AgenticIdentityBridge::new())
    .build();
```

### Bridge Health Monitoring

The engine monitors bridge health and reports it as part of topology awareness:

```rust
let bridge_health = engine.query().get_bridge_health();
// Returns health status for each registered bridge
```

## SDK Reference

All sisters reference `agentic-sdk v0.2.0` for shared types and protocols.
The SDK provides common type definitions used across bridge boundaries.

---

*Part of the AgenticOS ecosystem by Agentra Labs*
