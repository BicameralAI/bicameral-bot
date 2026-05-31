//! Gateway route definitions.

use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use bicameral_api::candidate::DecisionCandidate;
use bicameral_api::dashboard::{
    CandidateCommandKind, CandidatePreview, DashboardReviewCommand, IngestionGateItem,
    LedgerCandidate, LedgerReviewItem,
};
use bicameral_api::review::{ReviewCommand, ReviewState, ReviewStatus};
use bicameral_api::source::{Source, SourceFreshness, SourceSnapshot};
use bicameral_audit::receipt::AuditAction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Health routes ────────────────────────────────────────────────

pub fn health_routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

// ── Ingest routes ────────────────────────────────────────────────

pub fn ingest_routes() -> Router<AppState> {
    Router::new().route("/api/v1/ingest", post(ingest_candidate))
}

#[derive(Deserialize)]
struct IngestRequest {
    title: String,
    description: String,
    level: bicameral_api::candidate::DecisionLevel,
    source: String,
    #[serde(default)]
    tags: Vec<String>,
}

async fn ingest_candidate(
    State(state): State<AppState>,
    Json(req): Json<IngestRequest>,
) -> Result<(StatusCode, Json<DecisionCandidate>), StatusCode> {
    let mut candidate = DecisionCandidate::new(req.title, req.description, req.level, req.source);
    candidate.tags = req.tags;

    let id = candidate.id;

    // Record audit receipt
    {
        let mut audit = state.audit.write().await;
        let _ = audit.record(
            AuditAction::CandidateIngested,
            "gateway".to_string(),
            Some(id),
            format!("Ingested candidate: {}", candidate.title),
        );
    }

    // Store candidate
    state.candidates.write().await.insert(id, candidate.clone());

    // Create pending review state
    let review = ReviewState::new(id);
    state.reviews.write().await.insert(id, review);

    tracing::info!(candidate_id = %id, "Candidate ingested via gateway");

    Ok((StatusCode::CREATED, Json(candidate)))
}

// ── Review routes ────────────────────────────────────────────────

pub fn review_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/review", post(submit_review_command))
        .route("/api/v1/review/:id", get(get_review_state))
}

async fn submit_review_command(
    State(state): State<AppState>,
    Json(cmd): Json<ReviewCommand>,
) -> Result<(StatusCode, Json<ReviewState>), StatusCode> {
    let candidate_id = cmd.candidate_id;

    let mut reviews = state.reviews.write().await;
    let review = reviews
        .get_mut(&candidate_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    // Update review state based on command kind
    use bicameral_api::review::ReviewCommandKind;
    match cmd.kind {
        ReviewCommandKind::Submit => {
            review.status = ReviewStatus::InReview;
            review.submitted_at = Some(cmd.issued_at);
        }
        ReviewCommandKind::Approve => {
            review.status = ReviewStatus::Approved;
            review.resolved_at = Some(cmd.issued_at);
            review.resolution = Some(ReviewCommandKind::Approve);
        }
        ReviewCommandKind::Reject => {
            review.status = ReviewStatus::Rejected;
            review.resolved_at = Some(cmd.issued_at);
            review.resolution = Some(ReviewCommandKind::Reject);
        }
        ReviewCommandKind::RequestEvidence => {
            review.status = ReviewStatus::NeedsEvidence;
        }
        ReviewCommandKind::Escalate => {
            review.status = ReviewStatus::Escalated;
        }
    }

    review.history.push(cmd);

    // Record audit receipt
    {
        let mut audit = state.audit.write().await;
        let _ = audit.record(
            AuditAction::ReviewCommandIssued,
            "gateway".to_string(),
            Some(candidate_id),
            format!("Review command for candidate {}", candidate_id),
        );
    }

    Ok((StatusCode::OK, Json(review.clone())))
}

async fn get_review_state(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<ReviewState>, StatusCode> {
    let reviews = state.reviews.read().await;
    reviews
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// ── Status routes ────────────────────────────────────────────────

pub fn status_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/candidates", get(list_candidates))
        .route("/api/v1/status", get(service_status))
}

async fn list_candidates(State(state): State<AppState>) -> Json<Vec<DecisionCandidate>> {
    let candidates = state.candidates.read().await;
    Json(candidates.values().cloned().collect())
}

#[derive(Serialize)]
struct ServiceStatusResponse {
    status: String,
    candidate_count: usize,
    review_count: usize,
}

async fn service_status(State(state): State<AppState>) -> Json<ServiceStatusResponse> {
    let candidate_count = state.candidates.read().await.len();
    let review_count = state.reviews.read().await.len();

    Json(ServiceStatusResponse {
        status: "running".to_string(),
        candidate_count,
        review_count,
    })
}

// ── Dashboard API routes ─────────────────────────────────────────

pub fn dashboard_api_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/dashboard/ingestion-gate",
            get(dashboard_ingestion_gate),
        )
        .route("/api/v1/dashboard/ledger", get(dashboard_ledger))
        .route("/api/v1/dashboard/command", post(dashboard_command))
}

/// Returns the Ingestion Gate projection.
///
/// Groups in-memory candidates by source, synthesising a Source/SourceSnapshot
/// wrapper for each group so the dashboard can render them.
async fn dashboard_ingestion_gate(State(state): State<AppState>) -> Json<Vec<IngestionGateItem>> {
    let candidates = state.candidates.read().await;
    let reviews = state.reviews.read().await;

    // Group candidates by their source string.
    let mut by_source: std::collections::HashMap<String, Vec<&DecisionCandidate>> =
        std::collections::HashMap::new();
    for c in candidates.values() {
        by_source.entry(c.source.clone()).or_default().push(c);
    }

    let mut items: Vec<IngestionGateItem> = by_source
        .into_iter()
        .map(|(source_key, cands)| {
            let earliest = cands
                .iter()
                .map(|c| c.extracted_at)
                .min()
                .unwrap_or_else(chrono::Utc::now);
            let previews: Vec<CandidatePreview> = cands
                .iter()
                .map(|c| {
                    let review_state = reviews
                        .get(&c.id)
                        .map(|r| format!("{:?}", r.status).to_lowercase());
                    let confidence: Option<f64> = match c.extraction_confidence {
                        bicameral_api::candidate::ExtractionConfidence::Low => Some(0.3),
                        bicameral_api::candidate::ExtractionConfidence::Medium => Some(0.6),
                        bicameral_api::candidate::ExtractionConfidence::High => Some(0.9),
                    };
                    CandidatePreview {
                        id: Some(c.id.to_string()),
                        summary: c.title.clone(),
                        feature_hint: c.tags.first().cloned(),
                        evidence_refs: Vec::new(),
                        extraction_confidence: confidence,
                        conflict_hint: None,
                        review_state,
                    }
                })
                .collect();

            IngestionGateItem {
                source: Source {
                    uri: source_key.clone(),
                    source_type: "ingested".to_string(),
                },
                snapshot: SourceSnapshot {
                    snapshot_addr: format!("sha256:{:x}", md5_hash(&source_key)),
                    snapshot_ref: source_key.clone(),
                    captured_at: earliest,
                },
                source_title: source_key,
                source_freshness: SourceFreshness::Unknown,
                evidence: Vec::new(),
                candidates: previews,
                tracked: true,
            }
        })
        .collect();

    items.sort_by(|a, b| a.source_title.cmp(&b.source_title));
    Json(items)
}

/// Trivial hash for generating deterministic snapshot addresses.
fn md5_hash(s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

/// Returns the Ledger View projection.
///
/// Maps in-memory candidates + review states into `LedgerCandidate` items.
/// Promoted `LedgerDecision` items will appear once event-store governance
/// materialises decisions.
async fn dashboard_ledger(State(state): State<AppState>) -> Json<Vec<LedgerReviewItem>> {
    let candidates = state.candidates.read().await;
    let reviews = state.reviews.read().await;

    let mut items: Vec<LedgerReviewItem> = candidates
        .values()
        .map(|c| {
            let review_state = reviews
                .get(&c.id)
                .map(|r| format!("{:?}", r.status).to_lowercase())
                .unwrap_or_else(|| "pending".to_string());

            LedgerReviewItem::Candidate(LedgerCandidate {
                id: c.id.to_string(),
                summary: c.title.clone(),
                feature_hint: c.tags.first().cloned(),
                sources: Vec::new(),
                review_state,
                allowed_commands: vec![
                    CandidateCommandKind::AcceptCandidate,
                    CandidateCommandKind::RejectCandidate,
                    CandidateCommandKind::RequestContext,
                ],
            })
        })
        .collect();

    items.sort_by(|a, b| {
        let summary_a = match a {
            LedgerReviewItem::Decision(d) => &d.summary,
            LedgerReviewItem::Candidate(c) => &c.summary,
        };
        let summary_b = match b {
            LedgerReviewItem::Decision(d) => &d.summary,
            LedgerReviewItem::Candidate(c) => &c.summary,
        };
        summary_a.cmp(summary_b)
    });

    Json(items)
}

/// Accepts a dashboard review command.
///
/// Commands are substrate-neutral — the dashboard never writes event-store
/// internals directly. The gateway validates and routes them through
/// governance policy.
async fn dashboard_command(
    State(state): State<AppState>,
    Json(cmd): Json<DashboardReviewCommand>,
) -> Result<StatusCode, StatusCode> {
    // Record the command attempt in the audit trail
    {
        let mut audit = state.audit.write().await;
        let _ = audit.record(
            AuditAction::ReviewCommandIssued,
            "dashboard".to_string(),
            None,
            format!(
                "Dashboard command '{}' on target '{}'",
                cmd.command, cmd.target_id
            ),
        );
    }

    tracing::info!(
        command = %cmd.command,
        target = %cmd.target_id,
        "Dashboard review command received"
    );

    Ok(StatusCode::ACCEPTED)
}
