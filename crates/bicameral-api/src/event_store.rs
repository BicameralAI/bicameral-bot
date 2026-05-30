//! Event-store adapter trait and types.
//!
//! The event store is the substrate-neutral materialization boundary.
//! Accepted governance results are materialized here — and only here.
//! No ingest handler, mod, or gateway route may write canonical decisions
//! directly; they must go through governance and then the event-store adapter.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Substrate-neutral event store adapter trait.
///
/// Implementations may back onto git (JSONL event log in a repo), a local
/// folder compatible with Google Drive sync, or future cloud substrates.
/// The interface must not hardcode git assumptions.
pub trait EventStoreAdapter: Send + Sync {
    /// Append an accepted event to the store.
    fn append(&self, entry: EventStoreEntry) -> Result<(), EventStoreError>;

    /// Replay all events in order.
    fn replay(&self) -> Result<Vec<EventStoreEntry>, EventStoreError>;

    /// Get the current substrate kind.
    fn substrate_kind(&self) -> SubstrateKind;

    /// Check connectivity/freshness of the substrate.
    fn health_check(&self) -> Result<SubstrateHealth, EventStoreError>;
}

/// A single entry in the event store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStoreEntry {
    pub id: Uuid,
    pub event_type: EventType,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub actor: String,
    pub governance_result_id: Option<Uuid>,
}

impl EventStoreEntry {
    pub fn new(event_type: EventType, payload: serde_json::Value, actor: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            payload,
            timestamp: Utc::now(),
            actor,
            governance_result_id: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    DecisionAccepted,
    DecisionRejected,
    EvidenceAttached,
    ReviewCompleted,
    PolicyUpdated,
}

/// Which substrate backs the event store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubstrateKind {
    /// Git-backed JSONL event log in a repository.
    Git,
    /// Local folder compatible with Google Drive sync.
    DriveFolder,
    /// In-memory (testing only).
    Memory,
}

/// Health/freshness status of the event-store substrate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstrateHealth {
    pub kind: SubstrateKind,
    pub reachable: bool,
    pub last_synced: Option<DateTime<Utc>>,
    pub entry_count: usize,
    #[serde(default)]
    pub warnings: Vec<String>,
}

/// Errors from event-store operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum EventStoreError {
    #[error("substrate not reachable: {reason}")]
    Unreachable { reason: String },
    #[error("serialization error: {reason}")]
    Serialization { reason: String },
    #[error("io error: {reason}")]
    Io { reason: String },
    #[error("conflict: {reason}")]
    Conflict { reason: String },
}
