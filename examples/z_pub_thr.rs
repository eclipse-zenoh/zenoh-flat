//
// Throughput publisher for the zenoh-flat *native Rust* API.
//
// Structural mirror of eclipse-zenoh/zenoh `examples/examples/z_pub_thr.rs`,
// rewritten against zenoh-flat's flat API so it measures the flat API's own
// overhead (no C/FFI boundary, no boxing). Built once, the master payload is
// cheaply `zbytes_new_clone`d each iteration (refcount bump), since
// `publisher_put` consumes the payload it is given — matching the native
// example's `publisher.put(data.clone())`.
//
use clap::Parser;
use zenoh_flat::{
    CongestionControl, Priority, init_zenoh_logs_from_env_or, keyexpr_new_try_from, open,
    publisher_put, session_declare_publisher, zbytes_new_clone, zbytes_new_from_vec,
};

#[path = "common/mod.rs"]
mod common;
use common::CommonArgs;

fn main() -> Result<(), zenoh_flat::Error> {
    init_zenoh_logs_from_env_or("error");
    let args = Args::parse();

    let prio = args.priority.map(priority_from_u8);
    let payload_size = args.payload_size;

    let data = zbytes_new_from_vec(
        (0..payload_size)
            .map(|i| (i % 10) as u8)
            .collect::<Vec<u8>>(),
    );

    let session = open(args.common.try_into()?)?;

    let ke = keyexpr_new_try_from("test/thr".to_string())?;
    let publisher = session_declare_publisher(
        &session,
        ke,
        Some(CongestionControl::Block),
        prio,
        Some(args.express),
        #[cfg(feature = "unstable")]
        None,
    )?;

    println!("Press CTRL-C to quit...");
    let mut count: usize = 0;
    let mut start = std::time::Instant::now();
    loop {
        publisher_put(&publisher, zbytes_new_clone(&data), None, None)?;

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
