use bicameral_config::WorkspaceDiscovery;
use bicameral_runtime::service;
use std::path::Path;

pub async fn run(workspace: Option<&Path>) -> anyhow::Result<()> {
    println!("Bicameral Doctor");
    println!("================");
    println!();

    // Check workspace
    let start = workspace
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    match WorkspaceDiscovery::discover(&start) {
        Ok((paths, config)) => {
            println!("[ok] Workspace found: {}", paths.workspace_root.display());
            println!("     Config: {}", paths.config_file.display());
            println!(
                "     Event store substrate: {:?}",
                config.event_store.substrate
            );
        }
        Err(_) => {
            println!("[!!] No workspace found. Run `bicameral init` to create one.");
        }
    }

    // Check service status
    let status = service::status();
    match status {
        service::ServiceStatus::Running => println!("[ok] Daemon service: running"),
        service::ServiceStatus::Stopped => println!("[--] Daemon service: not running"),
        service::ServiceStatus::Unknown => println!("[??] Daemon service: unknown"),
    }

    // Check rust version
    println!();
    println!("Environment:");
    println!("  bicameral version: {}", env!("CARGO_PKG_VERSION"));

    Ok(())
}
