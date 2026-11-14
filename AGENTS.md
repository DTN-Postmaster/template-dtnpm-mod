# Directives pour Agents Autonomes & Assistants IA - template-dtnpm-mod

Ce référentiel s'applique impérativement à toute intervention d'un agent IA sur le dépôt `template-dtnpm-mod`.

## 1. Règles d'Architecture
- Tout module d'extension doit implémenter le trait `dtnpm_web_core::WebModule`.
- L'isolation des routes HTTP doit respecter le préfixe `/api/<module_id>/v1`.
- Les migrations SQLite s'effectuent par lots via des scripts `.sql` situés dans `migrations/`.

## 2. Standards de Sécurité Obligatoires
- **Zéro `unwrap()` et zéro `panic!`** dans le code de bibliothèque (`src/lib.rs`, `src/routes.rs`, `src/service.rs`) et le point d'entrée exécutable (`src/main.rs`). Les lints `-D clippy::unwrap_used` et `-D clippy::panic` sont activés.
- **Protection contre les fuites de secrets** : Ne jamais accepter uniquement `--token` en ligne de commande ; imposer `--token-file` / `DTN_HDY_TOKEN_FILE`.
- **Rendu HTML/Markdown sécurisé** : Réutiliser exclusivement `ui/src/utils/markdown.ts` pour le rendu de contenu utilisateur. Ne jamais utiliser `innerHTML` non échappé.
- **BPSec Strict par défaut** : Ne jamais relâcher la politique `VerifyPolicy::Strict` sauf demande explicite de l'utilisateur avec l'indicateur `--allow-unauthenticated-bps-opt-in`.

## 3. Procédure de Validation
Avant de valider tout commit :
```bash
./scripts/check-security.sh --strict
cargo test --workspace --all-targets
```
Tout avertissement du compilateur (`cargo clippy`) ou échec d'audit (`cargo-deny`, `gitleaks`) est bloquant.
