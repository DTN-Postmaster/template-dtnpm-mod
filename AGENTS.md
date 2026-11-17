# Guidelines for Autonomous Agents & AI Assistants — `template-dtnpm-mod`

This reference applies strictly to any AI agent intervention on the `template-dtnpm-mod` repository.

## Documentation Language
English must be used as the language for all documentation, unless the file is suffixed with `.fr.md` or another language is explicitly requested in the prompt.

## 1. Architecture Rules
- Every extension module must implement the `dtnpm_web_core::WebModule` trait.
- HTTP route isolation must follow the `/api/<module_id>/v1` prefix.
- SQLite migrations are executed in batches via `.sql` scripts located in `migrations/`.

## 2. Mandatory Security Standards
- **Zero `unwrap()` and zero `panic!`** in library code (`src/lib.rs`, `src/routes.rs`, `src/service.rs`) and the executable entry point (`src/main.rs`). The `-D clippy::unwrap_used` and `-D clippy::panic` lints are enabled.
- **Secret leakage protection**: Never accept only `--token` on the command line; enforce `--token-file` / `DTN_HDY_TOKEN_FILE`.
- **Secure HTML/Markdown rendering**: Exclusively reuse `ui/src/utils/markdown.ts` for rendering user content. Never use unescaped `innerHTML`.
- **Strict BPSec by default**: Never relax `VerifyPolicy::Strict` unless explicitly requested by the user with the `--allow-unauthenticated-bps-opt-in` flag.

## 3. Validation Procedure
Before committing any change:
```bash
./scripts/check-security.sh --strict
cargo test --workspace --all-targets
```
Any compiler warning (`cargo clippy`) or audit failure (`cargo-deny`, `gitleaks`) is blocking.
