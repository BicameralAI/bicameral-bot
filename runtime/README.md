# ZeroClaw Runtime Import

This branch imports ZeroClaw as a pinned upstream submodule at:

```text
third_party/zeroclaw
```

Purpose: use ZeroClaw as the scaffold for Bicameral's local daemon/gateway runtime, not as a wholesale product rename.

See `docs/adr/0005-adopt-zeroclaw-runtime-scaffold.md` for the decision, authority boundary, and extraction plan.

## Initial extraction targets

Start by reading these upstream areas:

```text
third_party/zeroclaw/crates/zeroclaw-api
third_party/zeroclaw/crates/zeroclaw-config
third_party/zeroclaw/crates/zeroclaw-runtime
third_party/zeroclaw/crates/zeroclaw-gateway
third_party/zeroclaw/crates/zeroclaw-plugins
third_party/zeroclaw/docs/book/src/architecture/overview.md
third_party/zeroclaw/docs/book/src/security/overview.md
third_party/zeroclaw/docs/book/src/sop/index.md
```

## Bicameral narrowing rule

Do not expose ZeroClaw's broad personal-assistant surface in the first Bicameral bot binary. Extract only what helps with:

- local daemon/gateway lifecycle;
- workspace-scoped config;
- protocol-shaped ingest/review commands;
- governance approval gates;
- audit receipts;
- EM-safe mod manifests and fixture runs;
- eventual local dashboard/review queue.

Agents and mods may emit candidates, evidence, hints, review commands, and advisory governance results. They must not directly create canonical decisions, ratify decisions, mark compliance resolved, or write canonical storage.
