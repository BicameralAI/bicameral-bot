//! Governance engine — evaluates policy against candidates and commands.

use bicameral_api::candidate::{DecisionCandidate, ExtractionConfidence};
use bicameral_api::governance::{GovernanceResult, GovernanceVerdict};
use bicameral_config::GovernancePolicyConfig;

/// The governance engine evaluates whether candidates meet policy requirements.
///
/// It produces advisory GovernanceResults — it does not directly write canonical
/// state. Only event-store adapter materialization can promote accepted results.
pub struct GovernanceEngine {
    config: GovernancePolicyConfig,
}

impl GovernanceEngine {
    pub fn new(config: GovernancePolicyConfig) -> Self {
        Self { config }
    }

    /// Evaluate a candidate against the current governance policy.
    pub fn evaluate(&self, candidate: &DecisionCandidate) -> GovernanceResult {
        // Check minimum extraction confidence
        if let Some(ref min_conf) = self.config.min_extraction_confidence {
            let threshold = match min_conf.as_str() {
                "high" => ExtractionConfidence::High,
                "medium" => ExtractionConfidence::Medium,
                _ => ExtractionConfidence::Low,
            };
            if candidate.extraction_confidence < threshold {
                return GovernanceResult::reject(
                    candidate.id,
                    format!(
                        "Extraction confidence {:?} below minimum {:?}",
                        candidate.extraction_confidence, threshold
                    ),
                );
            }
        }

        // If review is required, route to configured approvers
        if self.config.require_review {
            return GovernanceResult::needs_review(
                candidate.id,
                "Policy requires human review before acceptance".to_string(),
                self.config.approvers.clone(),
            );
        }

        GovernanceResult::accept(candidate.id, "Policy satisfied".to_string())
    }

    /// Check whether a governance result permits materialization.
    pub fn may_materialize(result: &GovernanceResult) -> bool {
        result.verdict == GovernanceVerdict::Accepted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bicameral_api::candidate::DecisionLevel;

    #[test]
    fn test_requires_review_by_default() {
        let engine = GovernanceEngine::new(GovernancePolicyConfig::default());
        let candidate = DecisionCandidate::new(
            "Test".to_string(),
            "Description".to_string(),
            DecisionLevel::Architecture,
            "test-source".to_string(),
        );
        let result = engine.evaluate(&candidate);
        assert_eq!(result.verdict, GovernanceVerdict::NeedsReview);
    }

    #[test]
    fn default_policy_does_not_permit_materialization() {
        let engine = GovernanceEngine::new(GovernancePolicyConfig::default());
        let candidate = DecisionCandidate::new(
            "Test".to_string(),
            "Description".to_string(),
            DecisionLevel::Architecture,
            "test-source".to_string(),
        );
        let result = engine.evaluate(&candidate);
        assert!(
            !GovernanceEngine::may_materialize(&result),
            "Default policy must not permit materialization without human review"
        );
    }

    #[test]
    fn test_rejects_low_confidence() {
        let config = GovernancePolicyConfig {
            require_review: false,
            min_extraction_confidence: Some("high".to_string()),
            approvers: Vec::new(),
        };
        let engine = GovernanceEngine::new(config);
        let candidate = DecisionCandidate::new(
            "Test".to_string(),
            "Description".to_string(),
            DecisionLevel::Product,
            "test-source".to_string(),
        );
        let result = engine.evaluate(&candidate);
        assert_eq!(result.verdict, GovernanceVerdict::Rejected);
    }
}
