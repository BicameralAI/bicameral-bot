# ADR-0002: HITL Boundaries Across Two Probabilistic Domains

Status: accepted

Date: 2026-05-27

## Context

Bicameral bridges two probabilistic domains:

1. Human operational interpretation — meetings, tickets, Slack threads, Linear/Jira issues, support notes, and agent sessions. The system must infer whether a real decision exists, what it means, and who owns it.
2. Code/workflow interpretation — files, symbols, diffs, dependencies, deploy paths, repo topology, and compliance state. The system must infer whether implementation evidence supports, contradicts, or depends on the decision.

If these uncertainties are collapsed into one confident governance answer, Bicameral can create new cognitive debt: false status, wrong reviewer attention, phantom blockers, stale dashboards, or misplaced trust.

## Decision

Separate confidence surfaces and route ambiguity to humans:

- `extraction_confidence` asks: is this a real decision and what does it mean? PM/product owner review owns semantic authority.
- `binding_confidence` asks: is this the right code/dependency/scope evidence? EM/Dev review owns technical grounding.
- `compliance_confidence` asks: does implementation satisfy or violate the decision? Governance policy may warn, but blocking requires grounded evidence and explicit authority.

HITL is not a fallback for model failure. HITL is the mechanism that prevents probabilistic interpretation from becoming unreviewed authority.

## Consequences

- Confidence routes review; it does not decide.
- Weak or unresolved evidence remains advisory/unbound.
- Mods and integrations emit typed objects, not canonical writes.
- Blocking and canonical promotion require policy plus reviewable evidence.
