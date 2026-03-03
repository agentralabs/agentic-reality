---
status: stable
---

# The .areal File Format

AgenticReality stores all grounding state in a single **`.areal`** binary file per agent instance. This document describes the format, layout, and design decisions.

## Overview

| Property | Value |
|---|---|
| Extension | `.areal` |
| Encoding | Binary (little-endian) |
| Compression | LZ4 per section |
| Integrity | BLAKE3 checksum per section |
| Encryption | Optional AES-256-GCM |
| Magic bytes | `0x61 0x72 0x65 0x61 0x6C` (`areal`) |

## File Layout

```
┌─────────────────────────────┐
│  Magic bytes (5 bytes)      │  "areal"
├─────────────────────────────┤
│  Format version (u8)        │  Current: 1
├─────────────────────────────┤
│  Flags (u8)                 │  Encrypted, compressed, sealed
├─────────────────────────────┤
│  Section count (u16 LE)     │
├─────────────────────────────┤
│  Section directory          │  [id(u8), offset(u64), len(u64)] × N
├─────────────────────────────┤
│  Section: Deployment        │  Soul, environment, fingerprint
├─────────────────────────────┤
│  Section: Resources         │  CPU, memory, I/O budgets
├─────────────────────────────┤
│  Section: Reality           │  Layers, anchors, hallucination log
├─────────────────────────────┤
│  Section: Topology          │  Mesh, neighbors, dependencies
├─────────────────────────────┤
│  Section: Temporal          │  Time context, causality chain
├─────────────────────────────┤
│  Section: Stakes            │  Risk field, blast radius
├─────────────────────────────┤
│  Section: Coherence         │  Engine state, transition log
├─────────────────────────────┤
│  Section: Indexes           │  Lookup tables for fast access
├─────────────────────────────┤
│  Global BLAKE3 checksum     │  32 bytes over all sections
└─────────────────────────────┘
```

## Sections

### Deployment Section (id: 0x01)

Stores the agent's deployment soul — its identity within the deployment context.

```
soul_id:        UUID (16 bytes)
environment:    u8 enum (Local=0, Container=1, Cloud=2, Edge=3, Serverless=4)
fingerprint:    BLAKE3 hash (32 bytes)
incarnation_n:  u32 — how many times this soul has restarted
created_at:     i64 Unix timestamp (seconds)
sealed_at:      i64 Unix timestamp or 0
```

### Resources Section (id: 0x02)

Encodes resource proprioception — the agent's awareness of its own resource body.

```
cpu_budget:     f64 (fraction of a core, 0.0–∞)
memory_bytes:   u64
io_budget:      u64 bytes/sec
cost_per_hour:  f64 (USD)
pressure_level: u8 enum (None=0, Low=1, Medium=2, High=3, Critical=4)
```

### Reality Section (id: 0x03)

Reality layers, freshness anchors, and hallucination event log.

```
layer_count:    u16
layers[]:       [layer_id(UUID), weight(f32), freshness(i64)] × layer_count
anchor_count:   u16
anchors[]:      [anchor_id(UUID), kind(u8), verified_at(i64)] × anchor_count
hallucination_count: u16
events[]:       [timestamp(i64), confidence_delta(f32), context_len(u16), context(bytes)]
```

### Temporal Section (id: 0x04)

```
wall_clock:     i64 Unix timestamp
monotonic_ns:   u64
timezone_offset: i32 seconds from UTC
causality_depth: u32
timeline_id:    UUID
```

## Compression

Each section is independently compressed with LZ4 frame format. This allows:

- Partial reads (load only what you need)
- Efficient updates (rewrite one section without decompressing others)
- Streaming writes for large agent histories

Compression is always applied. The `compressed` flag in the file header is always `1` in v1.

## Integrity

Every section carries a BLAKE3 checksum computed over its compressed bytes. The global checksum at the end of the file covers all section checksums concatenated.

On load, the library verifies:
1. Magic bytes match
2. Format version is supported
3. Each section checksum is valid
4. Global checksum matches

Any mismatch returns `RealityError::CorruptFile`.

## Encryption

When the `encrypted` flag is set:

- A 12-byte nonce is prepended to each section's payload
- AES-256-GCM encrypts the compressed section bytes
- The GCM authentication tag (16 bytes) is appended to each section
- The encryption key is derived via HKDF-SHA256 from a root key + soul_id

The encryption key is never stored in the file. It must be supplied at open time.

## Versioning and Migration

| Version | Notes |
|---|---|
| 1 | Initial release — all 8 sections |

Future versions add new section IDs. The reader ignores unknown section IDs, so v1 readers can safely open v2 files (with degraded functionality). The `section_count` field ensures forward compatibility.

## File Paths

The default path for an agent's reality file follows:

```
~/.local/share/agentic-reality/<soul_id>.areal     # Linux/macOS
%APPDATA%\agentic-reality\<soul_id>.areal           # Windows
```

Override with `AGENTIC_REALITY_PATH` environment variable or the `--path` CLI flag.

## Atomic Writes

All writes use atomic rename:

1. Write to `<path>.areal.tmp`
2. `fsync` the temp file
3. `rename` to `<path>.areal`

This guarantees the on-disk file is always complete and consistent.

## Tooling

```bash
# Inspect a .areal file
areal inspect path/to/agent.areal

# Validate checksums
areal validate path/to/agent.areal

# Export to JSON (human-readable)
areal export path/to/agent.areal --format json

# Migrate to latest format version
areal migrate path/to/agent.areal
```
