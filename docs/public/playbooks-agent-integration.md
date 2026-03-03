---
status: stable
---

# Playbooks: Agent Integration

This document provides practical integration playbooks for using AgenticReality as the grounding layer in production agent systems.

## Playbook 1: Reliability-First Assistant

Use AgenticReality to ensure assistants reason from verified runtime state before replying.

1. Configure MCP and initialize deployment state with `reality_deployment`.
2. Sense environment and resources using `reality_environment` and `reality_resource`.
3. Before high-impact actions, check `reality_stakes` and `reality_coherence`.
4. After execution, record outcomes and refresh context via `reality_context`.

## Playbook 2: Incident-Aware Operations Agent

Attach runtime incidents directly to agent decisions so degraded environments do not look healthy.

1. Stream incident and degradation signals into environment sensing.
2. Query `reality_environment` mood and incident state before each critical tool call.
3. Escalate to safer operating modes when pressure crosses thresholds.
4. Maintain post-incident audit trails through reality timeline and coherence records.

## Playbook 3: Deadline and Causality Agent

Use temporal grounding to reduce stale plans and broken sequencing under load.

1. Register deadlines and causal events with `reality_temporal`.
2. Query timeline state before producing recommendations.
3. Link decisions to causes/effects for later forensic analysis.
4. Reject execution when causality or deadline checks fail.

## Playbook 4: High-Stakes Policy Gate

Apply explicit risk policy before irreversible operations.

1. Set stakes level and consequence catalog with `reality_stakes`.
2. Mark irreversible actions and required safeguards.
3. Require approvals when stakes policy demands escalation.
4. Persist all gate outcomes for reproducibility and review.

## Playbook 5: Multi-Agent Grounding Mesh

Run multiple agents against shared grounded state without losing per-project isolation.

1. Use per-project workspace IDs and isolated storage paths.
2. Enforce lock-safe writes and deterministic read/write ordering.
3. Share verified anchors and coherence signals across cooperating agents.
4. Keep portability by exporting and importing `.areal` artifacts across environments.

