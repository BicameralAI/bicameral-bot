//! Configuration schema for `.bicameral/config.yaml`.

use bicameral_api::event_store::SubstrateKind;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Top-level Bicameral workspace configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicameralConfig {
    /// Schema version for forward compatibility.
    #[serde(default = "default_version")]
    pub version: u32,

    /// Workspace display name.
    #[serde(default)]
    pub workspace_name: Option<String>,

    /// Event-store substrate configuration.
    #[serde(default)]
    pub event_store: EventStoreConfig,

    /// Gateway configuration.
    #[serde(default)]
    pub gateway: GatewayConfig,

    /// Governance policy configuration.
    #[serde(default)]
    pub governance: GovernancePolicyConfig,

    /// Sources connected for ingest.
    #[serde(default)]
    pub sources: Vec<SourceConfig>,
}

impl Default for BicameralConfig {
    fn default() -> Self {
        Self {
            version: 1,
            workspace_name: None,
            event_store: EventStoreConfig::default(),
            gateway: GatewayConfig::default(),
            governance: GovernancePolicyConfig::default(),
            sources: Vec::new(),
        }
    }
}

impl BicameralConfig {
    /// Load configuration from a YAML file.
    pub fn load_from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Write configuration to a YAML file.
    pub fn save_to_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Event-store substrate configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStoreConfig {
    /// Which substrate kind to use.
    #[serde(default = "default_substrate")]
    pub substrate: SubstrateKind,

    /// Path for git-backed event store (relative to workspace root).
    #[serde(default)]
    pub git_path: Option<PathBuf>,

    /// Path for drive-folder-backed event store.
    /// Compatible with Google Drive sync; explicit freshness/offline states.
    #[serde(default)]
    pub drive_folder_path: Option<PathBuf>,
}

impl Default for EventStoreConfig {
    fn default() -> Self {
        Self {
            substrate: SubstrateKind::Git,
            git_path: None,
            drive_folder_path: None,
        }
    }
}

/// Gateway configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Host to bind the gateway to.
    #[serde(default = "default_host")]
    pub host: String,

    /// Port for the local gateway.
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

/// Governance policy settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernancePolicyConfig {
    /// Default policy: require review for all candidates.
    #[serde(default = "default_true")]
    pub require_review: bool,

    /// Automatically reject candidates below this extraction confidence.
    #[serde(default)]
    pub min_extraction_confidence: Option<String>,

    /// Roles/people who can approve candidates.
    #[serde(default)]
    pub approvers: Vec<String>,
}

impl Default for GovernancePolicyConfig {
    fn default() -> Self {
        Self {
            require_review: true,
            min_extraction_confidence: None,
            approvers: Vec::new(),
        }
    }
}

/// A connected source for ingest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    pub source_type: String,
    #[serde(default)]
    pub config: serde_json::Value,
}

fn default_version() -> u32 {
    1
}
fn default_substrate() -> SubstrateKind {
    SubstrateKind::Git
}
fn default_host() -> String {
    "127.0.0.1".to_string()
}
fn default_port() -> u16 {
    7525
}
fn default_true() -> bool {
    true
}
