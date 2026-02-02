# quiet_arbiter

Mission Learning Statement
- Mission: Build a Rust-first, low-latency Polymarket arbitrage engine with strict risk controls.
- Learning focus: latency budgeting, execution routing, and audit-grade logging.
- Project start date: 2026-02-02 (inferred from initial scaffold)

Rust-first, low-latency arbitrage engine that hunts long-tail mispricings with two-leg execution and conservative exposure.

## Features

- Latency budgets enforced as a hard gate
- Replay harness with JSONL market ticks
- Risk limits and edge thresholds in state config
- Deterministic logs for audit and replay

## Installation

### Requirements

- Rust toolchain (stable)

## Quick Start

```bash
cargo build --manifest-path core/Cargo.toml
cargo run --manifest-path core/Cargo.toml -- status
cargo run --manifest-path core/Cargo.toml -- run --ticks data/market_ticks.jsonl
```

## Usage

- `status` prints current latency budgets and thresholds.
- `run` replays market ticks and logs decisions to `logs/run_*.jsonl`.

## Architecture

```
Market Ticks (JSONL)
    |
    v
Ingest + Normalize
    |
    v
Latency Gate + Edge Check
    |
    v
Decision + Risk Check
    |
    v
Log Output (JSONL)
```

## Project Structure

```
core/               # Rust core
  src/main.rs       # CLI and replay
  Cargo.toml

data/               # State + replay inputs
logs/               # Run logs (generated)
```

## Building

```bash
cargo build --manifest-path core/Cargo.toml
```

## Contributing

Open tickets live in `docs/TICKETS.md`.

## License

MIT License - see [LICENSE](LICENSE) for details.
