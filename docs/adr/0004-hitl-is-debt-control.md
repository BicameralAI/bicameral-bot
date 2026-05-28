# ADR-0004: HITL Is Debt Control

**Date:** 2026-05-27  
**Status:** proposed  
**Level:** L1

## Problem

Bicameral bridges two probabilistic domains: interpreting human operational context and interpreting code/workflow evidence. Collapsing extraction, binding, and compliance uncertainty into one confident answer can create new cognitive debt.

## Decision

Preserve separate confidence surfaces: `extraction_confidence`, `binding_confidence`, and `compliance_confidence`. HITL review is the debt-control mechanism that prevents probabilistic interpretation from becoming unreviewed authority.

## Consequences

Weak extraction or grounding remains advisory, queued, or unbound. Blocking and canonical promotion require governance policy plus reviewable evidence.
