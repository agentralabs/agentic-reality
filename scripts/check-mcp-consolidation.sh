#!/usr/bin/env bash
set -euo pipefail

# check-mcp-consolidation.sh — Verify MCP code quality (zero unwraps, error codes).
# Exits 1 on any failure.

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FAIL=0

pass() { echo "  PASS  $1"; }
fail() { echo "  FAIL  $1"; FAIL=1; }

MCP_SRC="$REPO_ROOT/crates/agentic-reality-mcp/src"

echo "=== MCP Consolidation Check ==="
echo ""

if [[ ! -d "$MCP_SRC" ]]; then
    fail "MCP source directory not found: $MCP_SRC"
    echo ""
    echo "RESULT: FAIL"
    exit 1
fi

# Helper: count matching lines across .rs files (returns 0 when no matches)
count_matches() {
    local pattern="$1"
    local count
    count=$(grep -r "$pattern" "$MCP_SRC" --include='*.rs' 2>/dev/null || true)
    if [[ -z "$count" ]]; then
        echo 0
    else
        echo "$count" | wc -l | tr -d ' '
    fi
}

# --- 1. Zero .unwrap() in MCP source ---
echo "--- .unwrap() usage ---"
UNWRAP_COUNT=$(count_matches '\.unwrap()')
if [[ "$UNWRAP_COUNT" -eq 0 ]]; then
    pass "zero .unwrap() calls in MCP source"
else
    fail "$UNWRAP_COUNT .unwrap() call(s) found in MCP source"
    grep -rn '\.unwrap()' "$MCP_SRC" --include='*.rs' 2>/dev/null || true
fi

# --- 2. Zero .expect() in MCP source ---
echo ""
echo "--- .expect() usage ---"
EXPECT_COUNT=$(count_matches '\.expect(')
if [[ "$EXPECT_COUNT" -eq 0 ]]; then
    pass "zero .expect() calls in MCP source"
else
    fail "$EXPECT_COUNT .expect() call(s) found in MCP source"
    grep -rn '\.expect(' "$MCP_SRC" --include='*.rs' 2>/dev/null || true
fi

# --- 3. Error code -32803 present for ToolNotFound ---
echo ""
echo "--- ToolNotFound error code (-32803) ---"
FOUND_32803=$(count_matches '\-32803')
if [[ "$FOUND_32803" -gt 0 ]]; then
    pass "error code -32803 found for ToolNotFound ($FOUND_32803 occurrence(s))"
else
    fail "error code -32803 not found in MCP source"
fi

# --- 4. No unwrap_or_default in MCP source ---
echo ""
echo "--- unwrap_or_default usage ---"
UOD_COUNT=$(count_matches 'unwrap_or_default')
if [[ "$UOD_COUNT" -eq 0 ]]; then
    pass "zero unwrap_or_default calls in MCP source"
else
    fail "$UOD_COUNT unwrap_or_default call(s) found in MCP source"
    grep -rn 'unwrap_or_default' "$MCP_SRC" --include='*.rs' 2>/dev/null || true
fi

# --- Summary ---
echo ""
if [[ $FAIL -ne 0 ]]; then
    echo "RESULT: FAIL — MCP code quality issues detected."
    exit 1
else
    echo "RESULT: PASS — MCP code quality is clean."
fi
