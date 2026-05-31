/** Dashboard API contract types — mirrors ADR-0006 data contracts. */

// ── Source provenance ────────────────────────────────────────────

export type SourceRef = {
  uri: string;
  source_type: string;
};

export type SourceSnapshotRef = {
  snapshot_addr: string;
  snapshot_ref: string;
  captured_at: string;
};

export type SourceEvidenceItem = {
  id: string;
  source_uri: string;
  snapshot_addr: string;
  snapshot_ref: string;
  pointer_type: string;
  pointer_value: string;
  excerpt: string;
  captured_at: string;
};

// ── Ingestion Gate ───────────────────────────────────────────────

export type DecisionCandidatePreview = {
  id?: string;
  summary: string;
  feature_hint?: string;
  evidence_refs: string[];
  extraction_confidence?: number;
  conflict_hint?: boolean;
  review_state?: string;
};

export type IngestionGateItem = {
  source: SourceRef;
  snapshot: SourceSnapshotRef;
  source_title: string;
  source_freshness: "fresh" | "stale" | "offline" | "unknown";
  evidence: SourceEvidenceItem[];
  candidates: DecisionCandidatePreview[];
  tracked: boolean;
};

// ── Ledger View ──────────────────────────────────────────────────

export type SignoffState =
  | "proposed"
  | "approved"
  | "rejected"
  | "collision_pending"
  | "superseded";

export type ComplianceState =
  | "reflected"
  | "partial"
  | "drifted"
  | "pending"
  | "ungrounded";

export type CandidateCommandKind =
  | "accept_candidate"
  | "reject_candidate"
  | "edit_candidate"
  | "request_context"
  | "assign_reviewer";

export type DecisionCommandKind =
  | "approve_signoff"
  | "reject_signoff"
  | "request_context"
  | "assign_reviewer"
  | "bind_to_code"
  | "resolve_compliance"
  | "supersede_decision"
  | "mark_different_scopes";

export type CodeRegion = {
  file: string;
  start_line: number;
  end_line: number;
  content_hash?: string;
};

export type LedgerCandidate = {
  kind: "candidate";
  id: string;
  summary: string;
  feature_hint?: string;
  sources: SourceEvidenceItem[];
  review_state: string;
  allowed_commands: CandidateCommandKind[];
};

export type LedgerDecision = {
  kind: "decision";
  id: string;
  summary: string;
  feature: string;
  parent_id?: string;
  signoff: SignoffState;
  compliance: ComplianceState;
  sources: SourceEvidenceItem[];
  regions?: CodeRegion[];
  conflicts_with?: string[];
  discovered?: boolean;
  allowed_commands: DecisionCommandKind[];
};

export type LedgerReviewItem = LedgerDecision | LedgerCandidate;

// ── Review command (emitted by UI) ───────────────────────────────

export type ReviewCommandPayload = {
  target_id: string;
  command:
    | CandidateCommandKind
    | DecisionCommandKind
    | "ingest_batch"
    | "reject_candidate"
    | "demote_decision";
  reason?: string;
};
