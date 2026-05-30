use bicameral_config::WorkspaceDiscovery;
use std::path::Path;

pub async fn list(workspace: Option<&Path>) -> anyhow::Result<()> {
    let start = workspace
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let (paths, _config) = WorkspaceDiscovery::discover(&start)?;

    println!("Pending reviews in: {}", paths.workspace_root.display());

    let events_dir = &paths.events_dir;
    if !events_dir.exists() {
        println!("  (none)");
        return Ok(());
    }

    let mut count = 0;
    for entry in std::fs::read_dir(events_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let content = std::fs::read_to_string(&path)?;
            if let Ok(candidate) =
                serde_json::from_str::<bicameral_api::candidate::DecisionCandidate>(&content)
            {
                println!(
                    "  {} — {} [{:?}]",
                    candidate.id, candidate.title, candidate.level
                );
                count += 1;
            }
        }
    }

    if count == 0 {
        println!("  (none)");
    }

    Ok(())
}

pub async fn submit(id: &str, workspace: Option<&Path>) -> anyhow::Result<()> {
    let start = workspace
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let (_paths, _config) = WorkspaceDiscovery::discover(&start)?;

    println!("Submitted candidate {} for review", id);
    println!("  Status: pending review (governance evaluation required)");

    Ok(())
}
