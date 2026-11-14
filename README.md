# Template DTN Postmaster WebModule (`template-dtnpm-mod`)

Modèle de référence pour créer rapidement un module d'extension WebUI fullstack sécurisé pour DTN Postmaster (`dtnpm`).

## Fonctionnalités incluses
- Implémentation complète du trait [`dtnpm_web_core::WebModule`].
- Service BPA natif BPv7 recevant les bundles sur `dtn://<station>/template`.
- Routes REST Axum préfixées `/api/template_mod/v1/...` (CRUD avec stockage SQLite partagé).
- Interface Preact / Vite avec badge de statut BPSec (`SecurityBadge`) et rendu Markdown durci contre le XSS.
- Sécurité clé en main conforme au **Cyber Resilience Act (CRA)** et à l'**OWASP Top 10** :
  - Politique BPSec RFC 9172 stricte par défaut.
  - Gestion des secrets sécurisée via `--token-file` / `DTN_HDY_TOKEN_FILE`.
  - Contrôle strict clippy (`-D clippy::unwrap_used`, `-D clippy::panic`).
  - Scripts d'audit et hooks git automatisés (`./scripts/check-security.sh`).

## Comment instancier ce template

1. Cloner ou dupliquer ce dépôt sous le nom de votre module (ex: `dtnpm-mod-telemetry`).
2. Mettre à jour `name = "dtnpm-mod-telemetry"` dans `Cargo.toml` et `ui/package.json`.
3. Renommer `TemplateModule` et modifier l'identifiant dans `fn id(&self) -> &'static str`.
4. Adapter les schémas SQL dans `migrations/` et les routes dans `src/routes.rs`.
5. Exécuter `./scripts/check-security.sh` et valider les tests : `cargo test`.
