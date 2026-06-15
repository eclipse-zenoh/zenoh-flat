//
// Throughput subscriber for the zenoh-flat *native Rust* API.
//
// Structural mirror of eclipse-zenoh/zenoh `examples/examples/z_sub_thr.rs`,
// rewritten against zenoh-flat's flat API. The flat subscriber callback is
// `impl Fn(Sample) + Send + Sync` (not `FnMut`), so the per-message counter is
// kept **lock-free** (atomics) to avoid charging the native baseline a mutex
// that the C examples don't pay. The callback ignores the `Sample` (no
// expansion / no field access), matching the C and native thr subscribers.
//
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed},
    },
    time::Instant,
};

use clap::Parser;
use zenoh_flat::{
    Sample, init_zenoh_logs_from_env_or, keyexpr_new_try_from, open, session_declare_subscriber,
};

#[path = "common/mod.rs"]
mod common;
use common::CommonArgs;

struct Stats {
    base: Instant,
    round_size: usize,
    samples: usize,
    count: AtomicUsize,
    round_start_nanos: AtomicU64,
    finished_rounds: AtomicUsize,
}

impl Stats {
    fn new(round_size: usize, samples: usize) -> Self {
        Stats {
            base: Instant::now(),
            round_size,
            samples,
            count: AtomicUsize::new(0),
            round_start_nanos: AtomicU64::new(0),
            finished_rounds: AtomicUsize::new(0),
        }
    }

    fn increment(&self) {
        let n = self.count.fetch_add(1, Relaxed) + 1;
        if n.is_multiple_of(self.round_size) {
            let now = self.base.elapsed().as_nanos() as u64;
            let prev = self.round_start_nanos.swap(now, Relaxed);
            let elapsed = now.saturating_sub(prev) as f64 / 1e9;
            if elapsed > 0.0 {
                println!("{} msg/s", self.round_size as f64 / elapsed);
            }
            let fr = self.finished_rounds.fetch_add(1, Relaxed) + 1;
            if self.samples != 0 && fr >= self.samples {
                std::process::exit(0);
            }
        }
    }
}

fn main() -> Result<(), zenoh_flat::Error> {
    init_zenoh_logs_from_env_or("error");
    let args = Args::parse();

    let session = open(args.common.try_into()?)?;
    let ke = keyexpr_new_try_from("test/thr".to_string())?;

    let stats = Arc::new(Stats::new(args.number, args.samples));
    let s = stats.clone();
    let _subscriber =
        session_declare_subscriber(&session, ke, move |_sample: Sample| s.increment(), || {})?;

    println!("Press CTRL-C to quit...");
    std::thread::park();

    Ok(())
}

#[derive(Parser, Clone, Debug)]
struct Args {
    /// Number of throughput measurements (rounds), then exit. 0 = run forever.
    #[arg(short, long, default_value = "10")]
    samples: usize,
    /// Number of messages in each throughput measurement
    #[arg(short, long, default_value = "100000")]
    number: usize,
    #[command(flatten)]
    common: CommonArgs,
}
