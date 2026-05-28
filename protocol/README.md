# Bicameral Protocol

The Bicameral protocol is embedded in `bicameral-bot` because the bot is the local authority boundary where integrations, MCP tools, and cloud advisories enter governance.

The former standalone `bicameral-protocol` repository is deprecated. Keep schemas, conformance fixtures, and compatibility notes here.

## Core object families

- `SourceEvidence`
- `DecisionCandidate`
- `Decision`
- `BindingHint`
- `BindingEvidence`
- `DependencySignal`
- `ReviewCommand`
- `ReviewEvent`
- `ReviewState`
- `GovernanceResult`
- `ConflictSignal`
- `GroundingSuggestion`

## Rule

Protocol objects are reviewable claims, evidence, commands, or advisories. They are not canonical authority until governance policy accepts them and an event store adapter materializes the accepted event.
