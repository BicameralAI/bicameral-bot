//! Evidence types — linking decisions to sources and code.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Evidence linking a decision candidate back to its operational source.
///
/// Source evidence answers "where did this claim come from?" — a Jira ticket,
/// Slack message, meeting transcript, ADR, or code comment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceEvidence {
    pub id: Uuid,
    pub candidate_id: Uuid,
    pub source_type: String,
    pub source_url: Option<String>,
    pub excerpt: String,
    pub captured_at: DateTime<Utc>,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

impl SourceEvidence {
    pub fn new(candidate_id: Uuid, source_type: String, excerpt: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            candidate_id,
            source_type,
            source_url: None,
            excerpt,
            captured_at: Utc::now(),
            metadata: serde_json::Value::Null,
        }
    }
}

/// Evidence binding a decision to specific code symbols, files, or paths.
///
/// Binding evidence answers "what code does this decision constrain?" — a file
/// path, function symbol, dependency, deploy surface, or code region.
///
/// Binding confidence is intentionally separate from extraction confidence and
/// compliance confidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingEvidence {
    pub id: Uuid,
    pub candidate_id: Uuid,
    pub binding_type: BindingType,
    pub target: String,
    pub confidence: BindingConfidence,
    pub explanation: Option<String>,
    pub bound_at: DateTime<Utc>,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

impl BindingEvidence {
    pub fn new(
        candidate_id: Uuid,
        binding_type: BindingType,
        target: String,
        confidence: BindingConfidence,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            candidate_id,
            binding_type,
            target,
            confidence,
            explanation: None,
            bound_at: Utc::now(),
            metadata: serde_json::Value::Null,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingType {
    FilePath,
    Symbol,
    Dependency,
    DeploySurface,
    CodeRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingConfidence {
    Low,
    Medium,
    High,
}
