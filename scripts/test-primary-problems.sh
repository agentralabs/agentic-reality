#!/usr/bin/env bash
set -euo pipefail

# test-primary-problems.sh
# Runs the critical test suite and validates primary invariants for
# AgenticReality before any push or release.

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MCP_SRC="$REPO_ROOT/crates/agentic-reality-mcp/src"
FAILED=0

pass() { echo "  PASS: $1"; }
fail() { echo "  FAIL: $1"; FAILED=1; }

echo "=== AgenticReality Primary Problems Test Suite ==="
echo ""

# ---------------------------------------------------------------------------
# 1. Run critical cargo tests
# ---------------------------------------------------------------------------
echo "--- Phase 1: Core tests ---"

echo "  Running deployment tests..."
if cargo test -p agentic-reality --lib -- deployment 2>&1 | tail -1 | grep -q 'test result: ok'; then
  pass "deployment tests"
else
  fail "deployment tests"
fi

echo "  Running environment tests..."
if cargo test -p agentic-reality --lib -- environment 2>&1 | tail -1 | grep -q 'test result: ok'; then
  pass "environment tests"
else
  fail "environment tests"
fi

echo "  Running format tests..."
if cargo test -p agentic-reality --lib -- format 2>&1 | tail -1 | grep -q 'test result: ok'; then
  pass "format tests"
else
  fail "format tests"
fi

echo "  Running validation tests..."
if cargo test -p agentic-reality --lib -- validation 2>&1 | tail -1 | grep -q 'test result: ok'; then
  pass "validation tests"
else
  fail "validation tests"
fi

echo "  Running MCP tests..."
if cargo test -p agentic-reality-mcp 2>&1 | tail -1 | grep -q 'test result: ok'; then
  pass "MCP server tests"
else
  fail "MCP server tests"
fi

# ---------------------------------------------------------------------------
# 2. Integration test phases
# ---------------------------------------------------------------------------
echo ""
echo "--- Phase 2: Integration test phases ---"

for phase in phase01 phase02 phase03 phase04 phase05 phase06 phase07 phase08; do
  if ls "$REPO_ROOT/tests/${phase}_"*.rs 1>/dev/null 2>&1; then
    echo "  Running ${phase} tests..."
    if cargo test --test "${phase}_*" 2>&1 | tail -1 | grep -q 'test result: ok'; then
      pass "${phase} integration tests"
    else
      fail "${phase} integration tests"
    fi
  fi
done

# ---------------------------------------------------------------------------
# 3. MCP tool count = 15
# ---------------------------------------------------------------------------
echo ""
echo "--- Phase 3: MCP tool count ---"

TOOL_COUNT_CONST=$(grep -o 'MCP_TOOL_COUNT.*=.*[0-9]\+' "$MCP_SRC/tools/registry.rs" \
  | grep -o '[0-9]\+' || echo "0")

TOOL_DEFS=$(grep -c 'ToolDefinition {' "$MCP_SRC/tools/registry.rs" || echo "0")

if [ "$TOOL_COUNT_CONST" -eq 15 ]; then
  pass "MCP_TOOL_COUNT constant = 15"
else
  fail "MCP_TOOL_COUNT constant = $TOOL_COUNT_CONST (expected 15)"
fi

if [ "$TOOL_DEFS" -eq 15 ]; then
  pass "tool definitions count = 15"
else
  fail "tool definitions count = $TOOL_DEFS (expected 15)"
fi

# Also run the dedicated tool count test if it exists
if ls "$REPO_ROOT/tests/mcp_tool_count"*.rs 1>/dev/null 2>&1; then
  echo "  Running mcp_tool_count test..."
  if cargo test --test mcp_tool_count 2>&1 | tail -1 | grep -q 'test result: ok'; then
    pass "mcp_tool_count integration test"
  else
    fail "mcp_tool_count integration test"
  fi
fi

# ---------------------------------------------------------------------------
# 4. Zero unwraps in MCP production code
# ---------------------------------------------------------------------------
echo ""
echo "--- Phase 4: Zero unwraps in MCP source ---"

UNWRAP_HITS=$(grep -rn '\.unwrap()' "$MCP_SRC" --include='*.rs' \
  | grep -v 'mod tests' \
  | grep -v '#\[test\]' \
  | grep -v '#\[cfg(test)\]' \
  | grep -v '_test.rs' \
  | grep -cv '^$' || true)

EXPECT_HITS=$(grep -rn '\.expect(' "$MCP_SRC" --include='*.rs' \
  | grep -v 'mod tests' \
  | grep -v '#\[test\]' \
  | grep -v '#\[cfg(test)\]' \
  | grep -v '_test.rs' \
  | grep -cv '^$' || true)

TOTAL_UNSAFE=$((UNWRAP_HITS + EXPECT_HITS))

if [ "$TOTAL_UNSAFE" -eq 0 ]; then
  pass "zero unwrap/expect in MCP production code"
else
  fail "found $UNWRAP_HITS unwrap() and $EXPECT_HITS expect() in MCP source"
fi

# ---------------------------------------------------------------------------
# 5. Format roundtrip
# ---------------------------------------------------------------------------
echo ""
echo "--- Phase 5: Format roundtrip ---"

echo "  Running format roundtrip tests..."
if cargo test -p agentic-reality --lib -- format::file 2>&1 | tail -1 | grep -q 'test result: ok'; then
  pass "format roundtrip tests"
else
  # Try broader filter
  if cargo test -p agentic-reality --lib -- roundtrip 2>&1 | tail -1 | grep -q 'test result: ok'; then
    pass "format roundtrip tests (via roundtrip filter)"
  else
    fail "format roundtrip tests"
  fi
fi

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
echo ""
echo "=== Primary Problems Summary ==="
if [ "$FAILED" -eq 0 ]; then
  echo "ALL CHECKS PASSED"
  exit 0
else
  echo "SOME CHECKS FAILED -- review output above"
  exit 1
fi
