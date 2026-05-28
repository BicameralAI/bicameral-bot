# Bicameral Bot

**Bicameral Bot** is the public, local-first Bicameral runtime: daemon, gateway, review UX, local code grounding, governance policy, protocol contracts, and storage-adapter materialization.

It is OpenClaw for software teams. It runs at the team's trust boundary, captures implementation-constraining decisions from operational sources, grounds them in the current workspace, and routes ambiguity to human review before any claim becomes canonical.

## Key Features

- **Local daemon** – validates typed protocol objects, evaluates governance policy, preserves audit state, and materializes accepted events through storage adapters.
- **Gateway boundary** – accepts input from integrations, MCP sessions, files, meetings, and EM-safe mods without letting edge code become authority.
- **Local code grounding** – inspects the current repo/worktree, diffs, paths, symbols, commits, and PR context to produce reviewable `BindingEvidence`.
- **Embedded protocol contracts** – `protocol/` contains shared schemas and conformance fixtures used by the bot, MCP, integrations, and cloud clients.
- **Decision Ledger and Ledger View** – canonical state is replayed from the selected event store substrate; UI surfaces emit review commands.
- **HITL governance routing** – separates extraction, binding, and compliance confidence so uncertainty reaches the right reviewer.
- **Optional cloud oracle** – calls Bicameral Cloud for organization-scale code graph and conflict intelligence without giving cloud canonical authority.

## High-level Architecture

```text
External sources / agents / mods
Jira  Linear  Slack  GitHub  meetings  MCP sessions
        │
        ▼
┌──────────────────────────────────────────────────────────────┐
│ bicameral-bot                                                │
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

- [`bicameral-mcp`](https://github.com/BicameralAI/bicameral-mcp) – agent-facing MCP tool surface. Emits protocol-shaped commands/evidence into the bot.
- [`bicameral-integrations`](https://github.com/BicameralAI/bicameral-integrations) – source adapters and EM-safe mods. Emits candidates/evidence/signals into the gateway.
- [`bicameral-cloud`](https://github.com/BicameralAI/bicameral-cloud) – private hosted code graph and conflict oracle. Returns evidence/advisories, not authority.
- [`bicameral-daemon`](https://github.com/BicameralAI/bicameral-daemon) – legacy/private architecture staging repo and current gold-standard documentation structure.

## Protocol Contracts

The protocol lives in this repo under `protocol/` rather than in a separate repository. The bot owns protocol compatibility because it is the local authority boundary that all public edge surfaces enter.

Core object families: `SourceEvidence`, `DecisionCandidate`, `Decision`, `BindingHint`, `BindingEvidence`, `DependencySignal`, `ReviewCommand`, `ReviewEvent`, `GovernanceResult`, `ConflictSignal`, `GroundingSuggestion`.

## Governance Boundary

Bicameral Bot bridges two probabilistic domains: human operational interpretation and code/workflow interpretation. The bot therefore preserves separate `extraction_confidence`, `binding_confidence`, and `compliance_confidence`. HITL review is the debt-control boundary, not a fallback path.

## Gotchas

### Free does not mean ungrounded

The public bot includes local code grounding. Paid cloud adds organization-scale grounding: cross-branch, cross-repo, historical, and optimized code graph intelligence.

### Gateway code is not authority

Integrations and mods can be expressive at the edge, but they emit typed evidence and commands. The daemon/core decides what can be materialized.

## Testing

```bash
pytest -v tests/
pytest -v protocol/tests/
```

## Git Workflow

```bash
git checkout -b feature/local-daemon-change
pytest -v tests/
git commit -am "Describe local bot change"
git push origin HEAD
```
