# AgenticReality CLI Reference

> `areal` -- The reality grounding command-line interface

## Synopsis

```
areal [OPTIONS] <COMMAND>
```

## Global Options

| Option | Description |
|---|---|
| `--file <PATH>` | Path to .areal file (default: `./.areal`) |
| `--json` | Output in JSON format |
| `--quiet` | Suppress non-essential output |
| `--verbose` | Enable verbose output |
| `--version` | Print version information |
| `--help` | Print help |

## Subcommand Groups

The `areal` CLI organizes ~40 commands into 12 subcommand groups, one per
domain plus workspace and server commands.

---

### areal workspace

Workspace initialization and management.

```bash
areal workspace init [--path <DIR>]      # Initialize a new .areal workspace
areal workspace sense                    # Sense environment and resources
areal workspace status                   # Show workspace status
areal workspace summary                  # Full reality summary
areal workspace export [--format json]   # Export state
areal workspace import <FILE>            # Import state
```

**Examples:**

```bash
# Initialize in current directory
areal workspace init

# Initialize in specific directory
areal workspace init --path /opt/myapp

# Full workspace sense (deployment + environment + resources)
areal workspace sense

# Get JSON status for scripting
areal workspace status --json
```

---

### areal soul

Deployment soul management.

```bash
areal soul init [--spawned-by <ID>] [--purpose <TEXT>]  # Initialize soul
areal soul summary                                       # Soul summary
areal soul vitals                                        # Current vitals
areal soul lineage                                       # Deployment lineage
areal soul nature                                        # Existential nature
areal soul birth                                         # Birth context
```

**Examples:**

```bash
# Initialize with metadata
areal soul init --spawned-by "kubernetes" --purpose "API server"

# Check vitals
areal soul vitals
# Output:
#   Health:       0.98
#   Uptime:       12h 34m
#   Restarts:     0
#   Last Check:   2s ago
```

---

### areal env

Environment sensing and management.

```bash
areal env sense                          # Sense environment
areal env show                           # Show current environment
areal env mood                           # Show environment mood
areal env mood set <MOOD> [--cause <TEXT>]  # Set mood manually
areal env incidents                      # List active incidents
areal env incident add <DESC> --severity <LEVEL>  # Record incident
areal env incident clear <ID>            # Clear resolved incident
```

**Examples:**

```bash
# Show environment details
areal env show
# Output:
#   Type:        Production (tier-1, us-east-1)
#   Mood:        Calm
#   Health:      Healthy
#   Pressure:    Normal
#   Incidents:   0

# Record a degradation
areal env incident add "Database latency elevated" --severity medium
```

---

### areal resource

Resource proprioception.

```bash
areal resource sense                     # Sense all resources
areal resource body                      # Show resource body
areal resource mind                      # Memory (mind) status
areal resource energy                    # CPU (energy) status
areal resource reach                     # Network (reach) status
areal resource storage                   # Storage status
areal resource pressure                  # Pressure gradient
areal resource bottleneck                # Current bottleneck
areal resource sensations                # Active sensations
```

**Examples:**

```bash
# Check resource body
areal resource body
# Output:
#   Mind:       Active (6.2 GB / 16 GB, 39%)
#   Energy:     Steady (4 cores, 28%)
#   Reach:      Connected (avg 4ms latency)
#   Storage:    Clear (120 GB / 500 GB)
#   GPU:        None
#   Bottleneck: None

# Check pressure gradient
areal resource pressure
# Output:
#   Memory:     0.39 (normal)
#   CPU:        0.28 (normal)
#   Network:    0.15 (low)
#   Storage:    0.24 (normal)
#   Bottleneck: Memory (highest pressure)
```

---

### areal capability

Capability discovery and management.

```bash
areal capability discover                # Discover capabilities
areal capability list                    # List all capabilities
areal capability check <NAME>            # Check if capability available
areal capability missing                 # Show expected but missing capabilities
```

**Examples:**

```bash
# Discover what this agent can do
areal capability discover
# Output:
#   Discovered 12 capabilities:
#     [OK] compute/cpu-bound
#     [OK] storage/local-fs
#     [OK] network/http-client
#     [OK] network/dns
#     [--] compute/gpu (not available)
#     ...

# Check specific capability
areal capability check "gpu-compute"
# Output: Not available (no GPU detected)
```

---

### areal anchor

Reality anchor management.

```bash
areal anchor add --type <TYPE> --source <SRC> [--frequency <SECS>]  # Add anchor
areal anchor list                        # List all anchors
areal anchor verify <ID>                 # Verify specific anchor
areal anchor verify-all                  # Verify all anchors
areal anchor remove <ID>                 # Remove anchor
areal anchor drift                       # Show anchor drift report
```

**Anchor types:** `time`, `configuration`, `state`, `external`, `heartbeat`,
`checksum`, `version`

**Examples:**

```bash
# Add a time anchor
areal anchor add --type time --source ntp.org --frequency 300

# Add a state anchor
areal anchor add --type state --source postgres --frequency 120

# Verify all anchors
areal anchor verify-all
# Output:
#   Verifying 3 anchors...
#     [OK]  time (ntp.org) - drift: 12ms, trust: 0.99
#     [OK]  configuration (env) - drift: 0, trust: 0.95
#     [WARN] state (postgres) - drift: 47 rows, trust: 0.78
#   2/3 fully verified, 1 drifting
```

---

### areal layer

Reality layer management.

```bash
areal layer show                         # Show current reality layer
areal layer list                         # List all layers and their trust
areal layer trust                        # Show trust levels
areal layer set <LAYER>                  # Set current layer
```

**Layers:** `physical`, `virtual`, `logical`, `cached`, `predicted`,
`simulated`, `hallucinated`

---

### areal topology

Topology awareness.

```bash
areal topology show                      # Show topology map
areal topology position                  # Show this agent's position
areal topology upstream                  # List upstream entities
areal topology downstream               # List downstream dependencies
areal topology siblings                  # List siblings
areal topology observers                 # List observers
areal topology health                    # Topology health report
areal topology add-downstream <SVC> --type <TYPE> --criticality <LEVEL>
areal topology add-sibling <ID>
```

**Examples:**

```bash
# Show topology map
areal topology show
# Output:
#   Position: Application layer, edge-distance=2
#   Upstream:
#     nginx (load-balancer) - healthy
#   Downstream:
#     postgres (database, critical) - healthy, 3ms
#     redis (cache, degraded) - healthy, 1ms
#     stripe-api (external, optional) - healthy, 45ms
#   Siblings:
#     replica-0 (healthy, load: normal)
#     replica-2 (healthy, load: high)
#   Observers:
#     prometheus (monitoring) - active
#     datadog (apm) - active
```

---

### areal temporal

Temporal grounding.

```bash
areal temporal context                   # Show temporal context
areal temporal deadlines                 # List deadlines
areal temporal deadline add <DESC> --at <TIME>  # Add deadline
areal temporal causality <EVENT>         # Show causality chain
areal temporal business                  # Business time context
```

---

### areal stakes

Stakes perception.

```bash
areal stakes level                       # Show current stakes level
areal stakes set <LEVEL>                 # Set stakes level
areal stakes should-proceed <ACTION> --tolerance <0-1>  # Check action
areal stakes risk                        # Show risk field
areal stakes blast-radius                # Show blast radius
areal stakes guardrails                  # List active guardrails
areal stakes guardrail add <DESC>        # Add guardrail
```

**Stakes levels:** `trivial`, `low`, `medium`, `high`, `critical`

---

### areal coherence

Coherence maintenance.

```bash
areal coherence check                    # Run coherence check
areal coherence level                    # Show coherence level
areal coherence violations               # List violations
areal coherence transitions              # Show transition history
```

---

### areal context

Context fingerprint operations.

```bash
areal context fingerprint                # Show current fingerprint
areal context shifted [--threshold <0-1>]  # Check if context shifted
areal context diff                       # Show what changed
```

---

### areal serve

Start the MCP server.

```bash
areal serve [OPTIONS]

Options:
  --mode <MODE>      Transport mode: stdio (default), http, sse
  --port <PORT>      Port for HTTP/SSE mode (default: 3010)
  --auth             Require authentication (reads AGENTIC_AUTH_TOKEN)
  --cors             Enable CORS for browser clients
  --log <LEVEL>      Log level: error, warn, info, debug, trace
```

**Examples:**

```bash
# Start stdio MCP server (for Claude Desktop)
areal serve

# Start HTTP server with auth
AGENTIC_AUTH_TOKEN=secret areal serve --mode http --port 3010 --auth

# Start SSE server for browser clients
areal serve --mode sse --port 3010 --cors
```

---

### areal completions

Generate shell completions.

```bash
areal completions bash > ~/.bash_completion.d/areal
areal completions zsh > ~/.zfunc/_areal
areal completions fish > ~/.config/fish/completions/areal.fish
```

---

## Exit Codes

| Code | Meaning |
|---|---|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | File not found |
| 4 | Permission denied |
| 5 | Not initialized |
| 6 | Coherence violation |
| 7 | Stakes rejection |

---

*Part of the AgenticOS ecosystem by Agentra Labs*
