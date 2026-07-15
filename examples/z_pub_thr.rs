//
// Throughput publisher for the zenoh-flat *native Rust* API.
//
// Structural mirror of eclipse-zenoh/zenoh `examples/examples/z_pub_thr.rs`,
// rewritten against zenoh-flat's `z_*` API so it measures the flat API's own
// overhead (no C/FFI boundary, no boxing). Built once, the master payload is
// cheaply `z_zbytes_clone`d each iteration (refcount bump), since
// `z_publisher_put` consumes the payload it is given — matching the native
// example's `publisher.put(data.clone())`.
//
use clap::Parser;
use zenoh_flat::{
    init_zenoh_logs_from_env_or, z_config_default, z_config_from_file, z_config_insert_json5,
    z_keyexpr_try_from, z_open, z_publisher_put, z_session_declare_publisher, z_zbytes_clone,
    z_zbytes_from_vec, CongestionControl, Priority, ZConfig,
};

fn main() {
    init_zenoh_logs_from_env_or("error");
    let args = Args::parse();

    let prio = args.priority.map(priority_from_u8);
    let payload_size = args.payload_size;

    let data = z_zbytes_from_vec((0..payload_size).map(|i| (i % 10) as u8).collect::<Vec<u8>>());

    let session = z_open(build_config(&args.common)).unwrap_or_else(|e| panic!("{e}"));

    let ke = z_keyexpr_try_from("test/thr".to_string()).unwrap_or_else(|e| panic!("{e}"));
    let publisher = z_session_declare_publisher(
        &session,
        ke,
        Some(CongestionControl::Block),
        prio,
        Some(args.express),
        #[cfg(feature = "unstable")]
        None,
    )
    .unwrap_or_else(|e| panic!("{e}"));

    println!("Press CTRL-C to quit...");
    let mut count: usize = 0;
    let mut start = std::time::Instant::now();
    loop {
        z_publisher_put(&publisher, z_zbytes_clone(&data), None, None).unwrap_or_else(|e| panic!("{e}"));

        if args.print {
            if count < args.number {
                count += 1;
            } else {
                let thpt = count as f64 / start.elapsed().as_secs_f64();
                println!("{thpt} msg/s");
                count = 0;
                start = std::time::Instant::now();
            }
        }
    }
}

fn priority_from_u8(p: u8) -> Priority {
    match p {
        1 => Priority::RealTime,
        2 => Priority::InteractiveHigh,
        3 => Priority::InteractiveLow,
        4 => Priority::DataHigh,
        5 => Priority::Data,
        6 => Priority::DataLow,
        7 => Priority::Background,
        other => panic!("invalid priority {other} (expected 1..=7)"),
    }
}

#[derive(Parser, Clone, Debug)]
struct Args {
    /// express for sending data
    #[arg(long, default_value = "false")]
    express: bool,
    /// Priority for sending data [1=RealTime .. 7=Background]
    #[arg(short, long)]
    priority: Option<u8>,
    /// Print the statistics
    #[arg(short = 't', long)]
    print: bool,
    /// Number of messages in each throughput measurement
    #[arg(short, long, default_value = "100000")]
    number: usize,
    /// Sets the size of the payload to publish
    payload_size: usize,
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
        z_config_insert_json5(&mut c, "mode", &format!("\"{m}\"")).unwrap_or_else(|e| panic!("{e}"));
    }
    if !a.connect.is_empty() {
        z_config_insert_json5(&mut c, "connect/endpoints", &json_list(&a.connect)).unwrap_or_else(|e| panic!("{e}"));
    }
    if !a.listen.is_empty() {
        z_config_insert_json5(&mut c, "listen/endpoints", &json_list(&a.listen)).unwrap_or_else(|e| panic!("{e}"));
    }
    if a.no_multicast_scouting {
        z_config_insert_json5(&mut c, "scouting/multicast/enabled", "false").unwrap_or_else(|e| panic!("{e}"));
    }
    for kv in &a.cfg {
        let (k, v) = kv.split_once(':').expect("--cfg expects KEY:VALUE");
        z_config_insert_json5(&mut c, k, v).unwrap_or_else(|e| panic!("{e}"));
    }
    c
}
