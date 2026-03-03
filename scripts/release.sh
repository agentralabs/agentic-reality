#!/usr/bin/env bash
# AgenticReality Release Script
# Usage: bash scripts/release.sh [VERSION]
#
# This script:
# 1. Validates the release is ready
# 2. Updates version numbers
# 3. Runs full test suite
# 4. Builds release binaries
# 5. Creates git tag
# 6. Builds release artifacts
#
# Prerequisites:
# - cargo, rustup
# - git
# - cross (for cross-compilation, optional)

set -euo pipefail

VERSION="${1:-}"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

info()    { echo -e "${BLUE}[INFO]${NC} $1"; }
success() { echo -e "${GREEN}[OK]${NC} $1"; }
warn()    { echo -e "${YELLOW}[WARN]${NC} $1"; }
error()   { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }
step()    { echo -e "\n${BOLD}=== $1 ===${NC}\n"; }

# Validate version argument
validate_version() {
    if [ -z "$VERSION" ]; then
        echo "Usage: bash scripts/release.sh <VERSION>"
        echo "  Example: bash scripts/release.sh 0.1.0"
        exit 1
    fi

    if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
        error "Invalid version format: $VERSION (expected: MAJOR.MINOR.PATCH)"
    fi

    info "Preparing release v$VERSION"
}

# Check prerequisites
check_prerequisites() {
    step "Checking prerequisites"

    command -v cargo &>/dev/null || error "cargo not found"
    command -v git &>/dev/null || error "git not found"
    command -v rustup &>/dev/null || error "rustup not found"

    # Check for clean working directory
    if [ -n "$(git status --porcelain)" ]; then
        warn "Working directory is not clean"
        git status --short
        echo ""
        read -p "Continue anyway? (y/N) " -n 1 -r
        echo ""
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi

    # Check we are on main branch
    local branch
    branch=$(git branch --show-current)
    if [ "$branch" != "main" ] && [ "$branch" != "develop" ]; then
        warn "Not on main or develop branch (current: $branch)"
    fi

    success "Prerequisites satisfied"
}

# Run guardrails
run_guardrails() {
    step "Running guardrails"

    if [ -f "$PROJECT_ROOT/scripts/check-canonical-consistency.sh" ]; then
        info "Running canonical consistency check..."
        bash "$PROJECT_ROOT/scripts/check-canonical-consistency.sh" || warn "Canonical consistency check had warnings"
    fi

    if [ -f "$PROJECT_ROOT/scripts/check-command-surface.sh" ]; then
        info "Running command surface check..."
        bash "$PROJECT_ROOT/scripts/check-command-surface.sh" || warn "Command surface check had warnings"
    fi

    if [ -f "$PROJECT_ROOT/scripts/check-mcp-consolidation.sh" ]; then
        info "Running MCP consolidation check..."
        bash "$PROJECT_ROOT/scripts/check-mcp-consolidation.sh" || warn "MCP consolidation check had warnings"
    fi

    success "Guardrails complete"
}

# Update version in Cargo.toml files
update_versions() {
    step "Updating versions to $VERSION"

    # Update workspace version
    sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" "$PROJECT_ROOT/Cargo.toml"
    rm -f "$PROJECT_ROOT/Cargo.toml.bak"

    # Update each crate's Cargo.toml if they have their own version
    for crate_dir in "$PROJECT_ROOT"/crates/*/; do
        local cargo_toml="$crate_dir/Cargo.toml"
        if [ -f "$cargo_toml" ]; then
            sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" "$cargo_toml"
            rm -f "$cargo_toml.bak"
        fi
    done

    success "Versions updated to $VERSION"
}

# Run full test suite
run_tests() {
    step "Running full test suite"

    cd "$PROJECT_ROOT"

    info "Formatting check..."
    cargo fmt --all -- --check || error "Formatting check failed"

    info "Clippy lints..."
    cargo clippy --all-targets --all-features -- -D warnings || error "Clippy failed"

    info "Unit tests..."
    cargo test --lib || error "Unit tests failed"

    info "Integration tests..."
    cargo test --test '*' || error "Integration tests failed"

    info "Doc tests..."
    cargo test --doc || error "Doc tests failed"

    # Count tests
    local test_output
    test_output=$(cargo test 2>&1)
    local passed
    passed=$(echo "$test_output" | grep -o '[0-9]* passed' | head -1 | grep -o '[0-9]*')
    info "Tests passed: ${passed:-0}"

    if [ "${passed:-0}" -lt 250 ]; then
        warn "Test count (${passed:-0}) is below target (250)"
    fi

    # MCP hardening check
    local unwraps
    unwraps=$(grep -r "\.unwrap()" crates/agentic-reality-mcp/src/ --include="*.rs" 2>/dev/null | grep -v test | wc -l | tr -d ' ')
    if [ "$unwraps" -gt 0 ]; then
        error "Found $unwraps unwrap() calls in MCP crate"
    fi
    success "Zero unwraps in MCP crate"

    success "All tests passed"
}

# Build release binaries
build_release() {
    step "Building release binaries"

    cd "$PROJECT_ROOT"

    # Native build
    info "Building native release..."
    cargo build --release

    # Verify binary
    if [ -f "target/release/areal" ]; then
        local size
        size=$(du -h "target/release/areal" | cut -f1)
        success "Built native binary: target/release/areal ($size)"
    else
        error "Native binary not found"
    fi

    # Cross-compilation targets (if cross is available)
    if command -v cross &>/dev/null; then
        local targets=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "x86_64-apple-darwin" "aarch64-apple-darwin")

        for target in "${targets[@]}"; do
            info "Cross-compiling for $target..."
            cross build --release --target "$target" 2>/dev/null && \
                success "Built for $target" || \
                warn "Cross-compilation for $target failed (may need target installed)"
        done
    else
        info "cross not found, skipping cross-compilation"
        info "Install with: cargo install cross"
    fi
}

# Create release artifacts
create_artifacts() {
    step "Creating release artifacts"

    local artifact_dir="$PROJECT_ROOT/target/release/artifacts"
    mkdir -p "$artifact_dir"

    # Determine current platform
    local os arch platform
    os="$(uname -s | tr '[:upper:]' '[:lower:]')"
    arch="$(uname -m)"
    [ "$os" = "darwin" ] && os="macos"
    [ "$arch" = "arm64" ] && arch="aarch64"
    platform="${os}-${arch}"

    # Package native binary
    local archive_name="agentic-reality-v${VERSION}-${platform}"
    local archive_dir="$artifact_dir/$archive_name"
    mkdir -p "$archive_dir"

    cp "$PROJECT_ROOT/target/release/areal" "$archive_dir/"
    cp "$PROJECT_ROOT/README.md" "$archive_dir/"
    cp "$PROJECT_ROOT/LICENSE" "$archive_dir/"
    cp "$PROJECT_ROOT/CHANGELOG.md" "$archive_dir/"

    cd "$artifact_dir"
    tar -czf "${archive_name}.tar.gz" "$archive_name"
    rm -rf "$archive_dir"

    # Generate checksums
    if command -v sha256sum &>/dev/null; then
        sha256sum "${archive_name}.tar.gz" > "${archive_name}.tar.gz.sha256"
    elif command -v shasum &>/dev/null; then
        shasum -a 256 "${archive_name}.tar.gz" > "${archive_name}.tar.gz.sha256"
    fi

    success "Created artifact: $artifact_dir/${archive_name}.tar.gz"
    cd "$PROJECT_ROOT"
}

# Update CHANGELOG
update_changelog() {
    step "Checking CHANGELOG"

    if ! grep -q "\[$VERSION\]" "$PROJECT_ROOT/CHANGELOG.md"; then
        warn "CHANGELOG.md does not have an entry for version $VERSION"
        warn "Please update CHANGELOG.md before tagging"
    else
        success "CHANGELOG.md has entry for $VERSION"
    fi
}

# Create git tag
create_tag() {
    step "Creating git tag"

    local tag="v$VERSION"

    if git tag -l | grep -qF "$tag"; then
        warn "Tag $tag already exists"
        read -p "Delete and recreate? (y/N) " -n 1 -r
        echo ""
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            git tag -d "$tag"
        else
            return
        fi
    fi

    git add -A
    git commit -m "chore: release v$VERSION" || info "Nothing to commit"
    git tag -a "$tag" -m "Release $tag"

    success "Created tag $tag"
    info "Push with: git push origin main --tags"
}

# Print summary
print_summary() {
    echo ""
    echo "============================================================"
    echo "         Release v$VERSION Summary"
    echo "============================================================"
    echo ""
    echo "  Binary:     target/release/areal"
    echo "  Artifacts:  target/release/artifacts/"
    echo "  Tag:        v$VERSION"
    echo ""
    echo "  Next steps:"
    echo "    1. Review the changes"
    echo "    2. Push: git push origin main --tags"
    echo "    3. Create GitHub release from tag"
    echo "    4. Upload artifacts to release"
    echo "    5. Publish to crates.io: cargo publish -p agentic-reality"
    echo ""
}

# Main
main() {
    validate_version
    check_prerequisites
    run_guardrails
    update_versions
    run_tests
    build_release
    create_artifacts
    update_changelog
    create_tag
    print_summary

    success "Release v$VERSION prepared successfully"
}

main
