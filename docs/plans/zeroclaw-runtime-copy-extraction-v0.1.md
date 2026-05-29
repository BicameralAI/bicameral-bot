# Plan: Bicameral Runtime Copy Extraction from ZeroClaw v0.1

## 1. Summary

This plan describes the implementation path for the first runnable `bicameral-bot` local runtime: selectively copying and adapting relevant ZeroClaw source into first-party Bicameral code.

This implements ADR-0009's runtime-scaffold decision, with one important product-shape change: the final product repository should not depend on ZeroClaw as a linked submodule/subrepo. ZeroClaw may be used as the upstream source during development, but the deliverable is a copied, renamed, narrowed Bicameral runtime with preserved license attribution and no gitlink dependency.

The goal is not to ship a general assistant. The goal is to get a boring local daemon/gateway/CLI skeleton that can validate Bicameral protocol-shaped inputs, queue review work, enforce governance boundaries, record audit receipts, and run EM-safe mod fixtures.

Core invariant:

```text
Edges are expressive.
Core is boring.
Governance decides what becomes canonical.
```

## 2. Background

`bicameral-bot` is currently architecture-first. Its ADRs define a local daemon/gateway shape, an authority boundary, and a substrate-neutral governance flow, but the repository does not yet have a runnable local runtime.

ADR-0009 identified ZeroClaw as a useful scaffold because it already contains local-first runtime machinery: CLI/service lifecycle, config loading, gateway, provider/tool abstractions, approval gates, audit concepts, SOP-style workflows, plugin surfaces, and a Rust workspace split around API/config/runtime/gateway/plugins/providers/memory/tools.

ADR-0009 originally described ZeroClaw as a submodule under `third_party/zeroclaw`. For this implementation, prefer a copied first-party extraction instead of retaining a linked subrepo. The copy must be selective, renamed, licensed, and constrained by Bicameral's protocol/governance model.

Relevant architecture docs:

- `docs/adr/0001-local-daemon-gateway-and-event-store-substrates.md`
- `docs/adr/0002-agent-surfaces-and-bot-runtime-interface.md`
- `docs/adr/0005-separate-event-store-ingestion-review-and-policy.md`
- `docs/adr/0007-substrate-neutral-governance-flow.md`
- `docs/adr/0009-adopt-zeroclaw-runtime-scaffold.md`
- `runtime/README.md`

Upstream source observed at ADR-0009 import time:

```text
https://github.com/zeroclaw-labs/zeroclaw
commit: cbf915d43a3c43116d63c122732942cf8782ff16
license: MIT OR Apache-2.0
```

## 3. Problem Statement

A submodule keeps upstream code visibly separate, but it is the wrong long-term product shape for the public bot runtime:

1. It makes `bicameral-bot` depend on a broad general-assistant repository during normal checkout/build.
2. It exposes too much unrelated assistant surface area near Bicameral's public runtime boundary.
3. It delays the rename/narrowing work required before publishing first-party Bicameral crates and commands.
4. It can confuse authority boundaries: ZeroClaw autonomy and assistant affordances must not become Bicameral governance authority.

We need a selective extraction that preserves the useful local runtime mechanics while making Bicameral's runtime, commands, package names, docs, tests, and license metadata first-party and product-specific.

## 4. Goals

This project should deliver:

1. A Rust workspace skeleton for `bicameral-bot` with copied/adapted runtime crates under Bicameral-owned names.
2. A `bicameral` CLI with narrow v0.1 commands: `init`, `ingest`, `preflight`, `review`, `mod validate`, `mod run`, `gateway start`, `service status`, and `doctor` where feasible.
3. A local config shape rooted in `.bicameral/` and/or a user-level Bicameral config path, not ZeroClaw's assistant config namespace.
4. A local gateway skeleton that can accept fixture/file-first ingest and future HTTP ingress without claiming canonical authority.
5. Governance/approval/audit scaffolding that records candidates, commands, receipts, and policy outcomes without directly writing canonical decisions except through an explicit event-store adapter path.
6. EM-safe mod manifest validation and fixture runner scaffolding.
7. License/NOTICE preservation for copied ZeroClaw source.
8. Removal of the ZeroClaw git submodule dependency from the final product diff.
9. Documentation that explains what was copied, what was rewritten, what was intentionally excluded, and how future upstream syncs should be performed manually.

## 5. Non-Goals

This project should not deliver:

- A wholesale ZeroClaw rename.
- A general personal assistant, chat bot, or broad channel runtime.
- Telegram, Discord, Slack, email, WhatsApp, hardware, voice, browser, shell-agent, cron, RAG, or broad memory features unless a narrow module is required for the runtime skeleton.
- Production hosted cloud integration.
- Production event-store materialization beyond a minimal explicit adapter seam.
- Automatic creation of canonical decisions.
- Direct `.bicameral/decisions/*.yaml` writes from ingest, mods, gateway handlers, MCP, dashboard, or generic tools.
- CI/merge blocking unless the selected substrate actually provides that boundary and policy explicitly permits it.
- Autonomous external actions.
- Retaining ZeroClaw's CLI command names, config namespace, dashboard branding, or personal-assistant language.

## 6. Users / Consumers

Primary consumers:

- Bicameral developers implementing the first runnable local bot runtime.
- Integration and MCP authors who need a local endpoint/CLI target for protocol-shaped objects.
- EMs testing declarative mods against fixtures.

Secondary consumers:

- Product/architecture reviewers checking that copied scaffold code does not import unwanted assistant authority.
- Future maintainers performing manual upstream comparison or selective syncs.
- Contractors estimating a scoped runtime extraction rather than a full agent-platform port.

## 7. Required Product Boundary

All copied/adapted code must serve this Bicameral flow:

```text
source/mod/MCP/CLI input
  → protocol-shaped object or ReviewCommand
  → bot validation
  → governance policy
  → review state / audit receipt / GovernanceResult
  → event store adapter materialization only when accepted
  → replayed/materialized state
```

No copied module may introduce a parallel canonical path.

### 7.1 Pilot team shape

The first pilot should be a fully runnable local runtime for developers on a team, not only a single-user fixture CLI. For operational simplicity, v0.1 may ship daemon and gateway in one process, but the product behavior must already point at the final team shape:

```text
Developer A local daemon/gateway ┐
Developer B local daemon/gateway ├── shared Decision Ledger via selected event-store substrate
Developer C local daemon/gateway ┘
```

Pilot users should be able to initialize or connect to a shared workspace, start the local daemon/gateway, inspect the same replayed ledger state, submit source evidence/candidates/review commands, and materialize accepted events only through the selected event-store adapter.

Keep both git-backed and Google Drive-backed event-store substrates in architectural scope for v0.1. Git may be the most complete first adapter for repo-centric pilots, but the implementation must not hardcode git as the ontology. A Drive-backed/shared-folder adapter path must remain part of the runtime contract so subsequent work can support teams whose source of decision authority is Drive or docs-first rather than PR-first.

If full Google Drive API authentication is too large for the first runtime extraction, implement the Drive substrate as a configured local folder compatible with Drive sync plus explicit freshness/offline states. Do not replace the substrate-neutral interface with git-specific assumptions.

Forbidden effects for v0.1:

- writing `.bicameral/decisions/*.yaml` from an ingest handler;
- treating a mod result as an accepted `Decision`;
- treating an advisory `GovernanceResult` as blocking;
- collapsing extraction, binding, compliance, and signoff into one confidence or status;
- allowing a plugin/mod/tool to approve its own authority expansion;
- reading ZeroClaw config as Bicameral config without a deliberate migration/adaptation layer.

## 8. Source Copy Strategy

### 8.1 Final repository shape

The final implementation should use Bicameral-owned crates and modules. Suggested layout:

```text
Cargo.toml
Cargo.lock
src/
  main.rs
  lib.rs
crates/
  bicameral-api/
  bicameral-config/
  bicameral-runtime/
  bicameral-gateway/
  bicameral-providers/        # optional minimal provider abstraction
  bicameral-mods/             # declarative v0.1 mod manifests and fixture runner
  bicameral-audit/            # optional, if audit/receipt code is split out
runtime/
  README.md
  UPSTREAM-ZEROCLAW.md
NOTICE
LICENSE-MIT
LICENSE-APACHE
```

Alternative crate names are acceptable if the implementation notes explain why, but no public crate, binary, module, config path, or command should retain `zeroclaw` naming except in attribution docs and comments identifying copied source origin.

### 8.2 Upstream source areas to inspect

Start from these upstream areas:

```text
third_party/zeroclaw/Cargo.toml
third_party/zeroclaw/src/main.rs
third_party/zeroclaw/src/lib.rs
third_party/zeroclaw/crates/zeroclaw-api/
third_party/zeroclaw/crates/zeroclaw-config/
third_party/zeroclaw/crates/zeroclaw-runtime/src/approval/
third_party/zeroclaw/crates/zeroclaw-runtime/src/daemon/
third_party/zeroclaw/crates/zeroclaw-runtime/src/security/
third_party/zeroclaw/crates/zeroclaw-runtime/src/service/
third_party/zeroclaw/crates/zeroclaw-runtime/src/sop/
third_party/zeroclaw/crates/zeroclaw-runtime/src/observability/
third_party/zeroclaw/crates/zeroclaw-gateway/
third_party/zeroclaw/crates/zeroclaw-plugins/
third_party/zeroclaw/crates/zeroclaw-log/
third_party/zeroclaw/crates/zeroclaw-infra/
third_party/zeroclaw/docs/book/src/security/tool-receipts.md
third_party/zeroclaw/docs/book/src/security/autonomy.md
third_party/zeroclaw/docs/book/src/sop/index.md
third_party/zeroclaw/docs/book/src/gateway/api.md
third_party/zeroclaw/LICENSE-MIT
third_party/zeroclaw/LICENSE-APACHE
third_party/zeroclaw/NOTICE
```

### 8.3 Copy/adapt mapping

| ZeroClaw area | Bicameral target | v0.1 treatment |
|---|---|---|
| `zeroclaw-api` traits | `bicameral-api` | Copy/adapt only traits needed for provider, tool/mod, gateway/runtime adapter, audit observer, and protocol validation seams. Remove hardware/channel/general memory traits unless used by the narrow runtime. |
| `zeroclaw-config` | `bicameral-config` | Copy/adapt config loading, validation, path handling, env overrides, and secret redaction. Rename config fields to Bicameral concepts: workspace, gateway, governance, event_store, providers, mods. |
| CLI in `src/main.rs`/`src/lib.rs` | `src/main.rs`/`src/lib.rs` | Do not port the large command surface. Rebuild a minimal `bicameral` command enum and route only v0.1 commands. |
| `zeroclaw-runtime/src/daemon` | `bicameral-runtime` | Copy/adapt daemon lifecycle and health patterns only. Remove agent-loop behavior that implies general autonomy. |
| `zeroclaw-runtime/src/service` | `bicameral-runtime` or `bicameral-service` | Copy/adapt service install/status patterns if portable and narrow. Avoid platform-specific complexity that delays a runnable local binary. |
| `zeroclaw-runtime/src/approval` | `bicameral-runtime::governance` | Reframe approvals as governance/review gates. Commands produce `ReviewCommand`, `ReviewEvent`, and `GovernanceResult`; they do not approve broad autonomy. |
| `zeroclaw-runtime/src/security` | `bicameral-runtime::policy` | Copy/adapt policy primitives that help enforce workspace boundaries, denied authority paths, and receipt checks. Replace autonomy language with governance language. |
| `zeroclaw-runtime/src/sop` | `bicameral-mods` or `bicameral-runtime::workflows` | Copy/adapt deterministic procedure/fixture execution concepts for declarative EM-safe mods. Executable plugin support is out of scope for v0.1 unless behind an off-by-default feature. |
| `zeroclaw-gateway` | `bicameral-gateway` | Copy/adapt HTTP/router/server lifecycle and typed handler patterns. Strip unrelated API endpoints and dashboard assets. Add fixture-first `/v0/ingest`-compatible seam if protocol schemas exist. |
| `zeroclaw-plugins` | `bicameral-mods` | Prefer declarative YAML mod manifests first. Only copy WASM/plugin host code if needed as an internal future-disabled module with no default runtime exposure. |
| `zeroclaw-log` | `bicameral-audit` or `bicameral-runtime::audit` | Copy/adapt structured logging/receipt concepts for audit receipts. Ensure receipt IDs/provenance map to Bicameral objects. |
| `zeroclaw-infra` | `bicameral-runtime` support modules | Copy only small reusable infrastructure needed by selected modules. Avoid importing broad session/runtime machinery by default. |
| Docs/license files | `runtime/UPSTREAM-ZEROCLAW.md`, `NOTICE`, licenses | Preserve source origin, commit, copied paths, license terms, and manual-sync instructions. |

### 8.4 Explicit exclusions

Do not copy into the v0.1 runtime unless the implementation plan separately justifies and gates them:

```text
crates/zeroclaw-channels/
crates/zeroclaw-hardware/
crates/zeroclaw-memory/
crates/zeroclaw-tools/ shell/browser/file tools
crates/zeroclaw-tui/ full onboarding wizard
crates/zeroclaw-providers/ full provider catalog
crates/zeroclaw-runtime/src/agent/
crates/zeroclaw-runtime/src/cron/
crates/zeroclaw-runtime/src/rag/
crates/zeroclaw-runtime/src/skills/ general assistant skills
crates/zeroclaw-runtime/src/subagent/
crates/zeroclaw-runtime/src/tunnel/
apps/tauri/
firmware/
web/ dashboard assets except minimal copied stubs if needed
marketplace/
```

If a copied module depends on an excluded module, prefer replacing the dependency with a Bicameral stub/interface over widening the copy.

## 9. Scope of Work

### 9.1 Workspace and binary setup

Create a Rust workspace and `bicameral` binary that builds from a clean checkout.

Minimum commands:

```bash
bicameral --help
bicameral init --dry-run
bicameral doctor
bicameral gateway start --help
bicameral ingest --help
bicameral preflight --help
bicameral review --help
bicameral mod validate --help
bicameral mod run --help
```

The first implementation may leave some commands as explicit `not yet implemented` stubs, but command help, routing, and authority descriptions must be present and tested.

### 9.2 Config extraction

Define a Bicameral config model. Suggested first shape:

```toml
[workspace]
root = "."
trust_boundary = "local"

[gateway]
host = "127.0.0.1"
port = 0

[governance]
mode = "auto_candidate_manual_approve"

[event_store]
kind = "git"
path = ".bicameral"

[providers]
default = "manual-fixture"

[mods]
paths = ["mods"]
```

Config requirements:

- workspace-local config must not be confused with canonical decision state;
- secrets must not be written into examples or canonical event content;
- config validation should fail closed when a field would grant stronger authority than supported;
- config docs must explain which fields are source of truth for runtime behavior.

### 9.3 Protocol validation seam

If `protocol/` schemas from the ingest integration spec exist, wire the runtime/gateway/CLI to validate example ingest payloads through them.

If schemas do not exist yet, create an interface and a minimal placeholder validator that clearly returns `unsupported_schema` or `not_implemented` without accepting arbitrary payloads as valid.

The runtime must keep these object families distinct:

- `SourceEvidence`
- `DecisionCandidate`
- `BindingHint`
- `BindingEvidence`
- `DependencySignal`
- `ReviewCommand`
- `ReviewEvent`
- `GovernanceResult`

### 9.4 Governance and approval gates

Implement a minimal policy evaluator or scaffold that can return structured `GovernanceResult` values for fixture inputs.

Required v0.1 behavior:

- candidate creation may be queued;
- candidate acceptance requires an explicit `ReviewCommand` path;
- signoff approval is separate from candidate acceptance;
- binding/compliance remain separate axes;
- advisory results cannot become blocking unless policy and substrate capability both allow it;
- mods cannot emit canonical decisions or direct storage commands.

### 9.5 Audit receipts

Add structured audit receipt scaffolding for:

- ingest validation;
- candidate projection;
- policy evaluation;
- review command submission;
- mod validation;
- mod fixture run;
- gateway request handling.

Receipts must include enough provenance to support review, but must avoid persisting secrets. A receipt may be a local JSONL/log record for v0.1; it is not canonical decision authority.

### 9.6 Gateway skeleton

Implement a local gateway skeleton with:

- loopback default bind;
- explicit config gate for public bind;
- health endpoint;
- ingest endpoint stub or fixture-compatible endpoint;
- structured error responses;
- no unrelated ZeroClaw dashboard/channel/hardware endpoints.

Future target shape:

```http
POST /v0/ingest
Content-Type: application/json
```

Response semantics must avoid ambiguous `accepted: true`. Use staged statuses such as `received`, `validated`, `queued_for_review`, `policy_rejected`, `duplicate_ignored`, or `failed`.

### 9.7 EM-safe mod scaffold

Implement fixture-first declarative mod support:

```text
mods/<mod-id>/mod.yaml
mods/<mod-id>/prompt.md       # optional for later extractor use
mods/<mod-id>/fixtures/*.json
mods/<mod-id>/README.md
```

Minimum commands:

```bash
bicameral mod validate mods/jira-dependency-risk/mod.yaml
bicameral mod run mods/jira-dependency-risk --input fixtures/jira-issue.json
```

Mod output may include candidates, hints, dependency signals, suggested reviewers, and advisory governance results. It must not include canonical `Decision`, blocking `GovernanceResult`, direct storage commands, signoff approval, or compliance resolution.

### 9.8 Event-store adapter seam

Add an explicit adapter interface for materialization, but keep v0.1 narrow.

The interface should make this boundary obvious:

```text
ReviewCommand accepted by policy
  → ReviewEvent
  → event_store_adapter.materialize(review_event)
  → replay/materialized state
```

For v0.1, it is acceptable to implement a no-op or fixture/local-file adapter if it does not claim canonical authority. If a git-backed adapter is implemented, it must write accepted events through an explicit review/materialization path and must not let ingest/mod/gateway handlers write `.bicameral/decisions/*.yaml` directly.

The adapter interface must keep Google Drive-backed/shared-folder materialization in scope. At minimum, the implementation should model substrate capabilities and freshness separately for:

- `git` — reviewable text, branch/PR workflow, optional CI/merge enforcement when available;
- `drive` — replayable event files in a configured Drive-backed folder or local sync directory, stale/offline detection, dashboard/agent warnings, paused approval, and no CI-block claim;
- `fixture` or `noop` — local development/testing only, never represented as shared canonical authority.

The domain lifecycle must be identical across substrates: accepted review commands become accepted events, event-store adapters materialize them, and replay reconstructs Decision Ledger state. Substrate differences affect storage layout, sync/freshness, and enforcement capability; they must not alter the meaning of `DecisionCandidate`, `ReviewCommand`, `ReviewEvent`, `Decision`, `Signoff`, `ComplianceVerdict`, or `GovernanceResult`.

### 9.9 Submodule removal and attribution

The final product diff should:

1. Remove `.gitmodules` if ZeroClaw is the only submodule.
2. Remove the gitlink at `third_party/zeroclaw`.
3. Avoid relying on `git submodule update` for build/test/docs.
4. Add or update `NOTICE` with ZeroClaw attribution.
5. Preserve `LICENSE-MIT` and `LICENSE-APACHE` terms for copied source, or include them in a clearly documented third-party notice area.
6. Add `runtime/UPSTREAM-ZEROCLAW.md` documenting:
   - upstream repository URL;
   - source commit copied from;
   - copied/adapted path map;
   - intentionally excluded areas;
   - license and attribution obligations;
   - manual process for future upstream comparisons/syncs.

## 10. Required Design Decisions

The implementation must follow these decisions unless explicitly challenged in implementation notes.

### 10.1 Copy, do not link

The final deliverable must be self-contained. It may include copied/adapted source files and attribution metadata, but not a required ZeroClaw submodule/subrepo.

### 10.2 Rename before exposing

Public binary names, crate names, command names, config keys, docs, and examples must use Bicameral-owned naming. `zeroclaw` may appear only in attribution, source-origin comments, or upstream sync docs.

### 10.3 Narrow before expanding

Prefer stubs and explicit `unsupported` errors over copying a broad general-assistant subsystem. If a copied feature is not necessary for Bicameral ingest/review/governance/mod flows, exclude it.

### 10.4 Protocol is source of truth for domain objects

Runtime structs may adapt protocol objects, but must not fork the domain vocabulary. If protocol schemas/types exist, use them or generate from one canonical source. If they do not exist, create temporary internal types with TODOs and tests that prevent them from being mistaken for canonical protocol definitions.

### 10.5 Governance language replaces autonomy language

User-facing docs and command output should talk about governance, review gates, policy, evidence, and receipts. Do not expose broad assistant autonomy, YOLO modes, or self-approval language in the Bicameral runtime.

### 10.6 Local-first and fail-closed defaults

Default bind is loopback. Default policy is review-gated. Missing config, unknown source trust, unsupported schema, or missing substrate capability must not be interpreted as permission.

### 10.7 Receipts are not authority

Audit receipts explain what happened. They do not make candidates canonical and do not replace event-store replay.

## 11. Expected Deliverables

The final delivery should include:

1. Rust workspace and `bicameral` binary.
2. Bicameral-owned runtime/config/API/gateway/mod crates or modules.
3. Minimal CLI command surface with tests/snapshots for help output.
4. Config model, examples, docs, and validation.
5. Gateway skeleton with health endpoint and ingest-compatible handler or stub.
6. Governance/policy scaffold returning structured `GovernanceResult` values.
7. Audit receipt scaffold.
8. Declarative mod validation and fixture-run scaffolding.
9. Event-store adapter interface.
10. Removed ZeroClaw submodule dependency.
11. `runtime/UPSTREAM-ZEROCLAW.md` and updated `NOTICE`/license metadata.
12. Updated `runtime/README.md` and, if needed, a follow-up note or ADR amendment explaining that ADR-0009's submodule bootstrap has been converted into copied first-party runtime source.
13. Tests proving forbidden authority paths fail.
14. Short implementation notes explaining tradeoffs, copied paths, rewritten areas, and open questions.

## 12. Acceptance Criteria

The work is accepted when:

- `git submodule status` is not required for build/test of the product runtime.
- `.gitmodules` no longer references ZeroClaw in the final branch.
- A clean checkout can run `cargo test` or the documented equivalent without fetching ZeroClaw as a submodule.
- `bicameral --help` shows Bicameral command names and no ZeroClaw assistant branding.
- `bicameral init --dry-run` reports the files/config it would create without writing canonical decisions.
- `bicameral doctor` reports runtime/config/gateway readiness in local-first terms.
- `bicameral mod validate` rejects a mod that emits canonical `Decision`, signoff approval, blocking governance result, compliance resolution, or direct storage commands.
- Ingest/gateway paths do not accept arbitrary payloads as valid when schemas are missing or unsupported.
- Policy scaffolding keeps extraction, signoff, binding, compliance, source freshness, and enforcement capability as separate axes.
- Audit receipts are produced for at least one fixture ingest or mod run and do not include secrets.
- A test proves an ingest/mod/gateway handler cannot directly write `.bicameral/decisions/*.yaml`.
- A test proves advisory `GovernanceResult` does not become blocking without policy and substrate capability.
- License and NOTICE metadata visibly attribute copied ZeroClaw source.
- `runtime/UPSTREAM-ZEROCLAW.md` names the upstream commit and copied/adapted paths.
- Documentation explains that copied runtime code is a scaffold, not a domain authority decision.

## 13. Testing / Verification

Minimum verification commands should be documented and pass in CI or local review:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
bicameral --help
bicameral doctor
bicameral mod validate <valid-fixture-mod>
bicameral mod validate <invalid-authority-bypass-mod> # must fail
```

If the implementation is staged before full Rust workspace CI exists, provide equivalent scripts and mark incomplete checks clearly in implementation notes.

Required negative tests:

1. Mod emits `Decision` → rejected.
2. Mod emits blocking `GovernanceResult` directly → rejected unless routed through policy with substrate capability.
3. Ingest payload includes top-level generic `confidence` → rejected if protocol schemas are present.
4. Ingest/gateway handler attempts direct canonical write → rejected or impossible by type/module boundary.
5. Public gateway bind without explicit config permission → rejected.
6. Missing source provenance → rejected or queued as invalid, not treated as reviewed evidence.

## 14. Out of Scope / Explicit Rejections

Do not implement:

- direct ZeroClaw submodule retention as the runtime dependency;
- all ZeroClaw crates under new names;
- channel integrations;
- hardware support;
- general-purpose assistant tools;
- broad memory/RAG;
- autonomous scheduled actions;
- model-provider catalog parity;
- WASM plugin execution by default;
- production dashboard UI;
- hosted cloud code graph;
- event-store conflict resolution beyond the interface/stub unless separately scoped;
- automatic candidate acceptance or signoff approval.

## 15. Implementation Notes Format

Implementation notes should include:

1. Understanding of the Bicameral authority boundary.
2. Proposed copy/adapt plan and crate layout.
3. Exact ZeroClaw paths you expect to copy, rewrite, or exclude.
4. File tree you expect to create/change.
5. How you will remove the submodule dependency while preserving attribution.
6. How you will prevent broad assistant/autonomy surfaces from leaking into `bicameral-bot`.
7. Test plan, including negative authority-boundary tests.
8. Rough effort estimate or sequencing notes.
9. Risks or design decisions needing clarification.
10. Optional alternatives you recommend.

## 16. Review Criteria

Review the implementation based on:

- Respect for Bicameral's authority and governance boundaries.
- Minimal, understandable runtime slice.
- Clear copied-source attribution and license handling.
- Removal of submodule dependency without hiding provenance.
- Simplicity of crate layout and command surface.
- Quality of negative tests for forbidden authority paths.
- Ability to build from a clean checkout.
- Avoidance of broad assistant feature sprawl.
- Fit with future protocol, MCP, integration, cloud, and event-store work.

## 17. Preferred First Milestone

Before copying the full selected slice, first submit a branch that proves the pilot runtime shape is operational, not only syntactically scaffolded:

```text
Cargo.toml
src/main.rs
src/lib.rs
crates/bicameral-api/
crates/bicameral-config/
crates/bicameral-runtime/
crates/bicameral-gateway/
crates/bicameral-event-store/       # or equivalent module split if fewer crates are chosen
runtime/UPSTREAM-ZEROCLAW.md
NOTICE
```

Milestone behavior:

```bash
cargo test
bicameral --help
bicameral init --dry-run
bicameral daemon start --port 0
bicameral gateway start --port 0
bicameral doctor
```

This milestone should also include a written path map showing which ZeroClaw modules are planned for copying, which are planned for rewrite/stub, and which are intentionally excluded.

The daemon/gateway may run as a single process for v0.1. It must expose a real health endpoint, report the bound address when using port `0`, write non-secret local runtime state/receipts, and make `doctor` able to detect whether the local runtime is reachable.

The milestone must also prove shared-ledger architecture without boxing future development into git-only assumptions:

- `bicameral init --event-store git --dry-run` shows the git-backed `.bicameral/` layout it would create.
- `bicameral init --event-store drive --path <folder> --dry-run` shows the Drive-backed/shared-folder event layout and freshness metadata it would use.
- runtime config records selected substrate capability separately from governance result semantics.
- replay code is invoked through an event-store adapter trait/interface, not by hardcoded `.bicameral/decisions/*.yaml` reads in gateway/ingest handlers.
- Drive/shared-folder support may be minimal in the first milestone, but it must be represented as a real substrate mode with honest stale/offline/no-CI-block semantics, not merely a TODO comment.

## 18. Open Questions

The implementer may propose answers, but should not silently decide:

1. Should the first copied runtime use separate crates immediately, or start with fewer crates and split after the daemon/gateway seam stabilizes?
2. Should `bicameral-config` use TOML only in v0.1, or support YAML because `.bicameral/` governance policy examples are YAML-heavy?
3. Should the first gateway implement `/v0/ingest` fully, or wait for the ingest protocol schemas to land?
4. Should copied ZeroClaw files retain source-origin comments per file, or is `runtime/UPSTREAM-ZEROCLAW.md` plus `NOTICE` sufficient?
5. How much of the git and Drive-backed event-store adapters should materialize accepted review events in the first milestone, versus sharing a fixture/local-file adapter beneath both substrate modes?
6. Should EM-safe mods support only YAML declarative rules in v0.1, or allow prompt-backed extraction behind an explicit provider config?
