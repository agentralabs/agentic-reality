# AgenticReality Troubleshooting

> Common issues and solutions

## Installation Issues

### Binary not found after install

**Symptom:** `command not found: areal` after running the installer.

**Solution:** The installer adds the binary path to your shell RC file, but
the current session does not pick it up automatically. Restart your terminal
or source your RC file:

```bash
source ~/.bashrc   # or ~/.zshrc
```

Verify the binary is in your PATH:

```bash
which areal
# Should output: /home/user/.agentic/bin/areal
```

### Permission denied on install

**Symptom:** `Permission denied` when running the installer.

**Solution:** The installer writes to `~/.agentic/bin/`. Ensure your home
directory is writable. If using a custom `AGENTIC_INSTALL_DIR`, ensure that
directory is writable:

```bash
AGENTIC_INSTALL_DIR=/opt/agentic bash install.sh
```

### Build fails from source

**Symptom:** `cargo build` fails with dependency errors.

**Solution:** Ensure Rust 1.70+ is installed. AgenticReality depends on
`agentic-sdk v0.2.0` which must be available. If building from the workspace,
ensure the SDK path is correct in `Cargo.toml`:

```bash
rustup update stable
cargo clean
cargo build --release
```

## Runtime Issues

### "No deployment soul initialized"

**Symptom:** Operations fail with `Error: No deployment soul initialized`.

**Solution:** The deployment soul must be initialized before any other
operations. This is always the first step:

```bash
areal soul init --spawned-by "manual" --purpose "my application"
```

Or via MCP:

```json
{ "tool": "reality_deployment", "arguments": { "operation": "initialize" } }
```

### "No environment sensed"

**Symptom:** Environment queries fail with `Error: No environment sensed`.

**Solution:** Run environment sensing after initializing the soul:

```bash
areal workspace sense
```

This senses both environment and resources. Alternatively, sense individually:

```bash
areal env sense
areal resource sense
```

### Stale .areal file

**Symptom:** Loaded state shows information from a previous deployment that
is no longer accurate.

**Solution:** The .areal file persists state from the last save. If the
deployment context has changed significantly, re-initialize:

```bash
areal workspace init    # Creates fresh .areal
areal workspace sense   # Re-senses everything
```

To preserve incarnation memory while re-initializing:

```bash
# The old incarnation becomes a "past life"
areal soul init --circumstances resurrected --purpose "updated context"
```

### Environment auto-detection is wrong

**Symptom:** The environment type is detected incorrectly (e.g., production
detected as development).

**Solution:** Set the `ENVIRONMENT` environment variable explicitly:

```bash
ENVIRONMENT=production areal workspace sense
```

Valid values: `production`, `staging`, `development`, `testing`, `sandbox`.

For CI/CD environments, set `CI=true` or the platform-specific variable
(e.g., `GITHUB_ACTIONS=true`).

### Resource sensing shows zeros

**Symptom:** Memory, CPU, or other resources show as 0 or unavailable.

**Solution:** Resource sensing uses system APIs that may require permissions
on some platforms. On Linux, ensure `/proc` and `/sys` are accessible. On
macOS, no special permissions are needed.

In containerized environments, resource limits may not be detectable if
cgroups v2 information is not exposed. Set resource limits explicitly:

```bash
areal resource sense --memory-total 4GB --cpu-cores 2
```

## MCP Issues

### MCP server does not start

**Symptom:** `areal serve` exits immediately or produces no output.

**Solution:** Check that no other process is using the same transport. For
stdio mode, ensure the process is launched correctly by the MCP client.
For HTTP mode, check the port is available:

```bash
# Check if port is in use
lsof -i :3010

# Start with explicit mode and logging
areal serve --mode http --port 3010 --log debug
```

### Authentication failures

**Symptom:** MCP calls return `AuthError::InvalidToken`.

**Solution:** Ensure the `AGENTIC_AUTH_TOKEN` environment variable is set
identically on both server and client:

```bash
# Server
AGENTIC_AUTH_TOKEN=my-secret-token areal serve --mode http --auth

# Client must use the same token
```

If you see `AuthError::RateLimited`, wait for the rate limit window to
expire (default: 60 seconds after 5 failed attempts).

### Tool returns "No deployment soul initialized"

**Symptom:** MCP tool calls return an error about missing soul.

**Solution:** Initialize the deployment via MCP before other operations:

```json
{
  "tool": "reality_deployment",
  "arguments": {
    "operation": "initialize",
    "spawned_by": "mcp_client",
    "purpose": "agent grounding"
  }
}
```

### Unknown tool error (-32803)

**Symptom:** Tool call returns error code -32803.

**Solution:** The tool name is not recognized. Verify the tool name is one
of the 15 supported tools:

```
reality_deployment, reality_memory, reality_environment, reality_resource,
reality_capability, reality_layer, reality_anchor, reality_hallucination,
reality_topology, reality_temporal, reality_stakes, reality_coherence,
reality_context, reality_ground, reality_workspace
```

## Coherence Issues

### Persistent coherence violations

**Symptom:** Coherence checks consistently report violations.

**Solution:** Identify which domains are contradicting. Common contradictions:

1. **Production environment + expendable nature** -- If in production, set
   expendability appropriately: `areal soul nature --expendability 0.1`

2. **Healthy network + failing dependencies** -- Re-sense network:
   `areal resource sense` and update dependency health

3. **Critical stakes + sandbox environment** -- Align stakes with
   environment: `areal stakes set medium`

### Context keeps shifting

**Symptom:** Context fingerprint shows constant shifts.

**Solution:** Some fingerprint components are naturally volatile (load, network
latency). Increase the shift detection threshold:

```bash
areal context shifted --threshold 0.5  # Higher = less sensitive
```

Or investigate which components are changing:

```bash
areal context diff
```

## Performance Issues

### Slow environment sensing

**Symptom:** `areal workspace sense` takes more than 1 second.

**Solution:** Environment sensing probes multiple sources. On slow networks
or in environments with many services, this can take longer. Use async
sensing for non-blocking operation, or limit the sensing scope:

```bash
areal env sense --skip-inhabitants --skip-boundaries
```

### Large .areal file

**Symptom:** The .areal file grows beyond expected size.

**Solution:** Large topologies (1000+ dependencies) or extensive incarnation
memory can increase file size. Prune old data:

```bash
areal workspace compact  # Remove historical data beyond retention period
```

---

*Part of the AgenticOS ecosystem by Agentra Labs*
