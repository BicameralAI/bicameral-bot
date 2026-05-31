# Minimal Dependency / Collision Check

## Purpose

This note records a deferred implementation slice discovered while shaping the
dashboard v0.1 RFQ.

The Ingestion Gate promotes remaining `DecisionCandidate`s into
`Decision(signoff.state = proposed)`. That proposed state is useful only if a
follow-up dependency/collision check can decide whether the Decision can advance
to `approved` or must stop at `collision_pending`.

Dashboard v0.1 may ship before this check is implemented, but it must not
pretend proposed Decisions are approved. If the check is absent, proposed
Decisions remain visibly proposed.

## Target Behavior

After candidate promotion:

```text
DecisionCandidate
  -> Ingestion Gate Ingest
  -> Decision(signoff.state = proposed)
  -> dependency/collision check
  -> approved | collision_pending | proposed
```

The check should be deterministic and local-first for v0.1.

## Minimal v0.1 Check

Inputs:

- the newly promoted Decision;
- existing Decisions in the selected event store substrate or current local
  materialized view;
- source refs and feature/scope hints when available;
- optional explicit `conflicts_with` metadata when available.

Suggested first rules:

1. If explicit conflict metadata points at an active Decision, set
   `signoff.state = collision_pending`.
2. If normalized summary plus feature/scope strongly duplicates an active
   Decision, set `signoff.state = collision_pending` or require review.
3. If no conflict rule fires and workspace policy allows auto-approval after
   the check, transition to `approved`.
4. If the check cannot run because dependencies are unavailable, leave
   `signoff.state = proposed`.

Non-goals:

- hosted code graph conflict analysis;
- semantic LLM conflict judgment;
- cross-repo dependency analysis;
- CI blocking;
- policy language beyond what ADR-0007 already requires.

## Dashboard Implication

Until this check exists, the dashboard should display proposed Decisions as
pending dependency/collision review. It should not auto-approve them solely
because `Ingest` completed.

When this check exists, Ledger View should make `approved`, `collision_pending`,
and still-`proposed` outcomes visible.
