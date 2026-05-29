//! Local daemon, service lifecycle, and governance engine for Bicameral.
//!
//! The runtime provides the "boring middle": daemon lifecycle, signal handling,
//! governance policy evaluation, event-store substrate management, and the
//! coordination that ties gateway, mods, and audit together.
//!
//! Derived from ZeroClaw's runtime layer (daemon, service, approval, SOP).
//! See NOTICE and runtime/UPSTREAM-ZEROCLAW.md for attribution.

pub mod daemon;
pub mod event_store_adapters;
pub mod governance;
pub mod service;

pub use daemon::{DaemonConfig, DaemonExit};
pub use governance::GovernanceEngine;
