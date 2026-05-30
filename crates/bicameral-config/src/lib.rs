//! Configuration loading and workspace discovery for the Bicameral runtime.
//!
//! Configuration is rooted in `.bicameral/` within a workspace or a user-level
//! config path (`~/.config/bicameral/` on Linux/macOS). This crate provides
//! discovery, loading, validation, and the typed config schema.
//!
//! Derived from ZeroClaw's config layer, narrowed to Bicameral's governance domain.
//! See NOTICE and runtime/UPSTREAM-ZEROCLAW.md for attribution.

pub mod paths;
pub mod schema;
pub mod workspace;

pub use paths::BicameralPaths;
pub use schema::{BicameralConfig, EventStoreConfig, GatewayConfig, GovernancePolicyConfig};
pub use workspace::WorkspaceDiscovery;
