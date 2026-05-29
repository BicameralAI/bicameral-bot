//! Mod manifest validation.

use crate::manifest::ModManifest;

/// Validation errors for mod manifests.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    #[error("manifest missing required field: {field}")]
    MissingField { field: String },
    #[error("manifest has no triggers defined")]
    NoTriggers,
    #[error("manifest has no actions defined")]
    NoActions,
    #[error("invalid filter in trigger: {reason}")]
    InvalidFilter { reason: String },
    #[error("mod name is empty")]
    EmptyName,
}

/// Validate a mod manifest for correctness.
///
/// Returns a list of validation errors (empty means valid).
pub fn validate_manifest(manifest: &ModManifest) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    if manifest.name.is_empty() {
        errors.push(ValidationError::EmptyName);
    }

    if manifest.triggers.is_empty() {
        errors.push(ValidationError::NoTriggers);
    }

    if manifest.actions.is_empty() {
        errors.push(ValidationError::NoActions);
    }

    for trigger in &manifest.triggers {
        for filter in &trigger.filters {
            if filter.field.is_empty() {
                errors.push(ValidationError::InvalidFilter {
                    reason: "filter field is empty".to_string(),
                });
            }
        }
    }

    errors
}

/// Load and validate a manifest from a YAML file.
pub fn load_and_validate(path: &std::path::Path) -> anyhow::Result<(ModManifest, Vec<ValidationError>)> {
    let content = std::fs::read_to_string(path)?;
    let manifest: ModManifest = serde_yaml::from_str(&content)?;
    let errors = validate_manifest(&manifest);
    Ok((manifest, errors))
}
