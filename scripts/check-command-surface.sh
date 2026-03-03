#!/usr/bin/env bash
set -euo pipefail

# check-command-surface.sh — Verify MCP tool count, naming, and CLI binary.
# Exits 1 on any failure.

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FAIL=0

pass() { echo "  PASS  $1"; }
fail() { echo "  FAIL  $1"; FAIL=1; }

REGISTRY="$REPO_ROOT/crates/agentic-reality-mcp/src/tools/registry.rs"

echo "=== Command Surface Check ==="
echo ""

# --- 1. MCP tool count = 15 ---
echo "--- MCP tool count ---"
if [[ -f "$REGISTRY" ]]; then
    # Count ToolDefinition blocks in the list_tools() vec
    TOOL_COUNT=$(grep -c 'ToolDefinition {' "$REGISTRY" || true)
    if [[ "$TOOL_COUNT" -eq 15 ]]; then
        pass "MCP tool count = $TOOL_COUNT (expected 15)"
    else
        fail "MCP tool count = $TOOL_COUNT (expected 15)"
    fi
else
    fail "registry.rs not found at $REGISTRY"
fi

# --- 2. All MCP tools start with "reality_" ---
echo ""
echo "--- MCP tool naming convention ---"
if [[ -f "$REGISTRY" ]]; then
    # Extract tool names from the registry (macOS-compatible, no -P flag)
    TOOL_NAMES=$(grep -o 'name: *"[^"]*"' "$REGISTRY" | sed 's/name: *"//;s/".*//' || true)
    BAD_NAMES=0
    while IFS= read -r tool_name; do
        if [[ -z "$tool_name" ]]; then
            continue
        fi
        if [[ "$tool_name" == reality_* ]]; then
            pass "tool '$tool_name' starts with reality_"
        else
            fail "tool '$tool_name' does NOT start with reality_"
            BAD_NAMES=$((BAD_NAMES + 1))
        fi
    done <<< "$TOOL_NAMES"

    if [[ $BAD_NAMES -eq 0 && -n "$TOOL_NAMES" ]]; then
        pass "all MCP tools use reality_ prefix"
    fi
else
    fail "registry.rs not found"
fi

# --- 3. CLI binary "areal" in Cargo.toml ---
echo ""
echo "--- CLI binary name ---"
CLI_CARGO="$REPO_ROOT/crates/agentic-reality-cli/Cargo.toml"
if [[ -f "$CLI_CARGO" ]]; then
    if grep -q 'name = "areal"' "$CLI_CARGO"; then
        pass "CLI binary 'areal' defined in Cargo.toml"
    else
        fail "CLI binary 'areal' not found in $CLI_CARGO"
    fi
else
    fail "crates/agentic-reality-cli/Cargo.toml not found"
fi

# --- 4. docs/CLI.md exists with content ---
echo ""
echo "--- CLI documentation ---"
CLI_DOC="$REPO_ROOT/docs/CLI.md"
if [[ -f "$CLI_DOC" ]]; then
    LINE_COUNT=$(wc -l < "$CLI_DOC" | tr -d ' ')
    if [[ "$LINE_COUNT" -gt 0 ]]; then
        pass "docs/CLI.md exists ($LINE_COUNT lines)"
    else
        fail "docs/CLI.md exists but is empty"
    fi
else
    fail "docs/CLI.md not found"
fi

# --- Summary ---
echo ""
if [[ $FAIL -ne 0 ]]; then
    echo "RESULT: FAIL — command surface issues detected."
    exit 1
else
    echo "RESULT: PASS — command surface is correct."
fi
