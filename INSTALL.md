# Installing AgenticReality

AgenticReality (Sister #10 -- The Ground) gives AI agents existential grounding:
deployment awareness, resource proprioception, reality physics, topology,
temporal grounding, stakes perception, and coherence maintenance.

This guide covers every supported installation method. Pick the one that fits
your workflow, or combine profiles as needed.

---

## Table of Contents

1. [Quick Install](#quick-install)
2. [Desktop Profile (Claude Desktop / MCP)](#desktop-profile)
3. [Terminal Profile (CLI Only)](#terminal-profile)
4. [Server Profile (Headless)](#server-profile)
5. [Build from Source](#build-from-source)
6. [Rust Crate Dependency](#rust-crate-dependency)
7. [Python FFI (Placeholder)](#python-ffi)
8. [Verifying the Installation](#verifying-the-installation)
9. [Troubleshooting](#troubleshooting)

---

## Quick Install

The default installer detects your platform and installs the desktop profile,
which includes both the CLI (`areal`) and the MCP server binary.

```bash
curl -fsSL https://agentralabs.tech/install/reality | bash
```

This is equivalent to the desktop profile below. It will:

- Download the latest release artifact for your OS and architecture.
- Place binaries in `~/.agentic-reality/bin/`.
- Merge MCP configuration into Claude Desktop's config (non-destructive).
- Fall back to building from source if no pre-built artifact exists.

After installation, restart Claude Desktop (or your MCP client) to pick up the
new server entry.

---

## Desktop Profile

Installs the MCP server plus the CLI, and configures Claude Desktop
automatically.

```bash
curl -fsSL https://agentralabs.tech/install/reality/desktop | bash
```

### What gets installed

| Component            | Location                                 |
|----------------------|------------------------------------------|
| `areal` CLI          | `~/.agentic-reality/bin/areal`           |
| MCP server binary    | `~/.agentic-reality/bin/agentic-reality-mcp` |
| Data directory       | `~/.agentic-reality/`                    |

### MCP client configuration

The installer merges the following entry into
`~/Library/Application Support/Claude/claude_desktop_config.json` (macOS)
or `~/.config/claude-desktop/config.json` (Linux):

```json
{
  "mcpServers": {
    "agentic-reality": {
      "command": "~/.agentic-reality/bin/agentic-reality-mcp",
      "args": ["--mode", "stdio"]
    }
  }
}
```

Existing keys are preserved. The merge is additive and never overwrites
unrelated server entries.

### Other MCP clients

If you use Codex, Cursor, Windsurf, VS Code with Cline, or any other
MCP-compatible client, point it at the MCP server binary with the same
command and args shown above. Consult your client's documentation for the
exact config location.

---

## Terminal Profile

Installs only the `areal` CLI. No MCP server binary, no config merging.

```bash
curl -fsSL https://agentralabs.tech/install/reality/terminal | bash
```

The CLI is placed in `~/.agentic-reality/bin/areal`. Add it to your PATH:

```bash
export PATH="$HOME/.agentic-reality/bin:$PATH"
```

### Quick test

```bash
areal --version
areal status
```

---

## Server Profile

Installs the MCP server for headless or remote operation with
authentication enabled.

```bash
curl -fsSL https://agentralabs.tech/install/reality/server | bash
```

### Authentication

Server mode requires a token. Set one of:

```bash
export AGENTIC_REALITY_AUTH_TOKEN="your-secret-token"
# or
export AGENTIC_TOKEN="your-secret-token"
```

Start the server in HTTP mode:

```bash
agentic-reality-mcp --mode http --port 3010
```

The server will reject unauthenticated requests when running in HTTP
mode with a token configured.

### systemd unit (Linux)

A sample unit file for persistent operation:

```ini
[Unit]
Description=AgenticReality MCP Server
After=network.target

[Service]
ExecStart=/home/deploy/.agentic-reality/bin/agentic-reality-mcp --mode http --port 3010
Environment=AGENTIC_REALITY_AUTH_TOKEN=changeme
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

---

## Build from Source

### Prerequisites

- Rust 1.75+ (install via https://rustup.rs)
- Git

### Steps

```bash
git clone https://github.com/agentralabs/agentic-reality.git
cd agentic-reality
cargo build --workspace --release
```

Binaries are placed in `target/release/`:

```
target/release/areal                    # CLI
target/release/agentic-reality-mcp      # MCP server
```

### Install locally

```bash
cargo install --path crates/agentic-reality-cli
```

Or use the Makefile:

```bash
make install
```

---

## Rust Crate Dependency

To use AgenticReality as a library in your own Rust project:

```toml
[dependencies]
agentic-reality = "0.1"
```

### Feature flags

| Feature      | Default | Description                            |
|--------------|---------|----------------------------------------|
| `cli`        | yes     | CLI infrastructure (clap)              |
| `format`     | yes     | .areal binary file format (LZ4)       |
| `ffi`        | yes     | C FFI bindings                         |
| `encryption` | no      | AES-256-GCM encryption for .areal     |

Disable defaults for a minimal library dependency:

```toml
[dependencies]
agentic-reality = { version = "0.1", default-features = false }
```

### Minimal example

```rust
use agentic_reality::engine::RealityEngine;

fn main() {
    let engine = RealityEngine::new();
    let reader = engine.reader();
    let summary = reader.get_context_summary();
    println!("{:?}", summary);
}
```

---

## Python FFI

> **Status: Placeholder.** Python bindings via the C FFI are planned but not
> yet available. The `agentic-reality-ffi` crate exposes a C ABI that can be
> loaded by ctypes or cffi. Documentation and a pip-installable wrapper will
> ship in a future release.

Planned usage:

```python
import agentic_reality

engine = agentic_reality.Engine()
engine.initialize()
print(engine.context_summary())
```

---

## Verifying the Installation

After any installation method, verify everything is working:

```bash
# Check CLI version
areal --version
# Expected: areal 0.1.0

# Check MCP server version
agentic-reality-mcp --version
# Expected: agentic-reality-mcp 0.1.0

# Quick status check
areal status

# Test MCP handshake (stdio)
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1.0"}}}' | agentic-reality-mcp
```

---

## Troubleshooting

### Binary not found after install

Ensure the bin directory is on your PATH:

```bash
export PATH="$HOME/.agentic-reality/bin:$PATH"
```

Add this line to `~/.bashrc`, `~/.zshrc`, or your shell's profile file.

### Claude Desktop does not show tools

1. Verify the config was merged:
   ```bash
   cat ~/Library/Application\ Support/Claude/claude_desktop_config.json
   ```
2. Restart Claude Desktop completely (quit and reopen).
3. Check the MCP server logs:
   ```bash
   agentic-reality-mcp 2>/tmp/reality-debug.log
   ```

### Build fails with missing dependencies

Make sure Rust is up to date:

```bash
rustup update stable
```

If you see linker errors on Linux, install build-essential:

```bash
sudo apt-get install build-essential pkg-config
```

### Permission denied

The installer needs write access to `~/.agentic-reality/`. If installing
globally, use `cargo install` which respects `$CARGO_HOME`.

### Server auth errors

In server profile, ensure the token environment variable is set before
starting the server:

```bash
export AGENTIC_REALITY_AUTH_TOKEN="your-token"
agentic-reality-mcp --mode http --port 3010
```

### Data directory conflicts

Each workspace derives its own identity from the canonical path. Two
projects in different directories will never share state, even if they
have the same folder name. To override the data directory:

```bash
export AGENTIC_REALITY_DATA_DIR=/path/to/custom/dir
```

### Reporting issues

Open an issue at https://github.com/agentralabs/agentic-reality/issues
with the output of:

```bash
areal --version
uname -a
rustc --version
```
