//! Daemon lifecycle — start, run, and shutdown the Bicameral local runtime.
//!
//! Adapted from ZeroClaw's daemon module. The daemon owns the main event loop,
//! spawns the gateway, and coordinates graceful shutdown via signal handlers
//! and a watch channel for in-process reload requests.

use bicameral_audit::store::AuditStore;
use bicameral_config::{BicameralConfig, BicameralPaths};
use bicameral_gateway::{AppState, serve};
use tokio::sync::watch;

/// Why the daemon's main loop exited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DaemonExit {
    Shutdown,
    Reload,
}

/// Configuration for the daemon process.
#[derive(Debug, Clone)]
pub struct DaemonConfig {
    pub paths: BicameralPaths,
    pub config: BicameralConfig,
}

/// Run the Bicameral daemon.
///
/// This starts the gateway in the same process (v0.1 simplification per plan
/// section 7.1) and waits for a shutdown signal. Returns the exit reason so
/// the caller can decide whether to restart.
pub async fn run(daemon_config: DaemonConfig) -> anyhow::Result<DaemonExit> {
    tracing::info!("Starting Bicameral daemon");

    let audit = AuditStore::open(&daemon_config.paths.audit_dir)?;
    let state = AppState::new(audit);

    // Record daemon start
    {
        let mut audit = state.audit.write().await;
        let _ = audit.record(
            bicameral_audit::AuditAction::GatewayStarted,
            "daemon".to_string(),
            None,
            "Bicameral daemon started".to_string(),
        );
    }

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // Spawn gateway
    let gateway_config = daemon_config.config.gateway.clone();
    let gateway_state = state.clone();
    let gateway_handle = tokio::spawn(async move {
        if let Err(e) = serve(&gateway_config, gateway_state, shutdown_rx).await {
            tracing::error!(error = %e, "Gateway error");
        }
    });

    // Wait for exit signal
    let exit_reason = wait_for_exit_signal().await?;

    // Signal shutdown to gateway
    let _ = shutdown_tx.send(true);

    // Wait for gateway to finish
    let _ = gateway_handle.await;

    tracing::info!(reason = ?exit_reason, "Bicameral daemon exiting");
    Ok(exit_reason)
}

async fn wait_for_exit_signal() -> anyhow::Result<DaemonExit> {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let mut sigint = signal(SignalKind::interrupt())?;
        let mut sigterm = signal(SignalKind::terminate())?;

        tokio::select! {
            _ = sigint.recv() => {
                tracing::info!("Received SIGINT, shutting down");
                Ok(DaemonExit::Shutdown)
            }
            _ = sigterm.recv() => {
                tracing::info!("Received SIGTERM, shutting down");
                Ok(DaemonExit::Shutdown)
            }
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await?;
        tracing::info!("Received Ctrl+C, shutting down");
        Ok(DaemonExit::Shutdown)
    }
}
