---
status: stable
---

# Architecture

AgenticReality is a Rust-native operational context engine structured as a Cargo workspace. The architecture separates concerns across four crates and seven internal domains.

## Crate Structure

| Crate | Published As | Role |
|:---|:---|:---|
| `agentic-reality` | `agentic-reality` on crates.io | Core library — types, engines, 26 capabilities, bridge traits |
| `agentic-reality-mcp` | `agentic-reality-mcp` on crates.io | MCP server — 15 tools, JSON-RPC 2.0 over stdio |
| `agentic-reality-cli` | `agentic-reality-cli` on crates.io | CLI binary — `areal`, ~40 commands |
| `agentic-reality-ffi` | `agentic-reality-ffi` on crates.io | FFI bindings — C ABI, Python (maturin), WASM |

```
agentic-reality/
├── Cargo.toml                      # Workspace root
├── crates/
│   ├── agentic-reality/            # Core library
│   │   ├── src/
│   │   │   ├── engine/             # RealityEngine main struct
│   │   │   ├── domains/            # 7 domain modules
│   │   │   ├── capabilities/         # 26 capability impls
│   │   │   ├── physics/            # Reality physics rules
│   │   │   ├── bridges/            # Sister bridge traits
│   │   │   └── formats/            # .areal binary format
│   │   └── tests/
│   ├── agentic-reality-mcp/        # MCP server
│   │   ├── src/
│   │   │   ├── server.rs           # JSON-RPC dispatch
│   │   │   ├── tools/              # 15 compact tool facades
│   │   │   └── types/              # MCP type definitions
│   │   └── tests/
│   ├── agentic-reality-cli/        # CLI binary
│   │   ├── src/
│   │   │   ├── main.rs             # Argument parsing
│   │   │   └── commands/           # ~40 command handlers
│   │   └── tests/
│   └── agentic-reality-ffi/        # FFI bindings
│       ├── src/
│       │   ├── lib.rs              # C ABI exports
│       │   └── python.rs           # PyO3 bindings
│       └── python/                 # maturin package
├── tests/                          # Integration tests
├── docs/                           # Reference docs
│   └── public/                     # Docs site pages
├── assets/                         # SVG diagrams
├── scripts/                        # Build and install scripts
└── .github/workflows/              # CI pipelines
```

## Seven Domains

The Reality Engine is organized around seven operational domains, each independently senseable and queryable.

| Domain | What it models |
|:---|:---|
| **Deployment Consciousness** | Environment tier, substrate, incarnation identity, restart history |
| **Resource Proprioception** | Memory, CPU, network, storage utilization and pressure gradients |
| **Reality Physics** | Layer stack (ground truth → inferred), reality anchors, freshness |
| **Stakes Perception** | Consequence level, risk fields, blast radius assessment |
| **Topology Awareness** | Upstream deps, downstream consumers, sibling replicas, observers |
| **Temporal Grounding** | Causality chains, temporal anchors, timeline tracking |
| **Coherence Maintenance** | Cross-domain consistency, contradiction detection, transitions |

## Write and Query Engines

The core library separates mutations from reads:

- **Write Engine (90 operations)** — state updates, anchor creation, incarnation recording, topology mutations
- **Query Engine (78 operations)** — read-only projections, coherence checks, hallucination scoring, topology queries

Both engines operate through the `RealityEngine` struct, which serializes and deserializes the `.areal` binary format using atomic writes.

## 26 Capabilities

Each capability is a self-contained capability layer built on top of the core engines. Capabilities span Tier A (core sensing), Tier B (grounding), Tier C (topology), and higher tiers for advanced use cases like temporal archaeology and context drift detection.

See [Command Surface](command-surface.md) for the full inventory.

## Bridge Traits

Sister integrations use typed bridge traits with NoOp defaults:

```rust
pub trait MemoryBridge: Send + Sync {
    fn load_incarnation_context(&self, id: &str) -> Option<MemoryContext>;
}

pub struct NoOpMemoryBridge;
impl MemoryBridge for NoOpMemoryBridge { ... }
```

No sister is required. When connected, sisters enhance capability. When absent, the engine operates with full functionality using NoOp fallbacks.

## Data Flow

```
Sense trigger
  → domain sensors (parallel)
  → engine state update (write engine)
  → context fingerprint generation
  → coherence check
  → .areal atomic write
```

## Thread Safety

The `RealityEngine` is `Send + Sync`. The MCP server runs tools concurrently under Tokio. The `.areal` format uses atomic temp-file-plus-rename writes; concurrent readers always see a complete consistent state.
