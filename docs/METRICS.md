# Metrics and Ratings

All metrics are computed from local data and logged runs. Latency budgets are the first gate.

Latency SLOs (initial draft):
- market_data_max_age_ms: <= 100
- decision_max_ms: <= 5
- order_submit_max_ms: <= 25
- end_to_end_max_ms: <= 200

Core metrics:
- Opportunity count: candidates per hour and per market.
- Edge (bps): estimated EV before fees.
- Fill rate: fills / attempts.
- Slippage: expected vs realized price.
- Latency: decision-to-order and order-to-fill (p50/p95/p99).
- PnL: realized and unrealized, with attribution.

Risk metrics:
- Gross and net exposure by market.
- Max drawdown and daily loss.
- Cancel rate and stale-order rate.
- Kill-switch triggers and reasons.

Quality notes:
- Every metric should cite the log IDs used.
- Decisions must explain which data points caused changes.
- If a latency budget is exceeded, the decision is invalid.
