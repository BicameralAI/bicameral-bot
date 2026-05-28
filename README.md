# Bicameral Bot

OpenClaw for software teams.

Bicameral Bot is a local-first, code-grounded decision and context layer for AI-assisted software teams. It runs inside the team trust boundary, captures decisions from operational sources, grounds them in the current workspace, and routes ambiguity to PM / EM / Dev review before anything becomes canonical.

## Why local-first

Cognitive debt lives in each team’s operational substrate: Jira, Linear, Slack, meetings, GitHub, ADRs, PRs, repos, support tickets, and informal channels. The right product shape is therefore daemon + gateway:

- gateway adapts messy edge sources into typed Bicameral protocol objects
- daemon/core validates, grounds, reviews, audits, and materializes canonical state
- optional cloud/oracle provides organization-scale code graph intelligence

## Free/open-source scope

- local daemon/runtime
- local gateway
- local review queue
- local code grounding for the current repo/worktree
- storage adapter writes, including git-backed `.bicameral/decisions/N.yaml`
- governance policy evaluation
- audit trail
- optional calls to `bicameral-cloud` for hosted graph intelligence

## Non-goals

- hosted code graph ownership
- cross-branch/cross-repo conflict oracle
- silent canonical promotion
- model-generated blocking without policy and grounded evidence
