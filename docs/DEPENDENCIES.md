# Dependency Mapping (Approach)

Define how the project should map and surface dependencies.

## Dependency Types
- Market data feeds and snapshots
- Venue API/SDK access
- Fee schedules and collateral rules
- Latency and execution constraints

## Output Style
- Clear prerequisites for any strategy or connector
- Explicit risk-limit dependencies
- Fallback paths when data is stale or missing

## Data Plan
- Store dependency data in `data/reference.json`.
