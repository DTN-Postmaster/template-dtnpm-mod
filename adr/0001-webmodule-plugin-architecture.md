# ADR 0001: Architecture modulaire fullstack WebModule

## Statut
Accepté

## Contexte
DTN Postmaster (`dtnpm`) permet d'intégrer des fonctionnalités applicatives (Chat, Monitoring, Télémétrie, Messagerie) sans modifier le cœur du serveur web ni le BPA Hardy. Le trait `dtnpm_web_core::WebModule` standardise le cycle de vie, les migrations de bases SQLite partagées, l'enregistrement des services DTN et le montage des routes REST sous Axum.

## Décision
1. Chaque extension implémente `dtnpm_web_core::WebModule`.
2. Les migrations SQL sont isolées dans `migrations/` et appliquées via la méthode `migrate(&self, conn: &Connection)`.
3. Les routes API sont préfixées dynamiquement par `/api/<module_id>/v1`.
4. La réception de bundles DTN s'effectue via un service `hardy_bpa::services::Service` enregistré auprès du BPA.
5. Toute communication BPA suit le profil strict BPSec (RFC 9172) par défaut.

## Conséquences
- Découplage strict entre le cœur dtnpm et les extensions tierces.
- Facilité d'instanciation et de tests intégrés grâce à `TestServerBuilder`.
- Conformité Cyber Resilience Act (CRA) garantie par construction.
