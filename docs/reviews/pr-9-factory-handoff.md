# PR #9 factory handoff: address P1 risks

Source review: `BicameralAI/bicameral-bot` PR #9, `feat: implement zeroclaw-runtime-copy-extraction v0.1`.

## Human review requests to preserve

1. Promote domain entity candidates 1-4:
   - `DecisionCandidate` — already in `CONTEXT.md`; preserve as canonical product language.
   - `GovernanceResult` — already in `CONTEXT.md`; preserve as canonical product language.
   - `EventStoreAdapter` — promoted in `CONTEXT.md` as the materialization boundary.
   - `ModManifest` — promoted in `CONTEXT.md` as the EM-safe declarative mod artifact.
2. Drop candidate 5:
   - `AuditReceipt` should remain implementation vocabulary for the audit crate, not a canonical product/domain entity yet.
3. Keep factory scratch artifacts out of the product diff. This handoff is a product review note, not a factory run bundle.

## P1 risk 1: operational paths bypass the advertised event-store boundary

The PR states that only the `EventStoreAdapter::append()` path materializes canonical state, but `src/commands/ingest.rs` currently writes candidate JSON directly under `paths.events_dir` with `std::fs::write`. That may be acceptable as a pending-candidate store only if the naming and path make it clear that it is not canonical event materialization.

Factory agent instructions:

1. Inspect the intended runtime model before editing:
   - `CONTEXT.md`
   - `docs/adr/0005-separate-event-store-ingestion-review-and-policy.md`
   - `docs/adr/0007-substrate-neutral-governance-flow.md`
   - `crates/bicameral-api/src/event_store.rs`
   - `crates/bicameral-runtime/src/event_store_adapters/*`
   - `src/commands/ingest.rs`
   - `src/commands/review.rs`
2. Decide and implement one of these, preferring the smallest coherent v0.1 fix:
   - Rename/configure the pending candidate location so it is explicitly non-canonical, and add tests/docs proving direct writes are only pending review artifacts; or
   - Route accepted/materialized events through an `EventStoreAdapter` and reserve direct file writes for non-canonical candidate inbox state.
3. Add a regression test or CLI smoke fixture showing that candidate ingestion does not create an accepted canonical decision/event without governance review.
4. Verify with:
   - `cargo fmt --all -- --check`
   - `cargo build --workspace --locked`
   - `cargo test --workspace --locked`
   - `cargo clippy --workspace --locked -- -D warnings`

Acceptance signal: a reviewer can grep for direct writes and clearly distinguish pending candidate/inbox writes from canonical event materialization.

## P1 risk 2: review submission is a placeholder, not a governance transition

`src/commands/review.rs::submit` currently prints that a candidate was submitted for review, but it does not load the candidate, evaluate governance policy, append a review command/result, or change durable review state. This creates a risk that the CLI appears to honor review while no governed transition occurred.

Factory agent instructions:

1. Treat `review submit` as a product behavior gap, not just a UI wording issue.
2. Implement a minimal honest v0.1 path:
   - If true review-state mutation is in scope, load the candidate, evaluate it with `GovernanceEngine`, and persist the resulting review/governance event through the appropriate event-store path.
   - If true mutation is not in scope, rename/reword the command output so it cannot be mistaken for durable review submission, and document the limitation in README/ADR/runtime notes.
3. Add a regression test or command-level smoke test for the selected behavior.
4. Do not allow `review submit` to imply signoff, acceptance, or compliance resolution.

Acceptance signal: command output and persisted state agree; no command claims a governance transition that did not happen.

## P1 risk 3: gateway local-only/auth boundary should fail honestly

The PR body says the gateway has no auth and is acceptable for local-only v0.1. Ensure the implementation keeps that promise operationally.

Factory agent instructions:

1. Inspect `crates/bicameral-gateway/src/routes.rs`, `crates/bicameral-gateway/src/state.rs`, `crates/bicameral-runtime/src/daemon.rs`, and CLI gateway start behavior.
2. Verify the gateway binds only to loopback by default and does not silently expose unauthenticated mutation endpoints on external interfaces.
3. If binding can be configured, add docs and guardrails that make non-loopback unauthenticated use explicit and risky.
4. Add a minimal test or documented smoke check for `/health` and one ingest route.

Acceptance signal: unauthenticated gateway behavior remains explicitly local-only, and any broader bind requires deliberate user configuration.

## P1 risk 4: mods must remain evidence-producing, not authority-granting

`ModManifest` is now promoted as product language, but the implementation must preserve the boundary: mods may emit candidates, routing hints, binding hints, and advisory warnings; they must not approve signoff, resolve compliance, or grant their own authority.

Factory agent instructions:

1. Inspect `crates/bicameral-mods/src/manifest.rs`, `runner.rs`, and `validate.rs`.
2. Add or tighten tests that enumerate allowed `ModAction`/`ModActionType` variants and reject authority-expanding actions.
3. Keep the docs aligned with `CONTEXT.md`: `ModManifest` is declarative and EM-safe, not arbitrary code execution authority.

Acceptance signal: adding an approval/signoff/compliance-resolution mod action would fail validation or tests.

## Process notes for the continuing agent

- Start by restating each human review request as a checklist with planned disposition: implement, defer with explicit limitation, or ask for clarification.
- Ask for clarification before coding if any review request is ambiguous enough that multiple materially different fixes could satisfy it.
- Do not broaden scope into full product hardening. This is a v0.1 PR; fix the review-identified authority-boundary gaps with narrow tests/docs.
- Keep `AuditReceipt` out of canonical domain language for now unless a human reviewer explicitly reverses that decision.
