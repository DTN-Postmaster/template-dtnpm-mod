# Guidelines for Autonomous Agents & AI Assistants — `template-dtnpm-mod`

This reference applies strictly to any AI agent intervention on the `template-dtnpm-mod` repository.

## Documentation & User Interface Language
- **Documentation**: English must be used for all documentation, unless the file is suffixed with `.fr.md` or another language is explicitly requested in the prompt.
- **User Interface (100% English)**: Every user-facing string in `ui/src/` and `ui/tests/` (headings, form labels, placeholders, buttons, `SecurityBadge` text, `title` tooltips, `aria-label`s, dialogs, and UI error messages) MUST be written in English. Never introduce French or mixed-language copy.

## 1. Architecture & WebUI Navigation Rules
- Every extension module must implement the `dtnpm_web_core::WebModule` trait.
- HTTP route isolation must follow the `/api/<module_id>/v1` prefix.
- SQLite migrations are executed in batches via `.sql` scripts located in `migrations/`.
- **WebUI Navigation Placement**: When integrating into `dtnpm-web-core`, administrative, routing, filtering, security, storage, or diagnostic modules MUST be placed as sub-tabs under **Station** (`StationSubTab` in `dtnpm-web-core/ui/src/components/ConfigView.tsx`, e.g., `Rego Routing` between `Routing (RIB)` and `BPSec Security`), gated via `/api/capabilities` (`enabledModules`). Never add administrative modules as top-level tabs in `Navbar.tsx` (reserved strictly for primary operational workflows: `Station`, `Chat`, `Files`, `SenML`, `Send Bundle`).
- **Shared UI Parity**: Keep shared UI components (`SecurityBadge.tsx`, `utils/markdown.ts`) synchronized with `dtnpm-web-core/ui`.

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
cd ui && pnpm test:e2e
```
If UI changes are integrated into `dtnpm-web-core`, also run `cd ../dtnpm-web-core/ui && pnpm build && pnpm test:e2e` and verify zero non-English UI strings remain.
Any compiler warning (`cargo clippy`) or audit failure (`cargo-deny`, `gitleaks`) is blocking.
