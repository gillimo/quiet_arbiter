# End State (quiet_arbiter)

This describes the target outcome for the quiet_arbiter project.

## Product Scope
- Rust-first, low-latency arbitrage engine for Polymarket.
- Two-leg trade execution with hardened risk controls.
- Latency budgets are the first gate; no trade if the budget is exceeded.
- No LLMs in the hot path; analysis tooling is offline-only.

## Latency Budgets (Initial Draft)
- market_data_max_age_ms: 100
- decision_max_ms: 5
- order_submit_max_ms: 25
- end_to_end_max_ms: 200

## Data Model (Authoritative)
- `data/state.json` is the source of truth and is versioned.
- Required sections:
  - system: environment, venue, mode, versioning, latency budgets.
  - risk_limits: exposure, order sizing, loss caps, edge thresholds.
  - strategies: enabled flags and parameters.
  - markets: canonical market snapshots with pricing metadata and timestamps.
  - positions/orders: open exposure and execution history.
  - pnl: realized and unrealized tracking.

## Opportunity Engine
- Normalize market data and fee schedules.
- Identify long-tail mispricings and two-leg opportunities.
- Estimate expected value (EV) and confidence.
- Enforce min-edge and latency thresholds before routing.

## Execution Engine
- Atomic two-leg execution plan with contingency paths.
- Hard risk checks at pre-trade and post-fill.
- Cancel/replace logic for stale or adverse moves.

## Risk Controls
- Max gross/net exposure and per-market caps.
- Daily loss limits and kill-switch.
- Spread, slippage, and latency guardrails.
- Position unwind playbooks.

## Logging and Evidence
- Structured logs for every decision and fill.
- Deterministic replay from captured market data.
- Metrics that show positive EV over time.

## Operator UX
- CLI for status, opportunities, and risk.
- Optional GUI for monitoring and audits.

## Distribution
- Local install docs and reproducible builds.
- Versioned releases and changelog.
