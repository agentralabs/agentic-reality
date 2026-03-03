# AgenticReality FAQ

> Frequently asked questions about AgenticReality

## General

### What is AgenticReality?

AgenticReality is Sister #10 ("The Ground") in the AgenticOS ecosystem. It
provides AI agents with existential grounding: awareness of where they are
running, what resources they have, what is real, what surrounds them, when
they are, what the stakes are, and how to stay coherent across context
changes.

### Why do agents need "reality grounding"?

AI agents today operate in a reality vacuum. They process requests without
knowing if they are in production or a test environment, whether they have
1GB or 100GB of memory, if their database is healthy, or what the stakes of
their actions are. This blindness leads to production bugs, resource
exhaustion, stale data being served as fresh, and catastrophic mistakes in
high-stakes environments.

### How is this different from monitoring?

Monitoring watches from the outside. Reality grounding is perception from the
inside. A monitored agent has dashboards about it. A reality-grounded agent
feels its own state. The difference is like reading a medical chart versus
feeling your own body.

### What are the 26 inventions?

The 26 inventions are novel concepts organized across 7 tiers:
deployment consciousness (5), resource proprioception (5), reality physics (4),
topology awareness (4), temporal grounding (3), stakes perception (3), and
coherence maintenance (2). See [INVENTIONS.md](./INVENTIONS.md) for details.

### What is the .areal file format?

The `.areal` format is a binary file that persists the complete reality state:
deployment soul, environment, resources, reality layers, topology, temporal
context, stakes, and coherence. Each section is individually compressed and
the file includes integrity checksums.

## Installation

### What platforms are supported?

macOS (x86_64, aarch64), Linux (x86_64, aarch64), and Windows via WSL.
Native Windows support is planned for a future release.

### What Rust version is required?

Rust 1.70 or later (MSRV).

### Can I use it without Rust?

Yes. The CLI binary `areal` can be used standalone. The FFI crate provides
C-compatible bindings for integration with other languages. The MCP server
can be used from any MCP-compatible client.

### Does it work offline?

Yes. AgenticReality does not require network access for core functionality.
Some features like time anchor verification benefit from network access
but degrade gracefully when offline.

## Architecture

### What are the four crates?

1. `agentic-reality` -- core library with types, engines, storage, validation
2. `agentic-reality-mcp` -- MCP server with 15 tools
3. `agentic-reality-cli` -- CLI binary (`areal`) with ~40 commands
4. `agentic-reality-ffi` -- C-compatible FFI bindings

### Does it require other Agentra sisters?

No. All sister bridge traits have NoOp default implementations, so
AgenticReality works standalone. However, integration with AgenticTime,
AgenticContract, and AgenticIdentity enhances functionality.

### How much memory does it use?

Base engine: < 10MB. Typical deployment: < 50MB. Large topology with 1000+
dependencies: < 200MB. Per MCP session overhead: < 5MB.

### What is the performance like?

Direct queries (get_soul, get_body) complete in < 100 microseconds. Sensing
operations (environment, resources) take < 100ms. File persistence takes
< 50ms for typical state. MCP tool calls complete in < 5ms for simple
operations.

## MCP Integration

### How many MCP tools are there?

15 tools covering all 7 reality domains plus workspace management and
cross-domain operations.

### How do I add it to Claude Desktop?

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

Then restart Claude Desktop.

### Can I run it as an HTTP server?

Yes. Use `areal serve --mode http --port 3010 --auth` for HTTP transport.
Set the `AGENTIC_AUTH_TOKEN` environment variable for authentication.

### What error code is used for unknown tools?

Error code `-32803` (TOOL_NOT_FOUND), per the MCP quality standard.

## Security

### Is data encrypted?

Optional AES-256-GCM encryption is available for .areal files. Set the
`AGENTIC_ENCRYPTION_KEY` environment variable with a 64-character hex key.

### How does authentication work?

Token-based authentication for MCP server mode. Set `AGENTIC_AUTH_TOKEN`
and use the `--auth` flag. Tokens are compared in constant time. Failed
attempts are rate-limited.

### Is there an audit trail?

Yes. All operations are logged with timestamps and session information.
Sensitive operations receive additional audit logging.

### What about sensitive data in .areal files?

Sensitive data fields are flagged in the format. Memory zeroization is
used for sensitive values in memory. See [PRIVACY.md](./PRIVACY.md).

## Usage

### What is the minimum viable reality?

Deployment soul plus environment sensing. These two provide the foundation
for all other reality domains. You can add more domains incrementally.

### Can I use it in CI/CD pipelines?

Yes. The environment auto-detection recognizes CI/CD environments
(GitHub Actions, GitLab CI, Jenkins, etc.) and sets appropriate environment
type, stakes level, and behavioral expectations.

### How do I handle restarts?

Save state before shutdown with `engine.save("app.areal")`. On restart,
load the previous incarnation's state and use incarnation memory to learn
from the past life. The current incarnation gets a new soul, but it
inherits wisdom from its predecessor.

### What happens if a coherence check fails?

A failed coherence check means contradictions exist between reality domains.
The recommended approach is to re-sense (environment and resources), re-verify
anchors, and re-run the coherence check. If incoherence persists, increase
caution and log the contradictions for investigation.

---

*Part of the AgenticOS ecosystem by Agentra Labs*
