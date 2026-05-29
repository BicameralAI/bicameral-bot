use bicameral_config::WorkspaceDiscovery;
use bicameral_runtime::daemon::{DaemonConfig, run as run_daemon};
use std::path::Path;

pub async fn start(workspace: Option<&Path>) -> anyhow::Result<()> {
    let start = workspace
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let (paths, config) = WorkspaceDiscovery::discover(&start)?;

    println!(
        "Starting Bicameral daemon/gateway at {}:{}",
        config.gateway.host, config.gateway.port
    );

    let daemon_config = DaemonConfig { paths, config };
    let exit = run_daemon(daemon_config).await?;

    println!("Daemon exited: {:?}", exit);
    Ok(())
}
