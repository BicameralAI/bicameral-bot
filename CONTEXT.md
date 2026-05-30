# Bicameral

Bicameral captures implementation-constraining decisions from product, code, and collaboration evidence, then routes them through review into a durable event authority.

## Domain Model

### Entities

`Decision`
: Binding constraint on implementation.

`DecisionCandidate`
: Extracted claim awaiting governed promotion into the Decision Ledger.

`Source`
: Mutable external object linkage identified by URI.

`SourceSnapshot`
: Immutable captured view of a Source, identified by content address.

`SourceEvidence`
: Pointer into one SourceSnapshot.

`BindingEvidence`
: Reviewable evidence relating a Decision to a code path, symbol, diff, dependency, workflow, or deploy surface.

`GovernanceResult`
: Substrate-neutral outcome of governance or conflict analysis.

`ModManifest`
: Declarative configuration artifact for EM-safe evidence-producing mods.

### Lifecycle Verbs

`Promote`
: Governed transition that turns a DecisionCandidate into a Decision Ledger record.

`Demote`
: Governed Ledger View transition that lowers a Decision's authority without recreating it as a candidate.

## Relationships

`DecisionCandidate` cites `SourceEvidence` [0..*].

`Decision` cites `SourceEvidence` [1..*].

`SourceEvidence` points into `SourceSnapshot` [1].

`SourceSnapshot` captures `Source` [1].

`Source` has `SourceSnapshot` [0..*].

`Promote` turns `DecisionCandidate` into `Decision` [0..1].

`Demote` targets `Decision` [1].

## Supporting Terms

`Decision Ledger`
: Canonical materialized decision state derived by replaying the selected event store substrate.

`Ledger View`
: Human-facing surface for inspecting Decision Ledger state and emitting review commands.

`Governance policy`
: Rules that decide how candidates, review commands, and evidence route to review, advisory state, materialization, or enforcement.

`EventStoreAdapter`
: Substrate-specific materialization boundary for accepted governance events.

`Signoff`
: Ownership lifecycle on a Decision, separate from candidate acceptance and code compliance.

`Status / compliance state`
: Code-compliance state for a Decision, computed or reviewed from grounding and drift evidence.

`Read/write path`
: Review surfaces, MCP tools, integrations, and mods emit substrate-neutral commands/evidence; governance policy and event store adapters decide materialization.

`Local daemon`
: Local core that validates protocol objects, evaluates governance policy, preserves audit state, performs local grounding, and materializes accepted events.

`Gateway`
: Local edge boundary that adapts operational sources into typed protocol objects before they enter daemon governance.

`Protocol folder`
: In-repo `protocol/` module containing shared object schemas and conformance fixtures used by bot, MCP, integrations, and cloud clients.

## Terminology Notes

- `Decision` is not a suggestion, opinion, note, or general product knowledge.
- `DecisionCandidate` is non-canonical until governance policy accepts a review command and the selected event store substrate materializes the event.
- `Source` is not immutable evidence; immutable evidence is cited through `SourceSnapshot` and `SourceEvidence`.
- `Ledger View`, dashboards, SurrealDB, search indexes, and hosted graphs are not durable authority.
- `Signoff` is not compliance, drift, or grounding status.
- Frontend, CLI, MCP, connector, and mod surfaces must not create Decisions directly; they use `Promote`.
