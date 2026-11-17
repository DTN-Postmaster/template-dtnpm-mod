# Template DTN Postmaster WebModule (`template-dtnpm-mod`)

Reference template to quickly create a secure fullstack WebUI extension module for DTN Postmaster (`dtnpm`).

## Included Features
- Full implementation of the [`dtnpm_web_core::WebModule`] trait.
- Native BPv7 BPA service receiving bundles on `dtn://<station>/template`.
- Axum REST routes prefixed `/api/template_mod/v1/...` (CRUD with shared SQLite storage).
- Preact / Vite frontend with BPSec status badge (`SecurityBadge`) and XSS-hardened Markdown rendering.
- Turnkey security compliant with the **Cyber Resilience Act (CRA)** and **OWASP Top 10**:
  - Strict RFC 9172 BPSec policy by default.
  - Secure secret management via `--token-file` / `DTN_HDY_TOKEN_FILE`.
  - Strict clippy enforcement (`-D clippy::unwrap_used`, `-D clippy::panic`).
  - Automated audit scripts and git hooks (`./scripts/check-security.sh`).

## How to Instantiate This Template

1. Clone or duplicate this repository under your module's name (e.g., `dtnpm-mod-telemetry`).
2. Update `name = "dtnpm-mod-telemetry"` in `Cargo.toml` and `ui/package.json`.
3. Rename `TemplateModule` and change the identifier in `fn id(&self) -> &'static str`.
4. Adapt SQL schemas in `migrations/` and routes in `src/routes.rs`.
5. Run `./scripts/check-security.sh` and validate tests: `cargo test`.
