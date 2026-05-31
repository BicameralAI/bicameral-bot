//! Gateway route definitions.

use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use bicameral_api::candidate::DecisionCandidate;
use bicameral_api::dashboard::{DashboardReviewCommand, IngestionGateItem, LedgerReviewItem};
use bicameral_api::review::{ReviewCommand, ReviewState, ReviewStatus};
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
/// In v0.1 this returns an empty list — real source snapshots are populated
/// once connectors/extractors produce DecisionCandidates.
async fn dashboard_ingestion_gate(State(_state): State<AppState>) -> Json<Vec<IngestionGateItem>> {
    Json(Vec::new())
}

/// Returns the Ledger View projection.
///
/// In v0.1 this returns an empty list — real Decisions appear only after
/// governed candidate promotion through the Ingestion Gate.
async fn dashboard_ledger(State(_state): State<AppState>) -> Json<Vec<LedgerReviewItem>> {
    Json(Vec::new())
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
