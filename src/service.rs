//! BPA Service implementation for `template-dtnpm-mod`.

use std::sync::Arc;

use bytes::Bytes;
use dtnpm_core::internal_storage::InternalStorage;
use dtnpm_core::security::{AuthStatus, DirectionalKeyStore, VerifyPolicy, verify_parsed_bundle};
use dtnpm_core::service_tracker::ServiceTracker;
use hardy_async::async_trait;
use hardy_bpa::services::{Result as ServiceResult, Service, ServiceSink, StatusNotify};
use hardy_bpa::stream::{Receiver, Segment, buffer_stream};
use hardy_bpv7::bundle::Id;
use hardy_bpv7::eid::Eid;
use hardy_bpv7::parse::parse;
use hardy_bpv7::status_report::ReasonCode;
use time::OffsetDateTime;
use tokio::sync::OnceCell;
use tracing::{debug, error, info, warn};

/// Permanent DTN BPv7 service for handling module bundles.
#[derive(Clone)]
pub struct TemplateService {
    internal_storage: InternalStorage,
    service_tracker: ServiceTracker,
    keystore: Arc<DirectionalKeyStore>,
    sink: Arc<OnceCell<Box<dyn ServiceSink>>>,
}

impl TemplateService {
    /// Creates a new service bound to internal storage, tracker, and keystore.
    pub fn new(
        internal_storage: InternalStorage,
        service_tracker: ServiceTracker,
        keystore: Arc<DirectionalKeyStore>,
    ) -> Self {
        Self {
            internal_storage,
            service_tracker,
            keystore,
            sink: Arc::new(OnceCell::new()),
        }
    }

    /// Access the registered BPA sink if available.
    pub fn sink(&self) -> Option<&dyn ServiceSink> {
        self.sink.get().map(|s| s.as_ref())
    }

    /// Access the service tracker reference.
    pub fn service_tracker(&self) -> &ServiceTracker {
        &self.service_tracker
    }

    async fn handle_delivered_bundle(&self, data: Bytes) -> ServiceResult<()> {
        let Ok(parsed) = parse(data) else {
            debug!("TemplateService: failed to parse delivered bundle");
            return Ok(());
        };

        let auth_status = verify_parsed_bundle(&parsed, &self.keystore);
        let policy = self.keystore.get_policy();
        if policy == VerifyPolicy::Strict && auth_status != AuthStatus::Valid {
            warn!(
                auth_status = ?auth_status,
                "TemplateService: dropping bundle under strict policy without valid BPSec signature"
            );
            return Ok(());
        }

        let bundle = &parsed.bundle;
        let source_eid = &bundle.primary.id.source;
        let dest_eid = &bundle.primary.destination;

        if source_eid.is_null() || bundle.primary.flags.is_admin_record {
            debug!("TemplateService: ignoring null source or administrative record");
            return Ok(());
        }

        let Some(payload_slice) = bundle
            .blocks
            .get(&1)
            .and_then(|block| block.payload(&parsed.data))
        else {
            debug!("TemplateService: bundle has no payload block");
            return Ok(());
        };

        let content = String::from_utf8_lossy(payload_slice).to_string();
        let bundle_id = bundle.primary.id.to_key();
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let is_valid = if auth_status == AuthStatus::Valid {
            1
        } else {
            0
        };

        // Persist item safely using parameterized SQL
        let conn_handle = self.internal_storage.connection();
        if let Ok(conn) = conn_handle.lock() {
            let res = conn.execute(
                "INSERT INTO module_items (id, source, destination, title, content, created_at, is_valid_bpsec)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (
                    &bundle_id,
                    &source_eid.to_string(),
                    &dest_eid.to_string(),
                    "Inbound Item",
                    &content,
                    now,
                    is_valid,
                ),
            );
            if let Err(e) = res {
                error!(bundle_id = %bundle_id, error = %e, "Failed to persist inbound item");
            } else {
                info!(
                    bundle_id = %bundle_id,
                    source = %source_eid,
                    "Successfully persisted inbound module item"
                );
            }
        }

        Ok(())
    }
}

#[async_trait]
impl Service for TemplateService {
    async fn on_register(&self, _source: &Eid, sink: Box<dyn ServiceSink>) {
        let _ = self.sink.set(sink);
        info!("TemplateService registered with BPA");
    }

    async fn on_unregister(&self) {
        warn!("TemplateService unregistered from BPA");
    }

    async fn on_status_notify(
        &self,
        _bundle_id: &Id,
        _from: &Eid,
        _kind: StatusNotify,
        _reason: ReasonCode,
        _timestamp: Option<OffsetDateTime>,
    ) {
    }

    async fn on_deliver(
        &self,
        bundle_id: &Id,
        _expiry: OffsetDateTime,
        total_len: u64,
        stream: &mut dyn Receiver<Segment>,
    ) -> ServiceResult<()> {
        let max_size = 10 * 1024 * 1024; // 10 MiB limit
        if total_len > max_size {
            warn!(
                bundle_id = %bundle_id.to_key(),
                total_len,
                "Inbound bundle rejected: payload exceeds 10 MiB limit"
            );
            return Err(hardy_bpa::services::Error::PayloadTooLarge {
                size: total_len,
                max: max_size,
            });
        }

        let data = buffer_stream(stream, total_len).await?;
        self.handle_delivered_bundle(data).await
    }
}
