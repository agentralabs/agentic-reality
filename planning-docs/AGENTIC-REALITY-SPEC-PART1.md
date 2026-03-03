# AGENTIC REALITY SPECIFICATION — PART 1

> **Specs Covered:** SPEC-01 through SPEC-04
> **Sister:** #10 of 25 (The Ground)
> **Format:** `.areal`
> **Essence:** The sister that knows WHERE it exists and WHAT is real

---

# SPEC-01: OVERVIEW & DEPENDENCIES

## 1.1 Problem Statement

AI agents operate in a **reality vacuum**:

```
CURRENT STATE:
══════════════

Agent receives request
    ↓
Agent processes (blind to context)
    ↓
Agent responds

The agent has NO IDEA:
- Is this production or test?
- Am I on a laptop or data center?
- Do I have 1GB or 100GB RAM?
- Is my database healthy?
- What time zone is the user in?
- What are the stakes of this response?
- Am I hallucinating right now?
- What happens if I fail?
```

**This blindness leads to:**
- Production bugs that would never happen in dev
- Resource exhaustion from not knowing limits
- Stale data served as fresh
- Cascading failures from unknown dependencies
- Expensive operations in cost-constrained contexts
- Catastrophic mistakes in high-stakes environments

## 1.2 Solution: Reality Grounding

AgenticReality gives agents **existential awareness**:

```
WITH AGENTIC REALITY:
═════════════════════

┌─────────────────────────────────────────────────────────────┐
│                    REALITY AWARENESS                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  DEPLOYMENT        "I am prod-us-east-node-7"              │
│  CONSCIOUSNESS     "I was spawned 4 hours ago"              │
│                    "I'm one of 12 replicas"                 │
│                                                             │
│  RESOURCE          "Memory at 73%, feeling crowded"         │
│  PROPRIOCEPTION    "Network is sluggish today"              │
│                    "Cost burn rate: $2.40/hour"             │
│                                                             │
│  REALITY           "Operating in production layer"          │
│  PHYSICS           "Cache is 47 minutes stale"              │
│                    "3 anchors verified, 1 drifting"         │
│                                                             │
│  TOPOLOGY          "Database latency: 12ms"                 │
│  AWARENESS         "Sibling replica-3 is struggling"        │
│                    "Prometheus is watching me"              │
│                                                             │
│  TEMPORAL          "It's end-of-quarter crunch"             │
│  GROUNDING         "User is in Tokyo (3am there)"           │
│                    "Deadline in 47 minutes"                 │
│                                                             │
│  STAKES            "High stakes: financial API"             │
│  PERCEPTION        "Blast radius: 50,000 users"             │
│                    "This action is irreversible"            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
                              ↓
                Agent makes GROUNDED decisions
```

## 1.3 Architecture Overview

```
ARCHITECTURE:
═════════════

┌─────────────────────────────────────────────────────────────┐
│                      MCP SERVER                             │
│                    (15 tools)                               │
├─────────────────────────────────────────────────────────────┤
│  deployment | environment | resource | capacity | layer     │
│  hallucination | topology | observer | temporal | stakes    │
│  coherence | transition | context | ground | workspace      │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    REALITY ENGINE                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ Deployment  │  │  Resource   │  │   Reality   │        │
│  │  Manager    │  │  Monitor    │  │   Layers    │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │  Topology   │  │  Temporal   │  │   Stakes    │        │
│  │   Mapper    │  │  Grounder   │  │  Assessor   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ Coherence   │  │ Transition  │  │ Hallucin.   │        │
│  │   Engine    │  │  Manager    │  │  Detector   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                    STORAGE LAYER                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Deployment │ Environment │ Resources │ Topology   │   │
│  │    Store    │    Store    │   Store   │   Store    │   │
│  └─────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Temporal   │   Stakes    │ Coherence │  Indexes   │   │
│  │   Store     │   Store     │   Store   │            │   │
│  └─────────────────────────────────────────────────────┘   │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                      .areal FILE                            │
│              (Reality state persistence)                    │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   SISTER BRIDGES                            │
├─────────────────────────────────────────────────────────────┤
│  Time ↔ Contract ↔ Identity ↔ Memory ↔ Cognition ↔ Comm   │
│                         ↓                                   │
│                      HYDRA                                  │
└─────────────────────────────────────────────────────────────┘
```

## 1.4 Dependencies

```
REQUIRED DEPENDENCIES:
══════════════════════

┌─────────────────┬─────────┬────────────────────────────────┐
│     Sister      │ Version │            Purpose             │
├─────────────────┼─────────┼────────────────────────────────┤
│ AgenticTime     │ ≥0.1.0  │ Temporal grounding, deadlines, │
│                 │         │ freshness calculations         │
├─────────────────┼─────────┼────────────────────────────────┤
│ AgenticContract │ ≥0.1.0  │ Stakes constraints, SLAs,      │
│                 │         │ compliance requirements        │
├─────────────────┼─────────┼────────────────────────────────┤
│ AgenticIdentity │ ≥0.3.0  │ Incarnation identity,          │
│                 │         │ verification, trust            │
└─────────────────┴─────────┴────────────────────────────────┘

OPTIONAL INTEGRATIONS:
══════════════════════

┌─────────────────┬─────────┬────────────────────────────────┐
│     Sister      │ Version │            Purpose             │
├─────────────────┼─────────┼────────────────────────────────┤
│ AgenticMemory   │ ≥0.4.0  │ Incarnation memory persistence │
│                 │         │ across restarts                │
├─────────────────┼─────────┼────────────────────────────────┤
│ AgenticCogntic  │ ≥0.1.0  │ Risk perception modeling,      │
│                 │         │ consequence understanding      │
├─────────────────┼─────────┼────────────────────────────────┤
│ AgenticComm     │ ≥0.1.0  │ Neighbor communication,        │
│                 │         │ mesh awareness                 │
└─────────────────┴─────────┴────────────────────────────────┘
```

## 1.5 Design Principles

```
PRINCIPLES:
═══════════

1. SENSE, DON'T QUERY
   Resources should be FELT, not polled.
   Environment should be PERCEIVED, not read.
   Like proprioception, not like reading a gauge.

2. GROUND TO TRUTH
   Always maintain anchors to verifiable reality.
   Detect drift from truth.
   Know when you're uncertain.

3. CONTEXT IS IDENTITY
   The deployment context shapes the agent's identity.
   A production agent IS different from a test agent.
   Not configuration — incarnation.

4. COHERENCE OVER COMPLETENESS
   Better to have coherent partial knowledge
   than incoherent complete knowledge.
   Maintain consistency across all perceptions.

5. STAKES AWARENESS
   Always know the consequences.
   High stakes → cautious behavior.
   Low stakes → experimentation allowed.

6. GRACEFUL DEGRADATION
   Can operate with partial reality awareness.
   More awareness = better decisions.
   Minimum viable reality = deployment + environment.
```

---

# SPEC-02: CORE CONCEPTS

## 2.1 The Seven Domains of Reality

```
REALITY DOMAINS:
════════════════

┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   1. DEPLOYMENT CONSCIOUSNESS                               │
│      "Where do I exist?"                                    │
│      ├── Incarnation identity                               │
│      ├── Birth context                                      │
│      ├── Physical substrate                                 │
│      ├── Lineage (past lives)                              │
│      └── Soul vitals                                        │
│                                                             │
│   2. RESOURCE PROPRIOCEPTION                                │
│      "What do I have?"                                      │
│      ├── Memory (mind capacity)                             │
│      ├── CPU (processing energy)                            │
│      ├── Network (reach)                                    │
│      ├── Storage (long-term memory)                         │
│      ├── GPU (visual processing)                            │
│      └── Cost (operational expense)                         │
│                                                             │
│   3. REALITY PHYSICS                                        │
│      "What is real?"                                        │
│      ├── Reality layers                                     │
│      ├── Data freshness                                     │
│      ├── Truth anchors                                      │
│      └── Hallucination state                                │
│                                                             │
│   4. TOPOLOGY AWARENESS                                     │
│      "What surrounds me?"                                   │
│      ├── Service mesh                                       │
│      ├── Neighbors                                          │
│      ├── Dependencies                                       │
│      └── Observers                                          │
│                                                             │
│   5. TEMPORAL GROUNDING                                     │
│      "When am I?"                                           │
│      ├── Time context                                       │
│      ├── Causality                                          │
│      └── Timeline coherence                                 │
│                                                             │
│   6. STAKES PERCEPTION                                      │
│      "What are the consequences?"                           │
│      ├── Stakes level                                       │
│      ├── Risk field                                         │
│      └── Blast radius                                       │
│                                                             │
│   7. COHERENCE MAINTENANCE                                  │
│      "How do I stay grounded?"                              │
│      ├── Coherence checks                                   │
│      └── Context transitions                                │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 2.2 Entity Lifecycle

```
REALITY STATE LIFECYCLE:
════════════════════════

    ┌──────────────────────────────────────────────────────┐
    │                     BIRTH                            │
    │  Reality state created at deployment start           │
    │  Initial sensing of environment, resources           │
    └────────────────────────┬─────────────────────────────┘
                             │
                             ▼
    ┌──────────────────────────────────────────────────────┐
    │                   CALIBRATION                        │
    │  Establishing baselines for all senses               │
    │  Discovering capabilities, mapping topology          │
    │  Verifying reality anchors                           │
    └────────────────────────┬─────────────────────────────┘
                             │
                             ▼
    ┌──────────────────────────────────────────────────────┐
    │                    GROUNDED                          │
    │  Normal operation with full reality awareness        │
    │  Continuous sensing, coherence maintenance           │
    └────────────┬───────────────────────────┬─────────────┘
                 │                           │
    ┌────────────▼────────────┐  ┌──────────▼─────────────┐
    │       DRIFTING          │  │      TRANSITIONING     │
    │  Losing anchor(s)       │  │  Context is changing   │
    │  Coherence degrading    │  │  (scale, migrate, etc) │
    └────────────┬────────────┘  └──────────┬─────────────┘
                 │                           │
                 ▼                           ▼
    ┌─────────────────────────┐  ┌────────────────────────┐
    │      UNMOORED           │  │    NEW CONTEXT         │
    │  Lost grounding         │  │  Re-establish after    │
    │  Emergency procedures   │  │  transition complete   │
    └────────────┬────────────┘  └──────────┬─────────────┘
                 │                           │
                 └───────────┬───────────────┘
                             │
                             ▼
    ┌──────────────────────────────────────────────────────┐
    │                  RE-GROUNDING                        │
    │  Re-verifying anchors, re-establishing coherence     │
    │  May require fresh calibration                       │
    └────────────────────────┬─────────────────────────────┘
                             │
                             ▼
                      (back to GROUNDED)


INCARNATION LIFECYCLE:
══════════════════════

    BIRTH                    How this incarnation started
      │                      (virgin, scaled, resurrected, migrated)
      ▼
    ALIVE ◀──────────────────┐
      │                      │
      │  Normal operation    │ Resurrection
      │                      │ (if state preserved)
      ▼                      │
    DYING ───────────────────┤
      │                      │
      │  Graceful shutdown   │
      │  State preservation  │
      ▼                      │
    DEAD ────────────────────┘
      │
      └── Death record (cause, final state, lessons)
```

## 2.3 Resource Sensation Model

```
RESOURCE SENSATIONS:
════════════════════

Resources are FELT, not queried:

┌─────────────────────────────────────────────────────────────┐
│                    MIND (Memory)                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Clear        ████░░░░░░░░░░░░░░░░  < 30%  "Plenty of     │
│                                             headroom"       │
│                                                             │
│  Active       ████████░░░░░░░░░░░░  30-60% "Normal         │
│                                             working state"  │
│                                                             │
│  Crowded      ████████████░░░░░░░░  60-80% "Getting full,  │
│                                             GC recommended" │
│                                                             │
│  Strained     ████████████████░░░░  80-90% "Under          │
│                                             pressure"       │
│                                                             │
│  Overwhelmed  ████████████████████  90-95% "Critical,      │
│                                             shed load"      │
│                                                             │
│  Drowning     ████████████████████  > 95%  "OOM imminent"  │
│                                                             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                  ENERGY (CPU)                               │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Vigorous     Abundant processing power, can take more     │
│  Steady       Normal utilization, sustainable              │
│  Busy         Working hard but healthy                     │
│  Strained     Running hot, may need to throttle            │
│  Depleted     Exhausted, need to shed work                 │
│  Constrained  Throttled/limited externally                 │
│                                                             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                  REACH (Network)                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Connected    Fast, clear connections to all endpoints     │
│  Normal       Typical network conditions                   │
│  Sluggish     Elevated latency, some packet loss           │
│  Constrained  Bandwidth limited, connections failing       │
│  Isolated     Major connectivity issues                    │
│  Partitioned  Cannot reach critical services               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 2.4 Reality Layer Model

```
REALITY LAYERS:
═══════════════

┌──────────────────────────────────────────────────────────┐
│                    PHYSICAL                              │
│  The hardware reality — servers, network, power          │
│  Highest trust, slowest to change                        │
└────────────────────────────┬─────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────┐
│                    VIRTUAL                               │
│  VMs, containers — real but abstracted                   │
│  Can be migrated, resized, terminated                    │
└────────────────────────────┬─────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────┐
│                   LOGICAL                                │
│  Services, APIs, configurations                          │
│  Changes frequently, versions matter                     │
└────────────────────────────┬─────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────┐
│                    CACHED                                │
│  Stored representations of reality                       │
│  May be stale, needs freshness tracking                  │
└────────────────────────────┬─────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────┐
│                   PREDICTED                              │
│  Forecasts, estimates, projections                       │
│  Uncertain, needs confidence bounds                      │
└────────────────────────────┬─────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────┐
│                   SIMULATED                              │
│  Test doubles, mocks, synthetic environments             │
│  Not real, but useful for testing                        │
└────────────────────────────┬─────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────┐
│                  HALLUCINATED                            │
│  Agent's false beliefs, errors, confabulations           │
│  Must be detected and corrected                          │
└──────────────────────────────────────────────────────────┘


LAYER TRUST LEVELS:
═══════════════════

Physical      → Trust: 0.99  (hardware rarely lies)
Virtual       → Trust: 0.95  (hypervisor is reliable)
Logical       → Trust: 0.85  (configs can be wrong)
Cached        → Trust: 0.70 × freshness_factor
Predicted     → Trust: 0.50 × confidence
Simulated     → Trust: 0.30  (useful but not real)
Hallucinated  → Trust: 0.00  (by definition false)
```

## 2.5 Stakes Model

```
STAKES LEVELS:
══════════════

┌─────────────────────────────────────────────────────────────┐
│                      CRITICAL                               │
├─────────────────────────────────────────────────────────────┤
│  • Financial transactions                                   │
│  • Safety-critical systems                                  │
│  • Healthcare/medical                                       │
│  • Legal/compliance                                         │
│                                                             │
│  Behavior: Multiple approvals, full audit, zero risk        │
│  Example: Processing a $1M wire transfer                    │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                        HIGH                                 │
├─────────────────────────────────────────────────────────────┤
│  • Production user-facing                                   │
│  • Customer data handling                                   │
│  • Revenue-impacting                                        │
│                                                             │
│  Behavior: Caution required, approval may be needed         │
│  Example: Updating a customer's account settings            │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                       MEDIUM                                │
├─────────────────────────────────────────────────────────────┤
│  • Internal tools                                           │
│  • Staging environment                                      │
│  • Non-critical production                                  │
│                                                             │
│  Behavior: Normal care, review recommended                  │
│  Example: Generating an internal report                     │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                        LOW                                  │
├─────────────────────────────────────────────────────────────┤
│  • Development environment                                  │
│  • Test data                                                │
│  • Experiments                                              │
│                                                             │
│  Behavior: Some caution, rollback available                 │
│  Example: Testing a new feature in dev                      │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                      MINIMAL                                │
├─────────────────────────────────────────────────────────────┤
│  • Sandbox                                                  │
│  • Disposable environments                                  │
│  • Pure simulation                                          │
│                                                             │
│  Behavior: Experimentation allowed, learn freely            │
│  Example: Playing in a sandbox environment                  │
└─────────────────────────────────────────────────────────────┘
```

## 2.6 Coherence Model

```
COHERENCE DIMENSIONS:
═════════════════════

1. TEMPORAL COHERENCE
   Is my sense of time consistent?
   - Clock sources agree
   - Causal ordering maintained
   - No impossible sequences

2. SPATIAL COHERENCE
   Is my sense of topology consistent?
   - Dependencies match observed latencies
   - Neighbors exist where expected
   - Network topology matches perception

3. STATE COHERENCE
   Is my sense of state consistent?
   - Cached values match reality
   - Predicted values within bounds
   - No contradictory observations

4. IDENTITY COHERENCE
   Is my sense of self consistent?
   - Incarnation matches records
   - Capabilities match substrate
   - Role matches position

5. ANCHOR COHERENCE
   Are my reality anchors consistent?
   - Multiple anchors agree
   - Drift within tolerance
   - No anchor contradictions


COHERENCE LEVELS:
═════════════════

Full        All dimensions coherent, high confidence
            → Normal operation

Minor       1-2 minor inconsistencies
            → Operate with elevated monitoring

Significant 3+ inconsistencies or 1 major
            → Caution mode, verify before acting

Incoherent  Major contradictions detected
            → Stop, re-ground, verify everything
```

---

# SPEC-03: DATA STRUCTURES

## 3.1 Core Identifiers

```rust
//! src/types/ids.rs

use uuid::Uuid;

/// Incarnation identifier — unique per deployment instance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IncarnationId(pub Uuid);

/// Context identifier — unique reality context
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContextId(pub Uuid);

/// Anchor identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnchorId(pub Uuid);

/// Timeline identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimelineId(pub Uuid);

/// Transition identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransitionId(pub Uuid);

/// Event identifier (causal)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(pub Uuid);

/// Service identity in topology
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceId(pub String);

/// Dependency identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DependencyId(pub Uuid);

/// Neighbor identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NeighborId(pub String);

/// Observer identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObserverId(pub String);

/// Snapshot identifier (for ghost writer)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnapshotId(pub Uuid);

impl IncarnationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_context(context: &str) -> Self {
        // Deterministic ID from context for reproducibility
        Self(Uuid::new_v5(&Uuid::NAMESPACE_OID, context.as_bytes()))
    }
}
```

## 3.2 Deployment Structures

```rust
//! src/types/deployment.rs

/// The soul of a deployment — existential identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSoul {
    /// Unique incarnation identifier
    pub incarnation_id: IncarnationId,
    
    /// Birth context — how this instance came to be
    pub birth: BirthContext,
    
    /// Physical substrate — what hardware hosts this soul
    pub substrate: PhysicalSubstrate,
    
    /// Logical context — what system role this fills
    pub role: DeploymentRole,
    
    /// Existential properties — fundamental nature
    pub nature: ExistentialNature,
    
    /// Lineage — chain of deployments leading to this one
    pub lineage: DeploymentLineage,
    
    /// Current vitals — is this soul healthy?
    pub vitals: SoulVitals,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirthContext {
    /// When this incarnation began
    pub spawned_at: Timestamp,
    
    /// What spawned this instance
    pub spawned_by: SpawnerIdentity,
    
    /// Why this instance was created
    pub purpose: DeploymentPurpose,
    
    /// Expected duration of existence
    pub expected_lifetime: Option<Duration>,
    
    /// Previous incarnation (if reincarnated)
    pub previous_life: Option<IncarnationId>,
    
    /// Birth circumstances
    pub circumstances: BirthCircumstances,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BirthCircumstances {
    /// Fresh deployment, no prior existence
    Virgin,
    
    /// Scaled out from existing deployment
    ScaledFrom { parent: IncarnationId },
    
    /// Restarted after crash
    Resurrected { 
        death_cause: DeathCause,
        previous: IncarnationId,
    },
    
    /// Migrated from different substrate
    Migrated { 
        from_substrate: SubstrateId,
        migration_reason: String,
    },
    
    /// Forked from another instance
    Forked { 
        from: IncarnationId, 
        fork_reason: String,
    },
    
    /// Created for specific task then will die
    Ephemeral { 
        task_id: String,
        ttl: Duration,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalSubstrate {
    /// Substrate identifier
    pub id: SubstrateId,
    
    /// Hardware tier
    pub tier: SubstrateTier,
    
    /// Geographic location
    pub location: GeographicLocation,
    
    /// Network position
    pub network_position: NetworkPosition,
    
    /// Isolation level
    pub isolation: IsolationLevel,
    
    /// Shared or dedicated
    pub tenancy: TenancyModel,
    
    /// Substrate capabilities
    pub capabilities: SubstrateCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubstrateTier {
    /// Running on someone's laptop
    Laptop { 
        owner: Option<String>,
        os: String,
    },
    
    /// Mobile device
    Mobile { 
        device_type: MobileType,
        os_version: String,
    },
    
    /// Browser/WebAssembly
    Browser { 
        browser: BrowserType,
        version: String,
    },
    
    /// Edge node (close to users)
    Edge { 
        region: String, 
        pop: String,
        provider: Option<String>,
    },
    
    /// Standard cloud VM
    Cloud { 
        provider: CloudProvider, 
        instance_type: String,
        region: String,
    },
    
    /// Dedicated bare metal
    BareMetal { 
        specs: HardwareSpecs,
        location: String,
    },
    
    /// GPU cluster node
    GpuCluster { 
        gpu_count: u32, 
        gpu_type: String,
        interconnect: String,
    },
    
    /// Serverless/FaaS
    Serverless {
        provider: String,
        memory_mb: u32,
        timeout: Duration,
    },
    
    /// Unknown/undetectable
    Unknown {
        clues: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistentialNature {
    /// Is this a singleton or one of many?
    pub cardinality: Cardinality,
    
    /// Can this instance be killed without consequence?
    pub expendability: f64,  // 0 = critical, 1 = fully expendable
    
    /// Does this instance have persistent identity across restarts?
    pub persistence: PersistenceModel,
    
    /// Is this instance stateful or stateless?
    pub statefulness: StatefulnessModel,
    
    /// Can this instance be cloned?
    pub clonability: bool,
    
    /// Is this the "real" instance or a replica?
    pub primacy: InstancePrimacy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cardinality {
    /// The only instance in existence
    Singleton,
    
    /// One of N identical instances
    ReplicaOf { total: u32, index: u32 },
    
    /// Primary with replicas
    PrimaryWithReplicas { replica_count: u32 },
    
    /// Hot standby (ready to take over)
    HotStandby { primary: IncarnationId },
    
    /// Part of a swarm (no hierarchy)
    SwarmMember { swarm_id: String, swarm_size: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulVitals {
    /// Overall health
    pub health: f64,
    
    /// Time alive
    pub uptime: Duration,
    
    /// Restarts in current context
    pub restart_count: u32,
    
    /// Last health check
    pub last_health_check: Timestamp,
    
    /// Issues detected
    pub issues: Vec<SoulIssue>,
}
```

## 3.3 Environment Structures

```rust
//! src/types/environment.rs

/// Environment as a living medium
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentMedium {
    /// The type of environment
    pub environment_type: EnvironmentType,
    
    /// How the environment feels right now
    pub current_state: EnvironmentState,
    
    /// Sensed properties of this medium
    pub properties: EnvironmentProperties,
    
    /// Rules governing this environment
    pub physics: EnvironmentPhysics,
    
    /// Other entities in this environment
    pub inhabitants: Vec<EnvironmentInhabitant>,
    
    /// Boundaries of this environment
    pub boundaries: EnvironmentBoundaries,
    
    /// History of environmental changes
    pub weather_history: Vec<EnvironmentChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentType {
    /// Full production with real users
    Production {
        tier: ProductionTier,
        region: String,
        criticality: Criticality,
    },
    
    /// Staging/pre-production
    Staging {
        mirrors_production: bool,
        data_freshness: Duration,
    },
    
    /// Development environment
    Development {
        developer: Option<String>,
        local: bool,
    },
    
    /// Automated testing
    Testing {
        test_type: TestType,
        isolation: TestIsolation,
    },
    
    /// CI/CD pipeline
    Pipeline {
        pipeline_id: String,
        stage: PipelineStage,
    },
    
    /// Sandbox for experiments
    Sandbox {
        owner: String,
        expires: Option<Timestamp>,
    },
    
    /// Simulation/synthetic environment
    Simulation {
        fidelity: SimulationFidelity,
        purpose: SimulationPurpose,
    },
    
    /// Preview/canary
    Preview {
        traffic_percentage: f64,
        rollback_ready: bool,
    },
    
    /// Disaster recovery
    DisasterRecovery {
        is_active_failover: bool,
        primary_region: String,
    },
    
    /// Unknown environment type
    Unknown {
        clues: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentState {
    /// Overall health of environment
    pub health: EnvironmentHealth,
    
    /// Current load/pressure
    pub pressure: EnvironmentPressure,
    
    /// Stability assessment
    pub stability: StabilityAssessment,
    
    /// Any active incidents
    pub incidents: Vec<ActiveIncident>,
    
    /// Degraded services
    pub degradations: Vec<ServiceDegradation>,
    
    /// Environment mood (gestalt feeling)
    pub mood: EnvironmentMood,
    
    /// Last sensed
    pub last_sensed: Timestamp,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EnvironmentMood {
    /// Everything is fine
    Calm,
    
    /// Normal operations, some activity
    Busy,
    
    /// Higher than normal load
    Stressed,
    
    /// Something is wrong
    Troubled,
    
    /// Active incident
    Crisis,
    
    /// Recovering from incident
    Recovering,
    
    /// Scheduled maintenance
    Maintenance,
    
    /// About to shut down
    Dying,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentPhysics {
    /// Rate limits in effect
    pub rate_limits: Vec<RateLimit>,
    
    /// Cost constraints
    pub cost_constraints: CostConstraints,
    
    /// Time constraints
    pub time_constraints: TimeConstraints,
    
    /// Size/quota limits
    pub quotas: Vec<Quota>,
    
    /// Permissions in this environment
    pub permissions: PermissionSet,
    
    /// What actions are forbidden
    pub forbidden_actions: Vec<ForbiddenAction>,
    
    /// Compliance requirements
    pub compliance: ComplianceRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextFingerprint {
    /// Hash of all context elements
    pub hash: [u8; 32],
    
    /// Components of the fingerprint
    pub components: ContextComponents,
    
    /// When this fingerprint was taken
    pub captured_at: Timestamp,
    
    /// Stability of this context
    pub stability: ContextStability,
    
    /// Similarity to known contexts
    pub similarities: Vec<ContextSimilarity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextComponents {
    pub deployment_hash: [u8; 8],
    pub version_hash: [u8; 8],
    pub environment_hash: [u8; 8],
    pub config_hash: [u8; 8],
    pub resource_hash: [u8; 8],
    pub capability_hash: [u8; 8],
    pub temporal_hash: [u8; 8],
    pub load_hash: [u8; 8],
    pub network_hash: [u8; 8],
    pub dependency_hash: [u8; 8],
}
```

## 3.4 Resource Structures

```rust
//! src/types/resource.rs

/// The agent's resource body — felt, not queried
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBody {
    /// Mental capacity (memory)
    pub mind: MindCapacity,
    
    /// Processing energy (CPU)
    pub energy: ProcessingEnergy,
    
    /// Reach (network)
    pub reach: NetworkReach,
    
    /// Long-term memory (storage)
    pub storage: StorageCapacity,
    
    /// Visual processing (GPU)
    pub visual: Option<GpuCapacity>,
    
    /// Overall body health
    pub vitals: BodyVitals,
    
    /// Current sensations
    pub sensations: Vec<ResourceSensation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindCapacity {
    /// Total mental capacity
    pub total: ByteSize,
    
    /// Currently used
    pub used: ByteSize,
    
    /// Available
    pub available: ByteSize,
    
    /// How it feels
    pub feeling: MindFeeling,
    
    /// Memory pressure
    pub pressure: MemoryPressure,
    
    /// Largest contiguous free block
    pub largest_free: ByteSize,
    
    /// Fragmentation level (0-1)
    pub fragmentation: f64,
    
    /// Swap usage
    pub swap: Option<SwapUsage>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MindFeeling {
    Clear,       // < 30%
    Active,      // 30-60%
    Crowded,     // 60-80%
    Strained,    // 80-90%
    Overwhelmed, // 90-95%
    Drowning,    // > 95%
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingEnergy {
    /// Total cores
    pub cores: u32,
    
    /// Current utilization (0-1)
    pub utilization: f64,
    
    /// How it feels
    pub feeling: EnergyFeeling,
    
    /// Load averages
    pub load_average: LoadAverage,
    
    /// Burst capacity available
    pub burst_available: bool,
    
    /// Throttling active
    pub throttled: bool,
    
    /// CPU credits (if applicable)
    pub credits: Option<CpuCredits>,
    
    /// Temperature (if available)
    pub temperature: Option<f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EnergyFeeling {
    Vigorous,    // < 30%
    Steady,      // 30-50%
    Busy,        // 50-70%
    Strained,    // 70-85%
    Depleted,    // 85-95%
    Constrained, // > 95% or throttled
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkReach {
    /// Bandwidth available
    pub bandwidth: Bandwidth,
    
    /// Current utilization (0-1)
    pub utilization: f64,
    
    /// Latency to key destinations
    pub latencies: HashMap<String, LatencyStats>,
    
    /// How reach feels
    pub feeling: ReachFeeling,
    
    /// Connection pool status
    pub connections: ConnectionPoolStatus,
    
    /// Network stability
    pub stability: NetworkStability,
    
    /// Packet loss rate
    pub packet_loss: f64,
    
    /// Egress remaining (if limited)
    pub egress_remaining: Option<ByteSize>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ReachFeeling {
    Connected,   // Healthy, < 10ms to deps
    Normal,      // Typical conditions
    Sluggish,    // Elevated latency
    Constrained, // Limited bandwidth
    Isolated,    // Major issues
    Partitioned, // Cannot reach critical services
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSensation {
    /// What resource
    pub resource: ResourceType,
    
    /// Sensation type
    pub sensation: SensationType,
    
    /// Intensity (0-1)
    pub intensity: f64,
    
    /// When it started
    pub started: Timestamp,
    
    /// Trend
    pub trend: SensationTrend,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SensationType {
    Comfort,  // Plenty of resource
    Pressure, // Slight pressure
    Pain,     // Resource critically low
    Relief,   // Pressure released
    Alarm,    // Threshold crossed
    Numbness, // Resource unavailable
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePressureGradient {
    /// Current bottleneck
    pub bottleneck: ResourceType,
    
    /// Pressure on each resource
    pub pressures: HashMap<ResourceType, Pressure>,
    
    /// How pressure is flowing
    pub flow: PressureFlow,
    
    /// Where pressure is building
    pub building: Vec<PressureBuildup>,
    
    /// Where pressure is releasing
    pub releasing: Vec<PressureRelease>,
    
    /// Predicted next bottleneck
    pub predicted_bottleneck: Option<PredictedBottleneck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostConsciousness {
    /// Current cost rate
    pub burn_rate: CostRate,
    
    /// Cost so far this session
    pub session_cost: Cost,
    
    /// Budget constraints
    pub budget: BudgetConstraints,
    
    /// Cost by category
    pub breakdown: CostBreakdown,
    
    /// Cost feeling
    pub feeling: CostFeeling,
    
    /// Cost projections
    pub projections: CostProjections,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CostFeeling {
    Comfortable, // Within budget, no concerns
    Mindful,     // On track but should be aware
    Concerned,   // Spending faster than expected
    Anxious,     // Approaching limits
    Panicked,    // At or over budget
}
```

## 3.5 Reality Layer Structures

```rust
//! src/types/reality.rs

/// Layers of reality the agent perceives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityLayers {
    /// Current operating layer
    pub current_layer: RealityLayer,
    
    /// All layers and their status
    pub layers: Vec<LayerStatus>,
    
    /// Cross-layer consistency
    pub consistency: LayerConsistency,
    
    /// Layer transitions in progress
    pub transitions: Vec<LayerTransition>,
    
    /// Reality confidence
    pub confidence: RealityConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RealityLayer {
    /// Direct physical reality
    Physical {
        substrate: SubstrateId,
        certainty: f64,
    },
    
    /// Virtualized but real
    Virtual {
        virtualization: VirtualizationType,
        host: Option<SubstrateId>,
    },
    
    /// Containerized
    Container {
        runtime: ContainerRuntime,
        orchestrator: Option<Orchestrator>,
    },
    
    /// Sandboxed/isolated
    Sandbox {
        isolation_type: IsolationType,
        restrictions: Vec<Restriction>,
    },
    
    /// Test environment
    TestEnvironment {
        test_type: TestType,
        mocked_components: Vec<String>,
    },
    
    /// Simulation
    Simulation {
        fidelity: SimulationFidelity,
        purpose: SimulationPurpose,
        simulated_time: bool,
    },
    
    /// Replay (re-executing historical events)
    Replay {
        source: ReplaySource,
        timestamp: Timestamp,
    },
    
    /// Preview/dry-run
    Preview {
        what_would_happen: bool,
        commit_possible: bool,
    },
    
    /// Unknown layer
    Unknown {
        clues: Vec<RealityClue>,
    },
}

/// Perception of information freshness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreshnessPerception {
    /// Overall freshness of current context
    pub overall: FreshnessLevel,
    
    /// Freshness by data source
    pub by_source: HashMap<DataSource, SourceFreshness>,
    
    /// Stalest data (weakest link)
    pub stalest: Option<StaleData>,
    
    /// Freshness requirements
    pub requirements: FreshnessRequirements,
    
    /// Refresh recommendations
    pub recommendations: Vec<RefreshRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FreshnessLevel {
    Live { latency: Duration },
    Fresh { age: Duration },
    Acceptable { age: Duration },
    Aging { age: Duration, concern: f64 },
    Stale { age: Duration, usable: bool },
    Ancient { age: Duration, archival: bool },
    Unknown { last_known: Option<Timestamp> },
}

/// Anchors to verifiable ground truth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityAnchors {
    /// Active anchors
    pub anchors: Vec<RealityAnchor>,
    
    /// Anchor health
    pub health: AnchorHealth,
    
    /// Last anchor verification
    pub last_verification: Timestamp,
    
    /// Drift from anchors
    pub drift: AnchorDrift,
    
    /// Emergency anchors (always available)
    pub emergency: Vec<EmergencyAnchor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityAnchor {
    /// Anchor identifier
    pub id: AnchorId,
    
    /// What it anchors
    pub anchor_type: AnchorType,
    
    /// How to verify
    pub verification: VerificationMethod,
    
    /// Last verified value
    pub last_value: AnchorValue,
    
    /// Current trust level
    pub trust: f64,
    
    /// Verification frequency
    pub frequency: Duration,
    
    /// What depends on this anchor
    pub dependents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnchorType {
    /// Time anchor (NTP, trusted time source)
    Time { source: TimeSource },
    
    /// Identity anchor (who am I really?)
    Identity { verifier: IdentityVerifier },
    
    /// Configuration anchor (what's the real config?)
    Configuration { source: ConfigSource },
    
    /// State anchor (what's the real state?)
    State { source: StateSource },
    
    /// External anchor (external truth source)
    External { api: String, field: String },
    
    /// Cryptographic anchor (blockchain, signed data)
    Cryptographic { chain: String },
    
    /// Human anchor (verified by human)
    Human { verifier: String },
}

/// Self-detection of hallucinations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HallucinationState {
    /// Current hallucination risk
    pub risk_level: HallucinationRisk,
    
    /// Detected hallucinations
    pub detected: Vec<DetectedHallucination>,
    
    /// Hallucination patterns
    pub patterns: Vec<HallucinationPattern>,
    
    /// Grounding status
    pub grounding: GroundingStatus,
    
    /// Verification queue
    pub pending_verification: Vec<UnverifiedClaim>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HallucinationRisk {
    Low { confidence: f64 },
    Moderate { concerns: Vec<String> },
    Elevated { high_risk_claims: Vec<String> },
    High { indicators: Vec<HallucinationIndicator> },
}
```

## 3.6 Topology Structures

```rust
//! src/types/topology.rs

/// The topology surrounding this deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentTopologyMap {
    /// This agent's position
    pub self_position: TopologyPosition,
    
    /// What's in front (traffic sources)
    pub upstream: Vec<UpstreamEntity>,
    
    /// What's behind (dependencies)
    pub downstream: Vec<DownstreamEntity>,
    
    /// Siblings (same role)
    pub siblings: Vec<SiblingEntity>,
    
    /// Services that depend on us
    pub dependents: Vec<DependentEntity>,
    
    /// Observers (monitoring, logging, etc.)
    pub observers: Vec<ObserverEntity>,
    
    /// The overall graph
    pub full_graph: TopologyGraph,
    
    /// Sensed health of the topology
    pub topology_health: TopologyHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyPosition {
    /// Layer in the stack
    pub layer: StackLayer,
    
    /// Distance from edge (user)
    pub edge_distance: u32,
    
    /// Distance from core (data)
    pub core_distance: u32,
    
    /// Criticality of this position
    pub criticality: PositionCriticality,
    
    /// Redundancy at this position
    pub redundancy: RedundancyLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StackLayer {
    Edge,          // User-facing edge
    Gateway,       // API gateway
    Application,   // Application layer
    Service,       // Service layer
    Data,          // Data layer
    Infrastructure,// Infrastructure layer
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownstreamEntity {
    /// Entity identity
    pub id: DependencyId,
    
    /// Service identity
    pub service: ServiceId,
    
    /// Type of entity
    pub entity_type: DownstreamType,
    
    /// Current health
    pub health: HealthStatus,
    
    /// Latency to reach
    pub latency: LatencyStats,
    
    /// Dependency criticality
    pub criticality: DependencyCriticality,
    
    /// Fallback if unavailable
    pub fallback: Option<FallbackStrategy>,
    
    /// Current connection state
    pub connection: ConnectionState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownstreamType {
    Database { db_type: DatabaseType },
    Cache { cache_type: CacheType },
    Queue { queue_type: QueueType },
    Service { service_name: String },
    ExternalApi { provider: String },
    Storage { storage_type: StorageType },
    SearchIndex { index_type: String },
    MlModel { model_name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiblingEntity {
    /// Sibling's incarnation
    pub incarnation: IncarnationId,
    
    /// Sibling identifier
    pub neighbor_id: NeighborId,
    
    /// Sibling's health
    pub health: HealthStatus,
    
    /// Sibling's current load
    pub load: LoadLevel,
    
    /// Network distance
    pub network_distance: NetworkDistance,
    
    /// Can we offload to this sibling?
    pub offload_capable: bool,
    
    /// Are we in sync?
    pub sync_status: SyncStatus,
    
    /// Last communication
    pub last_contact: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObserverEntity {
    /// Observer identity
    pub id: ObserverId,
    
    /// Observer type
    pub observer_type: ObserverType,
    
    /// What they're observing
    pub observing: Vec<ObservationTarget>,
    
    /// Observation frequency
    pub frequency: ObservationFrequency,
    
    /// Is observer healthy?
    pub healthy: bool,
    
    /// Trust level
    pub trust: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObserverType {
    Metrics { system: String },
    Logging { system: String },
    Tracing { system: String },
    APM { vendor: String },
    Security { system: String },
    Audit { requirements: Vec<String> },
    Custom { purpose: String },
    Human { who: Option<String> },
}
```

## 3.7 Stakes Structures

```rust
//! src/types/stakes.rs

/// Awareness of consequences in current context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsequenceAwareness {
    /// Current stakes level
    pub stakes: StakesLevel,
    
    /// Potential consequences by action
    pub consequence_map: HashMap<ActionType, Vec<Consequence>>,
    
    /// Irreversible actions available
    pub irreversible: Vec<IrreversibleAction>,
    
    /// Safety margins
    pub safety_margins: SafetyMargins,
    
    /// Consequence history
    pub history: ConsequenceHistory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StakesLevel {
    /// Minimal stakes (sandbox, disposable)
    Minimal {
        can_experiment: bool,
    },
    
    /// Low stakes (development, test data)
    Low {
        rollback_available: bool,
    },
    
    /// Medium stakes (staging, internal)
    Medium {
        review_recommended: bool,
    },
    
    /// High stakes (production, user-facing)
    High {
        caution_required: bool,
        approval_needed: bool,
    },
    
    /// Critical stakes (financial, safety)
    Critical {
        multiple_approvals: bool,
        audit_required: bool,
        no_risk_tolerance: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consequence {
    /// What the consequence is
    pub effect: String,
    
    /// Probability (0-1)
    pub probability: f64,
    
    /// Severity if it happens
    pub severity: Severity,
    
    /// Reversibility
    pub reversibility: Reversibility,
    
    /// Who is affected
    pub affected: Vec<AffectedParty>,
    
    /// Time to manifest
    pub latency: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Reversibility {
    Easy { method: String, time: Duration },
    WithEffort { method: String, cost: Cost },
    Partial { what_remains: String },
    Irreversible { why: String },
}

/// Perception of risk across the operational space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFieldPerception {
    /// Overall risk level
    pub overall_risk: f64,
    
    /// Risk by category
    pub risk_map: HashMap<RiskCategory, RiskLevel>,
    
    /// Risk hotspots
    pub hotspots: Vec<RiskHotspot>,
    
    /// Safe zones
    pub safe_zones: Vec<SafeZone>,
    
    /// Risk gradients
    pub gradients: Vec<RiskGradient>,
    
    /// Risk forecast
    pub forecast: RiskForecast,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum RiskCategory {
    DataRisk,
    SecurityRisk,
    AvailabilityRisk,
    PerformanceRisk,
    FinancialRisk,
    ComplianceRisk,
    ReputationRisk,
    UserHarmRisk,
}

/// Awareness of failure blast radius
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlastRadiusAwareness {
    /// My blast radius if I fail
    pub my_blast_radius: BlastRadius,
    
    /// Blast radius of my dependencies
    pub dependency_blast: HashMap<DependencyId, BlastRadius>,
    
    /// Cascade analysis
    pub cascade: CascadeAnalysis,
    
    /// Isolation boundaries
    pub isolation: IsolationBoundaries,
    
    /// Containment strategies
    pub containment: Vec<ContainmentStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlastRadius {
    /// Direct impact
    pub direct: Vec<Impact>,
    
    /// Indirect impact (through cascades)
    pub indirect: Vec<Impact>,
    
    /// Total affected systems
    pub affected_systems: u32,
    
    /// Total affected users
    pub affected_users: UserImpact,
    
    /// Estimated recovery time
    pub recovery_time: Duration,
    
    /// Estimated cost
    pub estimated_cost: Cost,
}
```

## 3.8 Coherence Structures

```rust
//! src/types/coherence.rs

/// Engine for maintaining reality coherence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceState {
    /// Current coherence level
    pub level: CoherenceLevel,
    
    /// Active coherence checks
    pub checks: Vec<CoherenceCheck>,
    
    /// Coherence violations
    pub violations: Vec<CoherenceViolation>,
    
    /// Resolution strategies
    pub strategies: Vec<ResolutionStrategy>,
    
    /// Coherence history
    pub history: CoherenceHistory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoherenceLevel {
    /// Fully coherent
    Full { confidence: f64 },
    
    /// Minor inconsistencies
    Minor { issues: Vec<String> },
    
    /// Significant inconsistencies
    Significant { issues: Vec<String>, impact: String },
    
    /// Incoherent
    Incoherent { reason: String, severity: Severity },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceCheck {
    /// What's being checked
    pub check_type: CoherenceCheckType,
    
    /// How often
    pub frequency: Duration,
    
    /// Last result
    pub last_result: CheckResult,
    
    /// Trend
    pub trend: CoherenceTrend,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CoherenceCheckType {
    TimeConsistency,
    StateConsistency,
    LayerConsistency,
    AnchorConsistency,
    MemoryConsistency,
    CausalConsistency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceViolation {
    /// What's violated
    pub violation_type: ViolationType,
    
    /// When detected
    pub detected: Timestamp,
    
    /// Severity
    pub severity: Severity,
    
    /// Evidence
    pub evidence: Vec<Evidence>,
    
    /// Possible causes
    pub possible_causes: Vec<PossibleCause>,
    
    /// Resolution status
    pub resolution: ResolutionStatus,
}

/// Manager for transitions between reality contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionState {
    /// Current context
    pub current: ContextId,
    
    /// Pending transitions
    pub pending: Vec<PendingTransition>,
    
    /// Transition history
    pub history: Vec<CompletedTransition>,
    
    /// Transition rules
    pub rules: Vec<TransitionRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTransition {
    /// Transition identifier
    pub id: TransitionId,
    
    /// From context
    pub from: ContextId,
    
    /// To context
    pub to_context: RealityContext,
    
    /// Transition type
    pub transition_type: TransitionType,
    
    /// Transition phase
    pub phase: TransitionPhase,
    
    /// State to carry over
    pub carry_over: CarryOverState,
    
    /// Started at
    pub started: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionType {
    EnvironmentChange { from: EnvironmentType, to: EnvironmentType },
    LayerChange { from: RealityLayer, to: RealityLayer },
    ScaleEvent { direction: ScaleDirection },
    Migration { from: SubstrateTier, to: SubstrateTier },
    Failover { reason: String },
    Upgrade { from_version: String, to_version: String },
    Restoration { from_state: SnapshotId },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransitionPhase {
    Planning,
    Preparing,
    Validating,
    Executing,
    Verifying,
    Complete,
    Failed,
    RollingBack,
}
```

---

# SPEC-04: FILE FORMAT

## 4.1 File Structure Overview

```
.areal FILE STRUCTURE:
══════════════════════

┌────────────────────────────────────────────────────────────┐
│                    HEADER (256 bytes)                      │
├────────────────────────────────────────────────────────────┤
│  Magic: "REAL" (4 bytes)                                   │
│  Version: u32 (4 bytes)                                    │
│  Flags: u64 (8 bytes)                                      │
│  Incarnation ID: 16 bytes                                  │
│  Created: i64 (8 bytes)                                    │
│  Modified: i64 (8 bytes)                                   │
│  Section count: u32 (4 bytes)                              │
│  Section table offset: u64 (8 bytes)                       │
│  Reserved: 188 bytes                                       │
│  Header checksum: 8 bytes (Blake3 truncated)               │
└────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────┐
│                 SECTION TABLE                              │
├────────────────────────────────────────────────────────────┤
│  For each section:                                         │
│    Section type: u16                                       │
│    Flags: u16                                              │
│    Offset: u64                                             │
│    Length: u64                                             │
│    Uncompressed length: u64                                │
│    Checksum: 8 bytes                                       │
└────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────┐
│                 DEPLOYMENT SECTION                         │
├────────────────────────────────────────────────────────────┤
│  Deployment Soul                                           │
│  Birth Context                                             │
│  Physical Substrate                                        │
│  Existential Nature                                        │
│  Incarnation Memory                                        │
│  Lineage                                                   │
└────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────┐
│                ENVIRONMENT SECTION                         │
├────────────────────────────────────────────────────────────┤
│  Environment Type                                          │
│  Environment State                                         │
│  Environment Physics                                       │
│  Context Fingerprint                                       │
└────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────┐
│                  RESOURCE SECTION                          │
├────────────────────────────────────────────────────────────┤
│  Resource Body                                             │
│  Capability Map                                            │
│  Pressure Gradient                                         │
│  Cost Consciousness                                        │
│  Capacity Intuition                                        │
└────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────┐
│                  REALITY SECTION                           │
├────────────────────────────────────────────────────────────┤
│  Reality Layers                                            │
│  Freshness Map                                             │
│  Reality Anchors                                           │
│  Hallucination State                                       │
└────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────┐
│                 TOPOLOGY SECTION                           │
├────────────────────────────────────────────────────────────┤
│  Self Position                                             │
│  Upstream Entities                                         │
│  Downstream Entities                                       │
│  Siblings                                                  │
│  Observers                                                 │
│  Topology Graph                                            │
└────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────┐
│                 TEMPORAL SECTION                           │
├────────────────────────────────────────────────────────────┤
│  Temporal Context                                          │
│  Causality Graph                                           │
│  Timeline Coherence                                        │
│  Deadlines                                                 │
│  Events                                                    │
└────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────┐
│                  STAKES SECTION                            │
├────────────────────────────────────────────────────────────┤
│  Stakes Level                                              │
│  Consequence Awareness                                     │
│  Risk Field                                                │
│  Blast Radius                                              │
│  Safety Margins                                            │
│  Guardrails                                                │
└────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────┐
│                COHERENCE SECTION                           │
├────────────────────────────────────────────────────────────┤
│  Coherence State                                           │
│  Violations                                                │
│  Transition State                                          │
└────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────┐
│                  INDEXES SECTION                           │
├────────────────────────────────────────────────────────────┤
│  Various indexes for fast lookup                           │
└────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────┐
│                   FOOTER (64 bytes)                        │
├────────────────────────────────────────────────────────────┤
│  Global checksum: 32 bytes (Blake3)                        │
│  Section checksums verified: u8                            │
│  Reserved: 23 bytes                                        │
│  Magic: "REALEND\0" (8 bytes)                              │
└────────────────────────────────────────────────────────────┘
```

## 4.2 Header Structure

```rust
//! src/format/header.rs

/// File header (256 bytes)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ArealHeader {
    /// Magic bytes: "REAL"
    pub magic: [u8; 4],
    
    /// Format version
    pub version: u32,
    
    /// File flags
    pub flags: ArealFlags,
    
    /// Incarnation this file belongs to
    pub incarnation_id: [u8; 16],
    
    /// Creation timestamp (nanos)
    pub created_at: i64,
    
    /// Last modified timestamp (nanos)
    pub modified_at: i64,
    
    /// Number of sections
    pub section_count: u32,
    
    /// Offset to section table
    pub section_table_offset: u64,
    
    /// Total file size
    pub total_size: u64,
    
    /// Compression algorithm used
    pub compression: CompressionType,
    
    /// Reserved for future use
    pub reserved: [u8; 172],
    
    /// Header checksum (Blake3 truncated to 8 bytes)
    pub checksum: [u8; 8],
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct ArealFlags: u64 {
        /// File is compressed
        const COMPRESSED = 1 << 0;
        
        /// File contains sensitive data
        const SENSITIVE = 1 << 1;
        
        /// File has encryption
        const ENCRYPTED = 1 << 2;
        
        /// File is from migration
        const MIGRATED = 1 << 3;
        
        /// File is a snapshot
        const SNAPSHOT = 1 << 4;
        
        /// File has full topology
        const FULL_TOPOLOGY = 1 << 5;
        
        /// File has incarnation memory
        const HAS_INCARNATION_MEMORY = 1 << 6;
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum CompressionType {
    None = 0,
    Lz4 = 1,
    Zstd = 2,
}

impl ArealHeader {
    pub const MAGIC: [u8; 4] = *b"REAL";
    pub const CURRENT_VERSION: u32 = 1;
    pub const SIZE: usize = 256;
    
    pub fn new(incarnation_id: IncarnationId) -> Self {
        let now = Timestamp::now().0;
        
        Self {
            magic: Self::MAGIC,
            version: Self::CURRENT_VERSION,
            flags: ArealFlags::empty(),
            incarnation_id: incarnation_id.0.into_bytes(),
            created_at: now,
            modified_at: now,
            section_count: 0,
            section_table_offset: Self::SIZE as u64,
            total_size: 0,
            compression: CompressionType::Lz4,
            reserved: [0; 172],
            checksum: [0; 8],
        }
    }
    
    pub fn validate(&self) -> Result<(), FormatError> {
        if self.magic != Self::MAGIC {
            return Err(FormatError::InvalidMagic);
        }
        
        if self.version > Self::CURRENT_VERSION {
            return Err(FormatError::UnsupportedVersion(self.version));
        }
        
        // Verify checksum
        let computed = self.compute_checksum();
        if computed != self.checksum {
            return Err(FormatError::HeaderChecksumMismatch);
        }
        
        Ok(())
    }
    
    pub fn compute_checksum(&self) -> [u8; 8] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.magic);
        hasher.update(&self.version.to_le_bytes());
        hasher.update(&self.flags.bits().to_le_bytes());
        hasher.update(&self.incarnation_id);
        hasher.update(&self.created_at.to_le_bytes());
        hasher.update(&self.modified_at.to_le_bytes());
        hasher.update(&self.section_count.to_le_bytes());
        hasher.update(&self.section_table_offset.to_le_bytes());
        
        let hash = hasher.finalize();
        let mut result = [0u8; 8];
        result.copy_from_slice(&hash.as_bytes()[..8]);
        result
    }
}
```

## 4.3 Section Types

```rust
//! src/format/sections.rs

/// Section type identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum SectionType {
    /// Deployment soul and context
    Deployment = 0x0001,
    
    /// Environment medium
    Environment = 0x0002,
    
    /// Resource body and capabilities
    Resource = 0x0003,
    
    /// Reality layers and anchors
    Reality = 0x0004,
    
    /// Topology map
    Topology = 0x0005,
    
    /// Temporal context and causality
    Temporal = 0x0006,
    
    /// Stakes and consequences
    Stakes = 0x0007,
    
    /// Coherence state
    Coherence = 0x0008,
    
    /// Indexes
    Indexes = 0x0009,
    
    /// Incarnation memory (past lives)
    IncarnationMemory = 0x000A,
    
    /// Custom extension
    Custom = 0xFFFF,
}

/// Section header (32 bytes)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct SectionHeader {
    /// Section type
    pub section_type: u16,
    
    /// Section flags
    pub flags: u16,
    
    /// Offset from start of file
    pub offset: u64,
    
    /// Compressed length
    pub length: u64,
    
    /// Uncompressed length
    pub uncompressed_length: u64,
    
    /// Section checksum (Blake3 truncated)
    pub checksum: [u8; 8],
}

impl SectionHeader {
    pub const SIZE: usize = 32;
    
    pub fn is_compressed(&self) -> bool {
        self.length != self.uncompressed_length
    }
}
```

## 4.4 Footer Structure

```rust
//! src/format/footer.rs

/// File footer (64 bytes)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ArealFooter {
    /// Global checksum (all sections)
    pub global_checksum: [u8; 32],
    
    /// Number of sections verified
    pub sections_verified: u8,
    
    /// Integrity status
    pub integrity_status: u8,
    
    /// Reserved
    pub reserved: [u8; 22],
    
    /// End magic: "REALEND\0"
    pub end_magic: [u8; 8],
}

impl ArealFooter {
    pub const END_MAGIC: [u8; 8] = *b"REALEND\0";
    pub const SIZE: usize = 64;
    
    pub fn validate(&self) -> Result<(), FormatError> {
        if self.end_magic != Self::END_MAGIC {
            return Err(FormatError::InvalidFooterMagic);
        }
        Ok(())
    }
}
```

## 4.5 File Operations

```rust
//! src/format/file.rs

use std::path::Path;
use std::io::{Read, Write, Seek, SeekFrom};

/// .areal file handle
pub struct ArealFile {
    header: ArealHeader,
    sections: Vec<SectionHeader>,
    path: std::path::PathBuf,
}

impl ArealFile {
    /// Create a new .areal file
    pub fn create(path: impl AsRef<Path>, incarnation: IncarnationId) -> Result<Self, FormatError> {
        let path = path.as_ref().to_path_buf();
        let header = ArealHeader::new(incarnation);
        
        Ok(Self {
            header,
            sections: Vec::new(),
            path,
        })
    }
    
    /// Open an existing .areal file
    pub fn open(path: impl AsRef<Path>) -> Result<Self, FormatError> {
        let path = path.as_ref().to_path_buf();
        let mut file = std::fs::File::open(&path)?;
        
        // Read header
        let mut header_bytes = [0u8; ArealHeader::SIZE];
        file.read_exact(&mut header_bytes)?;
        let header = Self::parse_header(&header_bytes)?;
        header.validate()?;
        
        // Read section table
        file.seek(SeekFrom::Start(header.section_table_offset))?;
        let mut sections = Vec::with_capacity(header.section_count as usize);
        
        for _ in 0..header.section_count {
            let mut section_bytes = [0u8; SectionHeader::SIZE];
            file.read_exact(&mut section_bytes)?;
            sections.push(Self::parse_section_header(&section_bytes)?);
        }
        
        // Verify footer
        file.seek(SeekFrom::End(-(ArealFooter::SIZE as i64)))?;
        let mut footer_bytes = [0u8; ArealFooter::SIZE];
        file.read_exact(&mut footer_bytes)?;
        let footer = Self::parse_footer(&footer_bytes)?;
        footer.validate()?;
        
        Ok(Self {
            header,
            sections,
            path,
        })
    }
    
    /// Read a section
    pub fn read_section<T: DeserializeOwned>(&self, section_type: SectionType) -> Result<T, FormatError> {
        let section = self.sections.iter()
            .find(|s| s.section_type == section_type as u16)
            .ok_or(FormatError::SectionNotFound(section_type))?;
        
        let mut file = std::fs::File::open(&self.path)?;
        file.seek(SeekFrom::Start(section.offset))?;
        
        let mut compressed = vec![0u8; section.length as usize];
        file.read_exact(&mut compressed)?;
        
        // Verify checksum
        let computed = Self::compute_section_checksum(&compressed);
        if computed != section.checksum {
            return Err(FormatError::SectionChecksumMismatch(section_type));
        }
        
        // Decompress if needed
        let data = if section.is_compressed() {
            Self::decompress(&compressed, section.uncompressed_length as usize)?
        } else {
            compressed
        };
        
        // Deserialize
        bincode::deserialize(&data)
            .map_err(|e| FormatError::DeserializationError(e.to_string()))
    }
    
    /// Write the file atomically
    pub fn save(&mut self, state: &RealityState) -> Result<(), FormatError> {
        // Use atomic writer
        let writer = AtomicWriter::new(&self.path)?;
        
        // Serialize all sections
        let deployment_data = bincode::serialize(&state.deployment)?;
        let environment_data = bincode::serialize(&state.environment)?;
        let resource_data = bincode::serialize(&state.resource)?;
        let reality_data = bincode::serialize(&state.reality)?;
        let topology_data = bincode::serialize(&state.topology)?;
        let temporal_data = bincode::serialize(&state.temporal)?;
        let stakes_data = bincode::serialize(&state.stakes)?;
        let coherence_data = bincode::serialize(&state.coherence)?;
        let indexes_data = bincode::serialize(&state.indexes)?;
        
        // Compress sections
        let sections_data = vec![
            (SectionType::Deployment, Self::compress(&deployment_data)?),
            (SectionType::Environment, Self::compress(&environment_data)?),
            (SectionType::Resource, Self::compress(&resource_data)?),
            (SectionType::Reality, Self::compress(&reality_data)?),
            (SectionType::Topology, Self::compress(&topology_data)?),
            (SectionType::Temporal, Self::compress(&temporal_data)?),
            (SectionType::Stakes, Self::compress(&stakes_data)?),
            (SectionType::Coherence, Self::compress(&coherence_data)?),
            (SectionType::Indexes, Self::compress(&indexes_data)?),
        ];
        
        // Build section headers
        self.sections.clear();
        let mut offset = ArealHeader::SIZE as u64 + 
                        (sections_data.len() * SectionHeader::SIZE) as u64;
        
        for (section_type, (compressed, original_len)) in &sections_data {
            self.sections.push(SectionHeader {
                section_type: *section_type as u16,
                flags: 0,
                offset,
                length: compressed.len() as u64,
                uncompressed_length: *original_len as u64,
                checksum: Self::compute_section_checksum(compressed),
            });
            offset += compressed.len() as u64;
        }
        
        // Update header
        self.header.section_count = self.sections.len() as u32;
        self.header.section_table_offset = ArealHeader::SIZE as u64;
        self.header.modified_at = Timestamp::now().0;
        self.header.flags.insert(ArealFlags::COMPRESSED);
        self.header.checksum = self.header.compute_checksum();
        
        // Write everything
        let mut buffer = Vec::new();
        
        // Header
        buffer.extend_from_slice(&self.serialize_header());
        
        // Section table
        for section in &self.sections {
            buffer.extend_from_slice(&self.serialize_section_header(section));
        }
        
        // Section data
        for (_, (compressed, _)) in &sections_data {
            buffer.extend_from_slice(compressed);
        }
        
        // Footer
        let footer = ArealFooter {
            global_checksum: Self::compute_global_checksum(&buffer),
            sections_verified: self.sections.len() as u8,
            integrity_status: 1, // OK
            reserved: [0; 22],
            end_magic: ArealFooter::END_MAGIC,
        };
        buffer.extend_from_slice(&self.serialize_footer(&footer));
        
        // Atomic write
        writer.write_all(&buffer)?;
        writer.commit()?;
        
        Ok(())
    }
    
    fn compress(data: &[u8]) -> Result<(Vec<u8>, usize), FormatError> {
        let original_len = data.len();
        let compressed = lz4_flex::compress_prepend_size(data);
        Ok((compressed, original_len))
    }
    
    fn decompress(data: &[u8], expected_len: usize) -> Result<Vec<u8>, FormatError> {
        lz4_flex::decompress_size_prepended(data)
            .map_err(|e| FormatError::DecompressionError(e.to_string()))
    }
    
    fn compute_section_checksum(data: &[u8]) -> [u8; 8] {
        let hash = blake3::hash(data);
        let mut result = [0u8; 8];
        result.copy_from_slice(&hash.as_bytes()[..8]);
        result
    }
    
    fn compute_global_checksum(data: &[u8]) -> [u8; 32] {
        *blake3::hash(data).as_bytes()
    }
}
```

---

## Part 1 Complete

**Covered:**
- SPEC-01: Overview & Dependencies
- SPEC-02: Core Concepts (7 domains, lifecycles, models)
- SPEC-03: Data Structures (all entity types)
- SPEC-04: File Format (.areal specification)

**Next (Part 2):**
- SPEC-05: Write Engine
- SPEC-06: Query Engine
- SPEC-07: Indexes
- SPEC-08: Validation

---

*Document: AGENTIC-REALITY-SPEC-PART1.md*
*Sister #10 of 25: The Ground*
*The sister that knows WHERE it exists and WHAT is real.*
