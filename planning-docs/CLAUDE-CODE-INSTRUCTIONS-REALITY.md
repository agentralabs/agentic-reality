# CLAUDE CODE INSTRUCTIONS — AGENTIC REALITY

> **Sister:** #10 of 25 (The Ground)
> **Quality Benchmark:** AgenticMemory v0.4.2
> **Validation Source:** CANONICAL_SISTER_KIT.md
> **Target:** v0.1.0 release-ready

---

## CRITICAL MANDATE

**TWO NON-NEGOTIABLE REQUIREMENTS:**

1. **BENCHMARK AGAINST AGENTIC MEMORY** — Memory is the quality bar. Every metric must match or exceed Memory v0.4.2.

2. **VALIDATE AGAINST CANONICAL_SISTER_KIT.md** — Before completing ANY phase, verify compliance with the canonical sister specification.

---

## SECTION 1: MEMORY BENCHMARKING

### 1.1 Why Memory is the Benchmark

AgenticMemory is the most battle-tested sister:
- **291+ MCP tests** — The gold standard for MCP coverage
- **v0.4.2** — Most mature version across sisters
- **24 inventions** — Fully implemented and verified
- **Zero MCP unwraps** — Hardened against all edge cases
- **Transport WAL + Extraction + Daemon** — Production-proven architecture

### 1.2 Metrics to Match or Exceed

```
╔═══════════════════════════════════════════════════════════════════════╗
║                    MEMORY → REALITY BENCHMARK                         ║
╠═══════════════════════════════════════════════════════════════════════╣
║ Metric                    │ Memory v0.4.2    │ Reality Target         ║
╠═══════════════════════════════════════════════════════════════════════╣
║ MCP Tests                 │ 291+             │ 250+ minimum           ║
║ Total Tests               │ 350+             │ 300+ minimum           ║
║ MCP Tools                 │ 10               │ 15                     ║
║ Inventions                │ 24               │ 26                     ║
║ CLI Commands              │ 30+              │ 40+                    ║
║ Unwraps in MCP            │ 0                │ 0 (MANDATORY)          ║
║ Bridge Traits             │ 6                │ 9                      ║
║ Doc Pages                 │ 12               │ 12                     ║
║ SVG Diagrams              │ 4                │ 4                      ║
║ CI Guardrails             │ 50 sections      │ 50 sections            ║
║ Performance (ops/sec)     │ ~10,000          │ ~10,000                ║
╚═══════════════════════════════════════════════════════════════════════╝
```

### 1.3 Benchmark Verification Commands

Run these after EVERY implementation phase:

```bash
# ═══════════════════════════════════════════════════════════════════════
# BENCHMARK CHECK SCRIPT — Run after each phase
# ═══════════════════════════════════════════════════════════════════════

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║              MEMORY BENCHMARK VERIFICATION                     ║"
echo "╚════════════════════════════════════════════════════════════════╝"

# Test count comparison
echo ""
echo "📊 TEST COUNT:"
test_output=$(cargo test 2>&1)
passed=$(echo "$test_output" | grep -oP '\d+(?= passed)')
failed=$(echo "$test_output" | grep -oP '\d+(?= failed)')
echo "   Passed: $passed (Memory: 291+, Target: 250+)"
echo "   Failed: $failed (Must be 0)"

if [ "$passed" -lt 250 ]; then
    echo "   ⚠️  WARNING: Below Memory benchmark!"
fi

# MCP hardening check
echo ""
echo "🔒 MCP HARDENING:"
unwraps=$(grep -r "\.unwrap()" src/mcp/ --include="*.rs" 2>/dev/null | wc -l)
expects=$(grep -r "\.expect(" src/mcp/ --include="*.rs" 2>/dev/null | wc -l)
echo "   Unwraps in MCP: $unwraps (Memory: 0, Must be 0)"
echo "   Expects in MCP: $expects (Memory: 0, Must be 0)"

if [ "$unwraps" -gt 0 ] || [ "$expects" -gt 0 ]; then
    echo "   ❌ FAIL: MCP not hardened!"
    exit 1
fi

# MCP tool count
echo ""
echo "🔧 MCP TOOLS:"
tools=$(grep -c "pub fn tool_" src/mcp/tools.rs 2>/dev/null || echo "0")
echo "   Tool count: $tools (Memory: 10, Reality Target: 15)"

# Documentation check
echo ""
echo "📚 DOCUMENTATION:"
doc_count=$(ls docs/*.md 2>/dev/null | wc -l)
echo "   Doc pages: $doc_count (Memory: 12, Target: 12)"

for doc in docs/*.md; do
    if [ -f "$doc" ]; then
        lines=$(wc -l < "$doc")
        if [ "$lines" -lt 50 ]; then
            echo "   ⚠️  $doc is a stub ($lines lines)"
        fi
    fi
done

# Bridge traits
echo ""
echo "🌉 BRIDGES:"
bridges=$(grep -c "pub trait.*Bridge" src/bridges/*.rs 2>/dev/null || echo "0")
echo "   Bridge traits: $bridges (Memory: 6, Reality Target: 9)"

echo ""
echo "════════════════════════════════════════════════════════════════"
```

### 1.4 Memory Architecture Patterns to Follow

Study and replicate these Memory patterns:

```rust
// ═══════════════════════════════════════════════════════════════════════
// PATTERN 1: MCP Handler Structure (from Memory)
// ═══════════════════════════════════════════════════════════════════════

// Memory's pattern - COPY THIS EXACTLY
pub async fn handle_memory_core(&self, params: Value) -> Result<Value, McpError> {
    // 1. Always validate operation first
    let operation = McpValidator::require_string(&params, "operation")?;
    
    // 2. Validate operation is allowed
    McpValidator::validate_operation(
        &operation,
        &["create", "get", "update", "delete", "list", "search"],
    )?;
    
    // 3. Match with explicit error handling
    match operation.as_str() {
        "create" => {
            // 4. Validate all required params BEFORE acquiring locks
            let content = McpValidator::require_string(&params, "content")?;
            let memory_type = McpValidator::require_string(&params, "memory_type")?;
            
            // 5. Acquire lock with proper error
            let mut engine = self.engine.write()
                .map_err(|_| McpError::Internal("Lock poisoned".to_string()))?;
            
            // 6. Perform operation with proper error propagation
            let memory = engine.create_memory(content, memory_type)
                .map_err(|e| McpError::OperationFailed(e.to_string()))?;
            
            // 7. Serialize result
            serde_json::to_value(&memory)
                .map_err(|e| McpError::SerializationError(e.to_string()))
        }
        // ... other operations
        _ => Err(McpError::InvalidOperation(operation)),
    }
}

// ═══════════════════════════════════════════════════════════════════════
// PATTERN 2: Test Phase Structure (from Memory)
// ═══════════════════════════════════════════════════════════════════════

// Memory's test file structure - REPLICATE THIS
// tests/mcp/
//   phase01_core.rs           - Core CRUD operations
//   phase02_search.rs         - Search and query
//   phase03_relationships.rs  - Memory relationships
//   phase04_temporal.rs       - Time-based operations
//   phase05_indexes.rs        - Index operations
//   phase06_workspace.rs      - Workspace management
//   phase07_edge_cases.rs     - Edge cases and boundaries
//   phase08_stress.rs         - Stress and load tests
//   phase09_integration.rs    - Cross-server integration
//   phase10_regression.rs     - Regression tests
//   phase11_daemon.rs         - Daemon-specific tests
//   mcp_tool_count.rs         - Tool count verification

// ═══════════════════════════════════════════════════════════════════════
// PATTERN 3: Validation Pattern (from Memory)
// ═══════════════════════════════════════════════════════════════════════

// Memory's strict validation - NO SILENT FALLBACKS
pub fn require_string(params: &Value, field: &'static str) -> Result<String, McpError> {
    params.get(field)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| McpError::MissingParameter(field.to_string()))
    // ❌ NEVER: .unwrap_or_default()
    // ❌ NEVER: .unwrap_or("".to_string())
}
```

---

## SECTION 2: CANONICAL VALIDATION

### 2.1 CANONICAL_SISTER_KIT.md Compliance

The CANONICAL_SISTER_KIT.md defines the universal sister requirements. You MUST validate against it at every phase.

### 2.2 Canonical Validation Checkpoints

```
╔═══════════════════════════════════════════════════════════════════════╗
║                    CANONICAL VALIDATION CHECKPOINTS                   ║
╠═══════════════════════════════════════════════════════════════════════╣
║ Phase        │ Canonical Section to Verify                            ║
╠═══════════════════════════════════════════════════════════════════════╣
║ After init   │ §1 Project Structure                                   ║
║ After types  │ §2 Data Structures Pattern                             ║
║ After format │ §3 File Format Requirements                            ║
║ After engine │ §4 Engine Architecture                                 ║
║ After MCP    │ §5 MCP Tool Consolidation (CRITICAL)                   ║
║ After CLI    │ §6 CLI Structure                                       ║
║ After bridge │ §7 Sister Bridges (9 traits)                           ║
║ After tests  │ §8 Test Requirements                                   ║
║ After docs   │ §9 Documentation (12 pages, no stubs)                  ║
║ After CI     │ §10 CI Guardrails (50 sections)                        ║
║ Pre-release  │ ALL SECTIONS                                           ║
╚═══════════════════════════════════════════════════════════════════════╝
```

### 2.3 Canonical Validation Script

Run this after EVERY phase:

```bash
#!/usr/bin/env bash
# canonical-validate.sh — Run after each implementation phase

set -euo pipefail

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║           CANONICAL_SISTER_KIT.md VALIDATION                   ║"
echo "╚════════════════════════════════════════════════════════════════╝"

PASS=0
FAIL=0
WARN=0

check() {
    local name="$1"
    local condition="$2"
    local required="$3"
    
    if eval "$condition"; then
        echo "✅ $name"
        ((PASS++))
    elif [ "$required" = "required" ]; then
        echo "❌ $name (REQUIRED)"
        ((FAIL++))
    else
        echo "⚠️  $name (warning)"
        ((WARN++))
    fi
}

echo ""
echo "§1 PROJECT STRUCTURE"
echo "────────────────────"
check "Cargo.toml exists" "[ -f Cargo.toml ]" "required"
check "src/lib.rs exists" "[ -f src/lib.rs ]" "required"
check "src/types/ exists" "[ -d src/types ]" "required"
check "src/engine/ exists" "[ -d src/engine ]" "required"
check "src/mcp/ exists" "[ -d src/mcp ]" "required"
check "src/cli/ exists" "[ -d src/cli ]" "required"
check "src/bridges/ exists" "[ -d src/bridges ]" "required"

echo ""
echo "§5 MCP TOOL CONSOLIDATION (CRITICAL)"
echo "─────────────────────────────────────"
if [ -f src/mcp/server.rs ]; then
    tool_count=$(grep -c "pub fn tool_" src/mcp/tools.rs 2>/dev/null || echo "0")
    check "MCP tools = 15" "[ $tool_count -eq 15 ]" "required"
    
    unwraps=$(grep -r "\.unwrap()" src/mcp/ --include="*.rs" 2>/dev/null | wc -l)
    check "Zero unwraps in MCP" "[ $unwraps -eq 0 ]" "required"
    
    expects=$(grep -r "\.expect(" src/mcp/ --include="*.rs" 2>/dev/null | wc -l)
    check "Zero expects in MCP" "[ $expects -eq 0 ]" "required"
    
    validator_usage=$(grep -r "McpValidator" src/mcp/ --include="*.rs" 2>/dev/null | wc -l)
    check "McpValidator used" "[ $validator_usage -gt 10 ]" "required"
else
    echo "⏭️  MCP not yet implemented"
fi

echo ""
echo "§7 SISTER BRIDGES"
echo "─────────────────"
if [ -d src/bridges ]; then
    bridge_count=$(grep -c "pub trait.*Bridge" src/bridges/*.rs 2>/dev/null || echo "0")
    check "9 bridge traits" "[ $bridge_count -ge 9 ]" "required"
    
    check "TimeBridge exists" "grep -q 'trait TimeBridge' src/bridges/*.rs" "required"
    check "ContractBridge exists" "grep -q 'trait ContractBridge' src/bridges/*.rs" "required"
    check "IdentityBridge exists" "grep -q 'trait IdentityBridge' src/bridges/*.rs" "required"
    check "MemoryBridge exists" "grep -q 'trait MemoryBridge' src/bridges/*.rs" "required"
    check "CognitionBridge exists" "grep -q 'trait CognitionBridge' src/bridges/*.rs" "required"
    check "CommBridge exists" "grep -q 'trait CommBridge' src/bridges/*.rs" "required"
    check "CodebaseBridge exists" "grep -q 'trait CodebaseBridge' src/bridges/*.rs" "required"
    check "VisionBridge exists" "grep -q 'trait VisionBridge' src/bridges/*.rs" "required"
    check "PlanningBridge exists" "grep -q 'trait PlanningBridge' src/bridges/*.rs" "required"
    
    check "NoOpBridges exists" "grep -q 'struct NoOpBridges' src/bridges/*.rs" "required"
    check "HydraAdapter exists" "grep -q 'trait HydraAdapter' src/bridges/*.rs" "required"
    check "GhostWriter exists" "grep -q 'trait.*GhostWriter' src/bridges/*.rs" "required"
else
    echo "⏭️  Bridges not yet implemented"
fi

echo ""
echo "§8 TEST REQUIREMENTS"
echo "────────────────────"
if [ -d tests ]; then
    check "tests/mcp/ exists" "[ -d tests/mcp ]" "required"
    check "mcp_tool_count.rs exists" "[ -f tests/mcp/mcp_tool_count.rs ]" "required"
    check "tests/stress/ exists" "[ -d tests/stress ]" "required"
    check "tests/scenarios/ exists" "[ -d tests/scenarios ]" "required"
    
    test_count=$(cargo test 2>&1 | grep -oP '\d+(?= passed)' || echo "0")
    check "250+ tests" "[ $test_count -ge 250 ]" "required"
else
    echo "⏭️  Tests not yet implemented"
fi

echo ""
echo "§9 DOCUMENTATION"
echo "────────────────"
if [ -d docs ]; then
    doc_count=$(ls docs/*.md 2>/dev/null | wc -l)
    check "12+ doc pages" "[ $doc_count -ge 12 ]" "required"
    
    # Check each required doc
    for doc in QUICKSTART ARCHITECTURE API CLI MCP-TOOLS INVENTIONS SISTER-INTEGRATION CONCEPTS EXAMPLES FAQ TROUBLESHOOTING PRIVACY; do
        check "docs/${doc}.md exists" "[ -f docs/${doc}.md ]" "required"
    done
    
    # Check for stubs
    for doc in docs/*.md; do
        if [ -f "$doc" ]; then
            lines=$(wc -l < "$doc")
            name=$(basename "$doc")
            check "$name > 50 lines ($lines)" "[ $lines -ge 50 ]" "required"
        fi
    done
    
    svg_count=$(ls docs/assets/*.svg 2>/dev/null | wc -l)
    check "4+ SVG diagrams" "[ $svg_count -ge 4 ]" "required"
else
    echo "⏭️  Docs not yet created"
fi

echo ""
echo "§10 CI GUARDRAILS"
echo "─────────────────"
if [ -f .github/workflows/ci.yml ]; then
    section_count=$(grep -c "§[0-9]" .github/workflows/ci.yml 2>/dev/null || echo "0")
    check "50 CI sections" "[ $section_count -ge 50 ]" "required"
else
    echo "⏭️  CI not yet configured"
fi

echo ""
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "SUMMARY:"
echo "  ✅ Passed: $PASS"
echo "  ❌ Failed: $FAIL"
echo "  ⚠️  Warnings: $WARN"
echo ""

if [ $FAIL -gt 0 ]; then
    echo "❌ CANONICAL VALIDATION FAILED"
    echo "   Fix all required items before proceeding."
    exit 1
else
    echo "✅ CANONICAL VALIDATION PASSED"
fi
```

### 2.4 Canonical Requirements Summary

From CANONICAL_SISTER_KIT.md, these are MANDATORY:

```
CANONICAL SISTER REQUIREMENTS:
══════════════════════════════

1. PROJECT STRUCTURE
   ├── Cargo.toml (workspace member)
   ├── src/
   │   ├── lib.rs
   │   ├── types/          # All data structures
   │   ├── engine/         # Write + Query engines
   │   ├── format/         # .areal file format
   │   ├── storage/        # Domain stores
   │   ├── validation/     # Strict + MCP validators
   │   ├── mcp/            # MCP server
   │   ├── cli/            # CLI commands
   │   ├── bridges/        # Sister bridges
   │   ├── security/       # Auth, encryption, audit
   │   └── inventions/     # Invention implementations
   ├── tests/
   │   ├── mcp/            # MCP phase tests
   │   ├── stress/         # Stress tests
   │   └── scenarios/      # Scenario tests
   ├── docs/               # 12 doc pages + assets/
   ├── scripts/
   │   ├── install.sh      # Universal installer
   │   └── release.sh      # Release script
   └── .github/workflows/ci.yml  # 50 guardrail sections

2. MCP TOOL CONSOLIDATION
   - Exactly N tools (Reality: 15)
   - Each tool handles multiple operations via "operation" param
   - Zero unwrap/expect in handlers
   - McpValidator for all input validation
   - Proper error types, never panic

3. SISTER BRIDGES (9 required for Reality)
   - TimeBridge
   - ContractBridge
   - IdentityBridge
   - MemoryBridge
   - CognitionBridge
   - CommBridge
   - CodebaseBridge
   - VisionBridge
   - PlanningBridge
   + NoOpBridges (standalone operation)
   + HydraAdapter (future orchestration)
   + RealityGhostWriter (ghost capability)

4. HARDENING REQUIREMENTS
   - Strict MCP validation (no silent fallbacks)
   - Per-project isolation (canonical path hashing)
   - Concurrent lock handling + stale recovery
   - Merge-only MCP config (never destructive)
   - Atomic file operations only
   - Audit logging

5. DOCUMENTATION
   - 12 substantive doc pages (>50 lines each)
   - 4+ SVG diagrams
   - README with badges
   - CHANGELOG, LICENSE, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY

6. CI GUARDRAILS
   - 50 sections across 6 jobs
   - Must pass before any release
```

---

## SECTION 3: IMPLEMENTATION PHASES

### Phase 1: Foundation (Validate: §1)

```bash
# Create project structure
cargo new agentic-reality --lib
cd agentic-reality

# Create directory structure
mkdir -p src/{types,engine/{write,query},format,storage,validation,mcp,cli,bridges,security,inventions}
mkdir -p tests/{mcp,stress,scenarios}
mkdir -p docs/assets
mkdir -p scripts
mkdir -p .github/workflows

# After creating Cargo.toml and src/lib.rs:
./canonical-validate.sh
```

**Validation checkpoint:**
- [ ] Cargo.toml has correct metadata
- [ ] src/lib.rs declares all modules
- [ ] Directory structure matches canonical

### Phase 2: Types (Validate: §2)

Implement all types from SPEC-03:
- `src/types/ids.rs` — All ID types
- `src/types/deployment.rs` — Deployment structures
- `src/types/environment.rs` — Environment structures
- `src/types/resource.rs` — Resource structures
- `src/types/reality.rs` — Reality layer structures
- `src/types/topology.rs` — Topology structures
- `src/types/temporal.rs` — Temporal structures
- `src/types/stakes.rs` — Stakes structures
- `src/types/coherence.rs` — Coherence structures

**After completion:**
```bash
cargo build
./canonical-validate.sh
```

### Phase 3: Storage & Format (Validate: §3)

Implement .areal file format:
- `src/format/header.rs`
- `src/format/sections.rs`
- `src/format/footer.rs`
- `src/format/file.rs`

Implement domain stores:
- `src/storage/deployment.rs`
- `src/storage/environment.rs`
- `src/storage/resource.rs`
- `src/storage/reality.rs`
- `src/storage/topology.rs`
- `src/storage/temporal.rs`
- `src/storage/stakes.rs`
- `src/storage/coherence.rs`

**Validation checkpoint:**
```bash
# Test file round-trip
cargo test format
./canonical-validate.sh
```

### Phase 4: Engine (Validate: §4)

Implement Write Engine (~90 operations):
- `src/engine/write/deployment.rs`
- `src/engine/write/environment.rs`
- `src/engine/write/resource.rs`
- `src/engine/write/reality.rs`
- `src/engine/write/topology.rs`
- `src/engine/write/temporal.rs`
- `src/engine/write/stakes.rs`
- `src/engine/write/coherence.rs`

Implement Query Engine (~78 operations):
- `src/engine/query/deployment.rs`
- `src/engine/query/environment.rs`
- `src/engine/query/resource.rs`
- `src/engine/query/reality.rs`
- `src/engine/query/topology.rs`
- `src/engine/query/temporal.rs`
- `src/engine/query/stakes.rs`
- `src/engine/query/coherence.rs`

Implement Indexes:
- `src/indexes/mod.rs`
- `src/indexes/operations.rs`
- `src/indexes/queries.rs`

**Validation checkpoint:**
```bash
cargo test engine
cargo test indexes
./canonical-validate.sh
```

### Phase 5: Validation & MCP (Validate: §5 — CRITICAL)

**THIS IS THE MOST CRITICAL PHASE FOR CANONICAL COMPLIANCE**

Implement Validation:
- `src/validation/errors.rs`
- `src/validation/strict.rs` — NO SILENT FALLBACKS
- `src/validation/entities.rs`
- `src/validation/mcp.rs` — McpValidator

Implement MCP Server (15 tools):
- `src/mcp/server.rs`
- `src/mcp/tools.rs`
- `src/mcp/handlers.rs`
- `src/mcp/errors.rs`

**CRITICAL CHECKS:**
```bash
# Must be ZERO
grep -r "\.unwrap()" src/mcp/ --include="*.rs" | wc -l
grep -r "\.expect(" src/mcp/ --include="*.rs" | wc -l

# Must be 15
grep -c "pub fn tool_" src/mcp/tools.rs

# Must be extensive
grep -r "McpValidator" src/mcp/ | wc -l

# Run benchmark comparison
./benchmark-vs-memory.sh
./canonical-validate.sh
```

### Phase 6: CLI (Validate: §6)

Implement CLI (~40 commands):
- `src/cli/mod.rs`
- `src/cli/soul.rs`
- `src/cli/memory.rs`
- `src/cli/env.rs`
- `src/cli/resource.rs`
- `src/cli/capability.rs`
- `src/cli/reality.rs`
- `src/cli/anchor.rs`
- `src/cli/hallucination.rs`
- `src/cli/topology.rs`
- `src/cli/temporal.rs`
- `src/cli/stakes.rs`
- `src/cli/coherence.rs`
- `src/cli/workspace.rs`
- `src/cli/serve.rs`

**Validation checkpoint:**
```bash
cargo build --release
./target/release/areal --help
./canonical-validate.sh
```

### Phase 7: Bridges (Validate: §7)

Implement ALL 9 bridge traits + NoOp + Hydra + Ghost:
- `src/bridges/mod.rs`
- `src/bridges/time.rs`
- `src/bridges/contract.rs`
- `src/bridges/identity.rs`
- `src/bridges/memory.rs`
- `src/bridges/cognition.rs`
- `src/bridges/comm.rs`
- `src/bridges/codebase.rs`
- `src/bridges/vision.rs`
- `src/bridges/planning.rs`
- `src/bridges/noop.rs`
- `src/bridges/hydra.rs`
- `src/bridges/ghost.rs`
- `src/bridges/config.rs`

**Validation checkpoint:**
```bash
# Verify all traits
grep -c "pub trait.*Bridge" src/bridges/*.rs  # Must be 9+
grep "impl.*Bridge for NoOpBridges" src/bridges/noop.rs  # All implemented
./canonical-validate.sh
```

### Phase 8: Security (Validate: §4)

Implement security:
- `src/security/auth.rs`
- `src/security/authz.rs`
- `src/security/encryption.rs`
- `src/security/audit.rs`

### Phase 9: Tests (Validate: §8)

**CRITICAL: Match Memory's test coverage**

Create MCP phase tests:
```
tests/mcp/
├── phase01_deployment.rs
├── phase02_environment.rs
├── phase03_resource.rs
├── phase04_reality.rs
├── phase05_topology.rs
├── phase06_stakes.rs
├── phase07_coherence.rs
├── phase08_full_suite.rs
├── phase09_edge_cases.rs
├── phase10_stress.rs
└── mcp_tool_count.rs  ← CRITICAL
```

Create stress tests:
```
tests/stress/
├── test_concurrent.rs
├── test_large_state.rs
└── test_persistence.rs
```

Create scenario tests:
```
tests/scenarios/
├── scenario_production_deployment.rs
├── scenario_scale_event.rs
├── scenario_context_shift.rs
├── scenario_hallucination_detection.rs
└── scenario_disaster_recovery.rs
```

**Validation checkpoint:**
```bash
cargo test 2>&1 | grep -E "passed|failed"
# Must show 250+ passed, 0 failed

# Compare to Memory
echo "Memory benchmark: 291+ tests"
./canonical-validate.sh
```

### Phase 10: Documentation (Validate: §9)

Create ALL 12 doc pages (SUBSTANTIVE, NOT STUBS):

```
docs/
├── QUICKSTART.md      (100+ lines)
├── ARCHITECTURE.md    (200+ lines)
├── API.md             (300+ lines)
├── CLI.md             (200+ lines)
├── MCP-TOOLS.md       (300+ lines)
├── INVENTIONS.md      (500+ lines — 26 inventions!)
├── SISTER-INTEGRATION.md (150+ lines)
├── CONCEPTS.md        (200+ lines)
├── EXAMPLES.md        (200+ lines)
├── FAQ.md             (100+ lines)
├── TROUBLESHOOTING.md (100+ lines)
├── PRIVACY.md         (100+ lines)
└── assets/
    ├── architecture.svg
    ├── reality-domains.svg
    ├── resource-body.svg
    └── context-fingerprint.svg
```

**Validation checkpoint:**
```bash
# Check line counts
for doc in docs/*.md; do
    lines=$(wc -l < "$doc")
    echo "$doc: $lines lines"
done

# Check SVGs
ls -la docs/assets/*.svg

./canonical-validate.sh
```

### Phase 11: CI & Release (Validate: §10)

Create CI workflow with 50 sections:
- `.github/workflows/ci.yml`

Create scripts:
- `scripts/install.sh` (universal installer)
- `scripts/release.sh`

**Validation checkpoint:**
```bash
# Count CI sections
grep -c "§[0-9]" .github/workflows/ci.yml  # Must be 50

# Test installer locally
bash scripts/install.sh --dry-run

./canonical-validate.sh
```

---

## SECTION 4: FINAL VERIFICATION

### 4.1 Pre-Release Checklist

```bash
#!/usr/bin/env bash
# pre-release-check.sh

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║           AGENTIC REALITY PRE-RELEASE CHECK                    ║"
echo "╚════════════════════════════════════════════════════════════════╝"

READY=true

# 1. Inventions
echo ""
echo "📦 INVENTIONS:"
inventions=$(ls src/inventions/*.rs 2>/dev/null | wc -l)
echo "   Implemented: $inventions / 26"
[ $inventions -lt 26 ] && READY=false

# 2. MCP Tools
echo ""
echo "🔧 MCP TOOLS:"
tools=$(grep -c "pub fn tool_" src/mcp/tools.rs 2>/dev/null || echo "0")
echo "   Tools: $tools / 15"
[ $tools -ne 15 ] && READY=false

# 3. MCP Hardening
echo ""
echo "🔒 MCP HARDENING:"
unwraps=$(grep -r "\.unwrap()" src/mcp/ --include="*.rs" 2>/dev/null | wc -l)
expects=$(grep -r "\.expect(" src/mcp/ --include="*.rs" 2>/dev/null | wc -l)
echo "   Unwraps: $unwraps (must be 0)"
echo "   Expects: $expects (must be 0)"
[ $unwraps -gt 0 ] && READY=false
[ $expects -gt 0 ] && READY=false

# 4. CLI Commands
echo ""
echo "💻 CLI:"
commands=$(grep -c "pub fn handle_" src/cli/*.rs 2>/dev/null || echo "0")
echo "   Commands: $commands (target: 40+)"

# 5. Tests
echo ""
echo "🧪 TESTS:"
test_output=$(cargo test 2>&1)
passed=$(echo "$test_output" | grep -oP '\d+(?= passed)' || echo "0")
failed=$(echo "$test_output" | grep -oP '\d+(?= failed)' || echo "0")
echo "   Passed: $passed (Memory benchmark: 291+, target: 250+)"
echo "   Failed: $failed (must be 0)"
[ $passed -lt 250 ] && READY=false
[ $failed -gt 0 ] && READY=false

# 6. Documentation
echo ""
echo "📚 DOCUMENTATION:"
docs=$(ls docs/*.md 2>/dev/null | wc -l)
echo "   Doc pages: $docs / 12"
[ $docs -lt 12 ] && READY=false

stub_count=0
for doc in docs/*.md; do
    if [ -f "$doc" ]; then
        lines=$(wc -l < "$doc")
        if [ $lines -lt 50 ]; then
            echo "   ⚠️  STUB: $doc ($lines lines)"
            stub_count=$((stub_count + 1))
        fi
    fi
done
[ $stub_count -gt 0 ] && READY=false

svgs=$(ls docs/assets/*.svg 2>/dev/null | wc -l)
echo "   SVG diagrams: $svgs / 4"
[ $svgs -lt 4 ] && READY=false

# 7. Bridges
echo ""
echo "🌉 BRIDGES:"
bridges=$(grep -c "pub trait.*Bridge" src/bridges/*.rs 2>/dev/null || echo "0")
echo "   Bridge traits: $bridges / 9+"
[ $bridges -lt 9 ] && READY=false

# 8. CI Guardrails
echo ""
echo "🛡️ CI GUARDRAILS:"
ci_sections=$(grep -c "§[0-9]" .github/workflows/ci.yml 2>/dev/null || echo "0")
echo "   Sections: $ci_sections / 50"
[ $ci_sections -lt 50 ] && READY=false

# 9. Root files
echo ""
echo "📄 ROOT FILES:"
for file in README.md LICENSE CHANGELOG.md CONTRIBUTING.md CODE_OF_CONDUCT.md SECURITY.md; do
    if [ -f "$file" ]; then
        echo "   ✅ $file"
    else
        echo "   ❌ $file MISSING"
        READY=false
    fi
done

# 10. Memory benchmark
echo ""
echo "📊 MEMORY BENCHMARK COMPARISON:"
echo "   Memory v0.4.2: 291+ tests, 10 tools, 24 inventions"
echo "   Reality target: 250+ tests, 15 tools, 26 inventions"

# Final verdict
echo ""
echo "════════════════════════════════════════════════════════════════"
if $READY; then
    echo "✅ READY FOR RELEASE"
    echo ""
    echo "To release:"
    echo "  ./scripts/release.sh 0.1.0"
else
    echo "❌ NOT READY FOR RELEASE"
    echo ""
    echo "Fix all issues above before releasing."
    exit 1
fi
```

### 4.2 Memory Benchmark Comparison

```bash
#!/usr/bin/env bash
# benchmark-vs-memory.sh

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║           REALITY vs MEMORY BENCHMARK                          ║"
echo "╚════════════════════════════════════════════════════════════════╝"

echo ""
echo "                    │ Memory v0.4.2  │ Reality      │ Status"
echo "════════════════════│════════════════│══════════════│═════════"

# Tests
mem_tests=291
real_tests=$(cargo test 2>&1 | grep -oP '\d+(?= passed)' || echo "0")
status=$([ $real_tests -ge 250 ] && echo "✅" || echo "❌")
printf "Tests               │ %14s │ %12s │ %s\n" "$mem_tests+" "$real_tests" "$status"

# MCP Tools
mem_tools=10
real_tools=$(grep -c "pub fn tool_" src/mcp/tools.rs 2>/dev/null || echo "0")
status=$([ $real_tools -ge 15 ] && echo "✅" || echo "❌")
printf "MCP Tools           │ %14s │ %12s │ %s\n" "$mem_tools" "$real_tools" "$status"

# Inventions
mem_inv=24
real_inv=$(ls src/inventions/*.rs 2>/dev/null | wc -l)
status=$([ $real_inv -ge 26 ] && echo "✅" || echo "❌")
printf "Inventions          │ %14s │ %12s │ %s\n" "$mem_inv" "$real_inv" "$status"

# MCP Unwraps
mem_unwraps=0
real_unwraps=$(grep -r "\.unwrap()" src/mcp/ --include="*.rs" 2>/dev/null | wc -l)
status=$([ $real_unwraps -eq 0 ] && echo "✅" || echo "❌")
printf "MCP Unwraps         │ %14s │ %12s │ %s\n" "$mem_unwraps" "$real_unwraps" "$status"

# Bridge Traits
mem_bridges=6
real_bridges=$(grep -c "pub trait.*Bridge" src/bridges/*.rs 2>/dev/null || echo "0")
status=$([ $real_bridges -ge 9 ] && echo "✅" || echo "❌")
printf "Bridge Traits       │ %14s │ %12s │ %s\n" "$mem_bridges" "$real_bridges" "$status"

# Doc Pages
mem_docs=12
real_docs=$(ls docs/*.md 2>/dev/null | wc -l)
status=$([ $real_docs -ge 12 ] && echo "✅" || echo "❌")
printf "Doc Pages           │ %14s │ %12s │ %s\n" "$mem_docs" "$real_docs" "$status"

# CI Sections
mem_ci=50
real_ci=$(grep -c "§[0-9]" .github/workflows/ci.yml 2>/dev/null || echo "0")
status=$([ $real_ci -ge 50 ] && echo "✅" || echo "❌")
printf "CI Sections         │ %14s │ %12s │ %s\n" "$mem_ci" "$real_ci" "$status"

echo ""
```

---

## SECTION 5: COMMON MISTAKES TO AVOID

```
╔═══════════════════════════════════════════════════════════════════════╗
║                    MISTAKES TO AVOID                                  ║
╠═══════════════════════════════════════════════════════════════════════╣
║ ❌ MISTAKE                          │ ✅ CORRECT APPROACH              ║
╠═══════════════════════════════════════════════════════════════════════╣
║ Using .unwrap() in MCP handlers    │ Use ? operator or map_err()     ║
║ Using .unwrap_or_default()         │ Return explicit error           ║
║ Creating stub documentation        │ Write substantive content       ║
║ Skipping canonical validation      │ Validate after EVERY phase      ║
║ Not comparing to Memory            │ Benchmark constantly            ║
║ Forgetting mcp_tool_count test     │ Always verify tool count = 15   ║
║ Missing bridge traits              │ Implement all 9 + NoOp + Hydra  ║
║ Silent fallbacks in validation     │ Fail loudly with clear errors   ║
║ Incomplete CI guardrails           │ All 50 sections required        ║
║ Rushing to "complete"              │ Systematic verification         ║
╚═══════════════════════════════════════════════════════════════════════╝
```

---

## SECTION 6: SUCCESS CRITERIA

AgenticReality v0.1.0 is **COMPLETE** when:

```
[ ] All 26 inventions implemented (src/inventions/)
[ ] 15 MCP tools working (src/mcp/)
[ ] Zero unwrap/expect in MCP handlers
[ ] ~40 CLI commands functional (src/cli/)
[ ] 250+ tests passing (benchmark: Memory 291+)
[ ] 9 bridge traits implemented (src/bridges/)
[ ] 12 doc pages substantive (>50 lines each)
[ ] 4 SVG diagrams created (docs/assets/)
[ ] 50 CI guardrail sections (ci.yml)
[ ] Universal installer tested (scripts/install.sh)
[ ] Release script tested (scripts/release.sh)
[ ] README with badges
[ ] CHANGELOG with v0.1.0 entry
[ ] CANONICAL_SISTER_KIT.md compliance verified
[ ] Memory benchmark comparison passing
[ ] All root files present (LICENSE, CONTRIBUTING, etc.)
```

---

*Document: CLAUDE-CODE-INSTRUCTIONS-REALITY.md*
*Sister #10 of 25: The Ground*
*Quality Benchmark: AgenticMemory v0.4.2*
*Validation Source: CANONICAL_SISTER_KIT.md*
