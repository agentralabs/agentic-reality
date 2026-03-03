---
status: stable
---

# Initial Problem Coverage

This document captures the original operational failures AgenticReality was built to prevent.

## Problem: Agents operate without verified deployment context

Solved by deployment grounding primitives that require explicit soul identity, substrate metadata, deployment role, and lineage before high-impact operations are executed.

## Problem: Runtime decisions ignore real resource pressure

Solved by continuous resource proprioception for CPU, memory, I/O, and pressure-state modeling, so planning and execution are constrained by real system limits.

## Problem: Environment changes happen without situational awareness

Solved by environment sensing and incident tracking that represent health, pressure, degradation events, and inhabitant changes as first-class runtime state.

## Problem: Time-sensitive commitments are not anchored

Solved by temporal grounding with deadlines, causality links, and timeline models that preserve ordering and help detect stale assumptions.

## Problem: High-stakes actions are treated like low-stakes actions

Solved by stake classification, consequence modeling, irreversibility tracking, and guardrails that escalate approval and caution requirements.

## Problem: Hallucination risk is not connected to operational state

Solved by reality anchors, confidence-aware hallucination logging, and coherence checks that expose when claimed state diverges from verified state.

## Initial Scope Boundaries

The initial scope focused on runtime grounding primitives that can be verified in production:

1. Identity and deployment continuity
2. Resource and environment pressure awareness
3. Temporal and causality traceability
4. Stakes and guardrail enforcement
5. Coherence and hallucination drift detection

Out-of-scope for initial delivery were advanced policy DSL authoring, multi-region sync orchestration, and cross-sister orchestration automation. Those are layered on top of core grounding once baseline reliability is proven.

## Validation Strategy for Initial Problems

Initial problem claims are validated by executable evidence, not narrative claims:

- Stress and edge-case test suites for failure paths and boundary conditions
- Runtime hardening checks for MCP interface behavior
- Canonical guardrails that enforce sister parity and required public docs
- Benchmarks for real measured operation costs and throughput patterns

## Why This Matters

Without this initial problem coverage, agent systems may appear functional while silently drifting away from reliable operational behavior. AgenticReality's initial scope closes that gap by forcing core runtime context into explicit, inspectable state.
