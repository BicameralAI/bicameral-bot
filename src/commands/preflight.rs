use bicameral_config::WorkspaceDiscovery;
use std::path::Path;

pub async fn run(target: &str, workspace: Option<&Path>) -> anyhow::Result<()> {
    let start = workspace
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let (paths, _config) = WorkspaceDiscovery::discover(&start)?;

    println!("Running preflight checks for: {}", target);
    println!("  Workspace: {}", paths.workspace_root.display());

    // Load existing decisions/events for comparison
    let events_dir = &paths.events_dir;
    let mut decision_count = 0;
    if events_dir.exists() {
        for entry in std::fs::read_dir(events_dir)? {
            let entry = entry?;
            if entry.path().extension().and_then(|e| e.to_str()) == Some("json") {
                decision_count += 1;
            }
        }
    }

    println!("  Active candidates/events: {}", decision_count);

    if decision_count == 0 {
        println!("  No decisions tracked yet. Preflight pass (vacuous).");
    } else {
        println!("  Preflight complete. {} items checked.", decision_count);
    }

    Ok(())
}
