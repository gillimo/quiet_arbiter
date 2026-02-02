use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(name = "quiet_arbiterd", version, about = "Latency-first Polymarket arb engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, default_value = "data/state.json")]
    state: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    Status,
    Run {
        #[arg(long)]
        ticks: PathBuf,
    },
}

#[derive(Debug, Deserialize)]
struct State {
    version: u32,
    system: System,
    risk_limits: RiskLimits,
    strategies: Vec<Strategy>,
    markets: Vec<serde_json::Value>,
    positions: Vec<serde_json::Value>,
    orders: Vec<serde_json::Value>,
    pnl: Pnl,
    notes: Option<Notes>,
}

#[derive(Debug, Deserialize)]
struct System {
    name: String,
    mode: String,
    venue: String,
    latency_budget_ms: LatencyBudget,
}

#[derive(Debug, Deserialize)]
struct LatencyBudget {
    market_data_max_age: u64,
    decision_max: u64,
    order_submit_max: u64,
    end_to_end_max: u64,
}

#[derive(Debug, Deserialize)]
struct RiskLimits {
    max_gross_exposure: f64,
    max_net_exposure: f64,
    max_order_size: f64,
    max_daily_loss: f64,
    min_edge_bps: f64,
}

#[derive(Debug, Deserialize)]
struct Strategy {
    name: String,
    enabled: bool,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct Pnl {
    realized: f64,
    unrealized: f64,
}

#[derive(Debug, Deserialize)]
struct Notes {
    short: Vec<String>,
    mid: Vec<String>,
    long: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct MarketTick {
    market_id: String,
    ts_exchange_ms: u64,
    ts_recv_ms: u64,
    bid: f64,
    ask: f64,
}

#[derive(Debug, Serialize)]
struct LogEntry {
    run_ts_ms: u128,
    tick_index: u64,
    market_id: String,
    ts_exchange_ms: u64,
    ts_recv_ms: u64,
    data_age_ms: u64,
    decision_ms: u64,
    order_submit_ms: u64,
    end_to_end_ms: u64,
    spread_bps: f64,
    action: String,
    reason: String,
}

fn load_state(path: &Path) -> Result<State, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    let state: State = serde_json::from_str(&raw)?;
    Ok(state)
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn status(state: &State) {
    println!("Status");
    println!("- Name: {}", state.system.name);
    println!("- Mode: {}", state.system.mode);
    println!("- Venue: {}", state.system.venue);
    println!(
        "- Latency budgets (ms): data_age<= {}, decision<= {}, submit<= {}, end_to_end<= {}",
        state.system.latency_budget_ms.market_data_max_age,
        state.system.latency_budget_ms.decision_max,
        state.system.latency_budget_ms.order_submit_max,
        state.system.latency_budget_ms.end_to_end_max
    );
    println!("- Min edge (bps): {}", state.risk_limits.min_edge_bps);
}

fn compute_spread_bps(bid: f64, ask: f64) -> f64 {
    let mid = (bid + ask) / 2.0;
    if mid <= 0.0 {
        return 0.0;
    }
    ((ask - bid) / mid) * 10_000.0
}

fn run_replay(state: &State, ticks_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(ticks_path)?;
    let reader = BufReader::new(file);

    let log_dir = Path::new("logs");
    fs::create_dir_all(log_dir)?;
    let log_path = log_dir.join(format!("run_{}.jsonl", now_ms()));
    let mut log_file = File::create(&log_path)?;

    let mut total = 0u64;
    let mut considered = 0u64;
    let mut skipped = 0u64;

    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let tick: MarketTick = serde_json::from_str(&line)?;
        let data_age_ms = tick.ts_recv_ms.saturating_sub(tick.ts_exchange_ms);
        let decision_ms = 1u64;
        let order_submit_ms = 3u64;
        let end_to_end_ms = data_age_ms + decision_ms + order_submit_ms;
        let spread_bps = compute_spread_bps(tick.bid, tick.ask);

        let mut action = "consider".to_string();
        let mut reason = "ok".to_string();

        if data_age_ms > state.system.latency_budget_ms.market_data_max_age {
            action = "skip".to_string();
            reason = "stale_data".to_string();
        } else if decision_ms > state.system.latency_budget_ms.decision_max {
            action = "skip".to_string();
            reason = "decision_budget_exceeded".to_string();
        } else if order_submit_ms > state.system.latency_budget_ms.order_submit_max {
            action = "skip".to_string();
            reason = "submit_budget_exceeded".to_string();
        } else if end_to_end_ms > state.system.latency_budget_ms.end_to_end_max {
            action = "skip".to_string();
            reason = "end_to_end_budget_exceeded".to_string();
        } else if spread_bps < state.risk_limits.min_edge_bps {
            action = "skip".to_string();
            reason = "edge_below_threshold".to_string();
        }

        total += 1;
        if action == "consider" {
            considered += 1;
        } else {
            skipped += 1;
        }

        let entry = LogEntry {
            run_ts_ms: now_ms(),
            tick_index: idx as u64,
            market_id: tick.market_id,
            ts_exchange_ms: tick.ts_exchange_ms,
            ts_recv_ms: tick.ts_recv_ms,
            data_age_ms,
            decision_ms,
            order_submit_ms,
            end_to_end_ms,
            spread_bps,
            action,
            reason,
        };
        let serialized = serde_json::to_string(&entry)?;
        writeln!(log_file, "{}", serialized)?;
    }

    println!("Replay complete");
    println!("- Total ticks: {}", total);
    println!("- Considered: {}", considered);
    println!("- Skipped: {}", skipped);
    println!("- Log: {}", log_path.display());
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let state = load_state(&cli.state)?;

    match cli.command {
        Commands::Status => status(&state),
        Commands::Run { ticks } => run_replay(&state, &ticks)?,
    }

    Ok(())
}
