# Public Developer Survey Implications for Current ADRs

**Date:** 2026-05-28
**Status:** draft evidence memo
**Scope:** ADR-0001, ADR-0003, ADR-0005, ADR-0007, and ADR-0009

> Note: this memo was prepared from the repo ADRs and the team's discussion that
> Bicameral should use publicly accessible developer surveys to inform architecture.
> The implementation run did not perform fresh web lookups, so the claims below are
> framed as decision implications and evidence requirements rather than final
> benchmark numbers. Before changing an ADR from `proposed` to `accepted`, attach
> the exact report year, statistic, URL, and retrieval date.

## Why this belongs in the architecture repo

The current ADR set already makes several product-shaping assumptions about how
software teams work:

- teams can tolerate canonical decision state living next to code in git;
- local code grounding is useful even before hosted organization-scale graphing;
- ingestion and review should be split so different tools can contribute evidence
  without becoming authority;
- review and enforcement must degrade honestly across git, Drive, Slack,
  dashboards, and agent surfaces;
- a ZeroClaw-style local runtime is the right scaffold for trust-boundary work.

Those assumptions should not be argued only from internal taste. Public developer
surveys and marketplace review corpora should become repeatable input into the
ADR process, especially where the decision changes user workflow or pricing.

## Decision candidates that need survey evidence

| ADR | Decision under pressure | Public evidence to gather | Architectural implication |
|---|---|---|---|
| [ADR-0001](../adr/0001-local-daemon-gateway-and-event-store-substrates.md) | Whether git should be the first-class event store substrate versus only one adapter among peers. | Version-control adoption, PR/code-review usage, repo-host concentration, and developer trust in text/diff workflows from Stack Overflow, JetBrains, GitHub, DORA, and similar surveys. | Keep git as the default path for repo-centric engineering teams, but do not hardcode git into the domain lifecycle. Survey evidence should decide default onboarding order, not collapse the substrate-neutral contract. |
| [ADR-0003](../adr/0003-local-code-grounding-and-cloud-conflict-boundary.md) | What belongs in free/local grounding versus paid/hosted conflict intelligence. | DORA evidence about fast feedback, code review, trunk-based development, CI, and deployment performance; developer-experience surveys about time lost to finding context, dependencies, and ownership. | Free/local must ground in the current workspace because surveys generally reward fast local feedback. Paid cloud should concentrate on cross-repo, historical, cross-branch, and expensive graph intelligence where local inspection is insufficient. |
| [ADR-0005](../adr/0005-separate-event-store-ingestion-review-and-policy.md) | Whether ingestion, review UX, event authority, and governance policy are separate interfaces. | Surveys/reviews describing tool fragmentation across Jira/Linear, Slack/Teams, GitHub/GitLab, docs, meetings, email, and support systems; Atlassian/G2-style complaints about context switching and duplicated manual status work. | Do not make any one connector or UI the architecture. Connectors should emit evidence/candidates only; policy and review commands decide authority. Integration breadth is a market requirement, not an excuse to weaken governance boundaries. |
| [ADR-0007](../adr/0007-substrate-neutral-governance-flow.md) | Whether governance results should map to multiple enforcement capabilities instead of always becoming CI checks. | DORA/DevEx data on CI, code review, incident/change failure tradeoffs, and non-code collaboration channels; survey data on how often teams rely on Slack, tickets, docs, and dashboards for decisions. | Preserve `enforcement_capability` as an explicit field. Git workspaces may use `ci_block`; non-git or pre-code decisions should use warnings, paused approval, queued actions, or review states without pretending they can block a merge. |
| [ADR-0009](../adr/0009-adopt-zeroclaw-runtime-scaffold.md) | Whether a local daemon/gateway runtime scaffold is the right public product base. | Developer survey signals around local tooling, privacy/security posture, agent adoption, approval gates, and extension ecosystems. | Keep the ZeroClaw scaffold constrained by Bicameral protocol objects. Public research should validate local-first trust-boundary assumptions and identify where generic assistant runtime surface area should be removed. |

## Source categories to track

Use stable, public sources that can be re-checked when an ADR is promoted:

1. **DORA / State of DevOps** — delivery performance, feedback loops, CI/CD,
   code-review/process practices, reliability tradeoffs, AI/tooling impacts.
2. **Atlassian developer-experience research** — context switching, time lost to
   inefficient workflows, fragmentation across planning/docs/collaboration tools.
3. **Stack Overflow Developer Survey** — tool/platform adoption, collaboration
   tools, AI-assisted developer workflows, repo-host and VCS ecosystem signals.
4. **JetBrains State of Developer Ecosystem** — language/tooling/VCS adoption,
   local development behavior, team workflow norms.
5. **GitHub research / Octoverse** — pull-request, CI, AI, and open-source
   collaboration patterns.
6. **G2/Capterra/product-review corpora** — qualitative pain around Jira,
   Confluence, Linear, GitHub, GitLab, Notion, Slack, and developer productivity
   tools. Use these as market-text evidence, not as precise quantitative truth.

## Evidence standards for ADR updates

When a survey statistic is used to change an ADR, record:

- report title and year;
- source URL;
- retrieval date;
- exact quoted statistic or table name;
- population caveat, if provided by the source;
- the ADR decision it informs;
- whether the statistic supports a default, a fallback, or only a research bet.

Do not use a public statistic as direct authority for canonical architecture. It
should inform default ordering and risk framing. The architecture still needs a
local invariant when the market evidence is mixed.

## Current implications before numeric sourcing

1. **Git is a strong default, not the ontology.** Existing ADRs already make the
   right distinction: git-backed `.bicameral/` text should be first-class for
   repo-centric teams, while ADR-0001 and ADR-0007 preserve a substrate-neutral
   lifecycle for Drive and future adapters.
2. **Local grounding cannot be a paid-only feature.** If the public bot cannot
   inspect the current worktree, it becomes another documentation tool. ADR-0003's
   free/local versus cloud boundary should stay.
3. **Integration breadth increases the need for authority separation.** The more
   evidence comes from Slack, tickets, meetings, docs, support, and agent sessions,
   the more important ADR-0005's connector/review/policy/event-store split becomes.
4. **Enforcement must be honest.** Survey evidence may show broad CI adoption, but
   many important decisions happen before or outside a merge boundary. ADR-0007's
   `enforcement_capability` field prevents over-claiming.
5. **The runtime scaffold should be narrowed around governance.** ADR-0009 should
   keep useful daemon/gateway/security primitives while removing generic assistant
   features that do not serve decision capture, grounding, review, or audit.

## Follow-up work

- Add a checked-in `docs/research/survey-sources.yaml` once exact report URLs and
  statistics have been verified.
- Revisit ADR-0001 and ADR-0007 after the first numeric sourcing pass to decide
  whether the README should call git the default for engineering teams or only an
  example first-class substrate.
- Revisit ADR-0003 after collecting public data on context-finding, dependency
  discovery, and cross-repo coordination pain to sharpen the paid/cloud boundary.
