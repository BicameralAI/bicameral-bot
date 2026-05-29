//! Local HTTP gateway for Bicameral ingest, review, and mod execution.
//!
//! The gateway accepts protocol-shaped inputs via HTTP (file-first ingest,
//! future webhook integration, local dashboard). It does not claim canonical
//! authority — all inputs flow through governance before materialization.
//!
//! Derived from ZeroClaw's gateway layer, narrowed to Bicameral's governance domain.
//! See NOTICE and runtime/UPSTREAM-ZEROCLAW.md for attribution.

pub mod routes;
pub mod state;

pub use state::AppState;

use axum::Router;
use bicameral_config::GatewayConfig;
use std::net::SocketAddr;
use tokio::sync::watch;

/// Start the gateway HTTP server.
///
/// Returns when a shutdown signal is received via the `shutdown_rx` channel.
pub async fn serve(
    config: &GatewayConfig,
    state: AppState,
    mut shutdown_rx: watch::Receiver<bool>,
) -> anyhow::Result<()> {
    let app = build_router(state);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!(%addr, "Gateway listening");

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.wait_for(|v| *v).await;
            tracing::info!("Gateway shutting down");
        })
        .await?;

    Ok(())
}

fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(routes::health_routes())
        .merge(routes::ingest_routes())
        .merge(routes::review_routes())
        .merge(routes::status_routes())
        .with_state(state)
}
