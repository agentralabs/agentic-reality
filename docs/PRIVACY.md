# AgenticReality Privacy and Data Handling

> How AgenticReality collects, stores, and protects data

## Overview

AgenticReality senses and stores information about the operational environment
of AI agents. This document describes what data is collected, how it is stored,
what protections are in place, and how to control data handling.

## Data Collection

### What is collected

AgenticReality collects information about:

- **Deployment context** -- hostname, process ID, operating system, CPU
  architecture, cloud provider metadata, container runtime information
- **Environment** -- environment variables related to deployment configuration
  (ENVIRONMENT, CI, GITHUB_ACTIONS, etc.), not arbitrary environment variables
- **System resources** -- memory usage, CPU utilization, disk space, network
  statistics (from OS APIs, not from user data)
- **Topology** -- network latency measurements to configured dependencies,
  health check results for configured services
- **Temporal** -- system clock, NTP offsets, configured deadlines
- **Operational metadata** -- incarnation IDs, uptime, restart counts

### What is NOT collected

AgenticReality does NOT collect:

- User data or user-generated content
- Application-level data (database contents, API payloads)
- Authentication credentials (except its own auth token for MCP)
- Network traffic contents
- File system contents (beyond .areal state files)
- Browser history, cookies, or personal information
- Telemetry to external servers (no phone-home)

### Data minimization

Data collection follows the principle of minimization:

- Only data necessary for reality grounding is collected
- Environment variable scanning is limited to known deployment-related
  variables, not arbitrary env vars
- Resource sensing uses OS-level APIs, not application-level introspection
- Topology data only includes explicitly configured dependencies

## Data Storage

### .areal files

All reality state is stored in `.areal` binary files:

- **Location** -- the working directory where `areal workspace init` was run
- **Format** -- binary with per-section compression (LZ4)
- **Integrity** -- BLAKE3 checksums for each section and the overall file
- **Access** -- readable only by the user who created the file (mode 0600)

### Sensitive data handling

Certain fields are flagged as sensitive in the format:

- Deployment soul incarnation IDs
- Environment variables that might contain secrets
- Network topology details (service addresses, ports)

Sensitive fields receive:

1. **In-memory protection** -- memory zeroization on drop using the
   `SecureString` wrapper
2. **Storage protection** -- optional AES-256-GCM encryption for .areal files
3. **Logging protection** -- sensitive values are never written to log output

### Enabling encryption

To encrypt .areal files at rest:

```bash
# Generate a 256-bit key
openssl rand -hex 32 > ~/.agentic/encryption_key

# Set the environment variable
export AGENTIC_ENCRYPTION_KEY=$(cat ~/.agentic/encryption_key)

# Now all saves and loads use encryption
areal workspace init
```

The encryption key is never stored inside the .areal file. If the key is
lost, encrypted .areal files cannot be recovered.

## Authentication

### MCP server authentication

When running as an MCP server with `--auth`:

- Authentication uses a shared token set via `AGENTIC_AUTH_TOKEN`
- Token comparison uses constant-time equality to prevent timing attacks
- Failed authentication attempts are rate-limited (5 attempts per 60 seconds)
- Sessions are bound to prevent token reuse across clients
- Tokens are stored in memory using `SecureString` with zeroization on drop

### Token management

Best practices for token management:

```bash
# Generate a strong token
openssl rand -hex 32 > ~/.agentic/auth_token
chmod 600 ~/.agentic/auth_token

# Use environment variable, not command-line argument
export AGENTIC_AUTH_TOKEN=$(cat ~/.agentic/auth_token)

# Start server with auth
areal serve --mode http --auth
```

Do not:
- Pass tokens as command-line arguments (visible in process listings)
- Store tokens in version-controlled files
- Share tokens across environments
- Use weak or predictable tokens

## Authorization

### Permission model

Operations are authorized at two levels:

1. **Domain-level** -- read/write permissions per reality domain (deployment,
   environment, resource, reality, topology, stakes, coherence)
2. **Stakes-level** -- critical stakes require admin permissions for write
   operations

### Default permissions

| Mode | Read | Write | Admin |
|---|---|---|---|
| Unauthenticated (no --auth) | All | All | Yes |
| Authenticated (default) | All | All | Yes |
| Read-only mode | All | None | No |

## Audit Logging

### What is logged

All operations are logged to an audit trail:

- Operation name and parameters (sensitive values redacted)
- Timestamp
- Session ID
- Success or failure status
- Duration

### Audit log location

Audit logs are written to:

- `$AGENTIC_INSTALL_DIR/logs/audit.log` (server mode)
- Not written in library mode (caller controls logging)

### Log rotation

Audit logs rotate at 10MB with 5 generations retained by default. Configure
via environment:

```bash
AGENTIC_AUDIT_MAX_SIZE=10485760    # 10MB
AGENTIC_AUDIT_MAX_FILES=5          # 5 generations
```

## Network Communication

### Outbound connections

AgenticReality makes outbound connections only for:

- NTP time verification (when time anchors are configured)
- Dependency health checks (for configured topology entries)
- Cloud provider metadata (auto-detection, e.g., AWS IMDS at 169.254.169.254)

All outbound connections are:
- Initiated by the user (not automatic)
- Configurable and disablable
- Logged in the audit trail

### No telemetry

AgenticReality does not send telemetry, analytics, crash reports, or any
data to Agentra Labs or any third party. The software operates entirely
locally.

## Data Retention

### Incarnation memory

Past life records are retained in the .areal file until explicitly pruned:

```bash
areal workspace compact --retain-lives 10  # Keep last 10 incarnations
```

### Weather history

Environment change history is retained for the current session by default.
Configure retention:

```bash
areal workspace compact --retain-weather 7d  # Keep 7 days of weather
```

### Audit logs

Audit logs are retained per the rotation policy. To clear:

```bash
rm -f $AGENTIC_INSTALL_DIR/logs/audit.log*
```

## Compliance

### GDPR

AgenticReality does not process personal data in normal operation. If
deployment metadata contains personal information (e.g., developer names
in deployment roles), that data is stored locally in the .areal file and
never transmitted.

### SOC 2

The authentication, authorization, encryption, and audit logging features
support SOC 2 compliance requirements for access control and audit trails.

### HIPAA

For healthcare environments, enable encryption and restrict MCP access
to authenticated clients only. AgenticReality does not process PHI
(protected health information) directly.

## Responsible Disclosure

If you discover a security vulnerability in AgenticReality, please report it
to security@agentralabs.tech. Do not open public issues for security
vulnerabilities.

---

*Part of the AgenticOS ecosystem by Agentra Labs*
