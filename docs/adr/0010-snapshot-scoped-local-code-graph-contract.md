# ADR-0010: Snapshot-Scoped Local Code Graph Contract

**Date:** 2026-05-30  
**Status:** proposed  
**Level:** L1  
**Extends:** ADR-0003 local code grounding boundary and ADR-0008 protocol ownership

## Problem

Local code grounding must be faithful. A Bicameral code graph answer is only useful if the caller knows which repository snapshot it describes and whether the claim was verified from that snapshot.

Recent MCP failures exposed the missing contract:

- `validate_symbols` can report a match from a local SQLite index built at one effective commit.
- `bind` can then fail because it resolves through `git show` and tree-sitter at a different effective ref or commit.
- The caller sees a green validation result followed by a rejected bind, with no typed way to detect the snapshot mismatch before materialization.

This is the failure shape behind:

- [`bicameral-mcp` issue #334](https://github.com/BicameralAI/bicameral-mcp/issues/334): `validate_symbols` and `bind` disagree on indexed snapshot.
- [`bicameral-mcp` issue #332](https://github.com/BicameralAI/bicameral-mcp/issues/332): `bind` validates at authoritative SHA and rejects symbols introduced on feature branches after `link_commit`.
- [`bicameral-mcp` issue #280](https://github.com/BicameralAI/bicameral-mcp/issues/280): binding must not fall back to imprecise grounding paths that create plausible but wrong decision edges.

The root problem is not a particular storage engine. The root problem is that graph facts, search candidates, validation results, and binding materialization have not carried the same explicit snapshot identity and evidence semantics.

## Decision

Bicameral code graph claims are evidence-bearing claims over a named snapshot.

The local graph system may be incomplete, lazy, or backed by caches, but it must not present stale, approximate, or cross-snapshot evidence as a verified graph fact. When the local system cannot prove a graph claim for the requested snapshot, it returns an explicit non-verified state instead of substituting a best-effort answer.

The local daemon owns local graph indexing and exact local graph validation. MCP, integrations, and other agent-facing surfaces act primarily as clients that request graph/search/validation operations and receive protocol-shaped evidence.

Graph storage may be content-addressed, incremental, lazy, and shared across snapshots. The API contract is snapshot-scoped; it does not require a full physical graph copy per branch or per commit.

## Definitions

### GraphSnapshot

A `GraphSnapshot` identifies the code snapshot against which graph claims are made.

Required identity:

- `graph_snapshot_id`
- `repo_id`
- `commit_sha`
- `index_version`

Required provenance when known:

- `ref_name`
- `merge_base_sha`
- `indexed_at`
- `index_source`: `local_daemon | hosted | imported`

`graph_snapshot_id` may be derived from `repo_id`, `commit_sha`, and `index_version`, or assigned by a graph service that preserves those fields. `commit_sha` is the durable snapshot anchor for git-backed repositories. `ref_name` is provenance and user-facing context, not durable identity, because branch names move.

### Symbol

A `Symbol` is a stable-ish program identity, such as:

- language
- qualified name
- file path
- structural signature, when available

It is not by itself proof that the symbol exists at a particular snapshot.

### SymbolOccurrence

A `SymbolOccurrence` is a symbol observed at a specific `GraphSnapshot`.

It carries:

- `graph_snapshot_id`
- file path
- line/range
- blob or content hash
- structural hash, when available
- parser/resolver evidence metadata

### DecisionBinding

A `DecisionBinding` links a decision or candidate to either a verified `SymbolOccurrence` or a stable symbol identity plus the snapshot where that identity was validated.

It carries:

- decision or candidate id
- `graph_snapshot_id`
- `validated_ref`
- `validated_sha`
- occurrence evidence or stable symbol evidence

## Evidence States

Graph and validation APIs must distinguish evidence state from ranking confidence.

Allowed states:

- `verified`: proven against the requested snapshot.
- `not_found`: the relevant snapshot and required files were inspected, and the symbol or edge is absent.
- `unknown_not_indexed`: required files or neighborhoods have not been parsed or resolved.
- `unknown_stale`: available index data does not match the requested snapshot, parser version, or index version.
- `ambiguous`: multiple occurrences match and the API cannot select one without more caller input.
- `unsupported`: language, file type, generated artifact, or resolver behavior is outside supported exact validation.
- `approximate_candidate`: search or ranking found a possible match, but it is not verified graph evidence.

Only `verified` graph claims may materialize bindings, support compliance conclusions, or produce blocking governance results. `approximate_candidate`, `unknown_*`, `ambiguous`, and `unsupported` results may suggest next steps, but they cannot be promoted into verified `BindingEvidence`.

## Required API Contract

### validate_symbols

`validate_symbols` must validate against a named or resolved `GraphSnapshot`. If the caller does not supply a snapshot, the daemon must resolve one first, usually from the active checkout `HEAD`, and return that resolved identity.

Its response must include:

- evidence state
- `validated_ref`
- `validated_sha`
- `graph_snapshot_id`
- matched `SymbolOccurrence` id when verified
- path/range/blob evidence when verified
- parser or resolver evidence metadata

`validate_symbols` may use search to find candidates, but search output remains `approximate_candidate` until exact validation runs against the requested snapshot.

### bind

`bind` must materialize only verified graph evidence.

It must accept one of:

- a verified occurrence or validation token returned by validation; or
- a symbol/path request plus explicit snapshot identity, which `bind` revalidates before materializing.

Validation tokens must bind to at least `graph_snapshot_id`, `validated_sha`,
validated symbol or occurrence identity, validator identity, evidence state,
issue time, and expiry time. They must be integrity-protected by the issuing
validator, and `bind` must verify the token integrity, snapshot identity, and
`evidence_state = verified` before materializing. Expired, revoked, mismatched,
or non-verified tokens must be rejected or revalidated.

If validation occurred at one snapshot and binding is requested at another, `bind` must reject with a typed snapshot mismatch or revalidate at the requested snapshot. It must not silently reinterpret a successful validation result against a different ref or commit.

### preflight

`preflight` must report the graph snapshot scope used for local code grounding and distinguish verified graph findings from unknown or approximate findings.

Preflight may warn on potential conflicts, but blocking local graph claims require verified evidence over the relevant snapshot.

## Minimum Local Snapshot Set

The local daemon must be exact about what it can see, not broad by default.

The minimum local set is:

- current `HEAD` for the active checkout;
- authoritative base ref when it is locally available;
- merge-base between `HEAD` and authoritative base when running base-vs-HEAD or branch-delta checks.

The local daemon is not responsible for seeing every branch or every developer's private work unless those refs are locally present and the requested snapshot can be resolved.

## Free / Local Product Boundary

Free/local Bicameral includes exact grounding for locally available snapshots:

- current-checkout code grounding;
- validate/bind/preflight against a known local snapshot;
- local drift checks;
- base-vs-HEAD checks when the base ref and merge-base are locally available;
- simple same-symbol or same-occurrence contradiction checks across locally available snapshots;
- explicit unknown states when local visibility or index coverage is insufficient.

Local results must be faithful before they are convenient. A local install may defer indexing, request expansion, or return unknown; it must not pretend to have team-wide branch visibility.

## Hosted / Team Product Boundary

Hosted/team graph services own broader visibility and continuously refreshed multi-ref graph work:

- active PR branch indexing;
- cross-branch conflict detection across developers' branches;
- CI/PR merge-blocking integration;
- team-wide stale branch detection;
- continuously refreshed graph snapshots and branch manifests;
- Conflict Oracle behavior over multiple developers' branches;
- expensive graph-neighborhood expansion, historical analysis, and organization-scale dependency intelligence.

Hosted graph services still return evidence, hints, advisories, and `GovernanceResult` proposals. They do not become durable decision authority; bot governance and the selected event store substrate still decide what materializes.

## Substrate Neutrality

Git commits are snapshot locators for git-backed repositories, not the only possible durable authority.

The graph contract must allow future sources such as hosted graph snapshots, archive snapshots, generated source packages, or non-git substrates to provide equivalent snapshot identity and evidence metadata. Git-specific fields such as `commit_sha`, `ref_name`, and `merge_base_sha` are used when the source is git.

## Consequences

Positive:

- `validate_symbols`, `bind`, and `preflight` become an honest check-then-materialize workflow.
- Callers can detect stale, mismatched, unsupported, or incomplete graph evidence before binding.
- Local/free product value remains real without promising team-wide hosted visibility.
- Storage can stay efficient because snapshot-scoped APIs do not imply a full graph copy per branch.

Negative / risks:

- Protocol and MCP APIs must grow snapshot and evidence-state fields.
- Callers must handle unknown and mismatch responses instead of assuming every query returns a ranked answer.
- Lazy indexing will create cold-query paths where the right answer is "unknown until expanded."
- Parser/resolver limitations become explicit product behavior rather than hidden implementation detail.

## Non-Goals

This ADR does not define:

- the full hosted Conflict Oracle implementation;
- a physical graph database schema;
- a full graph-per-branch storage model;
- parser internals or language-specific resolver behavior;
- search ranking quality;
- a guarantee that free/local installs can see other developers' branches;
- imprecise fallback binding that bypasses exact snapshot validation.

## Follow-up Implementation Issues

1. Add `validated_ref`, `validated_sha`, `graph_snapshot_id`, and evidence state to `validate_symbols`.
2. Add explicit snapshot input or validation-token input to `bind`.
3. Make `bind` reject or revalidate snapshot mismatches instead of silently changing effective refs.
4. Add preflight output fields that distinguish verified graph facts from unknown and approximate candidates.
5. Add protocol conformance fixtures for validate-then-bind on `HEAD`, authoritative base, feature branch `HEAD`, and merge-base snapshots.
6. Define exact lazy expansion behavior for local graph indexing without requiring full per-branch graph materialization.
7. Create parser/resolver accuracy fixtures for supported languages and explicit `unsupported` behavior for unsupported constructs.
8. Define hosted multi-branch graph indexing and Conflict Oracle accuracy policy in separate ADRs or design docs.

## References

- [`bicameral-mcp` issue #334](https://github.com/BicameralAI/bicameral-mcp/issues/334): validate/bind snapshot disagreement.
- [`bicameral-mcp` issue #332](https://github.com/BicameralAI/bicameral-mcp/issues/332): feature-branch symbols rejected by authoritative-SHA bind.
- [`bicameral-mcp` issue #280](https://github.com/BicameralAI/bicameral-mcp/issues/280): imprecise binding regression and the requirement for verified grounding.
- [Git object model](https://git-scm.com/book/en/v2/Git-Internals-Git-Objects): commits point to tree snapshots, making commit SHA a natural git-backed snapshot anchor.
- [Martin Fowler on Architecture Decision Records](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html) and [Thoughtworks on lightweight ADRs](https://www.thoughtworks.com/radar/techniques/lightweight-architecture-decision-records): ADRs should capture context, decision, and consequences without becoming full implementation specs.
- [Code Property Graph specification](https://cpg.joern.io/): code graph literature describes program facts as graph data, but snapshot identity remains a product/API contract Bicameral must define.
