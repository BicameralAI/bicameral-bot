# RFQ: Bicameral Ingest Integration Protocol v0.1

## 1. Summary

We are requesting a quote to build the first version of Bicameral's ingest integration protocol contracts.

The goal is to let integration authors build Jira, Slack, Notion, GitHub, support inbox, and similar ingest adapters that can submit source evidence, decision candidates, dependency signals, binding hints, and advisory governance results into `bicameral-bot` without gaining authority over canonical decisions.

This RFQ is for a narrow protocol-contract spike, not a full integration platform.

## 2. Background

`bicameral-bot` is the public local daemon/gateway/runtime for Bicameral. It owns the local protocol boundary, governance policy entrypoint, review-command surface, local grounding, and storage-adapter materialization path.

Relevant architecture docs:

- ADR-0001: Local Daemon, Gateway, and Decision Event Store Substrates — accepted events materialize through a replayable substrate such as git or Drive.
- ADR-0002: Agent Surfaces and Bot Runtime Interface — integrations, MCP, CLI, dashboard, and cloud clients submit protocol-shaped objects into the bot.
- ADR-0004: HITL Boundaries for Probabilistic Governance — extraction, binding, and compliance confidence must remain separate.
- ADR-0005: Separate Event Store Substrate, Ingestion Connectors, Review UX, and Governance Policy — connectors produce evidence/candidates, not authority.
- ADR-0007: Substrate-Neutral Governance Flow — all sources enter the same governance path before canonical materialization.

Core principle:

```text
Integrations are expressive at the edge.
Bicameral core is boring in the middle.
Governance decides what becomes canonical.
```

## 3. Problem Statement

Today the ADRs define the architecture and object vocabulary, but `protocol/` does not yet contain concrete contracts an integration author can build against.

We need versioned schemas, examples, registries, and validation tooling so an integration author can safely emit source evidence and optional candidate/signal projections without accidentally designing a parallel authority path.

The hard part is not "can we parse Jira?" The hard part is whether external ingest integrations can produce safe, typed, governable Bicameral objects without directly creating canonical decisions.

## 4. Goals

This project should deliver:

1. Define the v0.1 ingest integration protocol contract.
2. Provide JSON Schemas and example fixtures.
3. Provide validation tooling for fixture-based integration development.
4. Make it obvious what integrations may emit and what they may not emit.
5. Preserve the Bicameral authority boundary: integrations produce claims/evidence/hints/advisories; governance and storage adapters decide what becomes canonical.

## 5. Non-Goals

This project should not deliver:

- Real Jira, Slack, Notion, GitHub, or support inbox authentication.
- Network polling or webhook implementations.
- Persistent event store materialization.
- Review UI.
- Cloud graph integration.
- CI blocking.
- Production policy engine.
- Direct `.bicameral/decisions/*.yaml` writes.
- Auto-ratification of decisions.

## 6. Users / Consumers

Primary consumers:

- Integration authors building Jira, Slack, Notion, GitHub, support inbox, or custom source adapters.
- `bicameral-bot`, validating ingest payloads before governance policy.
- Future `bicameral-mcp` tools and `bicameral-integrations` packages using the same protocol objects.

Secondary consumers:

- EMs reviewing whether a domain-specific integration is safe.
- Product/engineering reviewers checking that integration output remains non-authoritative.
- Future cloud advisory services that may emit compatible evidence/advisories.

## 7. Scope of Work

### 7.1 Protocol Contract

Define versioned protocol contracts under:

```text
protocol/
  CONTRACTS.md
  schemas/
    v0.1/
  examples/
    ingest/
  registries/
```

Required objects:

- `IngestEnvelope`
- `IntegrationIdentity`
- `SourceEvidence`
- `DecisionCandidate`
- `DependencySignal`
- `BindingHint`
- advisory `GovernanceResult`
- `SuggestedReviewer`
- `ProtocolError`
- `IngestResponse`

### 7.2 Schema Files

Create JSON Schema files:

```text
protocol/schemas/v0.1/common.schema.json
protocol/schemas/v0.1/ingest-envelope.schema.json
protocol/schemas/v0.1/source-evidence.schema.json
protocol/schemas/v0.1/decision-candidate.schema.json
protocol/schemas/v0.1/dependency-signal.schema.json
protocol/schemas/v0.1/binding-hint.schema.json
protocol/schemas/v0.1/governance-result.schema.json
protocol/schemas/v0.1/ingest-response.schema.json
protocol/schemas/v0.1/protocol-error.schema.json
```

Schemas may be hand-authored for v0.1. If the implementer recommends generating schemas from TypeScript, Rust, or Pydantic later, document that as a future option rather than expanding this RFQ.

### 7.3 Examples / Fixtures

Create example ingest payloads:

```text
protocol/examples/ingest/jira-issue.ingest.json
protocol/examples/ingest/slack-thread.ingest.json
protocol/examples/ingest/notion-transcript.ingest.json
```

Each example should validate against the schema and demonstrate:

- source evidence;
- at least one candidate or signal projection;
- explicit confidence surfaces;
- reviewer suggestions;
- advisory governance result where appropriate;
- no canonical writes or authoritative decisions.

### 7.4 Registries

Create lightweight registries:

```text
protocol/registries/source-types.md
protocol/registries/projection-types.md
protocol/registries/review-states.md
```

The registries should include common v0.1 values and an extension rule for custom integrations.

### 7.5 Validation Tooling

Provide a simple validation command or script.

If the Bicameral CLI exists by implementation time, target:

```bash
bicameral protocol validate protocol/examples/ingest/jira-issue.ingest.json
```

If not, provide a standalone script:

```bash
python scripts/validate_protocol.py protocol/examples/ingest/jira-issue.ingest.json
```

The script must:

- validate the envelope against JSON Schema;
- produce structured errors;
- validate all examples in one command;
- fail invalid fixtures in tests.

### 7.6 Documentation

Create:

```text
protocol/CONTRACTS.md
```

It should explain:

- what integrations may emit;
- what integrations may not emit;
- idempotency rules;
- versioning rules;
- source identity rules;
- confidence surface rules;
- metadata policy;
- example lifecycle from ingest payload to review queue;
- how v0.1 relates to the ADR authority boundary.

## 8. Required Design Decisions

The implementation must follow these decisions unless explicitly challenged in the proposal.

### 8.1 Envelope Model

Use one source item per envelope.

```ts
type IngestEnvelope = {
  protocol_version: '0.1';
  integration: IntegrationIdentity;
  source: SourceEvidence;
  projections: IngestProjection[];
  idempotency_key: string;
  observed_at: string;
};
```

Batch envelopes are out of scope for v0.1.

### 8.2 Integration Identity

```ts
type IntegrationIdentity = {
  integration_id: string;
  integration_kind: string;
  version: string;
  workspace_ref?: string;
};
```

`integration_id` participates in source identity and idempotency. Integrations may not self-declare trust level; bot-side policy maps integration identity to trust behavior.

### 8.3 Allowed Projection Types

For v0.1, integrations may emit only:

- `DecisionCandidate`
- `DependencySignal`
- `BindingHint`
- advisory `GovernanceResult`

These projections are claims, hints, and advisories. They are not canonical authority.

### 8.4 Forbidden Projection Types

Integrations must not emit:

- `Decision`
- `ReviewCommand`
- `ReviewEvent`
- canonical `BindingEvidence`
- authoritative `ComplianceVerdict`
- blocking `GovernanceResult`
- direct storage adapter commands
- direct canonical write instructions

### 8.5 Source Identity

Canonical source identity is:

```text
integration_id + source_type + source_ref
```

`source_ref` is only stable within the integration/source namespace; it is not globally unique by itself.

### 8.6 Idempotency

Each envelope must include an `idempotency_key`.

Recommended default:

```text
sha256(protocol_version + integration_id + source_type + source_ref + source_updated_at/content_hash)
```

If an integration cannot compute `content_hash`, it should include enough content or pointer metadata for the bot to compute or track one later.

### 8.7 Source Content Policy

`SourceEvidence` must include either:

- `source_excerpt`; or
- `content_ref`.

Full body content is optional. Large bodies should use pointers or excerpts. Source evidence must preserve enough audit context for a reviewer to understand why a candidate or signal exists.

### 8.8 Confidence Surfaces

Do not allow generic top-level `confidence`.

Allowed confidence fields must be explicit:

- `extraction_confidence`
- `binding_confidence`
- `dependency_confidence`
- `compliance_confidence`

A strong value on one surface must not compensate for weakness on another.

### 8.9 Reviewer Suggestions

Integrations may suggest reviewers, but may not assign canonical authority.

```ts
type SuggestedReviewer = {
  role: 'pm' | 'em' | 'dev' | 'security' | 'platform' | 'owner' | string;
  user_ref?: string;
  reason: string;
};
```

`user_ref` is an opaque integration-local or workspace-local hint. Bot policy or identity resolution decides whether it maps to a real reviewer.

### 8.10 Transport

v0.1 should be fixture/file-first.

Document the future HTTP shape, but do not require a running HTTP server for this RFQ.

Future local HTTP target:

```http
POST /v0/ingest
Content-Type: application/json
```

### 8.11 Response Semantics

Avoid ambiguous `accepted: true` responses.

Use staged status:

```ts
type IngestStatus =
  | 'received'
  | 'validated'
  | 'queued_for_review'
  | 'policy_rejected'
  | 'duplicate_ignored'
  | 'failed';
```

An ingest response must not imply canonical materialization unless a review command has run and the event store adapter has materialized accepted events.

### 8.12 Authority Boundary

Integrations produce claims, evidence, hints, and advisories.

Canonical state changes only through:

```text
integration output
  → bot validation
  → governance policy
  → review command
  → event store adapter
  → replayed canonical state
```

## 9. Expected Deliverables

The final delivery should include:

1. JSON Schemas under `protocol/schemas/v0.1/`.
2. Example fixtures under `protocol/examples/ingest/`.
3. Protocol docs under `protocol/CONTRACTS.md`.
4. Source/projection/review-state registries.
5. Validation script or command.
6. Tests proving examples validate and invalid examples fail.
7. Short implementation notes explaining tradeoffs and unresolved questions.

## 10. Acceptance Criteria

The work is accepted when:

- All example fixtures validate against schemas.
- Invalid examples fail with structured errors.
- A Jira issue example can emit:
  - `SourceEvidence`
  - `DecisionCandidate`
  - `DependencySignal`
  - advisory `GovernanceResult`
- The schema forbids canonical `Decision` objects from ingest integrations.
- The schema forbids direct canonical write instructions.
- The schema forbids generic top-level `confidence`.
- The schema forbids blocking `GovernanceResult` from ingest integrations.
- The docs clearly explain that integrations do not create authority.
- The protocol version is explicit and stable as `0.1`.
- The contract is understandable by an integration author without reading every ADR.
- The validation script can be run locally from a clean checkout.

## 11. Out of Scope / Explicit Rejections

The contractor should not implement:

- Real external API authentication.
- Persistent event store materialization.
- Review UI.
- Cloud graph integration.
- CI blocking.
- Production policy engine.
- Direct `.bicameral/decisions/*.yaml` writes.
- Auto-ratification of decisions.
- Batch ingest envelopes.
- Generic free-form confidence fields.

## 12. Proposal Response Format

Please respond with:

1. Understanding of the architecture boundary.
2. Proposed implementation plan.
3. File tree you expect to create/change.
4. Any schema design questions.
5. Estimated time/cost.
6. Risks or decisions you believe need clarification.
7. Optional alternatives you recommend.

## 13. Evaluation Criteria

We will evaluate proposals based on:

- Respect for Bicameral authority boundaries.
- Simplicity of the v0.1 contract.
- Schema quality and validation clarity.
- Example quality.
- Ability for future integrations to build from the contract.
- Minimal unnecessary abstraction.
- Clear separation between evidence, candidate, review, and canonical state.

## 14. Reference Example

A Jira issue ingest payload should look approximately like this:

```json
{
  "protocol_version": "0.1",
  "integration": {
    "integration_id": "jira-main",
    "integration_kind": "jira",
    "version": "0.1.0"
  },
  "source": {
    "source_type": "jira.issue",
    "source_ref": "PROJ-123",
    "source_url": "https://example.atlassian.net/browse/PROJ-123",
    "source_title": "Launch workflow requires deploy changes",
    "source_excerpt": "We need deploy pipeline support before launch.",
    "observed_at": "2026-05-27T18:00:00Z",
    "source_updated_at": "2026-05-27T17:45:00Z"
  },
  "projections": [
    {
      "type": "DecisionCandidate",
      "title": "Deploy pipeline must support launch workflow",
      "description": "The launch requires deploy pipeline changes before release.",
      "level_hint": "L2",
      "feature_hint": "launch-readiness",
      "extraction_confidence": 0.82,
      "suggested_reviewers": [
        {
          "role": "pm",
          "reason": "Product meaning needs ratification"
        },
        {
          "role": "em",
          "reason": "Potential dependency/scope risk"
        }
      ]
    },
    {
      "type": "DependencySignal",
      "owner_lens": "em",
      "summary": "Likely cross-team dependency on deploy infrastructure.",
      "dependency_confidence": 0.76,
      "suggested_action": "Ask EM to confirm blast radius before implementation begins."
    },
    {
      "type": "GovernanceResult",
      "severity": "warning",
      "enforcement_capability": "agent_warning",
      "reason": "Potential implementation dependency detected, but grounding is unresolved.",
      "required_action": "EM review required before treating as blocking."
    }
  ],
  "idempotency_key": "sha256:example",
  "observed_at": "2026-05-27T18:00:00Z"
}
```

## 15. Open Questions

The contractor may propose answers, but should not silently decide:

1. Should `source_excerpt` be mandatory even when `content_ref` exists?
2. Should `user_ref` be allowed in reviewer suggestions for v0.1, or deferred?
3. Should Slack/Notion examples include full transcript excerpts or pointer-only content refs?
4. Should schemas be generated from implementation types later?
5. Should invalid examples live beside valid examples or in a test fixture folder?

## 16. Preferred First Milestone

Before implementing all schemas, first submit a small draft with:

```text
protocol/CONTRACTS.md
protocol/schemas/v0.1/ingest-envelope.schema.json
protocol/examples/ingest/jira-issue.ingest.json
```

The first milestone should prove the envelope model and authority boundary before expanding the full fixture set.
