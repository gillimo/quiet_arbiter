# quiet_arbiter

Build a Rust-first, low-latency Polymarket arbitrage engine where latency is the first gate. The system hunts long-tail mispricings, executes two-leg trades with strict risk controls, and proves positive EV through auditable logs and iteration. No LLMs in the hot path.

Quickstart:
- Build: `cargo build --manifest-path core/Cargo.toml`
- Status: `cargo run --manifest-path core/Cargo.toml -- status`
- Run replay: `cargo run --manifest-path core/Cargo.toml -- run --ticks data/market_ticks.jsonl`

Notes:
- The Rust core is the source of truth for execution and risk.
- It prints to CLI now; later it will run as a background process with the same log outputs.
- Use `docs/DOCS_INDEX.md` to follow the operating order.
