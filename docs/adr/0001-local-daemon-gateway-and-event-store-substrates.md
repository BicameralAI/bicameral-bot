# ADR-0001: Local Daemon, Gateway, and Decision Event Store Substrates

**Date:** 2026-05-27  
**Status:** proposed  
**Level:** L1  
**Carries forward:** `bicameral-daemon` ADR-0001 plus the original `bicameral-bot` ADR-0001

## Problem

Bicameral must ingest from many messy operational edges without letting each source decide what is canonical. Teams differ by Jira/Linear/Slack/meeting/GitHub usage, privacy boundary, repo topology, and review culture.

The decision ledger previously treated an opaque database/cache as durable authority. That creates a split-brain problem: canonical decisions live somewhere teams do not naturally review while the work they govern lives in git, Drive, dashboards, and agent sessions. The load-bearing product requirement is predictable authority, not clever self-maintenance.

## Decision

`bicameral-bot` is a public local-first runtime with a daemon + gateway shape.

The gateway is expressive. It adapts edge inputs into typed Bicameral protocol objects: source evidence, candidates, dependency signals, binding hints, review commands, and advisory governance results.

The daemon/core is boring. It validates objects, evaluates governance policy, preserves audit state, performs local grounding, and materializes accepted events through storage adapters.

Define an **event store substrate** boundary for canonical decision state. The selected event store substrate owns the durable, replayable event stream for a workspace. The bot's local database, dashboard state, MCP materialization, hosted code graph state, and search indexes are caches/materializations rebuilt from that substrate plus code/source indexes. They are not canonical authority.

First-class substrate flows:

1. **Git-backed event store** — canonical decisions and review events live as reviewable text in the repository.
2. **Google Drive-backed event store** — canonical decisions and review events live as replayable files in a user-provisioned Drive folder.
3. **Future event store adapters** — must implement the same replay/materialization contract before they can claim canonical authority.

ADR-0007 defines the substrate-neutral governance flow: how source evidence, candidates, review commands, policy, and governance results become eligible for materialization. ADR-0001 only defines how accepted domain events are stored, replayed, and exposed by the chosen substrate.

### Survey-informed default

Public developer survey evidence supports git-backed `.bicameral/` text as the default onboarding path for repo-centric engineering teams, but not as the whole Bicameral ontology. Stack Overflow's 2024 survey shows that developer decision evidence is spread across Jira, Confluence, Markdown files, Notion, GitHub Discussions, Azure DevOps, Obsidian, Slack, Teams, Zoom, Discord, and other collaboration surfaces. See [Public Developer Survey Implications](../research/public-developer-survey-implications.md).

Implication: git remains the predictable first-class adapter where teams already use PR/code review as an authority boundary. The architectural invariant is still replayable, auditable accepted events through an event store substrate contract. Drive and future adapters remain real because the source of decision evidence and the source of code are not always the same substrate.

## Substrate Contract

Every event store substrate must implement the same contract even if its files, review mechanics, and enforcement capabilities differ.

### 1. Durable event log

The substrate must durably preserve accepted domain events, including:

- source/candidate acceptance events;
- review events;
- decision materialization events;
- supersession events;
- binding/compliance review events when those are accepted into durable history;
- governance result records that affect shared team state.

### 2. Replay

The bot must be able to rebuild current state from substrate artifacts:

```text
event store substrate
  → replay ordered events
  → materialized decisions / review state / signoff state / compliance state
  → local caches + dashboard + MCP/agent views
```

Replay must not depend on hidden local database rows.

### 3. Idempotence and ordering

Each accepted event must have a stable identity and enough ordering metadata for deterministic replay. Substrates may use different ordering sources:

- Git: commit order, branch/merge topology, file revision, event timestamp.
- Drive: event filename timestamp, event id, Drive revision id, last-confirmed sync cursor.

### 4. Conflict handling

The adapter must detect and surface duplicate candidate acceptance, incompatible decisions, concurrent edits, stale local state, substrate write conflicts, and replay failures.

Conflict explanation must be visible through governance results and the Ledger View, but the explanation mechanics can differ by substrate.

### 5. Auditability

A reviewer must be able to inspect source references, candidate text/rationale as reviewed, reviewer action and capability, policy result, materialized event artifacts, and supersession/collision resolution.

Secrets must not be persisted as canonical event content; adapters should store redacted excerpts or pointers when necessary.

### 6. Capability declaration

Each adapter must declare what it can enforce. It must not claim CI/merge blocking unless the selected substrate actually provides a merge boundary.

## Shared Domain Objects

The substrate stores accepted events for the shared governance vocabulary from ADR-0007:

- `SourceEvidence`
- `DecisionCandidate`
- `Decision`
- `ReviewCommand`
- `ReviewEvent`
- `ReviewState`
- `BindingHint`
- `BindingEvidence`
- `DependencySignal`
- `ComplianceVerdict`
- `GovernanceResult`

A substrate does not have to store all objects as separate files. It must store enough event data to replay the same materialized domain state.

## Git-Backed Event Store

Use git when the workspace wants boring, predictable, text-in-repo review.

Minimum layout:

```text
.bicameral/
  decisions/
    DEC-0001.yaml
  events/
    2026-05-27T180000Z-accept-candidate.yaml
  bindings/
    DEC-0001.yaml
  policy/
    governance.yaml
```

The exact layout can evolve, but git-backed materialization must preserve these rules:

- canonical state is reviewable text;
- accepted events are replayable without the local cache;
- branch-local proposals stay reviewable in ordinary PR workflow;
- CI/merge enforcement is only claimed when the repository actually has that boundary;
- mods, connectors, and agents never write `.bicameral/decisions/*.yaml` directly.

## Google Drive-Backed Event Store

Use Drive when the workspace starts from shared product/team memory rather than a repo PR boundary.

Drive-backed events must still be replayable, ordered, idempotent, and auditable. Because Drive cannot block a merge, enforcement should surface as dashboard flags, agent warnings, queued actions, or paused review states rather than pretending to be CI.

## Non-Goals

This ADR does not define extraction prompts, connector-specific ingest rules, review UI layout, owner/member capability policy, confidence thresholds, code graph indexing, or the default substrate choice.

Source-specific adapters belong in `bicameral-integrations`. Organization-scale hosted code graph behavior belongs in `bicameral-cloud`.

## Consequences

Bicameral can meet teams at their operational substrate while preserving one authority path. Git remains predictable for repo-centric teams; Drive and future substrates can exist without changing the domain lifecycle. Local cache corruption is recoverable by replay.
