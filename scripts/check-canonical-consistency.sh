#!/usr/bin/env bash
set -euo pipefail

# check-canonical-consistency.sh — Verify all canonical files exist.
# Exits 1 on any missing file.

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FAIL=0

pass() { echo "  PASS  $1"; }
fail() { echo "  FAIL  $1"; FAIL=1; }

echo "=== Canonical Consistency Check ==="
echo ""

# --- 1. Doc pages (12 required) ---
echo "--- Documentation pages (docs/*.md) ---"
DOC_PAGES=(
    QUICKSTART
    ARCHITECTURE
    API
    CLI
    MCP-TOOLS
    INVENTIONS
    SISTER-INTEGRATION
    CONCEPTS
    EXAMPLES
    FAQ
    TROUBLESHOOTING
    PRIVACY
)
for page in "${DOC_PAGES[@]}"; do
    if [[ -f "$REPO_ROOT/docs/${page}.md" ]]; then
        pass "docs/${page}.md"
    else
        fail "docs/${page}.md missing"
    fi
done

# --- 2. SVG diagrams (4 required) ---
echo ""
echo "--- SVG diagrams (docs/assets/*.svg) ---"
SVGS=(
    architecture
    reality-domains
    resource-body
    context-fingerprint
)
for svg in "${SVGS[@]}"; do
    if [[ -f "$REPO_ROOT/docs/assets/${svg}.svg" ]]; then
        pass "docs/assets/${svg}.svg"
    else
        fail "docs/assets/${svg}.svg missing"
    fi
done

# --- 3. Root governance files ---
echo ""
echo "--- Root governance files ---"
ROOT_FILES=(
    README.md
    LICENSE
    CHANGELOG.md
    CLAUDE.md
    SECURITY.md
    CONTRIBUTING.md
    CODE_OF_CONDUCT.md
)
for f in "${ROOT_FILES[@]}"; do
    if [[ -f "$REPO_ROOT/$f" ]]; then
        pass "$f"
    else
        fail "$f missing"
    fi
done

# --- 4. CI workflow ---
echo ""
echo "--- CI workflow ---"
if [[ -f "$REPO_ROOT/.github/workflows/ci.yml" ]]; then
    pass ".github/workflows/ci.yml"
else
    fail ".github/workflows/ci.yml missing"
fi

# --- 5. Install script ---
echo ""
echo "--- Install script ---"
if [[ -f "$REPO_ROOT/scripts/install.sh" ]]; then
    pass "scripts/install.sh"
else
    fail "scripts/install.sh missing"
fi

# --- 6. All 4 Cargo.toml files ---
echo ""
echo "--- Crate Cargo.toml files ---"
CRATES=(
    agentic-reality
    agentic-reality-mcp
    agentic-reality-cli
    agentic-reality-ffi
)
for crate in "${CRATES[@]}"; do
    if [[ -f "$REPO_ROOT/crates/${crate}/Cargo.toml" ]]; then
        pass "crates/${crate}/Cargo.toml"
    else
        fail "crates/${crate}/Cargo.toml missing"
    fi
done

# --- Summary ---
echo ""
if [[ $FAIL -ne 0 ]]; then
    echo "RESULT: FAIL — one or more canonical files are missing."
    exit 1
else
    echo "RESULT: PASS — all canonical files present."
fi
