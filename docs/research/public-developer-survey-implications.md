# Public Developer Survey Implications for Current ADRs

**Date:** 2026-05-28
**Status:** sourced evidence memo
**Scope:** ADR-0001, ADR-0003, ADR-0005, ADR-0007, and ADR-0009

This memo records public survey evidence that should inform several current
architecture decisions. The statistics below should not become architecture by
poll. They should shape defaults, product-ordering, and risk framing while the
ADRs preserve stable domain invariants.

## Sources checked

| ID | Source | Statistic / finding used | URL | Retrieved |
|---|---|---|---|---|
| S1 | Stack Overflow Developer Survey 2024 — Technology / asynchronous tools | Among all respondents, regularly used collaborative work-management/code-documentation tools include Jira 51.4%, Confluence 31.6%, Markdown files 29.1%, Notion 18.2%, GitHub Discussions 17.9%, Azure DevOps 16.3%, Obsidian 13%, Linear 2.8%. The chart reports 49,931 responses. | <https://survey.stackoverflow.co/2024/technology/> | 2026-05-28 |
| S2 | Stack Overflow Developer Survey 2024 — Communication tools | Microsoft Teams 53.1%, Slack 43.9%, Zoom 40%, Discord 38.4%, Google Meet 37.2%, WhatsApp 31.3%; 56,109 responses. | <https://survey.stackoverflow.co/2024/technology/> | 2026-05-28 |
| S3 | Stack Overflow Developer Survey 2024 — AI | 76% of all respondents are using or planning to use AI tools in development; current usage increased to 62% from 44%. Top hoped-for AI benefit is increased productivity at 81%. Top team-level blockers include “Don’t trust the output” at 66.2% and “AI tools lack context of codebase” at 63.3%. | <https://survey.stackoverflow.co/2024/ai/> | 2026-05-28 |
| S4 | Atlassian State of Developer Experience Report 2024 / Atlassian blog | Atlassian/DX surveyed 2,100+ developers and managers. Atlassian reports 97% of developers lose significant time to inefficiencies; DevEx is important to 63% of developers when considering whether to stay; two out of three consider leaving when unsatisfied with DevEx. | <https://www.atlassian.com/blog/development/developer-experience-report-2024> | 2026-05-28 |
| S5 | DX/Atlassian press summary | DX/Atlassian study of 900+ developers found 69% lose 8 or more hours each week to inefficiencies. | <https://getdx.com/news/dx-atlassian-report/> | 2026-05-28 |
| S6 | DORA Accelerate State of DevOps Report 2024 | DORA reports AI adoption increases individual productivity, flow, and job satisfaction but negatively impacts delivery stability and throughput; stable organizational priorities and user-centricity are key performance drivers; internal developer platforms can increase productivity but require careful stability monitoring. | <https://dora.dev/research/2024/dora-report/> | 2026-05-28 |
| S7 | JetBrains State of Developer Ecosystem 2024 | Half of developers surveyed work in teams of 2–7; 88% work in teams under 20. Report is based on 23,262 developers after data cleaning. JetBrains also reports 69% have tried ChatGPT for coding, 49% regularly use it; GitHub Copilot has been tried by 40% and regularly used by 26%. | <https://www.jetbrains.com/lp/devecosystem-2024/> | 2026-05-28 |
| S8 | GitHub 2024 developer AI survey | GitHub surveyed 2,000 people on enterprise software development teams across the U.S., Brazil, India, and Germany. Across markets, 30–40% said their organizations actively encourage and promote AI coding tools, while 29–49% said organizations allow them with limited encouragement. | <https://github.blog/news-insights/research/survey-ai-wave-grows/> | 2026-05-28 |
| S9 | G2 2024 Buyer Behavior Report | G2 surveyed 1,900+ B2B decision-makers. 56% purchased AI software in the last 3 months; 57% expect positive ROI within 3 months; 31% consult review sites more often than other sources; 81% consider a vendor's security-breach history. | <https://research.g2.com/2024-buyer-behavior-report> | 2026-05-28 |

## Decisions that should be survey-informed

| ADR | Decision under pressure | What the statistics imply |
|---|---|---|
| [ADR-0001](../adr/0001-local-daemon-gateway-and-event-store-substrates.md) | Should git be the event-store substrate, or only one adapter? | Keep git as the default for repo-centric engineering work, but do not make it the ontology. Stack Overflow's tool data shows developer decisions already span Jira, Confluence, Markdown files, Notion, GitHub Discussions, Azure DevOps, Obsidian, Slack, and Teams (S1, S2). That fragmentation supports ADR-0001's substrate contract: accepted events must be replayable and auditable, while git-backed `.bicameral/` text remains the predictable first-class path for teams whose authority boundary is PR/code review. |
| [ADR-0003](../adr/0003-local-code-grounding-and-cloud-conflict-boundary.md) | What belongs in free/local grounding versus paid hosted graph intelligence? | Local grounding cannot be paid-only. Stack Overflow says 63.3% cite AI tools lacking codebase context as a team adoption challenge and 66.2% cite lack of trust in output (S3). DORA says AI boosts individual productivity but can hurt delivery stability and throughput (S6). Therefore the public bot must inspect the current worktree and produce reviewable `BindingEvidence`; paid cloud should earn its place through cross-repo, cross-branch, historical, and expensive conflict intelligence that local inspection cannot provide. |
| [ADR-0005](../adr/0005-separate-event-store-ingestion-review-and-policy.md) | Should ingestion, review UX, event authority, and governance policy remain separate interfaces? | Yes. The survey evidence shows the decision surface is fragmented: Jira/Confluence/Markdown/Notion/GitHub Discussions/Azure DevOps/Obsidian for async knowledge (S1), Teams/Slack/Zoom/Discord/Meet for communication (S2), and heavy DevEx losses from inefficiencies (S4, S5). This argues for many source connectors, but it also raises false-authority risk. Connectors should emit `SourceEvidence`, `DecisionCandidate`, hints, and signals only; governance policy plus review commands decide authority. |
| [ADR-0007](../adr/0007-substrate-neutral-governance-flow.md) | Should `GovernanceResult` map to multiple enforcement capabilities instead of always becoming CI checks? | Yes. Public data supports a governance flow that spans code and non-code work. Important decisions originate in tickets, docs, chat, meetings, and AI-assisted sessions (S1, S2, S3, S8). DORA's warning about AI tradeoffs and stability (S6) supports review-resolved enforcement, but non-git decisions still need honest capabilities. Keep `enforcement_capability` explicit: `ci_block` for real merge boundaries, `pr_warning`, `dashboard_flag`, `agent_warning`, `slack_notification`, `queued_action`, or `paused_approval` elsewhere. |
| [ADR-0009](../adr/0009-adopt-zeroclaw-runtime-scaffold.md) | Is a local daemon/gateway scaffold the right public product base? | Yes, if narrowed around governance. AI usage is already mainstream or near-mainstream (S3, S7, S8), but developers and buyers remain sensitive to trust, security, context, and ROI (S3, S6, S9). A local daemon/gateway shape fits the trust-boundary requirement: expressive sources at the edge, deterministic validation/governance/audit in the daemon. ZeroClaw's generic assistant surface area should be cut unless it directly supports decision capture, grounding, review, audit, or safe extension. |

## Architectural implications

### 1. Git should be the onboarding default, not the domain model

The strongest argument for git is not that every team wants git as a universal
knowledge store. It is that repo-centric teams already trust text, diffs, branch
review, and CI. But Stack Overflow's 2024 data shows architectural decisions also
live across Jira, Confluence, Markdown files, Notion, GitHub Discussions, Azure
DevOps, Obsidian, Slack, Teams, Zoom, Discord, and other surfaces (S1, S2).

Implication for ADR-0001:

- README copy can say git-backed `.bicameral/` is the default path for engineering
  teams that already govern work through PR review.
- ADR-0001 should keep Drive/future adapters real, because source-of-decision and
  source-of-code are not always the same substrate.
- The invariant is replayable, auditable accepted events; git is one excellent
  adapter, not the whole product ontology.

### 2. Local code grounding is table stakes

Stack Overflow's AI section gives Bicameral a crisp product reason for local
grounding: developers want AI productivity, but the top blockers include lack of
trust in output and lack of codebase context (S3). DORA adds the delivery warning:
AI can improve individual productivity while hurting delivery stability and
throughput if fundamentals like small batches and robust testing are neglected
(S6).

Implication for ADR-0003:

- Free/public Bicameral should always produce useful local `BindingEvidence` for
  the current workspace.
- Cloud value should be framed as scale and precision: historical indexes,
  cross-branch/cross-repo conflicts, shared graph caches, blast radius, and
  optimized grounding latency.
- Governance should never allow “AI said so” to become a blocking result without
  reviewed extraction, binding, and compliance axes.

### 3. Connector breadth makes authority separation more important

The surveys do not point to one canonical collaboration system. They point to a
messy ecosystem. Atlassian/DX report significant time lost to inefficiency (S4,
S5), while Stack Overflow shows multiple async and communication surfaces used at
meaningful rates (S1, S2).

Implication for ADR-0005:

- Source integrations should be easy to add because teams' decision evidence is
  scattered.
- Source integrations must not become authority because scattered evidence also
  increases duplication, stale context, and partial interpretation.
- Policy should vary by source trust, decision level, evidence freshness, and
  reviewer capability instead of hardcoding assumptions into connector code.

### 4. Enforcement should be honest and substrate-capability aware

DORA's 2024 findings support careful enforcement: AI and platform changes can
raise productivity while also affecting stability/throughput (S6). But most
surveyed collaboration surfaces are not merge boundaries (S1, S2).

Implication for ADR-0007:

- Preserve `enforcement_capability` as a first-class field.
- Make CI blocking a mapping for git-backed workspaces with real merge control,
  not the default semantic meaning of a governance result.
- For chat/docs/Drive/dashboard-first work, prefer warnings, paused approvals,
  queued actions, review states, and visible audit over pretending to block code.

### 5. The runtime scaffold must optimize for trust, security, and ROI

AI tool use is broad: Stack Overflow reports 76% using or planning to use AI in
development (S3), JetBrains reports substantial ChatGPT/Copilot usage (S7), and
GitHub reports enterprise teams increasingly encourage or allow AI coding tools
(S8). But G2's buyer data says AI purchases face intense scrutiny: 57% expect ROI
within 3 months and 81% consider vendor security-breach history (S9).

Implication for ADR-0009:

- Keep ZeroClaw's local daemon/gateway/security/audit primitives.
- Remove or postpone generic assistant features that do not reinforce Bicameral's
  governance loop.
- Demonstrate value quickly with local preflight, decision capture, and grounding,
  because buyers expect fast proof and developers are already overloaded.

## Follow-up work

- Add a checked-in `docs/research/survey-sources.yaml` if these statistics become
  part of release-gating or ADR acceptance criteria.
- Revisit ADR-0001 and README onboarding copy after product decides whether “git
  default for repo-centric teams” should be explicit in the quick-start path.
- Revisit ADR-0003 after customer interviews validate which cross-repo/cross-branch
  grounding tasks teams would pay hosted cloud to solve.
- Revisit ADR-0009 during scaffold extraction to delete any ZeroClaw runtime
  surface that does not serve decision capture, grounding, review, audit, or safe
  extension.
