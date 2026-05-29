//! Drive-folder-backed event-store adapter.
//!
//! Compatible with Google Drive sync: events are stored as individual JSON
//! files in a configured local folder. The folder can be synced via Drive
//! Desktop or similar tools. Explicit freshness/offline state tracking is
//! included so the runtime knows when data may be stale.

use bicameral_api::event_store::{
    EventStoreAdapter, EventStoreEntry, EventStoreError, SubstrateHealth, SubstrateKind,
};
use chrono::Utc;
use std::path::{Path, PathBuf};

/// Drive-folder-backed event store adapter.
///
/// Each event is stored as a separate JSON file named by UUID. This avoids
/// JSONL merge conflicts when multiple team members sync through Drive.
pub struct DriveFolderAdapter {
    folder_path: PathBuf,
    /// Whether the folder appears to be synced (heuristic: .sync marker exists).
    synced: bool,
}

impl DriveFolderAdapter {
    pub fn new(folder_path: &Path) -> Self {
        let synced = folder_path.join(".bicameral-sync-marker").exists();
        Self {
            folder_path: folder_path.to_path_buf(),
            synced,
        }
    }

    /// Write a sync marker to indicate this folder is actively synced.
    pub fn mark_synced(&self) -> Result<(), EventStoreError> {
        let marker = self.folder_path.join(".bicameral-sync-marker");
        let content = Utc::now().to_rfc3339().to_string();
        std::fs::write(marker, content).map_err(|e| EventStoreError::Io {
            reason: e.to_string(),
        })?;
        Ok(())
    }
}

impl EventStoreAdapter for DriveFolderAdapter {
    fn append(&self, entry: EventStoreEntry) -> Result<(), EventStoreError> {
        std::fs::create_dir_all(&self.folder_path).map_err(|e| EventStoreError::Io {
            reason: e.to_string(),
        })?;

        let filename = format!("{}.json", entry.id);
        let path = self.folder_path.join(filename);
        let content =
            serde_json::to_string_pretty(&entry).map_err(|e| EventStoreError::Serialization {
                reason: e.to_string(),
            })?;

        std::fs::write(path, content).map_err(|e| EventStoreError::Io {
            reason: e.to_string(),
        })?;

        Ok(())
    }

    fn replay(&self) -> Result<Vec<EventStoreEntry>, EventStoreError> {
        if !self.folder_path.exists() {
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();

        let read_dir =
            std::fs::read_dir(&self.folder_path).map_err(|e| EventStoreError::Io {
                reason: e.to_string(),
            })?;

        for dir_entry in read_dir {
            let dir_entry = dir_entry.map_err(|e| EventStoreError::Io {
                reason: e.to_string(),
            })?;
            let path = dir_entry.path();

            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }

            let content = std::fs::read_to_string(&path).map_err(|e| EventStoreError::Io {
                reason: e.to_string(),
            })?;

            match serde_json::from_str::<EventStoreEntry>(&content) {
                Ok(entry) => entries.push(entry),
                Err(_) => continue, // Skip non-event JSON files
            }
        }

        entries.sort_by_key(|a| a.timestamp);
        Ok(entries)
    }

    fn substrate_kind(&self) -> SubstrateKind {
        SubstrateKind::DriveFolder
    }

    fn health_check(&self) -> Result<SubstrateHealth, EventStoreError> {
        let reachable = self.folder_path.exists();
        let entry_count = if reachable {
            std::fs::read_dir(&self.folder_path)
                .map(|rd| {
                    rd.filter_map(|e| e.ok())
                        .filter(|e| {
                            e.path().extension().and_then(|ext| ext.to_str()) == Some("json")
                        })
                        .count()
                })
                .unwrap_or(0)
        } else {
            0
        };

        let mut warnings = Vec::new();
        if !self.synced {
            warnings.push("Drive sync marker not found — data may be stale".to_string());
        }

        Ok(SubstrateHealth {
            kind: SubstrateKind::DriveFolder,
            reachable,
            last_synced: None,
            entry_count,
            warnings,
        })
    }
}
