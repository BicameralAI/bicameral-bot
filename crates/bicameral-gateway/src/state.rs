//! Shared application state for the gateway.

use bicameral_api::candidate::DecisionCandidate;
use bicameral_api::review::ReviewState;
use bicameral_audit::AuditStore;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Shared state accessible by all gateway route handlers.
#[derive(Clone)]
pub struct AppState {
    pub candidates: Arc<RwLock<HashMap<Uuid, DecisionCandidate>>>,
    pub reviews: Arc<RwLock<HashMap<Uuid, ReviewState>>>,
    pub audit: Arc<RwLock<AuditStore>>,
}

impl AppState {
    pub fn new(audit: AuditStore) -> Self {
        Self {
            candidates: Arc::new(RwLock::new(HashMap::new())),
            reviews: Arc::new(RwLock::new(HashMap::new())),
            audit: Arc::new(RwLock::new(audit)),
        }
    }
}
