# How To Operate (quiet_arbiter)

This project builds a Rust-first arbitrage engine with a thin CLI wrapper.
Latency is the primary constraint. No LLMs are used in the hot path.

## Reference Order
1) `PERMISSIONS.md`
2) This file
3) `LATENCY_PLAN.md`
4) `CAPABILITIES.md`
5) `DEPENDENCIES.md`
6) `METRICS.md`
7) `PROJECT_VISION.md`
8) `TICKETS.md` / `BUG_LOG.md`
9) `SIGNING_OFF.md`

## CLI Flow
- `cargo run --manifest-path core/Cargo.toml -- status` for current state and budgets.
- `cargo run --manifest-path core/Cargo.toml -- run --ticks data/market_ticks.jsonl` to replay.

## Data Sources
- Local JSON in `data/` is the source of truth for planning and logs.
- Market data ingestion and execution logic live in the Rust core.

## Logging
- Record changes in `docs/LOGBOOK.md` with handle + date.
- Log bugs in `docs/BUG_LOG.md` before code edits.
