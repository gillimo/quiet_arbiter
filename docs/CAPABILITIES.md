# Capabilities (Exhaustive Draft)

This list defines what the project can do. Trim as needed.

## Core
- Normalize Polymarket market data and fees.
- Enforce latency budgets before any decision.
- Detect long-tail mispricings with EV estimates.
- Plan two-leg trades with routing options.
- Enforce risk limits before and after fills.
- Maintain deterministic logs and replay artifacts.

## Execution
- Order sizing and edge thresholds.
- Cancel/replace for stale orders.
- Safe unwind procedures and kill-switch.

## Data
- Versioned schema and migrations.
- Market snapshots and execution history.
- Import/export for backtests.

## UX
- CLI for fast iteration.
- Optional GUI for monitoring.

## Analytics
- Opportunity hit-rate, slippage, and fill quality.
- Latency distribution tracking (p50/p95/p99).
- EV tracking and drawdown monitoring.
