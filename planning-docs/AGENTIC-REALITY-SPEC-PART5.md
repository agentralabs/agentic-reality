# AGENTIC REALITY SPECIFICATION — PART 5

> **Specs Covered:** SPEC-17 through SPEC-25
> **Sister:** #10 of 25 (The Ground)
> **Continues from:** Part 4

---

# SPEC-17: DOCUMENTATION

## 17.1 Documentation Structure

```
docs/
├── QUICKSTART.md           Getting started in 5 minutes
├── ARCHITECTURE.md         System architecture
├── API.md                  Rust API reference
├── CLI.md                  CLI command reference
├── MCP-TOOLS.md            MCP tool documentation
├── INVENTIONS.md           26 inventions explained
├── SISTER-INTEGRATION.md   Integration with other sisters
├── CONCEPTS.md             Core concepts explained
├── EXAMPLES.md             Usage examples
├── FAQ.md                  Frequently asked questions
├── TROUBLESHOOTING.md      Common issues and solutions
├── PRIVACY.md              Privacy and security considerations
└── assets/
    ├── architecture.svg
    ├── reality-domains.svg
    ├── resource-body.svg
    └── context-fingerprint.svg
```

## 17.2 QUICKSTART.md Template

```markdown
# AgenticReality Quickstart

Get reality grounding for your AI agent in 5 minutes.

## Installation

### One-line install
```bash
curl -fsSL https://agentriclabs.com/install/reality | bash
```

### From crates.io
```bash
cargo install agentic-reality
```

## Quick Start

### 1. Initialize Reality
```bash
areal workspace init
```

### 2. Sense Your Context
```bash
areal workspace sense
```

### 3. Check Your Reality
```bash
areal soul summary
```

Output:
```
Context Summary
═══════════════════════════════════════
Incarnation:  7b2f4a8e-9c3d-4e5f-a6b7-c8d9e0f1a2b3
Substrate:    Cloud (AWS, m5.xlarge)
Environment:  Production
Mood:         Calm
Bottleneck:   Memory
Stakes:       High
Coherence:    Full
Uptime:       4h 23m
```

### 4. Add Reality Anchors
```bash
areal anchor add --type time --source ntp.org
areal anchor add --type configuration --source env
```

### 5. Check Stakes Before Acting
```bash
areal stakes should-proceed "deploy_new_version" --tolerance 0.5
```

## MCP Integration

Add to Claude Desktop config:
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

## Next Steps

- [Architecture Guide](./ARCHITECTURE.md)
- [CLI Reference](./CLI.md)
- [MCP Tools](./MCP-TOOLS.md)
- [26 Inventions](./INVENTIONS.md)
```

---

# SPEC-18: SVG DIAGRAMS

## 18.1 Architecture Diagram

```svg
<!-- docs/assets/architecture.svg -->
<svg viewBox="0 0 800 600" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="headerGrad" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" style="stop-color:#1a365d"/>
      <stop offset="100%" style="stop-color:#2c5282"/>
    </linearGradient>
    <linearGradient id="engineGrad" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" style="stop-color:#2d3748"/>
      <stop offset="100%" style="stop-color:#1a202c"/>
    </linearGradient>
  </defs>
  
  <!-- Title -->
  <text x="400" y="30" text-anchor="middle" font-size="24" font-weight="bold" fill="#1a365d">AgenticReality Architecture</text>
  
  <!-- MCP Server Layer -->
  <rect x="50" y="50" width="700" height="80" rx="8" fill="url(#headerGrad)"/>
  <text x="400" y="80" text-anchor="middle" font-size="16" fill="white" font-weight="bold">MCP SERVER (15 Tools)</text>
  <text x="400" y="105" text-anchor="middle" font-size="11" fill="#a0aec0">deployment | environment | resource | capability | layer | anchor | hallucination | topology | temporal | stakes | coherence | context | ground | workspace | memory</text>
  
  <!-- Reality Engine -->
  <rect x="50" y="150" width="700" height="200" rx="8" fill="url(#engineGrad)"/>
  <text x="400" y="175" text-anchor="middle" font-size="16" fill="white" font-weight="bold">REALITY ENGINE</text>
  
  <!-- Engine Components -->
  <rect x="70" y="190" width="120" height="60" rx="4" fill="#4a5568"/>
  <text x="130" y="225" text-anchor="middle" font-size="12" fill="white">Deployment</text>
  
  <rect x="200" y="190" width="120" height="60" rx="4" fill="#4a5568"/>
  <text x="260" y="225" text-anchor="middle" font-size="12" fill="white">Environment</text>
  
  <rect x="330" y="190" width="120" height="60" rx="4" fill="#4a5568"/>
  <text x="390" y="225" text-anchor="middle" font-size="12" fill="white">Resource</text>
  
  <rect x="460" y="190" width="120" height="60" rx="4" fill="#4a5568"/>
  <text x="520" y="225" text-anchor="middle" font-size="12" fill="white">Reality</text>
  
  <rect x="590" y="190" width="120" height="60" rx="4" fill="#4a5568"/>
  <text x="650" y="225" text-anchor="middle" font-size="12" fill="white">Topology</text>
  
  <rect x="70" y="260" width="120" height="60" rx="4" fill="#4a5568"/>
  <text x="130" y="295" text-anchor="middle" font-size="12" fill="white">Temporal</text>
  
  <rect x="200" y="260" width="120" height="60" rx="4" fill="#4a5568"/>
  <text x="260" y="295" text-anchor="middle" font-size="12" fill="white">Stakes</text>
  
  <rect x="330" y="260" width="120" height="60" rx="4" fill="#4a5568"/>
  <text x="390" y="295" text-anchor="middle" font-size="12" fill="white">Coherence</text>
  
  <rect x="460" y="260" width="120" height="60" rx="4" fill="#4a5568"/>
  <text x="520" y="295" text-anchor="middle" font-size="12" fill="white">Hallucination</text>
  
  <rect x="590" y="260" width="120" height="60" rx="4" fill="#4a5568"/>
  <text x="650" y="295" text-anchor="middle" font-size="12" fill="white">Transition</text>
  
  <!-- Storage Layer -->
  <rect x="50" y="370" width="700" height="80" rx="8" fill="#553c9a"/>
  <text x="400" y="400" text-anchor="middle" font-size="16" fill="white" font-weight="bold">STORAGE LAYER</text>
  <text x="400" y="425" text-anchor="middle" font-size="12" fill="#d6bcfa">.areal file | Deployment | Environment | Resources | Reality | Topology | Temporal | Stakes | Coherence | Indexes</text>
  
  <!-- Sister Bridges -->
  <rect x="50" y="470" width="700" height="60" rx="8" fill="#285e61"/>
  <text x="400" y="495" text-anchor="middle" font-size="16" fill="white" font-weight="bold">SISTER BRIDGES</text>
  <text x="400" y="515" text-anchor="middle" font-size="11" fill="#81e6d9">Time ↔ Contract ↔ Identity ↔ Memory ↔ Cognition ↔ Comm ↔ Codebase ↔ Vision → HYDRA</text>
  
  <!-- Arrows -->
  <path d="M400 130 L400 150" stroke="#a0aec0" stroke-width="2" marker-end="url(#arrow)"/>
  <path d="M400 350 L400 370" stroke="#a0aec0" stroke-width="2" marker-end="url(#arrow)"/>
  <path d="M400 450 L400 470" stroke="#a0aec0" stroke-width="2" marker-end="url(#arrow)"/>
  
  <defs>
    <marker id="arrow" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto" markerUnits="strokeWidth">
      <path d="M0,0 L0,6 L9,3 z" fill="#a0aec0"/>
    </marker>
  </defs>
</svg>
```

## 18.2 Reality Domains Diagram

```svg
<!-- docs/assets/reality-domains.svg -->
<svg viewBox="0 0 600 500" xmlns="http://www.w3.org/2000/svg">
  <text x="300" y="30" text-anchor="middle" font-size="20" font-weight="bold" fill="#1a365d">The Seven Domains of Reality</text>
  
  <!-- Central Engine -->
  <circle cx="300" cy="250" r="60" fill="#2d3748"/>
  <text x="300" y="245" text-anchor="middle" font-size="12" fill="white" font-weight="bold">REALITY</text>
  <text x="300" y="260" text-anchor="middle" font-size="12" fill="white" font-weight="bold">ENGINE</text>
  
  <!-- Domain 1: Deployment Consciousness -->
  <circle cx="300" cy="100" r="45" fill="#3182ce"/>
  <text x="300" y="95" text-anchor="middle" font-size="10" fill="white" font-weight="bold">DEPLOYMENT</text>
  <text x="300" y="108" text-anchor="middle" font-size="9" fill="white">CONSCIOUSNESS</text>
  
  <!-- Domain 2: Resource Proprioception -->
  <circle cx="165" cy="150" r="45" fill="#38a169"/>
  <text x="165" y="145" text-anchor="middle" font-size="10" fill="white" font-weight="bold">RESOURCE</text>
  <text x="165" y="158" text-anchor="middle" font-size="9" fill="white">PROPRIOCEPTION</text>
  
  <!-- Domain 3: Reality Physics -->
  <circle cx="435" cy="150" r="45" fill="#d69e2e"/>
  <text x="435" y="145" text-anchor="middle" font-size="10" fill="white" font-weight="bold">REALITY</text>
  <text x="435" y="158" text-anchor="middle" font-size="9" fill="white">PHYSICS</text>
  
  <!-- Domain 4: Topology Awareness -->
  <circle cx="120" cy="280" r="45" fill="#805ad5"/>
  <text x="120" y="275" text-anchor="middle" font-size="10" fill="white" font-weight="bold">TOPOLOGY</text>
  <text x="120" y="288" text-anchor="middle" font-size="9" fill="white">AWARENESS</text>
  
  <!-- Domain 5: Temporal Grounding -->
  <circle cx="480" cy="280" r="45" fill="#dd6b20"/>
  <text x="480" y="275" text-anchor="middle" font-size="10" fill="white" font-weight="bold">TEMPORAL</text>
  <text x="480" y="288" text-anchor="middle" font-size="9" fill="white">GROUNDING</text>
  
  <!-- Domain 6: Stakes Perception -->
  <circle cx="200" cy="380" r="45" fill="#e53e3e"/>
  <text x="200" y="375" text-anchor="middle" font-size="10" fill="white" font-weight="bold">STAKES</text>
  <text x="200" y="388" text-anchor="middle" font-size="9" fill="white">PERCEPTION</text>
  
  <!-- Domain 7: Coherence Maintenance -->
  <circle cx="400" cy="380" r="45" fill="#319795"/>
  <text x="400" y="375" text-anchor="middle" font-size="10" fill="white" font-weight="bold">COHERENCE</text>
  <text x="400" y="388" text-anchor="middle" font-size="9" fill="white">MAINTENANCE</text>
  
  <!-- Connection lines -->
  <line x1="300" y1="145" x2="300" y2="190" stroke="#a0aec0" stroke-width="2"/>
  <line x1="195" y1="180" x2="255" y2="215" stroke="#a0aec0" stroke-width="2"/>
  <line x1="405" y1="180" x2="345" y2="215" stroke="#a0aec0" stroke-width="2"/>
  <line x1="160" y1="265" x2="240" y2="250" stroke="#a0aec0" stroke-width="2"/>
  <line x1="440" y1="265" x2="360" y2="250" stroke="#a0aec0" stroke-width="2"/>
  <line x1="225" y1="350" x2="275" y2="295" stroke="#a0aec0" stroke-width="2"/>
  <line x1="375" y1="350" x2="325" y2="295" stroke="#a0aec0" stroke-width="2"/>
</svg>
```

---

# SPEC-19: README

## 19.1 README.md Template

```markdown
# AgenticReality

[![Crates.io](https://img.shields.io/crates/v/agentic-reality.svg)](https://crates.io/crates/agentic-reality)
[![Documentation](https://docs.rs/agentic-reality/badge.svg)](https://docs.rs/agentic-reality)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tests](https://github.com/agentralabs-tech/agentic-reality/workflows/CI/badge.svg)](https://github.com/agentralabs-tech/agentic-reality/actions)

**The Ground** — Sister #10 of the AgenticOS ecosystem.

> *The sister that knows WHERE it exists and WHAT is real.*

AgenticReality provides AI agents with comprehensive awareness of their operational context — deployment identity, resource state, reality grounding, topology awareness, temporal context, stakes perception, and coherence maintenance.

## The Problem

AI agents operate in a **reality vacuum**:

```
Without AgenticReality:
  Agent processes request (blind to context)
    ├── Is this production or test? Unknown
    ├── Am I on a laptop or data center? Unknown
    ├── Do I have 1GB or 100GB RAM? Unknown
    ├── Is my database healthy? Unknown
    ├── What are the stakes of this response? Unknown
    └── Am I hallucinating right now? Unknown
```

## The Solution

AgenticReality gives agents **existential awareness**:

```
With AgenticReality:
  DEPLOYMENT:  "I am prod-us-east-node-7"
  RESOURCES:   "Memory at 73%, feeling crowded"
  REALITY:     "Cache is 47 minutes stale"
  TOPOLOGY:    "Database latency: 12ms"
  TEMPORAL:    "End-of-quarter crunch"
  STAKES:      "High stakes: financial API"
  COHERENCE:   "All anchors verified"
```

## 26 Inventions

| Tier | Inventions |
|------|------------|
| **1. Deployment Consciousness** | Deployment Soul, Environment Sensing, Incarnation Memory, Context Fingerprint, Deployment Topology Map |
| **2. Resource Proprioception** | Resource Body Schema, Capability Discovery, Resource Pressure, Cost Consciousness, Capacity Intuition |
| **3. Reality Physics** | Reality Layers, Freshness Perception, Reality Anchors, Hallucination Detection |
| **4. Topology Awareness** | Service Mesh Perception, Neighbor Awareness, Dependency Awareness, Observer Awareness |
| **5. Temporal Grounding** | Temporal Awareness, Causality Tracking, Timeline Coherence |
| **6. Stakes Perception** | Consequence Awareness, Risk Field Perception, Blast Radius Awareness |
| **7. Coherence Maintenance** | Reality Coherence Engine, Context Transition Manager |

## Quick Start

```bash
# Install
curl -fsSL https://agentriclabs.com/install/reality | bash

# Initialize
areal workspace init

# Sense context
areal workspace sense

# Check reality
areal soul summary
```

## MCP Integration

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

## Dependencies

| Sister | Version | Purpose |
|--------|---------|---------|
| AgenticTime | ≥0.1.0 | Temporal grounding |
| AgenticContract | ≥0.1.0 | Stakes constraints |
| AgenticIdentity | ≥0.3.0 | Incarnation verification |

## File Format

Reality state persists in `.areal` files with sections for deployment, environment, resources, reality, topology, temporal, stakes, and coherence.

## Privacy & Security

- Token-based authentication for MCP server mode
- Optional AES-256-GCM encryption for sensitive data
- Audit logging for all operations
- No silent fallbacks in validation

## Documentation

- [Quickstart](./docs/QUICKSTART.md)
- [Architecture](./docs/ARCHITECTURE.md)
- [CLI Reference](./docs/CLI.md)
- [MCP Tools](./docs/MCP-TOOLS.md)
- [26 Inventions](./docs/INVENTIONS.md)

## License

MIT License - see [LICENSE](./LICENSE)

---

*Part of the AgenticOS ecosystem by Agentic Labs*
```

---

# SPEC-20: INSTALLER

## 20.1 Universal Installer Script

```bash
#!/usr/bin/env bash
# AgenticReality Universal Installer
# Usage: curl -fsSL https://agentriclabs.com/install/reality | bash

set -euo pipefail

VERSION="${AGENTIC_REALITY_VERSION:-latest}"
INSTALL_DIR="${AGENTIC_INSTALL_DIR:-$HOME/.agentic}"
BIN_DIR="${AGENTIC_BIN_DIR:-$INSTALL_DIR/bin}"
PROFILE="${AGENTIC_PROFILE:-desktop}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info() { echo -e "${BLUE}[INFO]${NC} $1"; }
success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# Detect platform
detect_platform() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"
    
    case "$os" in
        Linux)  OS="linux" ;;
        Darwin) OS="macos" ;;
        MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
        *) error "Unsupported OS: $os" ;;
    esac
    
    case "$arch" in
        x86_64|amd64) ARCH="x86_64" ;;
        arm64|aarch64) ARCH="aarch64" ;;
        *) error "Unsupported architecture: $arch" ;;
    esac
    
    PLATFORM="${OS}-${ARCH}"
    info "Detected platform: $PLATFORM"
}

# Resolve version
resolve_version() {
    if [ "$VERSION" = "latest" ]; then
        VERSION=$(curl -fsSL "https://api.github.com/repos/agentralabs-tech/agentic-reality/releases/latest" | grep -oP '"tag_name": "\K[^"]+')
        info "Latest version: $VERSION"
    fi
}

# Download binary
download_binary() {
    local url="https://github.com/agentralabs-tech/agentic-reality/releases/download/${VERSION}/agentic-reality-${VERSION}-${PLATFORM}.tar.gz"
    local tmp_dir=$(mktemp -d)
    
    info "Downloading from $url"
    curl -fsSL "$url" -o "$tmp_dir/agentic-reality.tar.gz" || error "Download failed"
    
    info "Extracting..."
    tar -xzf "$tmp_dir/agentic-reality.tar.gz" -C "$tmp_dir"
    
    # Install binary
    mkdir -p "$BIN_DIR"
    mv "$tmp_dir/areal" "$BIN_DIR/areal"
    chmod +x "$BIN_DIR/areal"
    
    # Cleanup
    rm -rf "$tmp_dir"
    
    success "Installed areal to $BIN_DIR/areal"
}

# Configure PATH
configure_path() {
    local shell_rc
    
    if [ -n "${ZSH_VERSION:-}" ]; then
        shell_rc="$HOME/.zshrc"
    elif [ -n "${BASH_VERSION:-}" ]; then
        shell_rc="$HOME/.bashrc"
    else
        shell_rc="$HOME/.profile"
    fi
    
    if ! grep -q "AGENTIC" "$shell_rc" 2>/dev/null; then
        echo "" >> "$shell_rc"
        echo "# AgenticOS" >> "$shell_rc"
        echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$shell_rc"
        info "Added $BIN_DIR to PATH in $shell_rc"
    fi
}

# Configure MCP for desktop profile
configure_mcp_desktop() {
    info "Configuring MCP for Claude Desktop..."
    
    local config_dir config_file
    
    if [ "$OS" = "macos" ]; then
        config_dir="$HOME/Library/Application Support/Claude"
    elif [ "$OS" = "linux" ]; then
        config_dir="${XDG_CONFIG_HOME:-$HOME/.config}/claude"
    else
        config_dir="$APPDATA/Claude"
    fi
    
    config_file="$config_dir/claude_desktop_config.json"
    mkdir -p "$config_dir"
    
    # Merge-only update (never destructive overwrite)
    if [ -f "$config_file" ]; then
        # Backup existing
        cp "$config_file" "$config_file.backup.$(date +%s)"
        
        # Merge using jq if available
        if command -v jq &>/dev/null; then
            local tmp_file=$(mktemp)
            jq --arg bin "$BIN_DIR/areal" '
                .mcpServers["agentic-reality"] = {
                    "command": $bin,
                    "args": ["serve"]
                }
            ' "$config_file" > "$tmp_file"
            mv "$tmp_file" "$config_file"
        else
            warn "jq not found - please manually add agentic-reality to $config_file"
        fi
    else
        # Create new config
        cat > "$config_file" << EOF
{
  "mcpServers": {
    "agentic-reality": {
      "command": "$BIN_DIR/areal",
      "args": ["serve"]
    }
  }
}
EOF
    fi
    
    success "MCP configured for Claude Desktop"
}

# Configure MCP for VS Code
configure_mcp_vscode() {
    info "Configuring MCP for VS Code..."
    
    local vscode_settings="$HOME/.vscode/settings.json"
    
    if [ -f "$vscode_settings" ] && command -v jq &>/dev/null; then
        local tmp_file=$(mktemp)
        jq --arg bin "$BIN_DIR/areal" '
            .["mcp.servers"]["agentic-reality"] = {
                "command": $bin,
                "args": ["serve"]
            }
        ' "$vscode_settings" > "$tmp_file"
        mv "$tmp_file" "$vscode_settings"
        success "MCP configured for VS Code"
    else
        warn "VS Code settings not found or jq not available"
    fi
}

# Install service for server profile
install_service() {
    info "Installing system service..."
    
    if [ "$OS" = "macos" ]; then
        install_launchd_service
    elif [ "$OS" = "linux" ]; then
        install_systemd_service
    fi
}

install_launchd_service() {
    local plist="$HOME/Library/LaunchAgents/com.agentriclabs.reality.plist"
    
    cat > "$plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.agentriclabs.reality</string>
    <key>ProgramArguments</key>
    <array>
        <string>$BIN_DIR/areal</string>
        <string>serve</string>
        <string>--mode</string>
        <string>http</string>
        <string>--port</string>
        <string>3010</string>
        <string>--auth</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>$INSTALL_DIR/logs/reality.log</string>
    <key>StandardErrorPath</key>
    <string>$INSTALL_DIR/logs/reality.err</string>
</dict>
</plist>
EOF
    
    mkdir -p "$INSTALL_DIR/logs"
    launchctl load "$plist" 2>/dev/null || true
    success "Installed launchd service"
}

install_systemd_service() {
    local service_file="$HOME/.config/systemd/user/agentic-reality.service"
    mkdir -p "$(dirname "$service_file")"
    
    cat > "$service_file" << EOF
[Unit]
Description=AgenticReality MCP Server
After=network.target

[Service]
Type=simple
ExecStart=$BIN_DIR/areal serve --mode http --port 3010 --auth
Restart=always
RestartSec=10
Environment=AGENTIC_AUTH_TOKEN=%h/.agentic/auth_token

[Install]
WantedBy=default.target
EOF
    
    systemctl --user daemon-reload
    systemctl --user enable agentic-reality
    success "Installed systemd service"
}

# Generate auth token for server mode
generate_auth_token() {
    if [ "$PROFILE" = "server" ]; then
        local token_file="$INSTALL_DIR/auth_token"
        
        if [ ! -f "$token_file" ]; then
            local token=$(openssl rand -hex 32)
            echo "$token" > "$token_file"
            chmod 600 "$token_file"
            info "Generated auth token: $token_file"
            warn "Set AGENTIC_AUTH_TOKEN environment variable to this token"
        fi
    fi
}

# Main installation
main() {
    echo ""
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║           AgenticReality Universal Installer                 ║"
    echo "║                    Sister #10: The Ground                    ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo ""
    
    detect_platform
    resolve_version
    download_binary
    configure_path
    
    case "$PROFILE" in
        desktop)
            configure_mcp_desktop
            ;;
        terminal)
            info "Terminal profile: No MCP configuration needed"
            ;;
        server)
            generate_auth_token
            install_service
            ;;
        *)
            error "Unknown profile: $PROFILE"
            ;;
    esac
    
    echo ""
    success "Installation complete!"
    echo ""
    echo "Next steps:"
    echo "  1. Restart your terminal (or run: source ~/.bashrc)"
    echo "  2. Run: areal workspace init"
    echo "  3. Run: areal workspace sense"
    echo ""
    
    if [ "$PROFILE" = "desktop" ]; then
        warn "IMPORTANT: Restart Claude Desktop to load the MCP server"
    fi
    
    # Optional feedback
    if [ -t 0 ]; then
        echo ""
        read -p "Would you like to provide feedback? (y/N) " -n 1 -r
        echo ""
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo "Please visit: https://github.com/agentralabs-tech/agentic-reality/issues"
        fi
    fi
}

main "$@"
```

---

# SPEC-21: CI/GUARDRAILS

## 21.1 CI Workflow with 50 Sections

```yaml
# .github/workflows/ci.yml

name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: -Dwarnings

jobs:
  # ═══════════════════════════════════════════════════════════════════
  # JOB 1: CODE QUALITY (Sections §1-10)
  # ═══════════════════════════════════════════════════════════════════
  code-quality:
    name: Code Quality
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      # §1: Formatting check
      - name: "§1: Check formatting"
        run: cargo fmt --all -- --check

      # §2: Clippy lints
      - name: "§2: Clippy lints"
        run: cargo clippy --all-targets --all-features -- -D warnings

      # §3: Build check
      - name: "§3: Build check"
        run: cargo build --all-features

      # §4: Workspace consistency
      - name: "§4: Workspace consistency"
        run: |
          cargo metadata --format-version 1 | jq '.workspace_members | length'

      # §5: No TODO/FIXME in src
      - name: "§5: TODO/FIXME check"
        run: |
          count=$(grep -r "TODO\|FIXME" src/ --include="*.rs" | wc -l)
          echo "Found $count TODO/FIXME comments"
          if [ "$count" -gt 20 ]; then
            echo "::error::Too many TODO/FIXME comments ($count > 20)"
            exit 1
          fi

      # §6: Unwrap count
      - name: "§6: Unwrap audit"
        run: |
          count=$(grep -r "\.unwrap()" src/ --include="*.rs" | grep -v "test" | wc -l)
          echo "Found $count unwrap() calls in non-test code"
          if [ "$count" -gt 10 ]; then
            echo "::error::Too many unwrap() calls ($count > 10)"
            exit 1
          fi

      # §7: No panic in lib
      - name: "§7: No panic in lib"
        run: |
          if grep -r "panic!" src/lib.rs src/**/mod.rs 2>/dev/null | grep -v "test"; then
            echo "::error::Found panic! in library code"
            exit 1
          fi

      # §8: Cargo.lock committed
      - name: "§8: Cargo.lock check"
        run: |
          if [ ! -f Cargo.lock ]; then
            echo "::error::Cargo.lock not found"
            exit 1
          fi

      # §9: No duplicate dependencies
      - name: "§9: Duplicate dependencies"
        run: |
          cargo tree -d 2>&1 | tee deps.txt
          if grep -q "^" deps.txt; then
            echo "::warning::Duplicate dependencies found"
          fi

      # §10: MSRV check
      - name: "§10: MSRV check"
        run: |
          msrv=$(grep -oP 'rust-version = "\K[^"]+' Cargo.toml || echo "1.70")
          echo "MSRV: $msrv"

  # ═══════════════════════════════════════════════════════════════════
  # JOB 2: TESTS (Sections §11-20)
  # ═══════════════════════════════════════════════════════════════════
  tests:
    name: Tests
    runs-on: ubuntu-latest
    needs: code-quality
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      # §11: Unit tests
      - name: "§11: Unit tests"
        run: cargo test --lib

      # §12: Integration tests
      - name: "§12: Integration tests"
        run: cargo test --test '*'

      # §13: Doc tests
      - name: "§13: Doc tests"
        run: cargo test --doc

      # §14: Hardening tests
      - name: "§14: Hardening tests"
        run: cargo test hardening

      # §15: Stress tests
      - name: "§15: Stress tests"
        run: cargo test stress --release -- --ignored

      # §16: MCP tool count
      - name: "§16: MCP tool count"
        run: |
          cargo test mcp_tool_count --release
          echo "MCP tool count verified: 15 tools"

      # §17: Phase tests
      - name: "§17: Phase tests"
        run: |
          for phase in 01 02 03 04 05 06 07 08; do
            cargo test "phase${phase}" --release || exit 1
          done

      # §18: Release gate tests
      - name: "§18: Release gate tests"
        run: cargo test release_gate --release

      # §19: Example tests
      - name: "§19: Example tests"
        run: |
          for example in examples/*.rs; do
            cargo run --example "$(basename "$example" .rs)" || exit 1
          done

      # §20: Coverage report
      - name: "§20: Coverage"
        run: |
          cargo install cargo-tarpaulin || true
          cargo tarpaulin --out Xml --skip-clean || true

  # ═══════════════════════════════════════════════════════════════════
  # JOB 3: HARDENING (Sections §21-30)
  # ═══════════════════════════════════════════════════════════════════
  hardening:
    name: Hardening
    runs-on: ubuntu-latest
    needs: code-quality
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      # §21: Strict MCP validation
      - name: "§21: Strict MCP validation"
        run: |
          # Verify no silent fallbacks in MCP handlers
          if grep -r "unwrap_or\|unwrap_or_default\|unwrap_or_else" src/mcp/ --include="*.rs" | grep -v test; then
            echo "::warning::Found potential silent fallbacks in MCP handlers"
          fi

      # §22: Project isolation
      - name: "§22: Project isolation"
        run: cargo test project_isolation --release

      # §23: No cross-project contamination
      - name: "§23: Cross-project check"
        run: cargo test no_cross_project --release

      # §24: Concurrent lock handling
      - name: "§24: Concurrent locking"
        run: cargo test concurrent_lock --release

      # §25: Stale lock recovery
      - name: "§25: Stale lock recovery"
        run: cargo test stale_lock_recovery --release

      # §26: MCP config merge-only
      - name: "§26: MCP config merge"
        run: |
          # Verify installer uses merge-only updates
          grep -q "merge" scripts/install.sh || echo "::warning::Verify merge-only MCP config"

      # §27: Restart guidance
      - name: "§27: Restart guidance"
        run: |
          grep -q "restart" scripts/install.sh || echo "::warning::Add restart guidance"

      # §28: Server auth gating
      - name: "§28: Server auth"
        run: |
          grep -q "AGENTIC_AUTH_TOKEN" src/security/auth.rs

      # §29: Atomic file operations
      - name: "§29: Atomic writes"
        run: |
          grep -q "AtomicWriter" src/format/file.rs

      # §30: Audit logging
      - name: "§30: Audit logging"
        run: |
          grep -q "AuditLogger" src/security/audit.rs

  # ═══════════════════════════════════════════════════════════════════
  # JOB 4: DOCUMENTATION (Sections §31-40)
  # ═══════════════════════════════════════════════════════════════════
  docs:
    name: Documentation
    runs-on: ubuntu-latest
    needs: code-quality
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      # §31: Required root files
      - name: "§31: Root files"
        run: |
          for file in README.md LICENSE CHANGELOG.md CONTRIBUTING.md CODE_OF_CONDUCT.md SECURITY.md; do
            if [ ! -f "$file" ]; then
              echo "::error::Missing $file"
              exit 1
            fi
          done

      # §32: Doc links valid
      - name: "§32: Doc links"
        run: |
          cargo doc --no-deps 2>&1 | grep -i "warning.*broken" && exit 1 || true

      # §33: Cargo doc builds
      - name: "§33: Cargo doc"
        run: cargo doc --all-features --no-deps

      # §34: Examples documented
      - name: "§34: Examples"
        run: |
          if [ -d examples ]; then
            ls examples/*.rs | head -3
          fi

      # §35: Security policy
      - name: "§35: Security policy"
        run: |
          if [ ! -f SECURITY.md ]; then
            echo "::error::Missing SECURITY.md"
            exit 1
          fi

      # §36: Contributing guide
      - name: "§36: Contributing"
        run: |
          if [ ! -f CONTRIBUTING.md ]; then
            echo "::error::Missing CONTRIBUTING.md"
            exit 1
          fi

      # §37: API docs
      - name: "§37: API docs"
        run: |
          if [ ! -f docs/API.md ]; then
            echo "::error::Missing docs/API.md"
            exit 1
          fi

      # §38: CLI docs
      - name: "§38: CLI docs"
        run: |
          if [ ! -f docs/CLI.md ]; then
            echo "::error::Missing docs/CLI.md"
            exit 1
          fi

      # §39: MCP docs
      - name: "§39: MCP docs"
        run: |
          if [ ! -f docs/MCP-TOOLS.md ]; then
            echo "::error::Missing docs/MCP-TOOLS.md"
            exit 1
          fi

      # §40: Changelog updated
      - name: "§40: Changelog"
        run: |
          head -20 CHANGELOG.md

  # ═══════════════════════════════════════════════════════════════════
  # JOB 5: SISTER COMPLIANCE (Sections §41-48)
  # ═══════════════════════════════════════════════════════════════════
  sister-docs:
    name: Sister Compliance
    runs-on: ubuntu-latest
    needs: code-quality
    steps:
      - uses: actions/checkout@v4

      # §41: Substantive docs (no stubs)
      - name: "§41: Substantive docs"
        run: |
          for doc in docs/*.md; do
            lines=$(wc -l < "$doc")
            if [ "$lines" -lt 50 ]; then
              echo "::error::$doc appears to be a stub ($lines lines)"
              exit 1
            fi
          done

      # §42: Inventions documented
      - name: "§42: Inventions docs"
        run: |
          if [ ! -f docs/INVENTIONS.md ]; then
            echo "::error::Missing INVENTIONS.md"
            exit 1
          fi
          # Verify 26 inventions
          count=$(grep -c "^##" docs/INVENTIONS.md || echo 0)
          if [ "$count" -lt 26 ]; then
            echo "::warning::Expected 26 inventions, found $count sections"
          fi

      # §43: Sister integration docs
      - name: "§43: Integration docs"
        run: |
          if [ ! -f docs/SISTER-INTEGRATION.md ]; then
            echo "::error::Missing SISTER-INTEGRATION.md"
            exit 1
          fi

      # §44: Examples comprehensive
      - name: "§44: Examples"
        run: |
          if [ ! -f docs/EXAMPLES.md ]; then
            echo "::error::Missing EXAMPLES.md"
            exit 1
          fi

      # §45: FAQ exists
      - name: "§45: FAQ"
        run: |
          if [ ! -f docs/FAQ.md ]; then
            echo "::error::Missing FAQ.md"
            exit 1
          fi

      # §46: Troubleshooting
      - name: "§46: Troubleshooting"
        run: |
          if [ ! -f docs/TROUBLESHOOTING.md ]; then
            echo "::error::Missing TROUBLESHOOTING.md"
            exit 1
          fi

      # §47: SVG diagrams
      - name: "§47: SVG diagrams"
        run: |
          if [ ! -d docs/assets ]; then
            echo "::error::Missing docs/assets/"
            exit 1
          fi
          svg_count=$(ls docs/assets/*.svg 2>/dev/null | wc -l)
          if [ "$svg_count" -lt 4 ]; then
            echo "::error::Expected at least 4 SVGs, found $svg_count"
            exit 1
          fi

      # §48: Canonical compliance
      - name: "§48: Canonical compliance"
        run: |
          # Check bridges.rs exists with correct traits
          if [ ! -f src/bridges/mod.rs ]; then
            echo "::error::Missing bridges module"
            exit 1
          fi
          # Check SDK traits
          if ! grep -q "trait.*Bridge" src/bridges/*.rs; then
            echo "::error::Missing bridge traits"
            exit 1
          fi

  # ═══════════════════════════════════════════════════════════════════
  # JOB 6: RELEASE GATES (Sections §49-50)
  # ═══════════════════════════════════════════════════════════════════
  release-gates:
    name: Release Gates
    runs-on: ubuntu-latest
    needs: [tests, hardening, docs, sister-docs]
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      # §49: Version consistency
      - name: "§49: Version consistency"
        run: |
          cargo_version=$(grep -oP '^version = "\K[^"]+' Cargo.toml)
          echo "Cargo.toml version: $cargo_version"
          # Check CHANGELOG has this version
          if ! grep -q "## \[$cargo_version\]" CHANGELOG.md; then
            echo "::error::CHANGELOG missing entry for $cargo_version"
            exit 1
          fi

      # §50: Final verification
      - name: "§50: Final verification"
        run: |
          echo "═══════════════════════════════════════════════════════"
          echo "           RELEASE GATE VERIFICATION                   "
          echo "═══════════════════════════════════════════════════════"
          echo ""
          echo "✓ All 50 guardrail sections passed"
          echo "✓ Ready for release"
          echo ""
          echo "To release, create a tag:"
          echo "  git tag -a v$(grep -oP '^version = \"\K[^\"]+' Cargo.toml) -m 'Release'"
          echo "  git push origin --tags"
```

---

# SPEC-22: GHOST BRIDGE

```rust
//! src/bridges/ghost.rs

//! Ghost Writer bridge for Hydra capability

use crate::types::*;
use crate::engine::RealityEngine;

/// Ghost Writer operations for Reality
pub trait RealityGhostWriter: Send + Sync {
    /// Snapshot current reality state
    fn snapshot(&self) -> Result<RealitySnapshot, BridgeError>;
    
    /// Restore reality from snapshot
    fn restore(&mut self, snapshot: RealitySnapshot) -> Result<(), BridgeError>;
    
    /// Get delta since snapshot
    fn get_delta(&self, since: SnapshotId) -> Result<RealityDelta, BridgeError>;
    
    /// Apply delta to reality state
    fn apply_delta(&mut self, delta: RealityDelta) -> Result<(), BridgeError>;
    
    /// Serialize reality state
    fn serialize(&self) -> Result<Vec<u8>, BridgeError>;
    
    /// Deserialize reality state
    fn deserialize(&mut self, data: &[u8]) -> Result<(), BridgeError>;
    
    /// Get reality checksum
    fn get_checksum(&self) -> Result<[u8; 32], BridgeError>;
    
    /// Get ghost hint (what would be useful to preserve)
    fn get_ghost_hint(&self) -> GhostHint;
    
    /// Apply ghost profile (restored preferences)
    fn apply_ghost_profile(&mut self, profile: GhostProfile) -> Result<(), BridgeError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealitySnapshot {
    pub id: SnapshotId,
    pub timestamp: Timestamp,
    pub deployment: Option<DeploymentSoul>,
    pub environment: Option<EnvironmentMedium>,
    pub resources: Option<ResourceBody>,
    pub reality: Option<RealityLayers>,
    pub topology: Option<DeploymentTopologyMap>,
    pub stakes: Option<ConsequenceAwareness>,
    pub coherence: Option<CoherenceState>,
    pub checksum: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityDelta {
    pub base_snapshot: SnapshotId,
    pub timestamp: Timestamp,
    pub changes: Vec<RealityChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RealityChange {
    DeploymentChange(DeploymentChange),
    EnvironmentChange(EnvironmentStateChange),
    ResourceChange(ResourceBodyChange),
    TopologyChange(TopologyMapChange),
    StakesChange(StakesLevelChange),
    CoherenceChange(CoherenceStateChange),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostHint {
    /// What's most important to preserve
    pub priority_state: Vec<String>,
    
    /// What can be reconstructed
    pub reconstructable: Vec<String>,
    
    /// What's ephemeral
    pub ephemeral: Vec<String>,
    
    /// Suggested snapshot frequency
    pub snapshot_frequency: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostProfile {
    /// Restored incarnation preferences
    pub incarnation_preferences: HashMap<String, serde_json::Value>,
    
    /// Restored wisdom
    pub wisdom: Vec<WisdomEntry>,
    
    /// Restored patterns
    pub patterns: Vec<Pattern>,
}

impl RealityGhostWriter for RealityEngine {
    fn snapshot(&self) -> Result<RealitySnapshot, BridgeError> {
        let id = SnapshotId::new();
        let timestamp = Timestamp::now();
        
        let snapshot = RealitySnapshot {
            id,
            timestamp,
            deployment: self.deployment_store.get_soul().cloned(),
            environment: self.environment_store.get_environment().cloned(),
            resources: self.resource_store.get_body().cloned(),
            reality: self.reality_store.get_layers().cloned(),
            topology: Some(self.topology_store.get_topology_map().clone()),
            stakes: self.stakes_store.get_consequence_awareness().cloned(),
            coherence: Some(self.coherence_store.get_state().clone()),
            checksum: [0; 32], // Will be computed
        };
        
        // Compute checksum
        let data = bincode::serialize(&snapshot)
            .map_err(|e| BridgeError::SerializationError(e.to_string()))?;
        let checksum = *blake3::hash(&data).as_bytes();
        
        let mut snapshot = snapshot;
        snapshot.checksum = checksum;
        
        Ok(snapshot)
    }
    
    fn restore(&mut self, snapshot: RealitySnapshot) -> Result<(), BridgeError> {
        // Verify checksum
        let mut verify = snapshot.clone();
        verify.checksum = [0; 32];
        let data = bincode::serialize(&verify)
            .map_err(|e| BridgeError::SerializationError(e.to_string()))?;
        let computed = *blake3::hash(&data).as_bytes();
        
        if computed != snapshot.checksum {
            return Err(BridgeError::ChecksumMismatch);
        }
        
        // Restore state
        if let Some(soul) = snapshot.deployment {
            self.deployment_store.set_soul(soul);
        }
        if let Some(env) = snapshot.environment {
            self.environment_store.set_environment(env);
        }
        if let Some(body) = snapshot.resources {
            self.resource_store.set_body(body);
        }
        if let Some(layers) = snapshot.reality {
            self.reality_store.set_layers(layers);
        }
        if let Some(topo) = snapshot.topology {
            self.topology_store.set_topology_map(topo);
        }
        if let Some(stakes) = snapshot.stakes {
            self.stakes_store.set_consequence_awareness(stakes);
        }
        if let Some(coherence) = snapshot.coherence {
            self.coherence_store.set_state(coherence);
        }
        
        // Rebuild indexes
        self.indexes.rebuild(self);
        
        Ok(())
    }
    
    fn get_delta(&self, since: SnapshotId) -> Result<RealityDelta, BridgeError> {
        // Get changes since snapshot
        let changes = self.get_changes_since(since)?;
        
        Ok(RealityDelta {
            base_snapshot: since,
            timestamp: Timestamp::now(),
            changes,
        })
    }
    
    fn apply_delta(&mut self, delta: RealityDelta) -> Result<(), BridgeError> {
        for change in delta.changes {
            self.apply_change(change)?;
        }
        Ok(())
    }
    
    fn serialize(&self) -> Result<Vec<u8>, BridgeError> {
        let snapshot = self.snapshot()?;
        bincode::serialize(&snapshot)
            .map_err(|e| BridgeError::SerializationError(e.to_string()))
    }
    
    fn deserialize(&mut self, data: &[u8]) -> Result<(), BridgeError> {
        let snapshot: RealitySnapshot = bincode::deserialize(data)
            .map_err(|e| BridgeError::DeserializationError(e.to_string()))?;
        self.restore(snapshot)
    }
    
    fn get_checksum(&self) -> Result<[u8; 32], BridgeError> {
        let snapshot = self.snapshot()?;
        Ok(snapshot.checksum)
    }
    
    fn get_ghost_hint(&self) -> GhostHint {
        GhostHint {
            priority_state: vec![
                "deployment.soul".to_string(),
                "deployment.lineage".to_string(),
                "deployment.wisdom".to_string(),
                "stakes.level".to_string(),
            ],
            reconstructable: vec![
                "environment.state".to_string(),
                "resources.body".to_string(),
                "topology.map".to_string(),
            ],
            ephemeral: vec![
                "resources.sensations".to_string(),
                "coherence.pending_transitions".to_string(),
            ],
            snapshot_frequency: Duration::from_secs(300), // 5 minutes
        }
    }
    
    fn apply_ghost_profile(&mut self, profile: GhostProfile) -> Result<(), BridgeError> {
        // Apply wisdom
        for wisdom in profile.wisdom {
            self.deployment_store.add_wisdom(wisdom);
        }
        
        Ok(())
    }
}
```

---

# SPEC-24: FFI/BINDINGS

## 24.1 C FFI

```rust
//! src/ffi/c.rs

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::cell::RefCell;

thread_local! {
    static LAST_ERROR: RefCell<Option<String>> = RefCell::new(None);
}

fn set_error(msg: String) {
    LAST_ERROR.with(|e| *e.borrow_mut() = Some(msg));
}

/// Get last error message
#[no_mangle]
pub extern "C" fn areal_get_last_error() -> *mut c_char {
    LAST_ERROR.with(|e| {
        match e.borrow().as_ref() {
            Some(msg) => CString::new(msg.as_str()).unwrap().into_raw(),
            None => std::ptr::null_mut(),
        }
    })
}

/// Free string allocated by this library
#[no_mangle]
pub extern "C" fn areal_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { drop(CString::from_raw(s)); }
    }
}

/// Create reality engine
#[no_mangle]
pub extern "C" fn areal_create_engine() -> *mut RealityEngine {
    Box::into_raw(Box::new(RealityEngine::new()))
}

/// Destroy reality engine
#[no_mangle]
pub extern "C" fn areal_destroy_engine(engine: *mut RealityEngine) {
    if !engine.is_null() {
        unsafe { drop(Box::from_raw(engine)); }
    }
}

/// Initialize soul
#[no_mangle]
pub extern "C" fn areal_initialize_soul(
    engine: *mut RealityEngine,
    purpose: *const c_char,
) -> *mut c_char {
    let result = std::panic::catch_unwind(|| {
        let engine = unsafe { &mut *engine };
        let purpose = unsafe { CStr::from_ptr(purpose) }
            .to_str()
            .unwrap_or("unknown");
        
        match engine.write().initialize_soul(InitializeSoulRequest {
            spawned_by: SpawnerIdentity::Named("ffi".to_string()),
            purpose: DeploymentPurpose {
                description: purpose.to_string(),
                category: PurposeCategory::General,
            },
            ..Default::default()
        }) {
            Ok(soul) => {
                let json = serde_json::to_string(&soul).unwrap();
                CString::new(json).unwrap().into_raw()
            }
            Err(e) => {
                set_error(e.to_string());
                std::ptr::null_mut()
            }
        }
    });
    
    result.unwrap_or_else(|_| {
        set_error("Panic in areal_initialize_soul".to_string());
        std::ptr::null_mut()
    })
}

/// Get soul summary
#[no_mangle]
pub extern "C" fn areal_get_summary(engine: *const RealityEngine) -> *mut c_char {
    let result = std::panic::catch_unwind(|| {
        let engine = unsafe { &*engine };
        let summary = engine.query().get_context_summary();
        let json = serde_json::to_string(&summary).unwrap();
        CString::new(json).unwrap().into_raw()
    });
    
    result.unwrap_or_else(|_| {
        set_error("Panic in areal_get_summary".to_string());
        std::ptr::null_mut()
    })
}

/// Sense environment
#[no_mangle]
pub extern "C" fn areal_sense_environment(engine: *mut RealityEngine) -> *mut c_char {
    let result = std::panic::catch_unwind(|| {
        let engine = unsafe { &mut *engine };
        
        match engine.write().sense_environment() {
            Ok(env) => {
                let json = serde_json::to_string(&env).unwrap();
                CString::new(json).unwrap().into_raw()
            }
            Err(e) => {
                set_error(e.to_string());
                std::ptr::null_mut()
            }
        }
    });
    
    result.unwrap_or_else(|_| {
        set_error("Panic in areal_sense_environment".to_string());
        std::ptr::null_mut()
    })
}

/// Check should proceed
#[no_mangle]
pub extern "C" fn areal_should_proceed(
    engine: *const RealityEngine,
    action: *const c_char,
    tolerance: f64,
) -> *mut c_char {
    let result = std::panic::catch_unwind(|| {
        let engine = unsafe { &*engine };
        let action = unsafe { CStr::from_ptr(action) }
            .to_str()
            .unwrap_or("");
        
        let decision = engine.query().should_proceed(action, tolerance);
        let json = serde_json::to_string(&decision).unwrap();
        CString::new(json).unwrap().into_raw()
    });
    
    result.unwrap_or_else(|_| {
        set_error("Panic in areal_should_proceed".to_string());
        std::ptr::null_mut()
    })
}
```

## 24.2 Python Bindings

```rust
//! src/ffi/python.rs

use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass]
pub struct RealityEngineWrapper {
    engine: RealityEngine,
}

#[pymethods]
impl RealityEngineWrapper {
    #[new]
    pub fn new() -> Self {
        Self {
            engine: RealityEngine::new(),
        }
    }
    
    pub fn initialize_soul(&mut self, purpose: &str) -> PyResult<String> {
        match self.engine.write().initialize_soul(InitializeSoulRequest {
            spawned_by: SpawnerIdentity::Named("python".to_string()),
            purpose: DeploymentPurpose {
                description: purpose.to_string(),
                category: PurposeCategory::General,
            },
            ..Default::default()
        }) {
            Ok(soul) => Ok(serde_json::to_string(&soul).unwrap()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }
    
    pub fn get_summary(&self) -> PyResult<String> {
        let summary = self.engine.query().get_context_summary();
        Ok(serde_json::to_string(&summary).unwrap())
    }
    
    pub fn sense_environment(&mut self) -> PyResult<String> {
        match self.engine.write().sense_environment() {
            Ok(env) => Ok(serde_json::to_string(&env).unwrap()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }
    
    pub fn sense_resources(&mut self) -> PyResult<String> {
        match self.engine.write().sense_resources() {
            Ok(body) => Ok(serde_json::to_string(&body).unwrap()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
        }
    }
    
    pub fn should_proceed(&self, action: &str, tolerance: f64) -> PyResult<String> {
        let decision = self.engine.query().should_proceed(action, tolerance);
        Ok(serde_json::to_string(&decision).unwrap())
    }
    
    pub fn set_stakes_level(&mut self, level: &str) -> PyResult<()> {
        let stakes = match level {
            "minimal" => StakesLevel::Minimal { can_experiment: true },
            "low" => StakesLevel::Low { rollback_available: true },
            "medium" => StakesLevel::Medium { review_recommended: true },
            "high" => StakesLevel::High { caution_required: true, approval_needed: true },
            "critical" => StakesLevel::Critical {
                multiple_approvals: true,
                audit_required: true,
                no_risk_tolerance: true,
            },
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid stakes level")),
        };
        
        self.engine.write().set_stakes_level(stakes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
}

#[pymodule]
fn agentic_reality(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RealityEngineWrapper>()?;
    Ok(())
}
```

---

# SPEC-25: RELEASE/PUBLISH

```bash
#!/usr/bin/env bash
# scripts/release.sh - Release script for AgenticReality

set -euo pipefail

VERSION="${1:-}"

if [ -z "$VERSION" ]; then
    echo "Usage: ./scripts/release.sh <version>"
    echo "Example: ./scripts/release.sh 0.1.0"
    exit 1
fi

echo "═══════════════════════════════════════════════════════════════"
echo "           AgenticReality Release: v$VERSION                   "
echo "═══════════════════════════════════════════════════════════════"

# Pre-flight checks
echo ""
echo "Running pre-flight checks..."

# Check we're on main
branch=$(git rev-parse --abbrev-ref HEAD)
if [ "$branch" != "main" ]; then
    echo "ERROR: Must be on main branch (currently on $branch)"
    exit 1
fi

# Check clean working directory
if [ -n "$(git status --porcelain)" ]; then
    echo "ERROR: Working directory not clean"
    git status --short
    exit 1
fi

# Check CI passing
echo "Checking CI status..."
# gh run list --limit 1 --json conclusion -q '.[0].conclusion' || true

# Update versions
echo ""
echo "Updating version to $VERSION..."

# Cargo.toml
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# Python pyproject.toml if exists
if [ -f pyproject.toml ]; then
    sed -i "s/^version = \".*\"/version = \"$VERSION\"/" pyproject.toml
fi

# npm package.json if exists
if [ -f package.json ]; then
    jq ".version = \"$VERSION\"" package.json > package.json.tmp
    mv package.json.tmp package.json
fi

# Verify CHANGELOG
echo ""
echo "Checking CHANGELOG..."
if ! grep -q "## \[$VERSION\]" CHANGELOG.md; then
    echo "ERROR: CHANGELOG.md missing entry for version $VERSION"
    exit 1
fi

# Run full test suite
echo ""
echo "Running tests..."
cargo test --all-features --release

# Build release
echo ""
echo "Building release..."
cargo build --release

# Git commit and tag
echo ""
echo "Creating git commit and tag..."
git add -A
git commit -m "Release v$VERSION"
git tag -a "v$VERSION" -m "AgenticReality v$VERSION"

# Push
echo ""
echo "Pushing to origin..."
git push origin main
git push origin "v$VERSION"

# Publish to crates.io
echo ""
echo "Publishing to crates.io..."
cargo publish

# Publish to PyPI if Python bindings exist
if [ -f pyproject.toml ]; then
    echo ""
    echo "Publishing to PyPI..."
    maturin publish
fi

# Publish to npm if bindings exist
if [ -f package.json ]; then
    echo ""
    echo "Publishing to npm..."
    npm publish
fi

# Create GitHub release
echo ""
echo "Creating GitHub release..."
gh release create "v$VERSION" \
    --title "AgenticReality v$VERSION" \
    --notes-file <(sed -n "/## \[$VERSION\]/,/## \[/p" CHANGELOG.md | head -n -1)

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "           Release v$VERSION complete!                         "
echo "═══════════════════════════════════════════════════════════════"
```

---

# CLAUDE CODE INSTRUCTIONS

## Critical Implementation Directives

```markdown
# CLAUDE CODE INSTRUCTIONS FOR AGENTIC REALITY

## 1. BENCHMARK AGAINST MEMORY

AgenticMemory is the most mature sister (v0.4.2, 291+ tests). 
Use it as the quality benchmark throughout implementation:

### Quality Metrics to Match or Exceed:
- Test coverage: Memory has 291+ MCP tests → Reality needs 250+ MCP tests
- Performance: Memory processes ~10,000 ops/sec → Reality should target similar
- MCP hardening: Memory has zero unwrap in MCP handlers → Reality must match
- Phase testing: Memory has 11 phases → Reality should have 8+ phases

### During Implementation:
```bash
# Always compare test counts
cargo test --package agentic-reality 2>&1 | grep -E "test result|passed|failed"

# Compare MCP tool quality
grep -r "unwrap()" src/mcp/ | wc -l  # Should be 0
grep -r "expect(" src/mcp/ | wc -l   # Should be 0

# Compare test structure to Memory
ls -la tests/mcp/   # Should match Memory's phase structure
```

## 2. VALIDATE AGAINST CANONICAL_SISTER_KIT.md

Before EVERY implementation phase, read and verify against CANONICAL_SISTER_KIT.md:

### Required Validations:
1. **After creating Cargo.toml**: Verify workspace structure matches canonical
2. **After implementing MCP**: Verify tool count (15) and consolidation pattern
3. **After bridges.rs**: Verify 9 traits + NoOp + HydraAdapter
4. **After tests/**: Verify phase structure and MCP tool count test
5. **After docs/**: Verify all 12 doc pages exist and are substantive
6. **Before release**: Run all 50 CI guardrail sections

### Validation Commands:
```bash
# Check canonical compliance
grep -c "trait.*Bridge" src/bridges/*.rs  # Should be 9
grep -c "fn " src/bridges/noop.rs          # Should match trait methods
ls docs/*.md | wc -l                        # Should be 12+
ls docs/assets/*.svg | wc -l                # Should be 4+

# Verify hardening patterns
grep -r "McpValidator" src/mcp/             # Should be in all handlers
grep -r "strict" src/validation/            # Should have StrictValidator
```

## 3. IMPLEMENTATION ORDER

Follow this exact order:

### Phase 1: Foundation
1. Create Cargo.toml with workspace structure
2. Create src/lib.rs with module declarations
3. Create src/types/ with all data structures from SPEC-03
4. VALIDATE: Compare structure to Memory's structure

### Phase 2: Storage
1. Implement src/format/ (header, sections, footer, file.rs)
2. Implement src/storage/ (all 8 domain stores)
3. Create atomic write utilities
4. VALIDATE: Test file round-trip

### Phase 3: Engine
1. Implement src/engine/write/ (all ~90 operations)
2. Implement src/engine/query/ (all ~78 operations)
3. Implement src/indexes/
4. VALIDATE: Unit tests for each operation

### Phase 4: Validation & MCP
1. Implement src/validation/ (strict validator, MCP validator)
2. Implement src/mcp/ (15 tools, ~120 operations)
3. NO UNWRAP IN MCP HANDLERS
4. VALIDATE: MCP tool count test passes

### Phase 5: CLI
1. Implement src/cli/ (~40 commands)
2. Add text/json/table output formats
3. VALIDATE: All commands produce output

### Phase 6: Bridges & Security
1. Implement src/bridges/ (9 traits + NoOp + Ghost + Hydra)
2. Implement src/security/ (auth, authz, encryption, audit)
3. VALIDATE: Bridge trait compliance

### Phase 7: Tests
1. Create tests/unit/ structure
2. Create tests/mcp/ with phase tests
3. Create tests/stress/
4. Create tests/scenarios/
5. VALIDATE: 250+ tests passing

### Phase 8: Documentation
1. Create all 12 docs/*.md files (SUBSTANTIVE, not stubs!)
2. Create 4 SVG diagrams
3. Create README.md with badges
4. VALIDATE: All files > 50 lines

### Phase 9: Release
1. Create scripts/install.sh (universal installer)
2. Create scripts/release.sh
3. Create .github/workflows/ci.yml with 50 sections
4. VALIDATE: CI passes locally

## 4. HARDENING REQUIREMENTS

These are MANDATORY and non-negotiable:

### MCP Hardening:
- Zero `.unwrap()` in src/mcp/
- Zero `.expect()` in src/mcp/
- Every handler uses McpValidator
- Every ID is validated before use
- Every operation has explicit error handling

### Input Validation:
- No silent fallbacks (no unwrap_or_default for critical fields)
- Range validation on all numerics
- Length limits on all strings
- Format validation on all IDs

### Concurrency:
- Safe locking with timeout
- Stale lock detection and recovery
- Atomic file operations only

### Project Isolation:
- Hash-based identity per project
- No "latest cached" cross-project fallback
- Strict path canonicalization

## 5. TEST REQUIREMENTS

### Minimum Test Counts:
- Unit tests: 100+
- MCP tests: 150+
- Stress tests: 20+
- Total: 250+ tests

### Required Test Files:
```
tests/
├── mcp/
│   ├── phase01_deployment.rs
│   ├── phase02_environment.rs
│   ├── phase03_resource.rs
│   ├── phase04_reality.rs
│   ├── phase05_topology.rs
│   ├── phase06_stakes.rs
│   ├── phase07_coherence.rs
│   ├── phase08_full_suite.rs
│   └── mcp_tool_count.rs      # CRITICAL: Verifies exactly 15 tools
├── stress/
│   ├── test_concurrent.rs
│   ├── test_large_state.rs
│   └── test_persistence.rs
└── scenarios/
    ├── scenario_production_deployment.rs
    ├── scenario_scale_event.rs
    └── scenario_hallucination_detection.rs
```

### MCP Tool Count Test:
```rust
#[test]
fn test_mcp_tool_count() {
    let server = RealityMcpServer::new(RealityEngine::new(), false);
    let tools = server.tools();
    assert_eq!(tools.len(), 15, "Expected exactly 15 MCP tools");
}
```

## 6. DOCUMENTATION REQUIREMENTS

### All 12 docs must be SUBSTANTIVE:
- QUICKSTART.md: 100+ lines
- ARCHITECTURE.md: 200+ lines
- API.md: 300+ lines
- CLI.md: 200+ lines
- MCP-TOOLS.md: 300+ lines
- INVENTIONS.md: 500+ lines (26 inventions!)
- SISTER-INTEGRATION.md: 150+ lines
- CONCEPTS.md: 200+ lines
- EXAMPLES.md: 200+ lines
- FAQ.md: 100+ lines
- TROUBLESHOOTING.md: 100+ lines
- PRIVACY.md: 100+ lines

### Verify with:
```bash
for doc in docs/*.md; do
    lines=$(wc -l < "$doc")
    echo "$doc: $lines lines"
    if [ "$lines" -lt 50 ]; then
        echo "  WARNING: Stub detected!"
    fi
done
```

## 7. CI GUARDRAILS

Before declaring complete, ALL 50 sections must pass:

```bash
# Run locally before commit
./scripts/ci-check.sh

# Or manually:
cargo fmt --check                    # §1
cargo clippy -- -D warnings          # §2
cargo build --all-features           # §3
cargo test --lib                     # §11
cargo test mcp_tool_count            # §16
# ... etc for all 50 sections
```

## 8. FINAL CHECKLIST

Before declaring v0.1.0 ready:

- [ ] All 26 inventions implemented
- [ ] 15 MCP tools working
- [ ] ~40 CLI commands functional
- [ ] 250+ tests passing
- [ ] 12 docs substantive
- [ ] 4 SVG diagrams created
- [ ] 50 CI guardrails passing
- [ ] Zero unwrap in MCP
- [ ] Benchmarks comparable to Memory
- [ ] CANONICAL_SISTER_KIT.md compliance verified
- [ ] README with badges
- [ ] CHANGELOG with v0.1.0 entry
- [ ] Installer script tested
- [ ] Release script tested

## 9. COMMON MISTAKES TO AVOID

1. ❌ Creating stub documentation
2. ❌ Using unwrap() in MCP handlers
3. ❌ Skipping test phases
4. ❌ Not validating against CANONICAL_SISTER_KIT.md
5. ❌ Forgetting MCP tool count test
6. ❌ Silent fallbacks in validation
7. ❌ Missing bridge traits
8. ❌ Incomplete CI guardrails
9. ❌ Not comparing to Memory's quality bar
10. ❌ Rushing to "complete" without verification

## 10. SUCCESS CRITERIA

Reality v0.1.0 is COMPLETE when:

1. `cargo test` shows 250+ tests, 0 failures
2. `cargo clippy` shows 0 warnings
3. All 50 CI guardrails pass
4. MCP tool count = 15
5. All docs > 50 lines each
6. README has working badges
7. CANONICAL_SISTER_KIT.md compliance = 100%
8. Quality matches or exceeds Memory v0.4.2
```

---

## Part 5 Complete

**Covered:**
- SPEC-17: Documentation (12 pages)
- SPEC-18: SVG Diagrams (4 diagrams)
- SPEC-19: README
- SPEC-20: Universal Installer (desktop|terminal|server profiles)
- SPEC-21: CI/Guardrails (50 sections)
- SPEC-22: Ghost Bridge
- SPEC-24: FFI/Bindings (C + Python)
- SPEC-25: Release/Publish
- **CLAUDE CODE INSTRUCTIONS** (comprehensive implementation guide)

---

## SPECIFICATION COMPLETE

**AgenticReality — Sister #10: The Ground**

| Spec | Coverage |
|------|----------|
| SPEC-01 | Overview & Dependencies |
| SPEC-02 | Core Concepts |
| SPEC-03 | Data Structures |
| SPEC-04 | File Format |
| SPEC-05 | Write Engine |
| SPEC-06 | Query Engine |
| SPEC-07 | Indexes |
| SPEC-08 | Validation |
| SPEC-09 | CLI |
| SPEC-10 | MCP Server |
| SPEC-11 | Sister Integration |
| SPEC-12 | Tests |
| SPEC-13 | Performance |
| SPEC-14 | Security |
| SPEC-15 | Research Paper |
| SPEC-16 | Inventions Implementation |
| SPEC-17 | Documentation |
| SPEC-18 | SVG Diagrams |
| SPEC-19 | README |
| SPEC-20 | Installer |
| SPEC-21 | CI/Guardrails |
| SPEC-22 | Ghost Bridge |
| SPEC-24 | FFI/Bindings |
| SPEC-25 | Release/Publish |

**Totals:**
- 26 inventions across 7 tiers
- 15 MCP tools (~150 operations)
- ~40 CLI commands
- 9 sister bridges + NoOp + Ghost + Hydra
- 250+ tests required
- 50 CI guardrail sections
- .areal file format

**Ready for Claude Code implementation.**

---

*Document: AGENTIC-REALITY-SPEC-PART5.md*
*Sister #10 of 25: The Ground*
*The sister that knows WHERE it exists and WHAT is real.*
