//! Dashboard API contract types — typed payloads for the two-page dashboard.
//!
//! These types match the ADR-0006 data contracts and the Svelte dashboard's
//! TypeScript definitions. The gateway serves these as JSON from
//! `/api/v1/dashboard/*` endpoints.

use crate::source::{Source, SourceEvidenceRef, SourceFreshness, SourceSnapshot};
use serde::{Deserialize, Serialize};

// ── Ingestion Gate ───────────────────────────────────────────────

/// A single row in the Ingestion Gate page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionGateItem {
    pub source: Source,
    pub snapshot: SourceSnapshot,
    pub source_title: String,
    pub source_freshness: SourceFreshness,
    pub evidence: Vec<SourceEvidenceRef>,
    pub candidates: Vec<CandidatePreview>,
    pub tracked: bool,
}

/// Lightweight candidate preview shown in the Ingestion Gate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidatePreview {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_hint: Option<String>,
    pub evidence_refs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraction_confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_hint: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_state: Option<String>,
}

// ── Ledger View ──────────────────────────────────────────────────

/// An item in the Ledger View — either a promoted Decision or a queued candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LedgerReviewItem {
    Decision(LedgerDecision),
    Candidate(LedgerCandidate),
}

/// A promoted Decision in the Ledger View.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerDecision {
    pub id: String,
    pub summary: String,
    pub feature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub signoff: SignoffState,
    pub compliance: ComplianceState,
    pub sources: Vec<SourceEvidenceRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<CodeRegion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts_with: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovered: Option<bool>,
    pub allowed_commands: Vec<DecisionCommandKind>,
}

/// A queued candidate in the Ledger View.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerCandidate {
    pub id: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_hint: Option<String>,
    pub sources: Vec<SourceEvidenceRef>,
    pub review_state: String,
    pub allowed_commands: Vec<CandidateCommandKind>,
}

/// Signoff lifecycle on a Decision (ownership authority axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignoffState {
    Proposed,
    Approved,
    Rejected,
    CollisionPending,
    Superseded,
}

/// Compliance/grounding state (technical reality axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceState {
    Reflected,
    Partial,
    Drifted,
    Pending,
    Ungrounded,
}

/// Code region binding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeRegion {
    pub file: String,
    pub start_line: u32,
    pub end_line: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
}

/// Commands available for Decisions in the Ledger View.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionCommandKind {
    ApproveSignoff,
    RejectSignoff,
    RequestContext,
    AssignReviewer,
    BindToCode,
    ResolveCompliance,
    SupersedeDecision,
    MarkDifferentScopes,
}

/// Commands available for Candidates in the Ledger View.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateCommandKind {
    AcceptCandidate,
    RejectCandidate,
    EditCandidate,
    RequestContext,
    AssignReviewer,
}

// ── Review command payload (emitted by dashboard) ────────────────

/// A review command submitted by the dashboard UI.
///
/// The dashboard emits substrate-neutral commands — it does not write
/// event-store internals directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardReviewCommand {
    pub target_id: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
