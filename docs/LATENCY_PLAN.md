# Latency-First Plan (quiet_arbiter)

This plan treats latency as the primary constraint. Risk controls and logging are non-negotiable, but no decision is valid if the latency budget is blown.

## Guiding Principle
If data is stale or the budget is exceeded, do nothing. Speed is the first gate.

## Phase 0: Baseline the Budget (Week 0)
- Define initial latency budgets across the pipeline:
  - market_data_max_age_ms
  - decision_max_ms
  - order_submit_max_ms
  - end_to_end_max_ms
- Add timing fields to state and log schema.
- Establish clock sync assumptions (NTP/Chrony).
- Build a local replay harness that can measure p50/p95/p99 for each stage.

## Phase 1: Hot-Path Architecture (Week 1)
- Rust core with zero-alloc hot path (pre-allocated buffers, fixed-size queues).
- Single-writer event loop with bounded channels; no blocking I/O on the hot path.
- Dedicated threads for ingest, decision, and order submission.
- Timing instrumentation at every boundary.

## Phase 2: Data Ingest and Normalization (Week 2)
- Normalize Polymarket data into a minimal, cache-friendly struct.
- Track data age at ingest and at decision time.
- Drop any tick that violates freshness budget.

## Phase 3: Decision Engine (Week 3)
- Deterministic EV calculation with constant-time checks.
- Two-leg planning with precomputed risk constraints.
- Abort plan if projected submit time exceeds budget.

## Phase 4: Execution and Risk (Week 4)
- Order submit path with fastest available client.
- Pre-trade and post-fill checks are O(1) and inline.
- Kill-switch if latency or slippage breaches thresholds.

## Phase 5: Evidence and Iteration (Week 5+)
- Daily latency report (p50/p95/p99).
- Correlate latency spikes with missed edge.
- Iterate on hot path only after measurement proves wins.
