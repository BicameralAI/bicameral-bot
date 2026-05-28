# ADR-0001: Local Daemon and Gateway

**Date:** 2026-05-27  
**Status:** proposed  
**Level:** L1

## Problem

Bicameral must ingest from many messy operational edges without letting each source decide what is canonical. Teams differ by Jira/Linear/Slack/meeting/GitHub usage, privacy boundary, repo topology, and review culture.

## Decision

Use a public local-first runtime with a daemon + gateway shape. The gateway adapts edge inputs into typed Bicameral protocol objects. The local daemon validates objects, evaluates governance policy, preserves audit state, performs local grounding, and materializes accepted events through storage adapters.

## Non-Goals

Source-specific adapters belong in `bicameral-integrations`. Hosted code graph behavior belongs in `bicameral-cloud`.

## Consequences

Bicameral can meet teams at their operational substrate while preserving one authority path.
