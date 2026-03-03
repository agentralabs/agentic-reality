# AgenticReality Quick Guide

AgenticReality gives AI agents existential grounding -- awareness of where they
run, what resources they have, what is real, and what the stakes are.

## CLI basics

The CLI binary is `areal`. All commands follow `areal <domain> <action>`.

```bash
# Show current reality state
areal status

# Initialize a deployment soul
areal deployment init --purpose "production service"

# Sense the operating environment
areal environment sense

# Check resource body (CPU, memory, disk)
areal resource body

# Query reality layers
areal reality layers

# List reality anchors
areal anchor list

# Check hallucination risk
areal hallucination status

# View topology map
areal topology map

# Get temporal grounding
areal temporal now

# Assess stakes level
areal stakes level

# Check coherence
areal coherence check

# Save state to a .areal file
areal workspace save --path ./state.areal

# Load state from a .areal file
areal workspace load --path ./state.areal
```

## MCP tools

When running as an MCP server, 15 tools are available to any MCP client:

| Tool                    | Purpose                                     |
|-------------------------|---------------------------------------------|
| `reality_deployment`    | Manage deployment soul and identity          |
| `reality_memory`        | Access incarnation memory across past lives  |
| `reality_environment`   | Sense the operational environment            |
| `reality_resource`      | Monitor resource body                        |
| `reality_capability`    | Discover agent capabilities                  |
| `reality_layer`         | Manage reality layers                        |
| `reality_anchor`        | Create and verify reality anchors            |
| `reality_hallucination` | Detect hallucination risks                   |
| `reality_topology`      | Map deployment topology                      |
| `reality_temporal`      | Ground temporal awareness                    |
| `reality_stakes`        | Assess consequences and blast radius         |
| `reality_coherence`     | Maintain coherence across domains            |
| `reality_context`       | Get unified context summary                  |
| `reality_ground`        | Verify claims against reality anchors        |
| `reality_workspace`     | Manage workspace files and persistence       |

Each tool accepts an `operation` parameter. Use `"get"` for reads and
domain-specific verbs for writes.

## File format

State is persisted in `.areal` binary files with BLAKE3 checksums,
per-section LZ4 compression, and optional AES-256-GCM encryption.

## Further reading

- `INSTALL.md` -- full installation guide
- `docs/QUICKSTART.md` -- step-by-step quickstart
- `docs/CONCEPTS.md` -- seven reality domains explained
- `docs/MCP-TOOLS.md` -- detailed MCP tool reference
- `docs/CLI.md` -- complete CLI command reference
