# Changelog

All notable changes to AgenticReality will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-01-15

### Added

#### Core (agentic-reality)

- 7 reality domains: deployment consciousness, resource proprioception, reality
  physics, topology awareness, temporal grounding, stakes perception, coherence
  maintenance
- Write engine with ~90 operations across all domains
- Query engine with ~78 operations across all domains
- 7 domain stores: DeploymentStore, EnvironmentStore, ResourceStore,
  RealityStore, TopologyStore, TemporalStore, StakesStore, CoherenceStore
- Cross-domain index layer (RealityIndexes)
- Binary .areal file format with per-section LZ4 compression and BLAKE3
  integrity checksums
- Strict validation module with no silent fallbacks
- 26 capability modules across 7 tiers
- 9 sister bridge traits (Time, Contract, Identity, Memory, Cognition, Comm,
  Codebase, Vision, Hydra) with NoOp default implementations
- Security: token-based authentication, operation-level authorization,
  AES-256-GCM encryption, audit logging

#### MCP Server (agentic-reality-mcp)

- 15 MCP tools: reality_deployment, reality_memory, reality_environment,
  reality_resource, reality_capability, reality_layer, reality_anchor,
  reality_hallucination, reality_topology, reality_temporal, reality_stakes,
  reality_coherence, reality_context, reality_ground, reality_workspace
- Zero unwraps in production code paths
- Stdio, HTTP, and SSE transport modes
- Tool descriptions follow MCP quality standard (verb-first imperative, no
  trailing periods)
- Error code -32803 for unknown tools
- Resource and prompt definitions

#### CLI (agentic-reality-cli)

- ~40 commands organized into 12 subcommand groups
- Human-readable and JSON output modes
- Shell completion generation (bash, zsh, fish)
- MCP server launch via `areal serve`

#### FFI (agentic-reality-ffi)

- C-compatible function exports with opaque handles
- JSON parameter passing
- Thread safety guarantees

#### Documentation

- 12 documentation pages: QUICKSTART, ARCHITECTURE, API, CLI, MCP-TOOLS,
  CAPABILITIES, SISTER-INTEGRATION, CONCEPTS, EXAMPLES, FAQ, TROUBLESHOOTING,
  PRIVACY
- 4 SVG diagrams: architecture, reality-domains, resource-body,
  context-fingerprint

#### Testing

- 250+ tests across unit, integration, stress, MCP, and scenario categories
- Phase-based test organization (phase01 through phase08)
- MCP tool count verification test
- Concurrent access and persistence stress tests

#### CI/CD

- GitHub Actions CI workflow with 50 guardrail sections across 6 jobs
- Universal installer script with desktop/terminal/server profiles
- Release automation script

### Security

- Constant-time token comparison
- Rate limiting on failed authentication attempts
- Memory zeroization for sensitive values
- Optional AES-256-GCM encryption for .areal files at rest

---

*Part of the AgenticOS ecosystem by Agentra Labs*
