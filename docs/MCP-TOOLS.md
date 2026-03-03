# AgenticReality MCP Tools

> 15 MCP tools for reality grounding

All tools follow the MCP quality standard: verb-first imperative descriptions,
no trailing periods, `isError: true` for tool execution errors, and error code
`-32803` for unknown tools.

## Tool Index

| # | Tool | Domain | Description |
|---|---|---|---|
| 1 | `reality_deployment` | Deployment | Manage the deployment soul and incarnation identity |
| 2 | `reality_memory` | Deployment | Access incarnation memory across past lives |
| 3 | `reality_environment` | Environment | Sense and manage the operational environment |
| 4 | `reality_resource` | Resource | Sense and query the agent's resource body |
| 5 | `reality_capability` | Resource | Discover and check available capabilities |
| 6 | `reality_layer` | Reality | Manage reality layers and trust levels |
| 7 | `reality_anchor` | Reality | Manage reality anchors for truth grounding |
| 8 | `reality_hallucination` | Reality | Detect and manage hallucination state |
| 9 | `reality_topology` | Topology | Map and manage the deployment topology |
| 10 | `reality_temporal` | Temporal | Access temporal grounding and deadlines |
| 11 | `reality_stakes` | Stakes | Assess stakes and check action safety |
| 12 | `reality_coherence` | Coherence | Run coherence checks and manage transitions |
| 13 | `reality_context` | Cross-domain | Access the context fingerprint |
| 14 | `reality_ground` | Cross-domain | High-level grounding operations |
| 15 | `reality_workspace` | Workspace | Manage the .areal workspace |

---

## Tool 1: reality_deployment

**Description:** Manage the deployment soul and incarnation identity

### Operations

#### initialize

Create the deployment soul. Typically the first operation called.

```json
{
  "operation": "initialize",
  "spawned_by": "kubernetes",
  "purpose": "API server for payment processing",
  "expected_lifetime_seconds": 86400,
  "circumstances": "virgin"
}
```

**Response:**

```json
{
  "incarnation_id": "7b2f4a8e-9c3d-4e5f-a6b7-c8d9e0f1a2b3",
  "substrate": {
    "tier": "Cloud",
    "provider": "AWS",
    "instance_type": "m5.xlarge",
    "region": "us-east-1"
  },
  "birth": {
    "spawned_at": "2024-01-15T10:00:00Z",
    "spawned_by": "kubernetes",
    "purpose": "API server for payment processing",
    "circumstances": "Virgin"
  }
}
```

#### get_soul

Retrieve the current deployment soul.

```json
{ "operation": "get_soul" }
```

#### get_vitals

Retrieve current vitals (health, uptime, restart count).

```json
{ "operation": "get_vitals" }
```

#### get_summary

Retrieve a complete deployment summary.

```json
{ "operation": "get_summary" }
```

#### update_vitals

Update the soul's vitals.

```json
{
  "operation": "update_vitals",
  "health": 0.95,
  "issues": ["elevated_latency"]
}
```

#### set_nature

Set existential nature properties.

```json
{
  "operation": "set_nature",
  "cardinality": "replica_of",
  "total_replicas": 3,
  "replica_index": 1,
  "expendability": 0.33,
  "persistence": "ephemeral",
  "clonability": true
}
```

---

## Tool 2: reality_memory

**Description:** Access incarnation memory across past lives

### Operations

#### get_past_lives

List all past incarnations.

```json
{ "operation": "get_past_lives" }
```

#### get_wisdom

Retrieve accumulated wisdom from past lives.

```json
{ "operation": "get_wisdom" }
```

#### add_wisdom

Record a wisdom entry.

```json
{
  "operation": "add_wisdom",
  "category": "what_works",
  "content": "Batch database writes in groups of 100 for optimal throughput",
  "context": "postgres on m5.xlarge"
}
```

#### get_karma

Retrieve karmic balance (debts, credits, inherited problems).

```json
{ "operation": "get_karma" }
```

---

## Tool 3: reality_environment

**Description:** Sense and manage the operational environment

### Operations

#### sense

Perform environment sensing. Auto-detects environment type, state, and physics.

```json
{ "operation": "sense" }
```

**Response:**

```json
{
  "environment_type": "Production",
  "tier": "tier-1",
  "region": "us-east-1",
  "mood": "Calm",
  "health": "Healthy",
  "pressure": "Normal",
  "incidents": []
}
```

#### get

Retrieve the current environment state.

```json
{ "operation": "get" }
```

#### get_mood

Retrieve the current environment mood.

```json
{ "operation": "get_mood" }
```

#### set_mood

Set the environment mood with a cause.

```json
{
  "operation": "set_mood",
  "mood": "stressed",
  "cause": "Elevated error rate on payment endpoint"
}
```

#### record_incident

Record an active incident.

```json
{
  "operation": "record_incident",
  "description": "Database connection pool exhaustion",
  "severity": "high",
  "affected_services": ["postgres", "api"]
}
```

#### clear_incident

Clear a resolved incident.

```json
{
  "operation": "clear_incident",
  "incident_id": "inc-001"
}
```

#### get_physics

Retrieve environment physics (rate limits, quotas, constraints).

```json
{ "operation": "get_physics" }
```

---

## Tool 4: reality_resource

**Description:** Sense and query the agent's resource body

### Operations

#### sense

Perform resource sensing. Probes memory, CPU, network, storage, GPU.

```json
{ "operation": "sense" }
```

**Response:**

```json
{
  "mind": {
    "total_bytes": 17179869184,
    "used_bytes": 6710886400,
    "feeling": "Active",
    "pressure": "Normal"
  },
  "energy": {
    "cores": 4,
    "utilization": 0.28,
    "feeling": "Steady"
  },
  "reach": {
    "feeling": "Connected",
    "stability": "Stable"
  },
  "storage": {
    "total_bytes": 536870912000,
    "used_bytes": 128849018880,
    "feeling": "Clear"
  },
  "bottleneck": null
}
```

#### get_body

Retrieve the full resource body.

```json
{ "operation": "get_body" }
```

#### get_mind

Retrieve memory (mind) state only.

```json
{ "operation": "get_mind" }
```

#### get_pressure

Retrieve the resource pressure gradient.

```json
{ "operation": "get_pressure" }
```

#### get_bottleneck

Identify the current resource bottleneck.

```json
{ "operation": "get_bottleneck" }
```

#### get_cost

Retrieve cost consciousness state.

```json
{ "operation": "get_cost" }
```

---

## Tool 5: reality_capability

**Description:** Discover and check available capabilities

### Operations

#### discover

Discover all capabilities in the current context.

```json
{ "operation": "discover" }
```

#### list

List all discovered capabilities.

```json
{ "operation": "list" }
```

#### check

Check if a specific capability is available.

```json
{
  "operation": "check",
  "capability": "gpu-compute"
}
```

**Response:**

```json
{
  "available": false,
  "reason": "No GPU detected",
  "alternatives": ["cpu-parallel"]
}
```

#### missing

List capabilities that are expected but missing.

```json
{ "operation": "missing" }
```

---

## Tool 6: reality_layer

**Description:** Manage reality layers and trust levels

### Operations

#### get

Retrieve the current reality layer and its trust level.

```json
{ "operation": "get" }
```

#### list

List all reality layers with their status and trust levels.

```json
{ "operation": "list" }
```

**Response:**

```json
{
  "layers": [
    { "layer": "Physical", "trust": 0.99, "status": "Active" },
    { "layer": "Virtual", "trust": 0.95, "status": "Active" },
    { "layer": "Logical", "trust": 0.85, "status": "Active" },
    { "layer": "Cached", "trust": 0.63, "status": "Stale" },
    { "layer": "Predicted", "trust": 0.45, "status": "Low confidence" },
    { "layer": "Simulated", "trust": 0.30, "status": "Inactive" },
    { "layer": "Hallucinated", "trust": 0.00, "status": "None detected" }
  ],
  "current": "Virtual"
}
```

#### set

Set the current operating reality layer.

```json
{
  "operation": "set",
  "layer": "virtual"
}
```

---

## Tool 7: reality_anchor

**Description:** Manage reality anchors for truth grounding

### Operations

#### add

Add a new reality anchor.

```json
{
  "operation": "add",
  "anchor_type": "time",
  "source": "ntp.org",
  "frequency_seconds": 300
}
```

#### list

List all anchors.

```json
{ "operation": "list" }
```

#### get

Get a specific anchor by ID.

```json
{
  "operation": "get",
  "anchor_id": "anc-001"
}
```

#### verify

Verify a specific anchor.

```json
{
  "operation": "verify",
  "anchor_id": "anc-001"
}
```

#### verify_all

Verify all anchors.

```json
{ "operation": "verify_all" }
```

**Response:**

```json
[
  { "anchor_id": "anc-001", "verified": true, "drift": "12ms", "trust": 0.99 },
  { "anchor_id": "anc-002", "verified": true, "drift": "0", "trust": 0.95 },
  { "anchor_id": "anc-003", "verified": false, "drift": "47 rows", "trust": 0.72 }
]
```

#### get_drift

Get drift report across all anchors.

```json
{ "operation": "get_drift" }
```

#### remove

Remove an anchor.

```json
{
  "operation": "remove",
  "anchor_id": "anc-001"
}
```

---

## Tool 8: reality_hallucination

**Description:** Detect and manage hallucination state

### Operations

#### detect

Record a detected hallucination.

```json
{
  "operation": "detect",
  "description": "Claimed database has 10M rows but actual count is 8.7M",
  "source": "response_to_user",
  "severity": "medium"
}
```

#### list

List all detected hallucinations.

```json
{ "operation": "list" }
```

#### clear

Clear a resolved hallucination.

```json
{
  "operation": "clear",
  "hallucination_id": "hal-001"
}
```

#### add_claim

Add an unverified claim for tracking.

```json
{
  "operation": "add_claim",
  "claim": "Response time is under 100ms",
  "source": "self"
}
```

#### verify_claim

Mark a claim as verified or rejected.

```json
{
  "operation": "verify_claim",
  "claim_id": "clm-001",
  "verified": true
}
```

---

## Tool 9: reality_topology

**Description:** Map and manage the deployment topology

### Operations

#### get_map

Retrieve the full topology map.

```json
{ "operation": "get_map" }
```

#### add_downstream

Add a downstream dependency.

```json
{
  "operation": "add_downstream",
  "service": "postgres",
  "type": "database",
  "criticality": "critical"
}
```

#### add_sibling

Add a sibling replica.

```json
{
  "operation": "add_sibling",
  "sibling_id": "replica-2",
  "health": "healthy",
  "load": "normal"
}
```

#### add_observer

Add an observer (monitoring system).

```json
{
  "operation": "add_observer",
  "name": "prometheus",
  "type": "monitoring"
}
```

#### get_health

Get topology health report.

```json
{ "operation": "get_health" }
```

#### get_critical_deps

List critical dependencies.

```json
{ "operation": "get_critical_deps" }
```

#### get_failing

List failing or degraded dependencies.

```json
{ "operation": "get_failing" }
```

---

## Tool 10: reality_temporal

**Description:** Access temporal grounding and deadlines

### Operations

#### get_context

Retrieve the full temporal context (system time, business time, user time).

```json
{ "operation": "get_context" }
```

#### add_deadline

Add a deadline.

```json
{
  "operation": "add_deadline",
  "description": "API response SLA",
  "deadline": "2024-01-15T10:30:00Z",
  "severity": "high"
}
```

#### get_deadlines

List all deadlines ordered by proximity.

```json
{ "operation": "get_deadlines" }
```

#### has_pressure

Check if there is deadline pressure.

```json
{ "operation": "has_pressure" }
```

#### get_business_context

Get business time context (end-of-quarter, holiday, etc.).

```json
{ "operation": "get_business_context" }
```

---

## Tool 11: reality_stakes

**Description:** Assess stakes and check action safety

### Operations

#### set_level

Set the stakes level.

```json
{
  "operation": "set_level",
  "level": "high"
}
```

#### get_level

Get the current stakes level.

```json
{ "operation": "get_level" }
```

#### should_proceed

Check whether an action should proceed given current stakes.

```json
{
  "operation": "should_proceed",
  "action": "deploy_new_version",
  "risk_tolerance": 0.5
}
```

**Response:**

```json
{
  "ProceedWithCaution": {
    "conditions": [
      "Verify rollback plan exists",
      "Notify on-call engineer",
      "Monitor error rate for 15 minutes post-deploy"
    ],
    "blast_radius": "50,000 users",
    "stakes_level": "High"
  }
}
```

#### get_risk

Get the risk field perception.

```json
{ "operation": "get_risk" }
```

#### get_blast_radius

Get blast radius awareness.

```json
{ "operation": "get_blast_radius" }
```

#### add_guardrail

Add a guardrail.

```json
{
  "operation": "add_guardrail",
  "description": "Never delete production data without backup verification",
  "enforced": true
}
```

#### list_guardrails

List all guardrails.

```json
{ "operation": "list_guardrails" }
```

---

## Tool 12: reality_coherence

**Description:** Run coherence checks and manage transitions

### Operations

#### check

Run a coherence check across all domains.

```json
{ "operation": "check" }
```

**Response:**

```json
{
  "level": "Full",
  "violations": [],
  "contradictions": [],
  "drift": {
    "deployment_environment": 0.0,
    "resource_topology": 0.02,
    "stakes_coherence": 0.0
  }
}
```

#### get_level

Get the current coherence level.

```json
{ "operation": "get_level" }
```

#### get_violations

List coherence violations.

```json
{ "operation": "get_violations" }
```

#### begin_transition

Begin a context transition (e.g., scaling, migrating).

```json
{
  "operation": "begin_transition",
  "transition_type": "scale_out",
  "from": "3 replicas",
  "to": "5 replicas"
}
```

#### advance_transition

Advance to the next transition phase.

```json
{
  "operation": "advance_transition",
  "phase": "executing"
}
```

#### complete_transition

Complete the active transition.

```json
{ "operation": "complete_transition" }
```

---

## Tool 13: reality_context

**Description:** Access the context fingerprint

### Operations

#### get_fingerprint

Get the current context fingerprint.

```json
{ "operation": "get_fingerprint" }
```

**Response:**

```json
{
  "hash": "a7f3b2c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1",
  "stability": {
    "volatility": 0.15,
    "volatile_components": ["load", "network"],
    "expected_stability_seconds": 3600
  },
  "captured_at": "2024-01-15T10:00:00Z"
}
```

#### has_shifted

Check if the context has shifted beyond a threshold.

```json
{
  "operation": "has_shifted",
  "threshold": 0.3
}
```

**Response:**

```json
{
  "shifted": false,
  "distance": 0.12,
  "changed_components": ["load_hash"],
  "severity": "Minor"
}
```

#### diff

Show what changed since the last fingerprint.

```json
{ "operation": "diff" }
```

---

## Tool 14: reality_ground

**Description:** High-level grounding operations

### Operations

#### full_sense

Perform full reality sensing (deployment + environment + resources + topology).

```json
{ "operation": "full_sense" }
```

#### grounding_status

Get the overall grounding status.

```json
{ "operation": "grounding_status" }
```

**Response:**

```json
{
  "grounded": true,
  "anchors_verified": 3,
  "anchors_drifting": 0,
  "coherence_level": "Full",
  "hallucinations": 0,
  "context_stable": true,
  "stakes_level": "High"
}
```

#### reality_check

Perform a complete reality check (verify anchors, run coherence, check freshness).

```json
{ "operation": "reality_check" }
```

---

## Tool 15: reality_workspace

**Description:** Manage the .areal workspace

### Operations

#### init

Initialize a new workspace.

```json
{
  "operation": "init",
  "path": "/opt/myapp"
}
```

#### status

Get workspace status.

```json
{ "operation": "status" }
```

#### save

Save current state to .areal file.

```json
{ "operation": "save" }
```

#### load

Load state from .areal file.

```json
{
  "operation": "load",
  "path": "/opt/myapp/.areal"
}
```

#### export

Export state as JSON.

```json
{
  "operation": "export",
  "format": "json"
}
```

---

## Error Handling

### Tool Execution Errors

Returned with `isError: true` in the MCP response:

```json
{
  "content": [
    {
      "type": "text",
      "text": "Error: No deployment soul initialized. Call reality_deployment with operation 'initialize' first."
    }
  ],
  "isError": true
}
```

### Protocol Errors

Returned as JSON-RPC errors:

| Code | Meaning |
|---|---|
| `-32700` | Parse error |
| `-32600` | Invalid request |
| `-32601` | Method not found |
| `-32602` | Invalid params |
| `-32803` | Tool not found (TOOL_NOT_FOUND) |

### Common Error Conditions

| Error | Cause | Resolution |
|---|---|---|
| NoDeploymentSoul | Soul not initialized | Call `reality_deployment` with `initialize` |
| NoEnvironment | Environment not sensed | Call `reality_environment` with `sense` |
| NoResourceBody | Resources not sensed | Call `reality_resource` with `sense` |
| InvalidOperation | Unknown operation name | Check supported operations |
| AnchorNotFound | Anchor ID does not exist | Verify anchor ID |
| TransitionInProgress | Cannot start new transition | Complete or abort current transition |

---

*Part of the AgenticOS ecosystem by Agentra Labs*
