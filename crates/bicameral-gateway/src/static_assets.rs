//! Static asset serving for the compiled Svelte dashboard.
//!
//! When `bicameral dashboard` runs, the gateway serves pre-compiled static
//! assets from a directory on disk. This means users do not need Node.js or
//! Vite installed — only the `bicameral` binary.
//!
//! For development, contributors still use `npm run dev` inside `dashboard/`
//! with Vite's dev server proxying API calls to the gateway.

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Router;
use std::path::{Path, PathBuf};

/// Add static file serving as a fallback to the router.
///
/// API routes take priority (they are already merged before this call).
/// Any request that doesn't match an API route is served from the dashboard
/// directory. If the requested path doesn't exist, we serve `index.html`
/// so that client-side routing works.
pub fn add_static_serving(router: Router, dashboard_dir: &Path) -> Router {
    let dir = dashboard_dir.to_path_buf();
    router.fallback(move |req: Request<Body>| {
        let dir = dir.clone();
        async move { serve_static(dir, req).await }
    })
}

async fn serve_static(dir: PathBuf, req: Request<Body>) -> Response {
    let path = req.uri().path().trim_start_matches('/');
    let file_path = dir.join(if path.is_empty() { "index.html" } else { path });

    // Try the exact path first, then fall back to index.html for SPA routing
    let target = if file_path.is_file() {
        file_path
    } else {
        dir.join("index.html")
    };

    match tokio::fs::read(&target).await {
        Ok(contents) => {
            let mime = mime_from_path(&target);
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime)
                .body(Body::from(contents))
                .unwrap()
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

fn mime_from_path(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        _ => "application/octet-stream",
    }
}
