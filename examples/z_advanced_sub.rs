//
// Copyright (c) 2026 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

//! Flat-API port of eclipse-zenoh/zenoh zenoh-ext `examples/examples/z_advanced_sub.rs`.
//!
//! An advanced subscriber queries historical data (`history`), recovers missed
//! samples via heartbeat subscription (`recovery`), and is discoverable by
//! advanced publishers (`subscriber_detection`). A separate sample-miss listener
//! reports samples the subscriber could not recover. The flat API is
//! callback-based: both the sample stream and the miss stream deliver to the
//! closures passed here, and the returned handles keep the declarations alive.

use clap::Parser;
use zenoh_flat::{
    HistoryConfig, RecoveryConfig, RecoveryMode, advanced_subscriber_declare_sample_miss_listener,
    init_zenoh_logs_from_env_or, keyexpr_as_str, keyexpr_new_try_from, open, sample_get_key_expr,
    sample_get_kind, sample_get_payload, session_declare_advanced_subscriber, zbytes_to_bytes,
    zenoh_id_to_string,
};

#[path = "common/mod.rs"]
mod common;
use common::CommonArgs;

fn main() -> Result<(), zenoh_flat::Error> {
    init_zenoh_logs_from_env_or("error");
    let args = Args::parse();

    println!("Opening session...");
    let session = open(args.common.try_into()?)?;

    let ke = keyexpr_new_try_from(args.key.clone())?;
    println!("Declaring AdvancedSubscriber on '{}'...", args.key);
    let subscriber = session_declare_advanced_subscriber(
        &session,
        ke,
        |sample| {
            let bytes = zbytes_to_bytes(sample_get_payload(&sample));
            let payload = String::from_utf8_lossy(bytes.as_ref());
            println!(
                ">> [Subscriber] Received {:?} ('{}': '{}')",
                sample_get_kind(&sample),
                keyexpr_as_str(sample_get_key_expr(&sample)),
                payload
            );
        },
        || {},
        Some(HistoryConfig {
            detect_late_publishers: true,
            ..Default::default()
        }),
        Some(RecoveryConfig {
            mode: Some(RecoveryMode::Heartbeat),
        }),
        None,       // query_timeout
        Some(true), // subscriber_detection
    )?;

    // Report samples that could not be recovered.
    let _miss_listener = advanced_subscriber_declare_sample_miss_listener(
        &subscriber,
        |miss| {
            println!(
                ">> [Subscriber] Missed {} samples from {} !!!",
                miss.nb,
                zenoh_id_to_string(&miss.source.zid)
                    .unwrap_or_else(|e| format!("<unrenderable zid: {e}>"))
            );
        },
        || {},
    )?;

    println!("Press CTRL-C to quit...");
    std::thread::park();

    Ok(())
}

#[derive(Parser, Clone, Debug)]
struct Args {
    /// The key expression to subscribe to.
    #[arg(short, long, default_value = "demo/example/**")]
    key: String,
    #[command(flatten)]
    common: CommonArgs,
}
