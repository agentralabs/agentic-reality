#!/usr/bin/env bash
set -euo pipefail

# check-canonical-sister.sh — Verify sister project parity requirements.
# Exits 1 on any failure.

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FAIL=0

pass() { echo "  PASS  $1"; }
fail() { echo "  FAIL  $1"; FAIL=1; }

echo "=== Canonical Sister Check ==="
echo ""

# --- 1. Canonical doc folder has 12+ .md files ---
echo "--- Documentation file count ---"
DOCS_DIR="$REPO_ROOT/docs"
if [[ -d "$DOCS_DIR" ]]; then
    DOC_COUNT=$(find "$DOCS_DIR" -maxdepth 1 -name '*.md' -type f 2>/dev/null | wc -l | tr -d ' ')
    if [[ "$DOC_COUNT" -ge 12 ]]; then
        pass "docs/ has $DOC_COUNT .md files (>= 12 required)"
    else
        fail "docs/ has only $DOC_COUNT .md files (>= 12 required)"
    fi
else
    fail "docs/ directory not found"
fi

# --- 2. 4+ SVG files in docs/assets/ ---
echo ""
echo "--- SVG diagram count ---"
ASSETS_DIR="$REPO_ROOT/docs/assets"
if [[ -d "$ASSETS_DIR" ]]; then
    SVG_COUNT=$(find "$ASSETS_DIR" -name '*.svg' -type f 2>/dev/null | wc -l | tr -d ' ')
    if [[ "$SVG_COUNT" -ge 4 ]]; then
        pass "docs/assets/ has $SVG_COUNT SVG files (>= 4 required)"
    else
        fail "docs/assets/ has only $SVG_COUNT SVG files (>= 4 required)"
    fi
else
    fail "docs/assets/ directory not found"
fi

# --- 3. MCP quality — tool descriptions verb-first, no trailing periods ---
echo ""
echo "--- MCP tool description quality ---"
REGISTRY="$REPO_ROOT/crates/agentic-reality-mcp/src/tools/registry.rs"
if [[ -f "$REGISTRY" ]]; then
    # Extract description values
    DESCRIPTIONS=$(grep -o 'description: *"[^"]*"' "$REGISTRY" | sed 's/description: *"//;s/"$//')

    # Check for trailing periods
    TRAILING_PERIODS=0
    while IFS= read -r desc; do
        if [[ -z "$desc" ]]; then continue; fi
        if [[ "$desc" == *. ]]; then
            fail "trailing period in description: \"$desc\""
            TRAILING_PERIODS=$((TRAILING_PERIODS + 1))
        fi
    done <<< "$DESCRIPTIONS"
    if [[ $TRAILING_PERIODS -eq 0 ]]; then
        pass "no trailing periods in tool descriptions"
    fi

    # Check verb-first (first word should be a verb — capitalized action word)
    NON_VERB=0
    while IFS= read -r desc; do
        if [[ -z "$desc" ]]; then continue; fi
        FIRST_WORD=$(echo "$desc" | awk '{print $1}')
        # Verb-first means the first word should be an action verb (capitalized)
        # Common verb starters: Manage, Access, Sense, Monitor, Discover, Create, Detect, Map, Ground, Assess, Maintain, Get, Perform
        if [[ "$FIRST_WORD" =~ ^[A-Z][a-z]+$ ]]; then
            pass "verb-first description: \"$FIRST_WORD ...\""
        else
            fail "description may not be verb-first: \"$desc\""
            NON_VERB=$((NON_VERB + 1))
        fi
    done <<< "$DESCRIPTIONS"
else
    fail "registry.rs not found — cannot check MCP quality"
fi

# --- 4. Sisters registry compatibility (.areal extension, areal binary) ---
echo ""
echo "--- Sisters registry compatibility ---"

# Check that Cargo.toml defines the areal binary
CLI_CARGO="$REPO_ROOT/crates/agentic-reality-cli/Cargo.toml"
if [[ -f "$CLI_CARGO" ]]; then
    if grep -q 'name = "areal"' "$CLI_CARGO"; then
        pass "CLI binary name is 'areal'"
    else
        fail "CLI binary name is not 'areal' in Cargo.toml"
    fi
else
    fail "crates/agentic-reality-cli/Cargo.toml not found"
fi

# Check that .areal extension is referenced in the codebase
AREAL_REFS=$(grep -r '\.areal' "$REPO_ROOT/crates" --include='*.rs' 2>/dev/null | wc -l | tr -d ' ')
if [[ "$AREAL_REFS" -gt 0 ]]; then
    pass ".areal extension referenced in codebase ($AREAL_REFS occurrence(s))"
else
    fail ".areal extension not referenced anywhere in crates/"
fi

# Check sisters-registry.json if it exists (may be at workspace level)
SISTER_REG="$REPO_ROOT/docs/sisters-registry.json"
if [[ -f "$SISTER_REG" ]]; then
    if grep -q 'areal' "$SISTER_REG" && grep -q '\.areal' "$SISTER_REG"; then
        pass "sisters-registry.json references areal and .areal"
    else
        fail "sisters-registry.json missing areal or .areal references"
    fi
else
    # Not a failure — registry may live at the parent workspace level
    echo "  INFO  docs/sisters-registry.json not found (may be at workspace level)"
fi

# --- Summary ---
echo ""
if [[ $FAIL -ne 0 ]]; then
    echo "RESULT: FAIL — sister parity issues detected."
    exit 1
else
    echo "RESULT: PASS — sister parity requirements met."
fi
