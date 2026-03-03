# AgenticReality Core Concepts

> Foundational concepts for understanding existential grounding

## The Reality Vacuum

AI agents today operate in a reality vacuum. They receive requests and produce
responses without any awareness of:

- **Where** they are running (laptop, cloud, edge, browser?)
- **What** resources they have (memory, CPU, network, budget?)
- **Which** world they are in (production, staging, test, simulation?)
- **What** constrains them (rate limits, costs, permissions, quotas?)
- **What is real** versus cached, stale, simulated, or hallucinated
- **What surrounds them** (databases, siblings, monitors?)
- **What the stakes are** (test data or financial transactions?)
- **When** they are (3am server time, end-of-quarter, holiday?)

This blindness leads to identical behavior across wildly different contexts.
A production financial API should behave nothing like a test sandbox, yet
without reality awareness, both receive the same treatment.

## The Deployment Soul

The deployment soul is the agent's existential identity in its current
incarnation. It is not configuration -- it is who the agent IS.

A human does not "read" that they have two arms. They ARE a being with two
arms. Similarly, an agent should not "read" that it is on server-7. It should
BE the entity incarnated on server-7.

The soul encompasses:

- **Incarnation ID** -- a unique identifier for this specific instance of
  existence. When the agent restarts, it gets a new incarnation. The old one
  is a past life.

- **Birth context** -- how this instance came to be. Was it a fresh deployment
  (virgin)? Scaled out from a parent? Resurrected after a crash? Migrated from
  a different substrate? Forked from another instance? Created for a specific
  ephemeral task?

- **Physical substrate** -- the hardware reality hosting this soul. This
  ranges from laptop to mobile to browser to edge to cloud to bare metal to
  GPU cluster to HPC partition.

- **Existential nature** -- fundamental properties of this instance's
  existence. Is it a singleton or one of many? How expendable is it? Does it
  persist across restarts? Is it stateful or stateless? Can it be cloned? Is
  it the primary or a replica?

- **Vitals** -- current health, uptime, restart count, last health check,
  active issues. Like a heartbeat for the soul.

## The Environment Medium

The environment is not dead configuration. It is a living medium the agent
exists within, like water for a fish.

The environment has:

- **Type** -- production (with tier and criticality), staging, development,
  testing, CI/CD pipeline, sandbox, simulation, demo, disaster recovery. Each
  type carries implicit behavioral expectations.

- **State** -- the environment's current condition: health, pressure,
  stability, active incidents, degradations.

- **Mood** -- a gestalt assessment of how the environment "feels": calm, busy,
  stressed, troubled, crisis, recovering, maintenance, dying. Mood captures
  nuance that individual metrics miss.

- **Physics** -- the rules governing this environment: rate limits, cost
  constraints, time constraints, quotas, permissions, forbidden actions,
  compliance requirements.

- **Weather** -- a history of environmental changes, like weather patterns
  that help predict future conditions.

## The Resource Body

The agent has a body of resources it can feel, like proprioception for
computation. Resources are sensations, not metrics.

### Mind (Memory)

Memory is felt as mental capacity. When memory is low, the agent feels
"crowded" or "strained." When memory is abundant, it feels "clear." This
is not a number on a dashboard -- it is a felt sense of cognitive capacity.

Feelings: Clear (< 30%), Active (30-60%), Crowded (60-80%), Strained (80-90%),
Overwhelmed (90-95%), Drowning (> 95%).

### Energy (CPU)

CPU is felt as processing energy. When the CPU is underutilized, the agent
feels "vigorous." When it is maxed out, it feels "depleted."

Feelings: Vigorous (< 30%), Steady (30-50%), Busy (50-70%), Strained (70-85%),
Depleted (85-95%), Constrained (> 95%).

### Reach (Network)

Network is felt as reach -- the agent's ability to communicate with the world.
Good connectivity is "connected." Network issues feel like "isolation."

Feelings: Connected (excellent), Normal (typical), Sluggish (elevated latency),
Constrained (bandwidth limited), Isolated (major issues), Partitioned (cannot
reach critical services).

### Sensations

Resources generate sensations: comfort, pressure, pain, relief, alarm,
numbness. These sensations have intensity (0-1) and trend (improving, stable,
worsening). The agent can feel the transition from comfort to pressure to pain.

### Pressure Gradient

All resources are connected through a pressure gradient. When one resource is
constrained, it becomes the bottleneck. The gradient shows where pressure is
flowing, building, and releasing. The agent can identify not just what is
failing, but why the whole system feels the way it does.

## Reality Layers

Reality is not monolithic. There are layers, each with different trust levels:

1. **Physical** (trust: 0.99) -- Hardware reality. Servers, network cables,
   power supply. Hardware rarely lies, but it can fail.

2. **Virtual** (trust: 0.95) -- VMs, containers. Real but abstracted. The
   hypervisor is generally reliable, but live migration can cause brief
   inconsistencies.

3. **Logical** (trust: 0.85) -- Services, APIs, configurations. Changes
   frequently. Configs can be wrong. API contracts can drift.

4. **Cached** (trust: 0.70 * freshness) -- Stored representations of reality.
   May be stale. Trust degrades with age. A 5-second-old cache is nearly as
   good as live data; a 5-hour-old cache may be misleading.

5. **Predicted** (trust: 0.50 * confidence) -- Forecasts, estimates,
   projections. Inherently uncertain. Must carry confidence bounds.

6. **Simulated** (trust: 0.30) -- Test doubles, mocks, synthetic environments.
   Useful for testing but not real. An agent should know when it is operating
   in simulated reality.

7. **Hallucinated** (trust: 0.00) -- Agent's false beliefs, errors,
   confabulations. By definition false. Must be detected and corrected.

An agent should always know which layer it is operating in and adjust its
confidence accordingly.

## Reality Anchors

Reality anchors are verifiable reference points that ground the agent in
truth. Like a ship at anchor, they prevent drift.

Types of anchors:
- **Time anchors** -- verified against NTP or authoritative time sources
- **Configuration anchors** -- verified against authoritative config sources
- **State anchors** -- verified against authoritative state (database, API)
- **Heartbeat anchors** -- verified by periodic health checks
- **Checksum anchors** -- verified by data integrity checks

Each anchor has a trust level that can drift over time. When an anchor drifts
beyond its threshold, the agent knows its understanding of reality may be
inaccurate.

## The Context Fingerprint

The context fingerprint is a hash of the entire operational context. It
captures deployment, version, environment, configuration, resources,
capabilities, temporal state, load, network, and dependencies.

The fingerprint enables:
- **Shift detection** -- "something changed, I should re-evaluate"
- **Drift tracking** -- "the context is gradually changing"
- **Stability assessment** -- "this is a volatile context, stay cautious"
- **Similarity matching** -- "this context resembles one I operated in before"

## Stakes Perception

Stakes are not a configuration parameter. They are a felt sense of
consequence weight. An agent in a test sandbox feels lightness -- actions
are inconsequential. An agent processing production financial transactions
feels gravity -- every action carries weight.

Stakes levels carry behavioral implications:
- **Trivial** -- experiment freely, no audit needed
- **Low** -- standard care, basic logging
- **Medium** -- verify before acting, structured logging
- **High** -- multiple checks, full audit trail, caution
- **Critical** -- multiple approvals, zero risk tolerance, complete audit

Blast radius awareness quantifies who would be affected if something goes
wrong: how many users, which systems, what data, what revenue impact, how
long to recover.

## Coherence Maintenance

With seven reality domains generating independent perceptions, contradictions
can arise. The coherence engine cross-checks all domains:

- Production environment but expendable nature? Contradiction.
- Healthy network but all dependencies failing? Inconsistency.
- Critical stakes but sandbox environment? Mismatch.
- Time anchor says 3pm but temporal context says midnight? Drift.

Coherence levels:
- **Full** -- all domains consistent, all anchors verified
- **Partial** -- minor inconsistencies, some anchors drifting
- **Degraded** -- significant contradictions, reduced confidence
- **Incoherent** -- major contradictions, emergency re-grounding needed

## Context Transitions

Transitions are the dangerous moments when reality changes rapidly: scaling
events, migrations, failovers, deployments, configuration changes. During
these moments, the agent's perceptions may be temporarily invalid.

The context transition manager ensures:
1. Pre-transition state is captured
2. Transition is tracked through phases
3. Post-transition state is verified
4. New baseline is established
5. Rollback is available if verification fails

Without managed transitions, an agent can lose grounding: acting on stale
topology during a migration, maintaining incorrect stakes during a failover,
or missing new resources after a scale event.

## Design Principles

1. **Sense, do not query** -- Resources should be felt, not polled.
   Environment should be perceived, not read. Like proprioception, not like
   reading a gauge.

2. **Ground to truth** -- Always maintain anchors to verifiable reality.
   Detect drift from truth. Know when you are uncertain.

3. **Context is identity** -- The deployment context shapes the agent's
   identity. A production agent IS different from a test agent. Not
   configuration -- incarnation.

4. **Coherence over completeness** -- Better to have coherent partial
   knowledge than incoherent complete knowledge. Maintain consistency across
   all perceptions.

5. **Stakes awareness** -- Always know the consequences. High stakes leads
   to cautious behavior. Low stakes allows experimentation.

6. **Graceful degradation** -- Can operate with partial reality awareness.
   More awareness means better decisions. Minimum viable reality requires
   deployment plus environment.

---

*Part of the AgenticOS ecosystem by Agentra Labs*
