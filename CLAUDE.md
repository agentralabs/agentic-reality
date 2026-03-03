# AgenticReality -- Claude Code Instructions

> Sister #10 of 25 (The Ground)
> Format: .areal
> CLI binary: areal
> Version: 0.1.0

## Workspace Structure

Four-crate workspace following the Agentra sister pattern:

```
crates/
  agentic-reality/          Core library (types, engines, storage, validation, inventions, bridges)
  agentic-reality-mcp/      MCP server (15 tools, stdio/HTTP/SSE)
  agentic-reality-cli/      CLI binary (areal, ~40 commands)
  agentic-reality-ffi/      FFI bindings (C ABI)
```

## Commit Style

- Use conventional commit prefixes: `feat:`, `fix:`, `chore:`, `docs:`, `test:`, `refactor:`
- Never add "Co-Authored-By: Claude" to commits
- Keep commit messages concise and descriptive

## MCP Quality Standard

All MCP tools must comply with the Agentra MCP quality standard:

- **Tool descriptions:** verb-first imperative, no trailing periods
  - Good: "Manage the deployment soul and incarnation identity"
  - Bad: "This tool manages the deployment soul."
- **Error handling:** tool execution errors use `isError: true`; protocol errors use JSON-RPC error codes
- **Unknown tool:** error code `-32803` (TOOL_NOT_FOUND), never `-32601` or `-32602`
- **Zero unwraps:** no `.unwrap()` or `.expect()` in MCP crate production code
- **Strict validation:** no silent fallbacks (`unwrap_or_default`), always return explicit errors

## Key Metrics

| Metric | Target |
|---|---|
| MCP Tools | 15 |
| Inventions | 26 |
| CLI Commands | ~40 |
| Tests | 250+ |
| Bridge Traits | 9 |
| Doc Pages | 12 |
| SVG Diagrams | 4 |
| CI Guardrail Sections | 50 |
| Unwraps in MCP | 0 (mandatory) |

## Guardrails

Before pushing, run:

```bash
bash scripts/check-canonical-consistency.sh
bash scripts/check-command-surface.sh
bash scripts/check-mcp-consolidation.sh
```

## Test Organization

Tests follow the phase-based incremental pattern:

```
tests/
  phase01_deployment.rs     Deployment soul operations
  phase02_environment.rs    Environment sensing
  phase03_resource.rs       Resource proprioception
  phase04_reality.rs        Reality layers, anchors, hallucination
  phase05_topology.rs       Topology mapping
  phase06_temporal.rs       Temporal grounding
  phase07_stakes.rs         Stakes perception
  phase08_coherence.rs      Coherence maintenance
  mcp_tool_count.rs         Verify exactly 15 MCP tools
```

## Seven Reality Domains

1. **Deployment Consciousness** -- Where do I exist? (soul, environment, incarnation memory, fingerprint, topology map)
2. **Resource Proprioception** -- What do I have? (body, capabilities, pressure, cost, capacity)
3. **Reality Physics** -- What is real? (layers, freshness, anchors, hallucination)
4. **Topology Awareness** -- What surrounds me? (mesh, neighbors, dependencies, observers)
5. **Temporal Grounding** -- When am I? (time context, causality, timeline coherence)
6. **Stakes Perception** -- What are the consequences? (stakes level, risk field, blast radius)
7. **Coherence Maintenance** -- How do I stay grounded? (coherence engine, transition manager)

## Sister Bridge Traits

9 bridge traits with NoOp defaults for standalone operation:

| Bridge | Sister | Purpose |
|---|---|---|
| TimeBridge | AgenticTime | Temporal grounding |
| ContractBridge | AgenticContract | Stakes constraints |
| IdentityBridge | AgenticIdentity | Incarnation verification |
| MemoryBridge | AgenticMemory | Incarnation persistence |
| CognitionBridge | AgenticCognition | Risk modeling |
| CommBridge | AgenticComm | Neighbor communication |
| CodebaseBridge | AgenticCodebase | Code context |
| VisionBridge | AgenticVision | Visual sensing |
| HydraBridge | Hydra | Orchestrator integration |

## Performance Targets

- Query operations: < 100 microseconds
- Sensing operations: < 100ms
- MCP tool calls: < 5ms (simple), < 50ms (complex)
- File save/load: < 50ms typical
- Base memory: < 10MB

## File Format

Binary `.areal` format with:
- Magic bytes + version header
- Per-section LZ4 compression
- BLAKE3 integrity checksums
- Sections: deployment, environment, resources, reality, topology, temporal, stakes, coherence, indexes
- Optional AES-256-GCM encryption

## Dependencies

Required sisters (with NoOp fallback):
- AgenticTime >= 0.1.0
- AgenticContract >= 0.1.0
- AgenticIdentity >= 0.3.0

Workspace references agentic-sdk v0.2.0.

## Planning Documents

Specifications are in `planning-docs/`:
- `CLAUDE-CODE-INSTRUCTIONS-REALITY.md` -- Build instructions and quality benchmarks
- `AGENTIC-REALITY-26-INVENTIONS.md` -- All 26 inventions detailed
- `AGENTIC-REALITY-SPEC-PART1.md` through `SPEC-PART5.md` -- Complete specifications
