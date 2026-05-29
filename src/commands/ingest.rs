use bicameral_api::candidate::{DecisionCandidate, DecisionLevel};
use bicameral_audit::store::AuditStore;
use bicameral_audit::AuditAction;
use bicameral_config::WorkspaceDiscovery;
use std::path::Path;

pub async fn run(file: &Path, workspace: Option<&Path>) -> anyhow::Result<()> {
    let start = workspace
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    let (paths, _config) = WorkspaceDiscovery::discover(&start)?;

    let content = std::fs::read_to_string(file)?;
    let input: serde_json::Value = serde_json::from_str(&content)?;

    let title = input["title"]
        .as_str()
        .unwrap_or("Untitled")
        .to_string();
    let description = input["description"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let level = match input["level"].as_str() {
        Some("product") => DecisionLevel::Product,
        Some("architecture") => DecisionLevel::Architecture,
        Some("implementation") => DecisionLevel::Implementation,
        _ => DecisionLevel::Implementation,
    };
    let source = input["source"]
        .as_str()
        .unwrap_or("cli-ingest")
        .to_string();

    let candidate = DecisionCandidate::new(title.clone(), description, level, source);

    let mut audit = AuditStore::open(&paths.audit_dir)?;
    audit.record(
        AuditAction::CandidateIngested,
        "cli".to_string(),
        Some(candidate.id),
        format!("Ingested from file: {}", file.display()),
    )?;

    // Write candidate to events dir as pending
    let candidate_file = paths.events_dir.join(format!("{}.json", candidate.id));
    let candidate_json = serde_json::to_string_pretty(&candidate)?;
    std::fs::write(&candidate_file, candidate_json)?;

    println!("Ingested candidate: {} ({})", title, candidate.id);
    println!("  Level: {:?}", level);
    println!("  Written to: {}", candidate_file.display());

    Ok(())
}
