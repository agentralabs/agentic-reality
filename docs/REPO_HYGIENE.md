# Repository Hygiene Policy

## What NOT to commit

- **Secrets and credentials:** `.env` files, API keys, tokens, private keys,
  `AGENTIC_REALITY_AUTH_TOKEN` values, or any `AGENTIC_TOKEN` material.
- **Large binaries:** compiled artifacts, `.areal` data files larger than 1 MB,
  release tarballs, or vendored C libraries.
- **Planning docs that reference internal strategy:** files in `planning-docs/`
  are gitignored and must stay local. Never commit spec drafts, roadmaps, or
  competitive analysis.
- **Editor and OS artifacts:** `.DS_Store`, `Thumbs.db`, `*.swp`, `.idea/`,
  `.vscode/` (unless shared settings are intentional).
- **Build output:** the `target/` directory is always gitignored.

## Required controls

- `.gitignore` must cover `target/`, `planning-docs/`, `.env*`, `*.areal`
  (data files), and platform artifacts.
- Pre-push guardrails must pass before every push:
  ```bash
  bash scripts/check-canonical-consistency.sh
  bash scripts/check-command-surface.sh
  bash scripts/check-mcp-consolidation.sh
  ```
- Commit messages follow conventional prefixes: `feat:`, `fix:`, `chore:`,
  `docs:`, `test:`, `refactor:`.
- Never add `Co-Authored-By: Claude` to commits.

## Pre-push checklist

1. `cargo fmt --all -- --check` passes.
2. `cargo clippy --workspace --all-targets -- -D warnings` passes.
3. `cargo test --workspace` passes.
4. All three guardrail scripts listed above pass.
5. No secrets or large binaries are staged (`git diff --cached --stat`).
6. Commit message uses a conventional prefix.
