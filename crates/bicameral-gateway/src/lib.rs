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

/// Returns true if the host address is a loopback address.
pub fn is_loopback_host(host: &str) -> bool {
    host == "127.0.0.1" || host == "::1" || host == "localhost"
}

/// Start the gateway HTTP server.
///
/// Returns when a shutdown signal is received via the `shutdown_rx` channel.
pub async fn serve(
    config: &GatewayConfig,
    state: AppState,
    mut shutdown_rx: watch::Receiver<bool>,
) -> anyhow::Result<()> {
    if !is_loopback_host(&config.host) {
        tracing::warn!(
            host = %config.host,
            "Gateway binding to non-loopback address. \
             v0.1 has no authentication — exposing the gateway outside \
             localhost allows unauthenticated mutation of candidate state. \
             Use a loopback address (127.0.0.1) unless you understand the risk."
        );
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use bicameral_audit::store::AuditStore;
    use tower::ServiceExt;

    fn test_state() -> AppState {
        let dir = tempfile::tempdir().unwrap();
        let audit = AuditStore::open(dir.path()).unwrap();
        AppState::new(audit)
    }

    #[test]
    fn default_gateway_config_is_loopback() {
        let config = GatewayConfig::default();
        assert!(
            is_loopback_host(&config.host),
            "Default gateway host {} is not loopback",
            config.host
        );
    }

    #[test]
    fn is_loopback_host_rejects_external() {
        assert!(!is_loopback_host("0.0.0.0"));
        assert!(!is_loopback_host("192.168.1.1"));
        assert!(!is_loopback_host("10.0.0.1"));
    }

    #[test]
    fn is_loopback_host_accepts_loopback() {
        assert!(is_loopback_host("127.0.0.1"));
        assert!(is_loopback_host("::1"));
        assert!(is_loopback_host("localhost"));
    }

    #[tokio::test]
    async fn health_endpoint_returns_ok() {
        let app = build_router(test_state());
        let req = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn ingest_returns_created_with_candidate() {
        let app = build_router(test_state());
        let body = serde_json::json!({
            "title": "Test decision",
            "description": "A test candidate",
            "level": "architecture",
            "source": "test"
        });
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/ingest")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&body).unwrap()))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}
