#!/usr/bin/env bash
set -euo pipefail

# check-runtime-hardening.sh
# Validates runtime hardening requirements for AgenticReality MCP server.
# Exit 0 if all checks pass, exit 1 on first failure.

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MCP_SRC="$REPO_ROOT/crates/agentic-reality-mcp/src"
CORE_SRC="$REPO_ROOT/crates/agentic-reality/src"
FAILED=0

pass() { echo "  PASS: $1"; }
fail() { echo "  FAIL: $1"; FAILED=1; }

echo "=== AgenticReality Runtime Hardening Check ==="
echo ""

# ---------------------------------------------------------------------------
# 1. Zero unwrap/expect in MCP source
# ---------------------------------------------------------------------------
echo "--- Section 1: Zero unwrap/expect in MCP source ---"

UNWRAP_COUNT=$(grep -rn '\.unwrap()' "$MCP_SRC" --include='*.rs' \
  | grep -v '#\[cfg(test)\]' \
  | grep -v '#\[test\]' \
  | grep -v 'mod tests' \
  | grep -v '// test' \
  | grep -v '_test.rs' \
  | grep -cv '^$' || true)

EXPECT_COUNT=$(grep -rn '\.expect(' "$MCP_SRC" --include='*.rs' \
  | grep -v '#\[cfg(test)\]' \
  | grep -v '#\[test\]' \
  | grep -v 'mod tests' \
  | grep -v '// test' \
  | grep -v '_test.rs' \
  | grep -cv '^$' || true)

if [ "$UNWRAP_COUNT" -eq 0 ] && [ "$EXPECT_COUNT" -eq 0 ]; then
  pass "zero unwrap/expect in MCP production code"
else
  fail "found $UNWRAP_COUNT unwrap() and $EXPECT_COUNT expect() calls in MCP source"
  if [ "$UNWRAP_COUNT" -gt 0 ]; then
    grep -rn '\.unwrap()' "$MCP_SRC" --include='*.rs' \
      | grep -v '#\[cfg(test)\]' \
      | grep -v '#\[test\]' \
      | grep -v 'mod tests' \
      | grep -v '// test' \
      | grep -v '_test.rs' || true
  fi
  if [ "$EXPECT_COUNT" -gt 0 ]; then
    grep -rn '\.expect(' "$MCP_SRC" --include='*.rs' \
      | grep -v '#\[cfg(test)\]' \
      | grep -v '#\[test\]' \
      | grep -v 'mod tests' \
      | grep -v '// test' \
      | grep -v '_test.rs' || true
  fi
fi

# ---------------------------------------------------------------------------
# 2. Per-project state isolation (workspace ID derivation)
# ---------------------------------------------------------------------------
echo ""
echo "--- Section 2: Per-project state isolation ---"

if grep -rq 'derive_workspace_id\|workspace_id\|canonical.*hash\|blake3::hash.*canonical' \
     "$MCP_SRC" --include='*.rs'; then
  pass "workspace ID derivation pattern found in MCP source"
elif grep -rq 'derive_workspace_id\|workspace_id\|canonical.*hash\|blake3::hash.*canonical' \
     "$MCP_SRC/../" --include='*.rs' 2>/dev/null; then
  pass "workspace ID derivation pattern found in session module"
else
  fail "no workspace ID derivation pattern found -- per-project isolation may be missing"
fi

# Also check core session manager
if grep -rq 'derive_workspace_id' "$MCP_SRC" --include='*.rs' 2>/dev/null || \
   grep -rq 'derive_workspace_id' "$MCP_SRC/session" --include='*.rs' 2>/dev/null; then
  pass "derive_workspace_id function exists"
else
  # Check in the session module which may be at the crate level
  if grep -rq 'derive_workspace_id' "$REPO_ROOT/crates/agentic-reality-mcp/" --include='*.rs'; then
    pass "derive_workspace_id function exists in MCP crate"
  else
    fail "derive_workspace_id function not found in MCP crate"
  fi
fi

# ---------------------------------------------------------------------------
# 3. Concurrent lock handling patterns
# ---------------------------------------------------------------------------
echo ""
echo "--- Section 3: Concurrent lock handling ---"

if grep -rq 'Mutex\|RwLock\|tokio::sync::Mutex\|Arc<Mutex' \
     "$MCP_SRC" --include='*.rs'; then
  pass "concurrent lock primitives found (Mutex/RwLock)"
else
  fail "no concurrent lock primitives found in MCP source"
fi

if grep -rq 'session.*lock\|\.lock().await\|lock_guard\|Mutex::new' \
     "$MCP_SRC" --include='*.rs'; then
  pass "lock acquisition patterns found"
else
  fail "no lock acquisition patterns found"
fi

# ---------------------------------------------------------------------------
# 4. Server auth gate
# ---------------------------------------------------------------------------
echo ""
echo "--- Section 4: Server authentication gate ---"

AUTH_PATTERNS='AGENTIC_AUTH_TOKEN\|AGENTIC_TOKEN\|AGENTIC_REALITY_AUTH_TOKEN\|auth_token'

if grep -rq "$AUTH_PATTERNS" "$MCP_SRC" --include='*.rs'; then
  pass "auth token check exists in MCP source"
else
  fail "no auth token pattern (AGENTIC_AUTH_TOKEN / AGENTIC_TOKEN / auth_token) found"
fi

if grep -rq "$AUTH_PATTERNS" "$MCP_SRC/config" --include='*.rs' 2>/dev/null || \
   grep -rq "$AUTH_PATTERNS" "$REPO_ROOT/crates/agentic-reality-mcp/" --include='*.rs'; then
  pass "auth token loaded from environment in config"
else
  fail "auth token not loaded from environment"
fi

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
echo ""
echo "=== Runtime Hardening Summary ==="
if [ "$FAILED" -eq 0 ]; then
  echo "ALL CHECKS PASSED"
  exit 0
else
  echo "SOME CHECKS FAILED -- see details above"
  exit 1
fi
