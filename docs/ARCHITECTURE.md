# AgenticReality Architecture

> Sister #10 of 25 -- The Ground

This document describes the architecture of AgenticReality, which provides AI
agents with existential grounding across seven reality domains.

## Overview

AgenticReality follows the Agentra sister workspace pattern: four crates
organized around a central engine, exposed through MCP and CLI interfaces.

```
                    ┌───────────────────────────────┐
                    │        MCP SERVER              │
                    │       (15 tools)               │
                    └──────────────┬────────────────┘
                                   │
                    ┌──────────────▼────────────────┐
                    │       REALITY ENGINE           │
                    │  ┌─────────┐  ┌──────────┐   │
                    │  │  Write  │  │  Query   │   │
                    │  │ Engine  │  │  Engine  │   │
                    │  └────┬────┘  └────┬─────┘   │
                    │       │            │          │
                    │  ┌────▼────────────▼─────┐   │
                    │  │    Domain Stores       │   │
                    │  │  (7 stores + indexes)  │   │
                    │  └───────────┬────────────┘   │
                    │              │                 │
                    │  ┌───────────▼────────────┐   │
                    │  │   .areal Persistence   │   │
                    │  └────────────────────────┘   │
                    │                               │
                    │  ┌────────────────────────┐   │
                    │  │   Sister Bridges (9)   │   │
                    │  └────────────────────────┘   │
                    └───────────────────────────────┘
```

## Four Crates

### agentic-reality (core)

The core library containing all types, engines, storage, validation, inventions,
and bridge traits. This is the only crate that other sisters depend on.

**Responsibilities:**
- All data types (deployment, environment, resource, reality, topology, temporal, stakes, coherence)
- Write engine (~90 operations across 7 domains)
- Query engine (~78 operations)
- Domain stores (7 stores + index layer)
- Binary .areal format (persistence layer)
- Validation module (strict, no silent fallbacks)
- 26 invention modules
- 9 bridge trait definitions with NoOp defaults
- Security (auth, authz, encryption, audit)

### agentic-reality-mcp

The MCP server exposing 15 tools over JSON-RPC. Handles stdio and HTTP
transports. Zero unwraps in production code paths.

**Responsibilities:**
- 15 MCP tool definitions with JSON Schema input schemas
- Tool dispatch routing
- Parameter validation (MCP-level, before engine calls)
- Result serialization
- Transport handling (stdio, HTTP, SSE)
- Resource and prompt definitions
- Error mapping (tool errors vs protocol errors)

### agentic-reality-cli

The `areal` CLI binary with ~40 commands organized into 12 subcommand groups.

**Responsibilities:**
- Command parsing and routing
- Human-readable output formatting
- Interactive and non-interactive modes
- JSON output mode for scripting
- Shell completion generation
- MCP server launch (`areal serve`)

### agentic-reality-ffi

C-compatible FFI bindings for integration with non-Rust languages.

**Responsibilities:**
- C ABI-stable function exports
- Opaque handle management
- JSON string parameter passing
- Error code returns
- Thread safety guarantees

## Seven Reality Domains

Each domain is a first-class concept with its own store, write operations, and
query operations.

### 1. Deployment Consciousness

*"Where do I exist?"*

Manages the deployment soul, birth context, physical substrate, incarnation
memory, and deployment lineage. The soul is the agent's existential identity in
its current incarnation.

**Key types:** `DeploymentSoul`, `BirthContext`, `PhysicalSubstrate`,
`ExistentialNature`, `IncarnationMemory`

**Store:** `DeploymentStore`

**Write ops:** 12 (initialize_soul, update_substrate, record_birth, update_vitals,
record_death, add_past_life, update_lineage, set_role, set_nature,
update_cardinality, record_wisdom, update_karma)

**Query ops:** 10 (get_soul, get_vitals, get_lineage, get_past_lives,
get_wisdom, get_nature, get_cardinality, get_karma, get_birth,
get_soul_summary)

### 2. Resource Proprioception

*"What do I have?"*

The agent's resource body, felt as sensations rather than queried as metrics.
Memory is "mind," CPU is "energy," network is "reach." Includes pressure
gradients, capability discovery, cost consciousness, and capacity intuition.

**Key types:** `ResourceBody`, `MindCapacity`, `ProcessingEnergy`,
`NetworkReach`, `ResourcePressureGradient`, `CostConsciousness`,
`CapacityIntuition`

**Store:** `ResourceStore`

**Write ops:** 14 (sense_resources, update_mind, update_energy, update_reach,
update_storage, update_visual, add_sensation, clear_sensation,
update_pressure_gradient, discover_capability, lose_capability, update_cost,
update_capacity_intuition, set_budget)

**Query ops:** 14 (get_body, get_mind, get_energy, get_reach, get_storage,
get_visual, get_sensations, get_pressure_gradient, get_bottleneck,
get_capabilities, get_cost, get_capacity, can_do, get_body_summary)

### 3. Reality Physics

*"What is real?"*

Manages reality layers (physical through hallucinated), data freshness tracking,
truth anchors, and hallucination detection. Each layer has a trust level that
degrades from physical (0.99) to hallucinated (0.00).

**Key types:** `RealityLayers`, `RealityLayer`, `FreshnessPerception`,
`RealityAnchor`, `HallucinationState`

**Store:** `RealityStore`

**Write ops:** 12 (set_reality_layer, update_layer_status, update_freshness,
add_anchor, remove_anchor, verify_anchor, record_anchor_drift,
detect_hallucination, clear_hallucination, add_unverified_claim, verify_claim,
update_grounding)

**Query ops:** 12 (get_layer, get_layers, get_freshness, get_anchors,
get_anchor, get_anchor_drift, get_hallucinations, get_claims,
get_grounding_status, is_grounded, get_layer_trust, get_reality_summary)

### 4. Topology Awareness

*"What surrounds me?"*

The agent's perception of its position in the deployment topology: upstream
sources, downstream dependencies, sibling replicas, dependent services, and
observers (monitoring, logging).

**Key types:** `DeploymentTopologyMap`, `TopologyPosition`, `UpstreamEntity`,
`DownstreamEntity`, `SiblingEntity`, `ObserverEntity`

**Store:** `TopologyStore`

**Write ops:** 14 (set_position, add_upstream, remove_upstream, add_downstream,
remove_downstream, update_downstream_health, add_sibling, remove_sibling,
update_sibling_state, add_observer, remove_observer, update_topology_health,
record_mesh_event, update_graph)

**Query ops:** 12 (get_position, get_upstream, get_downstream, get_siblings,
get_observers, get_topology_health, get_critical_deps, get_failing_deps,
get_sibling_for_offload, get_mesh_events, get_dependents, get_topology_summary)

### 5. Temporal Grounding

*"When am I?"*

Time context, causality tracking, and timeline coherence. Goes beyond clock
time to include business time (end-of-quarter, holiday season), user time
(their timezone, their business hours), and system time (deployment age,
uptime).

**Key types:** `TemporalContext`, `CausalityGraph`, `TimelineCoherence`,
`Deadline`, `Timeline`

**Store:** `TemporalStore`

**Write ops:** 10 (ground_time, update_temporal_context, add_causal_event,
link_causality, add_deadline, remove_deadline, update_deadline, add_timeline,
record_timeline_event, resolve_timeline_conflict)

**Query ops:** 10 (get_temporal_context, get_deadlines, get_nearest_deadline,
get_causality_chain, get_timeline, get_timelines, has_deadline_pressure,
get_business_context, get_user_time, get_temporal_summary)

### 6. Stakes Perception

*"What are the consequences?"*

Stakes level assessment, risk field perception, and blast radius awareness.
The agent feels the weight of consequences, from trivial (test environment)
to critical (financial transactions in production).

**Key types:** `StakesLevel`, `RiskFieldPerception`, `BlastRadiusAwareness`,
`Consequence`, `Guardrail`

**Store:** `StakesStore`

**Write ops:** 10 (set_stakes_level, add_consequence, remove_consequence,
add_irreversible_action, update_safety_margins, add_guardrail,
remove_guardrail, update_risk_field, update_blast_radius, record_consequence)

**Query ops:** 8 (get_stakes_level, get_consequences, get_risk_field,
get_blast_radius, get_guardrails, should_proceed, get_safety_margins,
get_stakes_summary)

### 7. Coherence Maintenance

*"How do I stay grounded?"*

Cross-domain coherence checking and context transition management. Detects
contradictions (e.g., "production" environment but "expendable" nature),
drift, and ensures smooth transitions between contexts.

**Key types:** `CoherenceCheck`, `CoherenceViolation`, `ContextTransition`,
`TransitionPhase`

**Store:** `CoherenceStore`

**Write ops:** 8 (run_coherence_check, record_violation, resolve_violation,
begin_transition, advance_transition, complete_transition, abort_transition,
rollback_transition)

**Query ops:** 8 (get_coherence_level, get_violations, get_active_transition,
get_transition_history, is_coherent, get_contradictions, get_drift_report,
get_coherence_summary)

## Engine Architecture

### Write Engine

The write engine handles all mutations to reality state. Each write operation:

1. Validates input (strict, no silent fallbacks)
2. Applies mutation to the appropriate domain store
3. Updates cross-domain indexes
4. Marks state as dirty
5. Optionally logs to WAL

Write operations return `Result<T>` with descriptive error variants. There
are ~90 write operations across all 7 domains.

### Query Engine

The query engine provides read-only access to reality state. Queries are
non-blocking and use shared references. There are ~78 query operations.

Computed queries (like `should_proceed` or `has_context_shifted`) combine
data from multiple domains.

### Storage Layer

Each domain has a dedicated store that owns its data:

| Store | Contents |
|---|---|
| `DeploymentStore` | Soul, incarnation memory, lineage, wisdom |
| `EnvironmentStore` | Environment medium, fingerprint, weather history |
| `ResourceStore` | Resource body, capabilities, cost, capacity |
| `RealityStore` | Layers, anchors, freshness, hallucinations |
| `TopologyStore` | Position, upstream, downstream, siblings, observers |
| `TemporalStore` | Time context, causality, timelines, deadlines |
| `StakesStore` | Stakes level, consequences, risk, guardrails |
| `CoherenceStore` | Coherence checks, violations, transitions |

### Index Layer

`RealityIndexes` provides fast cross-domain lookups:

- Critical dependencies by service ID
- Anchors by type
- Past lives by incarnation ID
- Incidents by severity
- Deadlines by proximity
- Violations by domain

### Persistence

The `.areal` binary format stores the complete reality state:

```
┌──────────────────────────────────────┐
│  Header (magic, version, checksum)   │
├──────────────────────────────────────┤
│  Section: Deployment                 │
│  Section: Environment                │
│  Section: Resources                  │
│  Section: Reality                    │
│  Section: Topology                   │
│  Section: Temporal                   │
│  Section: Stakes                     │
│  Section: Coherence                  │
│  Section: Indexes                    │
├──────────────────────────────────────┤
│  Footer (section count, total size)  │
└──────────────────────────────────────┘
```

Each section is individually compressed (LZ4 by default) and can be
independently loaded for partial reads.

## Security Architecture

Five security layers protect the reality state:

1. **Authentication** - Token-based auth for MCP server mode (`AGENTIC_AUTH_TOKEN`)
2. **Authorization** - Operation-level permissions, read/write separation
3. **Input validation** - Strict validation, no silent fallbacks
4. **Data protection** - Optional AES-256-GCM encryption, memory zeroization
5. **Audit logging** - Timestamped, immutable audit trail

See [PRIVACY.md](./PRIVACY.md) for data handling details.

## Sister Bridge Architecture

Nine bridge traits enable integration with other Agentra sisters:

| Bridge | Sister | Purpose |
|---|---|---|
| `TimeBridge` | AgenticTime | Temporal grounding, deadlines |
| `ContractBridge` | AgenticContract | Stakes constraints, SLAs |
| `IdentityBridge` | AgenticIdentity | Incarnation verification |
| `MemoryBridge` | AgenticMemory | Incarnation memory persistence |
| `CognitionBridge` | AgenticCognition | Risk perception modeling |
| `CommBridge` | AgenticComm | Neighbor communication |
| `CodebaseBridge` | AgenticCodebase | Code context awareness |
| `VisionBridge` | AgenticVision | Visual environment sensing |
| `HydraBridge` | Hydra | Orchestrator integration |

All bridges have NoOp default implementations, allowing AgenticReality to
operate standalone without any sister dependencies.

## Performance Targets

| Category | Operation | Target |
|---|---|---|
| Sensing | sense_environment | < 50ms |
| Sensing | sense_resources | < 100ms |
| Query | get_soul | < 100us |
| Query | should_proceed | < 10ms |
| Write | add_anchor | < 500us |
| Persistence | save (typical) | < 50ms |
| Persistence | load (typical) | < 20ms |
| MCP | tool_call (simple) | < 5ms |
| Memory | base engine | < 10MB |

## Diagrams

- [Architecture Overview](./assets/architecture.svg)
- [Reality Domains](./assets/reality-domains.svg)
- [Resource Body](./assets/resource-body.svg)
- [Context Fingerprint](./assets/context-fingerprint.svg)

---

*Part of the AgenticOS ecosystem by Agentra Labs*
