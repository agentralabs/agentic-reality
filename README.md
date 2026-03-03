# AgenticReality

[![Crates.io](https://img.shields.io/crates/v/agentic-reality.svg)](https://crates.io/crates/agentic-reality)
[![Documentation](https://docs.rs/agentic-reality/badge.svg)](https://docs.rs/agentic-reality)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tests](https://github.com/agentralabs/agentic-reality/workflows/CI/badge.svg)](https://github.com/agentralabs/agentic-reality/actions)

**The Ground** -- Sister #10 of the AgenticOS ecosystem.

> *The sister that knows WHERE it exists and WHAT is real.*

AgenticReality provides AI agents with comprehensive awareness of their
operational context: deployment identity, resource state, reality grounding,
topology awareness, temporal context, stakes perception, and coherence
maintenance.

## The Problem

AI agents operate in a **reality vacuum**:

```
Without AgenticReality:
  Agent processes request (blind to context)
    |-- Is this production or test?        Unknown
    |-- Am I on a laptop or data center?   Unknown
    |-- Do I have 1GB or 100GB RAM?        Unknown
    |-- Is my database healthy?            Unknown
    |-- What are the stakes?               Unknown
    |-- Am I hallucinating?                Unknown
```

## The Solution

```
With AgenticReality:
  DEPLOYMENT:  "I am prod-us-east-node-7, one of 3 replicas"
  RESOURCES:   "Memory at 73%, feeling crowded"
  REALITY:     "Cache is 47 minutes stale, 3 anchors verified"
  TOPOLOGY:    "Database latency: 12ms, sibling-2 is struggling"
  TEMPORAL:    "End-of-quarter, user is in Tokyo (3am there)"
  STAKES:      "High stakes: financial API, blast radius 50k users"
  COHERENCE:   "All domains consistent, no contradictions"
```

## 26 Inventions Across 7 Tiers

| Tier | Domain | Inventions |
|---|---|---|
| 1 | Deployment Consciousness | Deployment Soul, Environment Sensing, Incarnation Memory, Context Fingerprint, Deployment Topology Map |
| 2 | Resource Proprioception | Resource Body Schema, Capability Discovery, Resource Pressure Gradient, Cost Consciousness, Capacity Planning Intuition |
| 3 | Reality Physics | Reality Layers, Freshness Perception, Reality Anchors, Hallucination Detection |
| 4 | Topology Awareness | Service Mesh Perception, Neighbor Awareness, Dependency Awareness, Observer Awareness |
| 5 | Temporal Grounding | Temporal Awareness, Causality Tracking, Timeline Coherence |
| 6 | Stakes Perception | Consequence Awareness, Risk Field Perception, Blast Radius Awareness |
| 7 | Coherence Maintenance | Reality Coherence Engine, Context Transition Manager |

## Quick Start

```bash
# Install
curl -fsSL https://agentralabs.com/install/reality | bash

# Initialize
areal workspace init

# Sense context
areal workspace sense

# Check reality
areal soul summary
```

## MCP Integration

Add to your Claude Desktop configuration:

```json
{
  "mcpServers": {
    "agentic-reality": {
      "command": "areal",
      "args": ["serve"]
    }
  }
}
```

15 MCP tools provide full reality grounding to AI agents.

## Library Usage

```toml
[dependencies]
agentic-reality = "0.1.0"
```

```rust
use agentic_reality::{RealityEngine, InitializeSoulRequest};

let mut engine = RealityEngine::new();
engine.write().initialize_soul(InitializeSoulRequest::default())?;
engine.write().sense_environment()?;
engine.write().sense_resources()?;

let soul = engine.query().get_soul().unwrap();
println!("Incarnation: {}", soul.incarnation_id);
println!("Substrate: {:?}", soul.substrate.tier);

if engine.query().has_context_shifted(0.3)?.shifted {
    println!("Context has shifted - increasing caution");
}

engine.save("state.areal")?;
```

## Architecture

Four crates following the Agentra sister workspace pattern:

| Crate | Purpose |
|---|---|
| `agentic-reality` | Core library: types, engines, storage, validation, inventions, bridges |
| `agentic-reality-mcp` | MCP server: 15 tools, stdio/HTTP/SSE transports |
| `agentic-reality-cli` | CLI binary: `areal` with ~40 commands |
| `agentic-reality-ffi` | FFI bindings: C-compatible interface |

## Sister Dependencies

| Sister | Version | Purpose |
|---|---|---|
| AgenticTime | >= 0.1.0 | Temporal grounding, verified time |
| AgenticContract | >= 0.1.0 | Stakes constraints, SLAs |
| AgenticIdentity | >= 0.3.0 | Incarnation identity verification |

All dependencies are optional via NoOp bridge traits.

## Documentation

- [Quickstart](./docs/QUICKSTART.md)
- [Architecture](./docs/ARCHITECTURE.md)
- [API Reference](./docs/API.md)
- [CLI Reference](./docs/CLI.md)
- [MCP Tools](./docs/MCP-TOOLS.md)
- [26 Inventions](./docs/INVENTIONS.md)
- [Core Concepts](./docs/CONCEPTS.md)
- [Examples](./docs/EXAMPLES.md)
- [Sister Integration](./docs/SISTER-INTEGRATION.md)
- [FAQ](./docs/FAQ.md)
- [Troubleshooting](./docs/TROUBLESHOOTING.md)
- [Privacy](./docs/PRIVACY.md)

## License

MIT License -- see [LICENSE](./LICENSE)

---

*Part of the AgenticOS ecosystem by Agentra Labs*
