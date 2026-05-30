# Bicameral

Bicameral captures implementation-constraining decisions from product, code, and collaboration evidence, then routes them through review into a durable event authority.

## Language

**Decision**:
A binding constraint on implementation. Not a suggestion, opinion, note, or general product knowledge.
_Avoid_: note, feedback, request, product knowledge

**DecisionCandidate**:
An extracted claim that has not yet been accepted into the Decision Ledger. It is non-canonical until governance policy accepts a review command and the selected event store substrate materializes the event.
_Avoid_: approved decision, canonical record, source note

**SourceEvidence**:
The excerpt, pointer, payload, or provenance record that supports a candidate, binding, dependency signal, or governance result.
_Avoid_: vague context, model memory

**BindingEvidence**:
Reviewable evidence that a decision relates to a code path, symbol, diff, dependency, workflow, or deploy surface.
_Avoid_: compliance verdict, signoff, status

**Decision Ledger**:
The canonical materialized decision record derived by replaying the selected event store substrate. Durable write authority remains the event store substrate.
_Avoid_: UI page, hosted cache, dashboard database

**Ledger View**:
The human-facing surface for inspecting Decision Ledger state and emitting review commands. It is not durable authority.
_Avoid_: Decision Ledger, source of truth

**Governance policy**:
Configurable rules that decide how candidates, review commands, and evidence route to review, advisory state, materialization, or enforcement according to workspace capability.
_Avoid_: connector logic, model prompt, fixed org-chart role

**GovernanceResult**:
A substrate-neutral outcome of governance or conflict analysis. It can express blocking, warning, or informational intent; each substrate maps it to honest enforcement channels.
_Avoid_: CI result only, dashboard warning only

**EventStoreAdapter**:
The substrate-specific materialization boundary for accepted governance events. Adapters append and replay canonical event entries for git, drive-folder, in-memory, or future substrates; callers must not bypass them with direct canonical writes.
_Avoid_: generic persistence helper, cache writer, direct decision writer

**ModManifest**:
A declarative configuration artifact for EM-safe mods: triggers, filters, and allowed evidence-producing actions. A manifest can shape candidate/routing/advisory behavior, but it cannot approve signoff, resolve compliance, or grant itself new authority.
_Avoid_: plugin with arbitrary code authority, policy override, approval script

**Signoff**:
The ownership lifecycle on a Decision. Approval is separate from candidate acceptance and separate from code compliance.
_Avoid_: status, compliance, drift, ratification

**Status / compliance state**:
The code-compliance state for a decision. It is computed or reviewed from grounding and drift evidence, not hand-authored as signoff.
_Avoid_: signoff, approval

**Read/write path**:
Review surfaces, MCP tools, integrations, and mods emit substrate-neutral commands/evidence. Governance policy and event store adapters decide materialization.
_Avoid_: UI writes YAML, connector writes canonical decisions directly

**Local daemon**:
The local Bicameral core that validates protocol objects, evaluates governance policy, preserves audit state, performs local grounding, and materializes accepted events through storage adapters.
_Avoid_: hosted daemon, cloud oracle

**Gateway**:
The local edge boundary that adapts operational sources into typed protocol objects before they enter daemon governance.
_Avoid_: canonical writer, source of truth

**Protocol folder**:
The in-repo `protocol/` module owned by `bicameral-bot`, containing shared object schemas and conformance fixtures used by bot, MCP, integrations, and cloud clients.
_Avoid_: separate protocol service, separate source-of-truth repo
