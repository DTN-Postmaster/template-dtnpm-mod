# Contributing Guide — `template-dtnpm-mod`

Thank you for contributing to the `template-dtnpm-mod` project.

## Prerequisites
- Rust 1.85 or later (2024 edition)
- `cargo-audit`, `cargo-deny`, `gitleaks`
- Node.js / pnpm (for the UI part)

## Installing Git Hooks
Immediately enable the pre-commit validation hook:
```bash
./scripts/install-hooks.sh
```

## Development Cycle
1. Modify the Rust files or the UI frontend.
2. Run local checks:
   ```bash
   ./scripts/check-security.sh
   cargo test --workspace --all-targets
   ```
3. Follow [Conventional Commits](https://www.conventionalcommits.org/) conventions.
