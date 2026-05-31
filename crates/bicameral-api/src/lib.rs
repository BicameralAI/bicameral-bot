//! Protocol types for the Bicameral governance runtime.
//!
//! This crate defines the typed objects that flow through the Bicameral system:
//! decision candidates, source/binding evidence, review commands, governance
//! results, and audit receipts.
//!
//! Derived from ZeroClaw's API layer, narrowed to Bicameral's governance domain.
//! See NOTICE and runtime/UPSTREAM-ZEROCLAW.md for attribution.

pub mod candidate;
pub mod dashboard;
pub mod event_store;
pub mod evidence;
pub mod governance;
pub mod review;
pub mod source;

pub use candidate::{DecisionCandidate, DecisionLevel, ExtractionConfidence};
pub use event_store::{EventStoreAdapter, EventStoreEntry, EventStoreError, SubstrateKind};
pub use evidence::{BindingEvidence, SourceEvidence};
pub use governance::{GovernancePolicy, GovernanceResult, GovernanceVerdict};
pub use review::{ReviewCommand, ReviewCommandKind, ReviewState};
