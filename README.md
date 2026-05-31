<p align="center">
  <h1 align="center">Bicameral Bot</h1>
</p>

<p align="center">
  Local agent for software teams, optimized for spec alignment and grounded in code.
</p>
<p align="center">
  Assist teams curate portable product context <a href="https://uor.foundation/">without vendor lock-in</a>.
</p>
<p align="center">
  <a href="https://github.com/BicameralAI/bicameral-bot">bot</a>&nbsp; • &nbsp;
  <a href="https://github.com/BicameralAI/bicameral-mcp">mcp</a>&nbsp; • &nbsp;
  <a href="https://github.com/BicameralAI/bicameral-integrations">integrations</a>&nbsp; • &nbsp;
  <a href="https://github.com/BicameralAI/bicameral-cloud">cloud</a>
</p>

---

## Why Bicameral Bot

Software teams do not usually lose because nobody wrote anything down. They lose because product intent, architectural constraints, dependency risk, and implementation evidence drift apart while work is happening.

The decision is in a Jira ticket. The real rationale is in a meeting. The dependency risk is in an EM's head. The implementation evidence is in a branch. The agent coding the feature sees only a slice of that context, then confidently proceeds.

That is cognitive debt: unresolved interpretation work accumulating until the team has to repay it through rework, surprise reviews, stale specs, phantom blockers, or yet another meeting to reconstruct what everyone meant.

Bicameral Bot works backwards from that failure mode. It is a local agent that sits at the team's trust boundary, captures implementation-constraining decisions from operational sources, grounds them in the current code workspace, and routes ambiguity to the right human before any claim becomes authority.

## The Shape

Bicameral Bot follows the OpenClaw-style shape: a local daemon plus a gateway.

The gateway is expressive. It meets teams where their cognitive debt already lives: Jira, Linear, Slack, GitHub, meetings, support threads, ADRs, docs, repos, and EM-authored mods.

The daemon is boring. It validates typed objects, keeps review state, applies governance policy, preserves audit trails, grounds claims in local code, and materializes accepted events through storage adapters.

That split is the product thesis:

```text
Expressiveness at the edge.
Determinism in the middle.
Governance decides what becomes canonical.
```

## Quick Start

This repository is currently the public product/runtime boundary and architecture seed. The implementation will grow around the ADRs in `docs/adr/` and the protocol contracts in `protocol/`.

Target local flow:

```bash
git clone https://github.com/BicameralAI/bicameral-bot.git
cd bicameral-bot
bicameral init
bicameral connect github --repo .
bicameral dashboard
```

Target agent flow:

```text
@bicameral preflight this branch against current decisions
@bicameral ingest the Jira ticket and route ambiguous claims for review
@bicameral bind this candidate to the deploy pipeline code path
```

Target EM flow:

```text
/add-jira-dependency-risk
/configure-governance
/test-mod mods/jira-dependency-risk --fixture fixtures/jira-issue.json
```

The important promise is not the exact command spelling. It is the operating model: install locally, connect only the sources you want, let agents propose candidates/evidence, and require governance before canonical state changes.

## Philosophy

**Grounded, not just documented.** Bicameral is not a note-taking system for decisions. It links decisions to source excerpts, code paths, diffs, symbols, dependencies, deploy surfaces, and review events.

**Local first by default.** The bot runs at the team's trust boundary. Free/public Bicameral should still inspect the current repo/worktree and produce useful `BindingEvidence`. Paid cloud adds organization-scale graph intelligence; it does not replace local authority.

**HITL is debt control.** Bicameral bridges two probabilistic domains: human operational interpretation and code/workflow interpretation. Human review is not a fallback for model failure. It is the mechanism that prevents probabilistic interpretation from becoming unreviewed authority.

**Separate the confidence surfaces.** Extraction confidence answers “is this actually a decision?” Binding confidence answers “is this the right code/dependency/scope evidence?” Compliance confidence answers “does implementation satisfy or violate the decision?” Collapsing these into one score creates cognitive debt.

**Skills over feature sprawl.** The base bot should ship the daemon, gateway, protocol, governance machinery, and local grounding. Source-specific integrations and governance recipes should be installed/configured through skills so each team gets the workflow it actually needs.

**EMs should be able to configure the operating system.** Engineering managers should not need to become platform engineers to express: “SOC2 mentions are high-risk,” “deploy pipeline changes need platform review,” or “Jira dependency-risk signals should route to me before implementation starts.” Bicameral should ship skills that help EMs configure integrations, routing, review roles, confidence surfaces, and governance modes safely.

**Mods emit evidence, not authority.** EM-authored mods can create candidates, dependency signals, binding hints, routing hints, and advisory warnings. They cannot directly write canonical decisions, approve signoff, resolve compliance, or create blocking CI results.

**Predictable beats clever.** Canonical decisions should be replayable, reviewable, and auditable. For git-backed workspaces, accepted decisions materialize as text under `.bicameral/decisions/`. Caches, dashboards, and hosted graphs are rebuildable views.

**Survey-informed defaults.** Bicameral's ADRs should be informed by public developer surveys and marketplace evidence, not only internal taste. Current public data points to fragmented decision surfaces (Jira, Confluence, Markdown, Notion, GitHub Discussions, Slack/Teams), strong AI adoption paired with codebase-context/trust concerns, and buyer pressure for security and fast ROI. Git can be the boring default for repo-centric teams while the architecture remains substrate-neutral. Local grounding should be useful before paid cloud; hosted graph intelligence should earn its place through cross-repo, historical, and conflict-analysis value. See [Public Developer Survey Implications](docs/research/public-developer-survey-implications.md).

## What It Supports

Planned public/local capabilities:

- **Decision capture** — turn tickets, meeting excerpts, Slack threads, GitHub issues, PRs, and agent sessions into reviewable `DecisionCandidate` objects.
- **Local code grounding** — inspect the current workspace to produce `BindingEvidence` for files, symbols, diffs, paths, dependencies, and deploy/release surfaces.
- **Preflight** — warn agents and developers about relevant decisions before implementation goes too deep.
- **Review routing** — send product meaning, dependency/scope risk, and code evidence to the right reviewer instead of dumping every uncertainty into one queue.
- **Decision Ledger** — materialize reviewed authority through the selected event store substrate, with git-backed `.bicameral/decisions/N.yaml` as the predictable first-class path.
- **Gateway integrations** — accept typed objects from `bicameral-integrations`, MCP sessions, manual fixtures, and future source adapters.
- **EM-safe mods** — let engineering managers configure lightweight domain-specific extraction/routing/governance behavior without bypassing the core.
- **Cloud advisory mode** — optionally query Bicameral Cloud for cross-branch, cross-repo, historical, and high-precision code graph intelligence.

## Skills for Engineering Managers

Bicameral Bot should ship with skills that make governance configuration conversational and safe.

Examples:

```text
/configure-integration jira
/configure-governance deploy-risk
/create-mod dependency-risk
/test-mod soc2-security-review
/explain-governance-result DEC-0042
```

A good skill should ask the EM questions like:

1. What source are you extending: Jira, Linear, Slack, GitHub, meetings, support, docs, or MCP sessions?
2. What signal do you want: decision candidate, dependency risk, scope creep, grounding hint, compliance concern, or reviewer routing?
3. Which owner lens applies: PM, EM, Dev, security, platform, shared?
4. Which review is required before authority changes?
5. Can this ever auto-ratify, or is it advisory-only?
6. Which source evidence must be preserved?
7. Which confidence surfaces apply: extraction, binding, compliance?
8. What should happen in git-backed workspaces?
9. What should happen in dashboard-only or cloud-assisted workspaces?

The output should be boring files, not invisible magic:

```text
mods/<mod-id>/mod.yaml
mods/<mod-id>/prompt.md
mods/<mod-id>/fixtures/example.json
mods/<mod-id>/README.md
```

## Architecture

```text
External sources / agents / mods
Jira  Linear  Slack  GitHub  meetings  support  MCP sessions
        │
        ▼
┌──────────────────────────────────────────────────────────────┐
│ bicameral-bot                                                │
│                                                              │
│  ┌─────────────┐     ┌────────────────────────────────────┐  │
│  │ Gateway     │────▶│ Local daemon/core                  │  │
│  │ adapters    │     │ validation + governance + audit    │  │
│  └─────────────┘     └───────────────┬────────────────────┘  │
│                                      │                       │
│  ┌─────────────┐     ┌───────────────▼────────────────────┐  │
│  │ protocol/   │◀───▶│ Local grounding + Review UX        │  │
│  │ schemas     │     │ BindingEvidence + ReviewCommand    │  │
│  └─────────────┘     └───────────────┬────────────────────┘  │
│                                      │                       │
│                         ┌────────────▼─────────────┐         │
│                         │ Event store adapter       │         │
│                         │ .bicameral/decisions YAML │         │
│                         └───────────────────────────┘         │
└──────────────────────────────────────────────────────────────┘
          │ optional advisory query
          ▼
 Bicameral Cloud: hosted code graph / conflict oracle
```

Local bot responsibility:

- gateway intake
- protocol validation
- review state
- governance policy
- local code grounding
- Decision Ledger materialization
- audit trail
- skill-installed integration/mod configuration

Cloud responsibility:

- hosted code graph
- cross-branch and cross-repo analysis
- expensive indexing
- blast-radius analysis
- conflict prediction
- grounding accuracy/latency optimization

Cloud returns evidence and advisories. Local/team governance decides enforcement.

## Repository Layout

```text
├── protocol/                # Shared schemas and conformance fixtures
│   ├── README.md
│   └── schemas/
├── docs/adr/                # Architecture decisions owned by the bot runtime
├── CONTEXT.md               # Project glossary and resolved terms
└── README.md                # You are here
```

## Related Repositories

- [`bicameral-mcp`](https://github.com/BicameralAI/bicameral-mcp) — agent-facing MCP tool surface. Emits protocol-shaped commands/evidence into the bot.
- [`bicameral-integrations`](https://github.com/BicameralAI/bicameral-integrations) — source adapters and EM-safe mods. Emits candidates/evidence/signals into the gateway.
- [`bicameral-cloud`](https://github.com/BicameralAI/bicameral-cloud) — private hosted code graph and conflict oracle. Returns evidence/advisories, not authority.
- [`bicameral-daemon`](https://github.com/BicameralAI/bicameral-daemon) — legacy/private architecture staging repo and current gold-standard documentation structure.

## Protocol Contracts

The protocol lives in this repo under `protocol/` rather than in a separate repository. The bot owns protocol compatibility because it is the local authority boundary that all public edge surfaces enter.

Core object families:

- `SourceEvidence`
- `DecisionCandidate`
- `Decision`
- `BindingHint`
- `BindingEvidence`
- `DependencySignal`
- `ReviewCommand`
- `ReviewEvent`
- `GovernanceResult`
- `ConflictSignal`
- `GroundingSuggestion`

## FAQ

**Is Bicameral Bot just an ADR generator?**

No. ADRs are one possible output. The core product is a grounded governance loop: evidence → candidate → review → materialization → preflight/enforcement/advisory feedback.

**Why local first?**

Because the most sensitive context is often local: code, branches, customer details, Slack excerpts, support threads, and implementation plans. A local bot lets teams start with useful grounding without outsourcing authority to a hosted control plane.

**Why daemon + gateway?**

Every team's edge is different, but authority must be predictable. The gateway can be customized aggressively. The daemon/core should stay boring, typed, audited, and deterministic.

**What is paid, then?**

Scale and optimization: hosted organization-wide code graph, cross-branch/cross-repo conflict detection, historical indexing, blast-radius analysis, and better grounding recall/latency. Free/local Bicameral remains code-grounded.

**Can an EM configure governance without writing production code?**

That is the goal. Bicameral should ship skills that help EMs configure integrations, manifests, review roles, confidence surfaces, source trust, and governance modes. The outputs are files and policies that the bot can validate.

**Can a mod block a merge?**

Not directly. A mod can emit a warning or dependency signal. Blocking requires governance policy, reviewed evidence, and a substrate that can honestly enforce the result.

## Status

This repo is currently architecture-first. The README, `CONTEXT.md`, `protocol/`, and ADRs define the product boundary that implementation should follow.
