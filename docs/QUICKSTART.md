# AgenticReality Quickstart

Get reality grounding for your AI agent in 5 minutes.

## Prerequisites

- Rust 1.70+ (for building from source)
- macOS, Linux, or Windows (WSL)

## Installation

### One-line install

```bash
curl -fsSL https://agentralabs.com/install/reality | bash
```

The installer auto-detects your platform and configures the appropriate profile.

### From crates.io

```bash
cargo install agentic-reality-cli
```

### From source

```bash
git clone https://github.com/agentralabs/agentic-reality.git
cd agentic-reality
cargo build --release
cp target/release/areal ~/.local/bin/
```

### Verify installation

```bash
areal --version
# agentic-reality 0.1.0
```

## Quick Start

### 1. Initialize a reality workspace

```bash
areal workspace init
```

This creates a `.areal` file in your project directory, which stores the complete
reality state: deployment soul, environment medium, resource body, topology map,
temporal context, stakes assessment, and coherence status.

### 2. Sense your context

```bash
areal workspace sense
```

The agent senses its deployment substrate, environment type, and available
resources. Output:

```
Sensing reality...
  Substrate:   Cloud (AWS, m5.xlarge, us-east-1)
  Environment: Production (tier-1, critical)
  Mind:        Active (6.2 GB / 16 GB, 39%)
  Energy:      Steady (4 cores, 28% utilization)
  Reach:       Connected (< 5ms to dependencies)
  Storage:     Clear (120 GB / 500 GB, 24%)
Reality grounded.
```

### 3. Check your deployment soul

```bash
areal soul summary
```

Output:

```
Deployment Soul
============================================
Incarnation:  7b2f4a8e-9c3d-4e5f-a6b7-c8d9e0f1a2b3
Substrate:    Cloud (AWS, m5.xlarge)
Environment:  Production
Mood:         Calm
Bottleneck:   None
Stakes:       High
Coherence:    Full
Uptime:       4h 23m
Cardinality:  ReplicaOf(3, index=1)
Expendability: 0.33
```

### 4. Add reality anchors

Reality anchors are verifiable reference points that ground the agent in truth:

```bash
areal anchor add --type time --source ntp.org --frequency 300
areal anchor add --type configuration --source env --frequency 60
areal anchor add --type state --source postgres --frequency 120
```

### 5. Verify anchors

```bash
areal anchor verify-all
```

Output:

```
Verifying 3 anchors...
  [OK]  time (ntp.org) - drift: 12ms
  [OK]  configuration (env) - drift: 0
  [OK]  state (postgres) - drift: 2 rows
All anchors verified.
```

### 6. Check stakes before acting

```bash
areal stakes should-proceed "deploy_new_version" --tolerance 0.5
```

Output:

```
Stakes Assessment
============================================
Current Level:  High
Action:         deploy_new_version
Risk Tolerance: 0.5
Blast Radius:   50,000 users
Decision:       ProceedWithCaution
Conditions:     Verify rollback plan, notify on-call
```

## MCP Integration

### Claude Desktop

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

Restart Claude Desktop. The agent now has access to 15 reality tools.

### VS Code

Add to `.vscode/settings.json`:

```json
{
  "mcp.servers": {
    "agentic-reality": {
      "command": "areal",
      "args": ["serve"]
    }
  }
}
```

### HTTP Server Mode

For production deployments:

```bash
AGENTIC_AUTH_TOKEN=your-secret-token areal serve --mode http --port 3010 --auth
```

## Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
agentic-reality = "0.1.0"
```

Basic Rust usage:

```rust
use agentic_reality::{RealityEngine, InitializeSoulRequest};

fn main() -> anyhow::Result<()> {
    let mut engine = RealityEngine::new();

    // Initialize deployment soul
    engine.write().initialize_soul(InitializeSoulRequest::default())?;

    // Sense environment and resources
    engine.write().sense_environment()?;
    engine.write().sense_resources()?;

    // Check reality
    let soul = engine.query().get_soul().unwrap();
    println!("Incarnation: {}", soul.incarnation_id);
    println!("Substrate: {:?}", soul.substrate.tier);

    // Check if context has shifted
    if engine.query().has_context_shifted(0.3)?.shifted {
        println!("Context has shifted - increasing caution");
    }

    // Save state
    engine.save("state.areal")?;

    Ok(())
}
```

## Next Steps

- [Architecture Guide](./ARCHITECTURE.md) - Understand the system design
- [CLI Reference](./CLI.md) - Full command reference
- [MCP Tools](./MCP-TOOLS.md) - All 15 MCP tools documented
- [Core Concepts](./CONCEPTS.md) - Deployment soul, resource body, reality layers
- [26 Inventions](./INVENTIONS.md) - The ideas behind AgenticReality
- [Examples](./EXAMPLES.md) - Code examples for common use cases

---

*Part of the AgenticOS ecosystem by Agentra Labs*
