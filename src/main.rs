use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use clap::Parser;
use dtnpm_core::cla_manager::ClaManager;
use dtnpm_core::config::{HamConfig, WebConfig};
use dtnpm_core::internal_storage::InternalStorage;
use dtnpm_core::log_buffer::LogBuffer;
use dtnpm_core::routing::RouteManager;
use dtnpm_core::security::DirectionalKeyStore;
use dtnpm_core::{BpaClient, GrpcBpaClient, ServiceTracker};
use dtnpm_web_core::{
    ModuleContext, WebBundleInjector, WebModule, register_modules, start_web_server,
};
use hardy_bpa::bpa::BpaRegistration;
use hardy_bpa::cla::ClaInit;
use template_dtnpm_mod::TemplateModule;
use tracing::{info, warn};

#[derive(Debug, Parser)]
#[command(
    name = "template-dtnpm-mod",
    about = "Standalone DTN Extension WebModule daemon"
)]
struct Args {
    /// Hardy BPA gRPC endpoint
    #[arg(long, default_value = "http://127.0.0.1:50051")]
    bpa_grpc: String,

    /// Web HTTP listen address
    #[arg(long, default_value = "127.0.0.1:8082")]
    listen: SocketAddr,

    /// Local DTN node EID
    #[arg(long, default_value = "dtn://local/")]
    node_id: String,

    /// Optional radio callsign
    #[arg(long)]
    callsign: Option<String>,

    /// Optional SQLite data directory (uses in-memory DB if omitted)
    #[arg(long)]
    data_dir: Option<PathBuf>,

    /// Optional Bearer authentication token for protected gRPC BPA endpoints (deprecated for security: prefer --token-file)
    #[arg(long, env = "DTN_HDY_TOKEN")]
    token: Option<String>,

    /// Path to a file containing the Bearer authentication token for protected gRPC BPA endpoints
    #[arg(long, env = "DTN_HDY_TOKEN_FILE")]
    token_file: Option<PathBuf>,

    /// Optional Unix Domain Socket path for connecting to a protected local BPA
    #[arg(long, env = "DTN_HDY_SOCKET")]
    socket: Option<String>,

    /// Explicit opt-in flag to allow unauthenticated or unsigned bundles (violates RFC 9172 strict profile)
    #[arg(
        long,
        env = "DTN_HDY_ALLOW_UNAUTHENTICATED_BPS",
        default_value = "false"
    )]
    allow_unauthenticated_bps_opt_in: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();

    if args.allow_unauthenticated_bps_opt_in {
        warn!(
            "SECURITY RISK: --allow-unauthenticated-bps-opt-in is enabled! Bundles without valid BPSec signatures will be processed."
        );
    }

    let mut config = HamConfig::default();
    config.node_id = args.node_id;
    config.callsign = args.callsign;
    config.web = Some(WebConfig {
        address: args.listen,
        static_dir: dtnpm_core::config::default_web_static_dir(),
    });
    if let Some(dir) = args.data_dir {
        config.storage.in_memory = false;
        config.storage.data_dir = Some(dir);
    } else {
        config.storage.in_memory = true;
    }
    config.normalize();

    let internal_storage = if config.storage.in_memory {
        InternalStorage::new_in_memory_with_quota(config.application.quota.max_storage)?
    } else {
        InternalStorage::new_disk_with_quota(
            config
                .storage
                .data_dir
                .as_deref()
                .unwrap_or_else(|| Path::new(".")),
            config.application.quota.max_storage,
        )?
    };

    let auth_token = resolve_token(args.token.as_deref(), args.token_file.as_deref())?;
    let grpc_bpa = Arc::new(GrpcBpaClient::with_auth(
        args.bpa_grpc,
        auth_token.as_deref(),
        args.socket.as_deref(),
    ));
    let bpa_reg: Arc<dyn BpaRegistration> = grpc_bpa.clone().as_registration();
    let bpa: Arc<dyn BpaClient> = grpc_bpa;
    let service_tracker = ServiceTracker::new(bpa_reg.clone());
    let injector = WebBundleInjector::new();
    let _ = bpa_reg
        .register_cla(
            "web-injector-template".to_string(),
            Arc::new(injector.clone()),
            None,
            ClaInit::default(),
        )
        .await;

    let modules: Vec<Arc<dyn WebModule>> = vec![Arc::new(TemplateModule::new())];
    let ctx = ModuleContext::new(
        Arc::new(config),
        bpa,
        injector,
        service_tracker,
        internal_storage,
        ClaManager::new(),
        Arc::new(RouteManager::new(Vec::new(), None)),
        LogBuffer::new(250),
        Arc::new(DirectionalKeyStore::empty()),
    )
    .with_capabilities(&modules);

    register_modules(&ctx, &modules).await?;
    let _web_handle = start_web_server(ctx, modules, args.listen).await?;

    info!(
        "Template WebModule running on {}. Press Ctrl+C to terminate.",
        args.listen
    );
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Template WebModule...");
    Ok(())
}

fn resolve_token(token: Option<&str>, token_file: Option<&Path>) -> anyhow::Result<Option<String>> {
    if let Some(path) = token_file {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read token file '{}': {e}", path.display()))?;
        let trimmed = raw.trim().to_string();
        if trimmed.is_empty() {
            anyhow::bail!("Token file '{}' is empty", path.display());
        }
        return Ok(Some(trimmed));
    }
    if let Some(tok) = token.filter(|t| !t.trim().is_empty()) {
        warn!("Passing token via CLI argument is deprecated for security; prefer --token-file");
        return Ok(Some(tok.trim().to_string()));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::resolve_token;

    #[test]
    fn resolve_token_reads_and_trims_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("token");
        std::fs::write(&path, "  super-secret-token\n").expect("write");
        let resolved = resolve_token(None, Some(&path)).expect("resolve");
        assert_eq!(resolved.as_deref(), Some("super-secret-token"));
    }

    #[test]
    fn resolve_token_rejects_empty_or_missing_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let empty = dir.path().join("empty");
        std::fs::write(&empty, "   \n").expect("write");
        assert!(resolve_token(None, Some(&empty)).is_err());

        let missing = dir.path().join("missing");
        assert!(resolve_token(None, Some(&missing)).is_err());
    }
}
