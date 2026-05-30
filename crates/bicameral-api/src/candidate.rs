//! Decision candidate types — proposed decisions awaiting human review.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A proposed decision extracted from an operational source, awaiting review.
///
/// Candidates are never canonical. They represent probabilistic extraction from
/// sources like Jira tickets, Slack messages, meeting notes, or code comments.
/// Only governance acceptance through an event-store adapter can promote a
/// candidate into the decision ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionCandidate {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub level: DecisionLevel,
    pub source: String,
    pub extraction_confidence: ExtractionConfidence,
    pub extracted_at: DateTime<Utc>,
    pub extracted_by: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

impl DecisionCandidate {
    pub fn new(title: String, description: String, level: DecisionLevel, source: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            level,
            source,
            extraction_confidence: ExtractionConfidence::Low,
            extracted_at: Utc::now(),
            extracted_by: String::from("unknown"),
            tags: Vec::new(),
            metadata: serde_json::Value::Null,
        }
    }
}

/// Decision tier classification.
///
/// Separating levels prevents collapsing product intent, architectural
/// constraint, and implementation detail into a single undifferentiated pile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionLevel {
    /// L1: Product-level decisions (what and why)
    Product,
    /// L2: Architecture-level decisions (structural how)
    Architecture,
    /// L3: Implementation-level decisions (tactical how)
    Implementation,
}

/// Extraction confidence — how certain the system is that this is actually a decision.
///
/// This is intentionally separate from binding confidence and compliance confidence.
/// Collapsing these into one score creates cognitive debt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtractionConfidence {
    Low,
    Medium,
    High,
}
