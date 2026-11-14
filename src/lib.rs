//! Fullstack WebUI extension module template for DTN Postmaster.
//! Implements [`dtnpm_web_core::WebModule`].

use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use hardy_async::async_trait;
use hardy_bpv7::eid::Service;
use rusqlite::Connection;

use dtnpm_web_core::{ModuleContext, WebModule};

pub mod routes;
pub mod service;

pub use service::TemplateService;

/// Initial SQLite database schema applied when registering this module.
pub const INITIAL_SCHEMA_SQL: &str = include_str!("../migrations/0001_initial_schema.sql");

/// Template WebModule exposing REST endpoints and registering a DTN service endpoint.
#[derive(Debug, Default, Clone)]
pub struct TemplateModule;

impl TemplateModule {
    /// Create a new instance of [`TemplateModule`].
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl WebModule for TemplateModule {
    fn id(&self) -> &'static str {
        "template_mod"
    }

    fn name(&self) -> &'static str {
        "Template Extension Module"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn services(&self) -> Vec<String> {
        vec!["template".to_string()]
    }

    fn migrate(&self, conn: &Connection) -> anyhow::Result<()> {
        conn.execute_batch(INITIAL_SCHEMA_SQL)?;
        Ok(())
    }

    async fn register_bpa(&self, ctx: &ModuleContext) -> anyhow::Result<()> {
        let template_service = Arc::new(TemplateService::new(
            ctx.internal_storage.clone(),
            ctx.service_tracker.clone(),
            ctx.keystore.clone(),
        ));

        ctx.service_tracker
            .register_service_with_type(
                Service::Dtn("template".into()),
                template_service,
                "Template DTN Extension",
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to register Template service: {e}"))?;

        Ok(())
    }

    fn api_routes(&self, ctx: &ModuleContext) -> Router<()> {
        Router::new()
            .route("/items", get(routes::list_items).post(routes::create_item))
            .route(
                "/items/{id}",
                get(routes::get_item).delete(routes::delete_item),
            )
            .with_state(ctx.clone())
    }
}
