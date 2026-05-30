//! Path resolution for Bicameral configuration and data directories.

use std::path::{Path, PathBuf};

/// Resolved paths for a Bicameral workspace.
#[derive(Debug, Clone)]
pub struct BicameralPaths {
    /// Root of the workspace (where `.bicameral/` lives).
    pub workspace_root: PathBuf,
    /// The `.bicameral/` directory itself.
    pub dot_bicameral: PathBuf,
    /// Config file path: `.bicameral/config.yaml`
    pub config_file: PathBuf,
    /// Decisions directory: `.bicameral/decisions/`
    pub decisions_dir: PathBuf,
    /// Candidates inbox: `.bicameral/candidates-inbox/`
    /// Non-canonical pending candidates awaiting governance review.
    /// This is NOT the canonical event store — only `EventStoreAdapter::append()`
    /// materializes accepted governance events.
    pub candidates_inbox_dir: PathBuf,
    /// Audit receipts directory: `.bicameral/audit/`
    pub audit_dir: PathBuf,
    /// Mod manifests directory: `.bicameral/mods/`
    pub mods_dir: PathBuf,
    /// Factory attestations directory: `.bicameral/factory-attestations/`
    pub attestations_dir: PathBuf,
}

impl BicameralPaths {
    /// Construct paths from a workspace root.
    pub fn from_workspace_root(root: impl AsRef<Path>) -> Self {
        let workspace_root = root.as_ref().to_path_buf();
        let dot_bicameral = workspace_root.join(".bicameral");
        Self {
            config_file: dot_bicameral.join("config.yaml"),
            decisions_dir: dot_bicameral.join("decisions"),
            candidates_inbox_dir: dot_bicameral.join("candidates-inbox"),
            audit_dir: dot_bicameral.join("audit"),
            mods_dir: dot_bicameral.join("mods"),
            attestations_dir: dot_bicameral.join("factory-attestations"),
            dot_bicameral,
            workspace_root,
        }
    }

    /// User-level config directory (`~/.config/bicameral/`).
    pub fn user_config_dir() -> Option<PathBuf> {
        dirs_path().map(|d| d.join("bicameral"))
    }

    /// Ensure all required directories exist.
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.dot_bicameral)?;
        std::fs::create_dir_all(&self.decisions_dir)?;
        std::fs::create_dir_all(&self.candidates_inbox_dir)?;
        std::fs::create_dir_all(&self.audit_dir)?;
        std::fs::create_dir_all(&self.mods_dir)?;
        std::fs::create_dir_all(&self.attestations_dir)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inbox_dir_is_not_named_events() {
        let paths = BicameralPaths::from_workspace_root("/tmp/test-ws");
        let inbox_name = paths
            .candidates_inbox_dir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(
            inbox_name, "candidates-inbox",
            "Inbox directory must be named 'candidates-inbox', not 'events'"
        );
        assert!(
            !inbox_name.contains("event"),
            "Inbox directory name must not contain 'event' to avoid confusion with canonical event store"
        );
    }
}

fn dirs_path() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join(".config"))
    }
    #[cfg(target_os = "linux")]
    {
        std::env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var("HOME")
                    .ok()
                    .map(|h| PathBuf::from(h).join(".config"))
            })
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA").ok().map(PathBuf::from)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        None
    }
}
