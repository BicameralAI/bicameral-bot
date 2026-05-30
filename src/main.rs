//! Bicameral CLI — local governance runtime for software teams.
//!
//! Commands: init, ingest, preflight, review, mod validate, mod run,
//! gateway start, service status, doctor.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;

#[derive(Parser)]
#[command(name = "bicameral")]
#[command(about = "Local-first governance runtime for software teams")]
#[command(version)]
struct Cli {
    /// Workspace root (defaults to current directory discovery)
    #[arg(long, global = true)]
    workspace: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Bicameral workspace
    Init {
        /// Path to initialize (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Ingest a source into the governance pipeline
    Ingest {
        /// Path to a JSON file containing the source evidence
        file: PathBuf,
    },
    /// Run preflight checks against current decisions
    Preflight {
        /// Path or ref to check
        #[arg(default_value = ".")]
        target: String,
    },
    /// Submit or manage review commands
    Review {
        #[command(subcommand)]
        action: ReviewAction,
    },
    /// Mod manifest operations
    Mod {
        #[command(subcommand)]
        action: ModAction,
    },
    /// Gateway operations
    Gateway {
        #[command(subcommand)]
        action: GatewayAction,
    },
    /// Check service status
    Service {
        #[command(subcommand)]
        action: ServiceAction,
    },
    /// Run diagnostic checks
    Doctor,
}

#[derive(Subcommand)]
enum ReviewAction {
    /// List pending reviews
    List,
    /// Submit a candidate for review
    Submit {
        /// Candidate ID
        id: String,
    },
}

#[derive(Subcommand)]
enum ModAction {
    /// Validate a mod manifest
    Validate {
        /// Path to mod manifest YAML
        manifest: PathBuf,
    },
    /// Run a mod against a fixture
    Run {
        /// Path to mod manifest YAML
        manifest: PathBuf,
        /// Path to fixture JSON file
        #[arg(long)]
        fixture: PathBuf,
    },
}

#[derive(Subcommand)]
enum GatewayAction {
    /// Start the local gateway
    Start,
}

#[derive(Subcommand)]
enum ServiceAction {
    /// Show daemon service status
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("bicameral=info".parse().unwrap()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => commands::init::run(&path).await,
        Commands::Ingest { file } => commands::ingest::run(&file, cli.workspace.as_deref()).await,
        Commands::Preflight { target } => {
            commands::preflight::run(&target, cli.workspace.as_deref()).await
        }
        Commands::Review { action } => match action {
            ReviewAction::List => commands::review::list(cli.workspace.as_deref()).await,
            ReviewAction::Submit { id } => {
                commands::review::submit(&id, cli.workspace.as_deref()).await
            }
        },
        Commands::Mod { action } => match action {
            ModAction::Validate { manifest } => commands::mods::validate(&manifest).await,
            ModAction::Run { manifest, fixture } => commands::mods::run(&manifest, &fixture).await,
        },
        Commands::Gateway { action } => match action {
            GatewayAction::Start => commands::gateway::start(cli.workspace.as_deref()).await,
        },
        Commands::Service { action } => match action {
            ServiceAction::Status => commands::service::status().await,
        },
        Commands::Doctor => commands::doctor::run(cli.workspace.as_deref()).await,
    }
}
