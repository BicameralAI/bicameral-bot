//! Source provenance types — Source, SourceSnapshot, and typed evidence pointers.
//!
//! These types implement the provenance model from ADR-0006 and PR #14:
//! - `Source` is the mutable external object linkage identified by URI.
//! - `SourceSnapshot` is the immutable captured view, content-addressed.
//! - `SourceEvidenceRef` has one typed pointer into one snapshot and a mandatory excerpt.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Mutable external object linkage identified by URI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub uri: String,
    pub source_type: String,
}

/// Immutable captured view of a Source, content-addressed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSnapshot {
    pub snapshot_addr: String,
    pub snapshot_ref: String,
    pub captured_at: DateTime<Utc>,
}

/// Typed pointer into a snapshot with a mandatory canonical excerpt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceEvidenceRef {
    pub id: String,
    pub source_uri: String,
    pub snapshot_addr: String,
    pub snapshot_ref: String,
    pub pointer_type: String,
    pub pointer_value: String,
    pub excerpt: String,
    pub captured_at: DateTime<Utc>,
}

/// Source freshness state — enforced at command gating, not only display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceFreshness {
    Fresh,
    Stale,
    Offline,
    Unknown,
}
