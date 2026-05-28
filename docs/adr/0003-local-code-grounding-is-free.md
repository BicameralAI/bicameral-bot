# ADR-0003: Local Code Grounding Is Part of the Free Bot

Status: accepted

Date: 2026-05-27

## Context

The open-core split must not imply that free Bicameral is ungrounded. If the public bot is only a note-taking tool, it fails the product thesis.

## Decision

`bicameral-bot` includes local code grounding for the current workspace.

It may inspect local files, diffs, paths, symbols, commits, and PR context to produce reviewable `BindingEvidence`.

It may not claim organization-wide safety, cross-branch conflict freedom, or cross-repo dependency completeness. Those are `bicameral-cloud` concerns.

## Consequences

- Free Bicameral is credible as a developer tool.
- Paid value is organization-scale grounding, not grounding itself.
- Local evidence remains evidence; governance decides enforcement.
