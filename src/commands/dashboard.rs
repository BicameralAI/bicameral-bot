use bicameral_config::WorkspaceDiscovery;
use bicameral_runtime::daemon::{run_with_dashboard, DaemonConfig};
use std::path::{Path, PathBuf};

/// Resolve the dashboard asset directory.
///
/// Checks, in order:
/// 1. BICAMERAL_DASHBOARD_DIR env var
/// 2. `dashboard/dist` relative to the workspace root
/// 3. `dashboard/dist` relative to the binary location
fn find_dashboard_dir(workspace_root: &Path) -> Option<PathBuf> {
    // Env override
    if let Ok(dir) = std::env::var("BICAMERAL_DASHBOARD_DIR") {
        let p = PathBuf::from(dir);
        if p.join("index.html").is_file() {
            return Some(p);
        }
    }

    // Relative to workspace
    let ws = workspace_root.join("dashboard/dist");
    if ws.join("index.html").is_file() {
        return Some(ws);
    }

    // Relative to binary
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            let rel = exe_dir.join("dashboard");
            if rel.join("index.html").is_file() {
                return Some(rel);
            }
        }
    }

    None
}

pub async fn start(workspace: Option<&Path>) -> anyhow::Result<()> {
    let start = workspace
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let (paths, config) = WorkspaceDiscovery::discover(&start)?;

    let dashboard_dir = find_dashboard_dir(&start);

    if let Some(ref dir) = dashboard_dir {
        tracing::info!(path = %dir.display(), "Serving dashboard from compiled assets");
    } else {
        tracing::warn!(
            "No compiled dashboard assets found. \
             Run `npm run build` in dashboard/ or set BICAMERAL_DASHBOARD_DIR. \
             API endpoints are still available."
        );
    }

    let url = format!("http://{}:{}", config.gateway.host, config.gateway.port);
    println!("Starting Bicameral dashboard at {url}");
    if dashboard_dir.is_some() {
        println!("Dashboard UI: {url}");
    } else {
        println!("Dashboard UI: not available (no compiled assets found)");
        println!("API only: {url}/api/v1/dashboard/ingestion-gate");
    }

    let daemon_config = DaemonConfig { paths, config };
    let exit = run_with_dashboard(daemon_config, dashboard_dir).await?;

    println!("Daemon exited: {exit:?}");
    Ok(())
}
