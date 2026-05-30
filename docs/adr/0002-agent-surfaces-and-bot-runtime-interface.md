# ADR-0002: Agent Surfaces and Bot Runtime Interface

**Date:** 2026-05-27  
**Status:** proposed  
**Level:** L1  
**Carries forward:** `bicameral-daemon` ADR-0002 plus the original `bicameral-bot` protocol ownership ADR

## Problem

The original daemon ADR described a split between `bicameral-mcp` as the local agent tool surface and `bicameral-daemon` as hosted/private infrastructure. The repo split changed that shape: `bicameral-bot` is now the public local daemon/gateway/runtime, and `bicameral-mcp` is one client surface that exposes bot capabilities to coding agents.

We still need the same boundary: agent-facing tools can propose, query, and submit review commands, but persistence authority sits behind bot governance and the selected event store adapter.

## Decision

`bicameral-bot` owns the local runtime interface. It exposes protocol-shaped command/query surfaces to:

- `bicameral-mcp` stdio tools;
- CLI/TUI commands;
- local dashboard actions;
- source integrations;
- EM-safe mod runners;
- optional cloud advisory clients.

These surfaces submit `SourceEvidence`, `DecisionCandidate`, `BindingHint`, `DependencySignal`, `ComplianceVerdict`, and `ReviewCommand` objects. The bot validates them, runs governance policy, and delegates accepted materialization to the selected event store substrate.

## MCP Server

`bicameral-mcp` is the coding agent's direct tool surface. It should provide tools such as:

- `bicameral.ingest` — submit source/session evidence or candidates;
- `bicameral.preflight` — surface relevant decisions before implementation;
- `bicameral.bind` — propose or inspect binding evidence;
- `bicameral.review` — submit review commands such as `accept_candidate`, `reject_candidate`, `approve_signoff`, `reject_signoff`, and `resolve_compliance`;
- `bicameral.history` / `bicameral.search` — read replayed/materialized decision state;
- `bicameral.dashboard` — open the local review UI.

MCP may keep helper caches for speed, but it is not canonical authority.

## CLI / TUI

The CLI is the boring operator surface for setup, smoke tests, local runs, fixture-based mod tests, and governance commands:

```bash
bicameral init
bicameral ingest fixtures/source.json
bicameral preflight --repo .
bicameral review accept-candidate DEC-CAND-123
bicameral mod validate mods/jira-dependency-risk/mod.yaml
bicameral mod run mods/jira-dependency-risk --input fixtures/jira-issue.json
```

## Dashboard / Review UX

The dashboard is a review surface, not a database editor. It renders allowed commands and emits `ReviewCommand`s. It must not write substrate-specific files directly.

## Integrations and Mods

Integrations and mods are edge producers. They can create candidates, hints, evidence, dependency signals, suggested reviewers, and advisory governance results. They cannot approve signoff, resolve compliance, or write canonical state directly.

## Protocol Ownership

`bicameral-bot/protocol/` owns the public-local contract vocabulary for bot, MCP, integrations, and cloud clients: schemas, conformance fixtures, object vocabulary, compatibility notes, and command/result contracts.

This replaces the old idea that `bicameral-daemon/protocol/` is the sole internal contract module and also replaces the temporary standalone `bicameral-protocol` repository. `bicameral-protocol` should remain a deprecated pointer until an independent protocol package is justified.

## Interface Rule

All surfaces must follow this path:

```text
surface action
  → protocol-shaped object or ReviewCommand
  → bot validation and governance policy
  → ReviewEvent / GovernanceResult
  → event store adapter materialization, if accepted
  → replayed/materialized state
```

No surface bypasses this path.

## What MCP / Agents Do Not Own

- Event store adapter internals.
- Canonical audit log.
- Direct `.bicameral/decisions/*.yaml` writes.
- Hosted organization-scale code graph.
- Cross-branch conflict advisory service.
- Multi-tenant auth for cloud services.
- Final authority over signoff or compliance unless governance policy explicitly accepts the submitted review command.

## Consequences

The local bot is the authority boundary; MCP stays a focused agent tool surface. Protocol contracts live where the local runtime lives. Integrations and cloud can evolve as clients without owning local authority.

## Rejected Alternatives

- **MCP writes substrate-specific state directly:** rejected because it couples agent UX to git/Drive internals and bypasses governance.
- **A separate protocol repo now:** rejected as premature.
- **All-hosted daemon authority:** rejected for the public bot path because local-first trust boundary and offline usefulness are core product requirements.
