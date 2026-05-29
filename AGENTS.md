# Agent instructions

Before doing Bicameral product work, read and follow the factory instructions:

`~/github/bicameral-factory/INSTRUCTIONS.md`

## Bicameral factory context

Before Bicameral product work, read `.bicameral/factory-context.local.json`, confirm the referenced `bicameral-factory` checkout/commit, and inspect copied factory skills under `.agents/skills/`.

Factory skills installed by `skills.sh` are local file-backed procedures. For factory-governed work, prefer the copied `bic-*` skills under `.agents/skills/`, such as `.agents/skills/bic-grill-with-docs/SKILL.md`, `.agents/skills/bic-tdd/SKILL.md`, and `.agents/skills/bic-pre-pr-factory-attestation/SKILL.md`.

Treat `.bicameral/factory-skills/` as the legacy symlink fallback only. Do not prefer it when `.agents/skills/` contains the copied `bic-*` factory skills.

For provenance, compare installed `bic-*` skill file hashes against `<factory_repo>/skills/factory-skill-manifest.json`. A similar name alone is not evidence that a factory-shipped skill was used.

Do not commit `.bicameral/factory-context.local.json`, `.bicameral/factory-skills`, `.agents/skills`, prompts, context bundles, run plans, raw logs, `RUN_SUMMARY.md`, or local agent habits.

For product PRs, create and validate the required factory attestation under `.bicameral/factory-attestations/<factory_commit>.json`.
