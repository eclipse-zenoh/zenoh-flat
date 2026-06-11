//
// Throughput subscriber for the zenoh-flat *native Rust* API.
//
// Structural mirror of eclipse-zenoh/zenoh `examples/examples/z_sub_thr.rs`,
// rewritten against zenoh-flat's `z_*` API. The flat subscriber callback is
// `impl Fn(ZSample) + Send + Sync` (not `FnMut`), so the per-message counter is
// kept **lock-free** (atomics) to avoid charging the native baseline a mutex
// that the C examples don't pay. The callback ignores the `ZSample` (no
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
    ZConfig, ZSample, init_zenoh_logs_from_env_or, z_config_default, z_config_from_file,
    z_config_insert_json5, z_keyexpr_try_from, z_open, z_session_declare_subscriber,
};

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
        if n % self.round_size == 0 {
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

fn main() {
    init_zenoh_logs_from_env_or("error");
    let args = Args::parse();

    let session = z_open(build_config(&args.common)).unwrap_or_else(|e| panic!("{e}"));
    let ke = z_keyexpr_try_from("test/thr".to_string()).unwrap_or_else(|e| panic!("{e}"));

    let stats = Arc::new(Stats::new(args.number, args.samples));
    let s = stats.clone();
    let _subscriber =
        z_session_declare_subscriber(&session, ke, move |_sample: ZSample| s.increment(), || {})
            .unwrap_or_else(|e| panic!("{e}"));

    println!("Press CTRL-C to quit...");
    std::thread::park();
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

// --- Minimal `CommonArgs` equivalent (mode / connect / listen / config), with
// the same flag names and config keys as the zenoh-flat-c examples' parse_args.h
// and the upstream zenoh `CommonArgs`. ---
#[derive(Parser, Clone, Debug)]
struct CommonArgs {
    /// A configuration file
    #[arg(short = 'c', long)]
    config: Option<String>,
    /// The zenoh session mode [peer|client|router]
    #[arg(short = 'm', long)]
    mode: Option<String>,
    /// Endpoint to connect to (repeatable)
    #[arg(short = 'e', long)]
    connect: Vec<String>,
    /// Locator to listen on (repeatable)
    #[arg(short = 'l', long)]
    listen: Vec<String>,
    /// Disable multicast scouting
    #[arg(long = "no-multicast-scouting")]
    no_multicast_scouting: bool,
    /// Arbitrary config changes as KEY:VALUE (repeatable)
    #[arg(long = "cfg")]
    cfg: Vec<String>,
}

fn json_list(items: &[String]) -> String {
    let quoted: Vec<String> = items.iter().map(|e| format!("\"{e}\"")).collect();
    format!("[{}]", quoted.join(","))
}

fn build_config(a: &CommonArgs) -> ZConfig {
    let mut c = match &a.config {
        Some(path) => z_config_from_file(path).unwrap_or_else(|e| panic!("{e}")),
        None => z_config_default(),
    };
    if let Some(m) = &a.mode {
        z_config_insert_json5(&mut c, "mode", &format!("\"{m}\""))
            .unwrap_or_else(|e| panic!("{e}"));
    }
    if !a.connect.is_empty() {
        z_config_insert_json5(&mut c, "connect/endpoints", &json_list(&a.connect))
            .unwrap_or_else(|e| panic!("{e}"));
    }
    if !a.listen.is_empty() {
        z_config_insert_json5(&mut c, "listen/endpoints", &json_list(&a.listen))
            .unwrap_or_else(|e| panic!("{e}"));
    }
    if a.no_multicast_scouting {
        z_config_insert_json5(&mut c, "scouting/multicast/enabled", "false")
            .unwrap_or_else(|e| panic!("{e}"));
    }
    for kv in &a.cfg {
        let (k, v) = kv.split_once(':').expect("--cfg expects KEY:VALUE");
        z_config_insert_json5(&mut c, k, v).unwrap_or_else(|e| panic!("{e}"));
    }
    c
}
