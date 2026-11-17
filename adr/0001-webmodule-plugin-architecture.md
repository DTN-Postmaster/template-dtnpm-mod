# ADR 0001: Modular Fullstack `WebModule` Architecture

## Status
Accepted

## Context
DTN Postmaster (`dtnpm`) allows integrating application features (Chat, Monitoring, Telemetry, Messaging) without modifying the core web server or the Hardy BPA. The `dtnpm_web_core::WebModule` trait standardizes the lifecycle, shared SQLite database migrations, DTN service registration, and REST route mounting under Axum.

## Decision
1. Every extension implements `dtnpm_web_core::WebModule`.
2. SQL migrations are isolated in `migrations/` and applied via the `migrate(&self, conn: &Connection)` method.
3. API routes are dynamically prefixed with `/api/<module_id>/v1`.
4. DTN bundle reception is handled via a `hardy_bpa::services::Service` registered with the BPA.
5. All BPA communication follows the strict BPSec (RFC 9172) profile by default.

## Consequences
- Strict decoupling between the `dtnpm` core and third-party extensions.
- Easy instantiation and integrated testing via `TestServerBuilder`.
- Cyber Resilience Act (CRA) compliance guaranteed by design.
