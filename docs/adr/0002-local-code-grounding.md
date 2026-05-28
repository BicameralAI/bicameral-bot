# ADR-0002: Local Code Grounding

**Date:** 2026-05-27  
**Status:** proposed  
**Level:** L1

## Problem

The open-core split must not imply that free Bicameral is ungrounded. If the local bot only captures notes, it fails the product thesis.

## Decision

The bot includes local code grounding for the current repo/worktree. It may inspect files, diffs, paths, symbols, commits, and PR context to produce `BindingEvidence` and route weak evidence to review. Organization-scale claims remain out of scope for local grounding.

## Non-Goals

Local grounding does not silently approve compliance or block work.

## Consequences

Free/public Bicameral is useful as a developer tool. Paid value concentrates in scale and precision.
