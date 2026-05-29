//! Git-backed event-store adapter.
//!
//! Stores events as a JSONL file in a git repository. Each append is a new
//! line in the events file. Sync happens through normal git push/pull.

use bicameral_api::event_store::{
    EventStoreAdapter, EventStoreEntry, EventStoreError, SubstrateHealth, SubstrateKind,
};
use std::path::{Path, PathBuf};

/// Git-backed event store adapter.
///
/// Events are stored as JSONL in `<path>/events.jsonl`. The caller is
/// responsible for git add/commit/push to share with other team members.
pub struct GitAdapter {
    events_file: PathBuf,
}

impl GitAdapter {
    pub fn new(repo_path: &Path) -> Self {
        Self {
            events_file: repo_path.join("events.jsonl"),
        }
    }
}

impl EventStoreAdapter for GitAdapter {
    fn append(&self, entry: EventStoreEntry) -> Result<(), EventStoreError> {
        let line = serde_json::to_string(&entry).map_err(|e| EventStoreError::Serialization {
            reason: e.to_string(),
        })?;

        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.events_file)
            .map_err(|e| EventStoreError::Io {
                reason: e.to_string(),
            })?;

        writeln!(file, "{}", line).map_err(|e| EventStoreError::Io {
            reason: e.to_string(),
        })?;

        Ok(())
    }

    fn replay(&self) -> Result<Vec<EventStoreEntry>, EventStoreError> {
        if !self.events_file.exists() {
            return Ok(Vec::new());
        }

        let content =
            std::fs::read_to_string(&self.events_file).map_err(|e| EventStoreError::Io {
                reason: e.to_string(),
            })?;

        let mut entries = Vec::new();
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let entry: EventStoreEntry =
                serde_json::from_str(line).map_err(|e| EventStoreError::Serialization {
                    reason: e.to_string(),
                })?;
            entries.push(entry);
        }

        Ok(entries)
    }

    fn substrate_kind(&self) -> SubstrateKind {
        SubstrateKind::Git
    }

    fn health_check(&self) -> Result<SubstrateHealth, EventStoreError> {
        let entry_count = if self.events_file.exists() {
            std::fs::read_to_string(&self.events_file)
                .map(|c| c.lines().filter(|l| !l.trim().is_empty()).count())
                .unwrap_or(0)
        } else {
            0
        };

        Ok(SubstrateHealth {
            kind: SubstrateKind::Git,
            reachable: true,
            last_synced: None,
            entry_count,
            warnings: Vec::new(),
        })
    }
}
