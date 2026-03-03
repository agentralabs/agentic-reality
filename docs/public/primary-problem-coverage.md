---
status: stable
---

# Primary Problem Coverage

AgenticReality targets four primary failure classes that break production agent behavior.

## P1: Ungrounded Runtime Identity

Without explicit deployment identity, agents cannot reliably answer where they are running, under which authority, or with what continuity guarantees. AgenticReality resolves this with deployment soul state, birth context, lineage memory, and persistent identity continuity in `.areal` artifacts.

## P2: Context-Free Execution Under Resource and Environment Pressure

Most systems execute logic without a live model of resource and environment stress. AgenticReality models memory pressure, compute and I/O limits, environmental incidents, and operational mood so decisions are constrained by the real runtime body.

## P3: Temporal and Causal Blind Spots

Failures often arise when deadlines, causality, and timeline branches are not represented explicitly. AgenticReality introduces temporal grounding, deadline tracking, causal graphs, and unified timelines to maintain sequence integrity and support post-incident reconstruction.

## P4: Uncalibrated Risk in High-Stakes Operations

Systems frequently apply uniform policies across low and high consequence operations. AgenticReality enforces stakes levels, guardrails, irreversibility declarations, and consequence catalogs so risk posture is explicit and enforceable before action.

## Coverage Principle

Each primary problem is covered by multiple engine layers (deployment, environment, resource, temporal, stakes, coherence) and is surfaced through MCP tools so enforcement is not documentation-only but runtime-verifiable.

## Coverage Matrix (Problem -> Runtime Surface)

1. Ungrounded Runtime Identity
Mapped surfaces: deployment store, soul metadata, lineage tracking, `.areal` persistence, `reality_deployment`.

2. Context-Free Execution
Mapped surfaces: resource body sensing, environment medium state, incident log, `reality_resource`, `reality_environment`.

3. Temporal and Causal Blind Spots
Mapped surfaces: temporal awareness store, deadline registry, causality graph, timeline merge view, `reality_temporal`.

4. Uncalibrated High-Stakes Actions
Mapped surfaces: stakes level model, consequence catalog, irreversible action registry, `reality_stakes`, `reality_coherence`.

## Operational Readiness Signals

Primary-problem coverage is considered active only when all of the following are true:

- MCP tools can query each primary domain without panics under malformed inputs.
- Stress tests exercise edge paths related to each problem class.
- Runtime hardening checks pass without silent fallback behavior.
- Public docs are in sync with the executable command surface.

## Failure-Mode Framing

These primary problems were chosen because they are high-cost failure multipliers:

- Identity ambiguity causes policy and accountability drift.
- Missing resource/environment context causes unsafe planning.
- Temporal blindness causes stale or contradictory actions.
- Risk flattening causes irreversible damage from routine operations.

AgenticReality reduces these multipliers by making each one explicit, testable, and stateful across restarts.
