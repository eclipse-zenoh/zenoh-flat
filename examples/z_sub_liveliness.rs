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

//! Flat-API port of eclipse-zenoh/zenoh `examples/examples/z_sub_liveliness.rs`.

use clap::Parser;
use zenoh_flat::{
    SampleKind, init_zenoh_logs_from_env_or, keyexpr_get_str, keyexpr_new_try_from,
    liveliness_declare_subscriber, open, sample_get_key_expr, sample_get_kind,
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
    println!("Declaring Liveliness Subscriber on '{}'...", args.key);
    let _subscriber = liveliness_declare_subscriber(
        &session,
        ke,
        args.history,
        |sample| {
            let ke = keyexpr_get_str(sample_get_key_expr(&sample));
            match sample_get_kind(&sample) {
                SampleKind::Put => {
                    println!(">> [LivelinessSubscriber] New alive token ('{ke}')")
                }
                SampleKind::Delete => {
                    println!(">> [LivelinessSubscriber] Dropped token ('{ke}')")
                }
            }
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
    #[arg(short, long, default_value = "group1/**")]
    key: String,
    /// Get historical liveliness tokens.
    #[arg(long)]
    history: bool,
    #[command(flatten)]
    common: CommonArgs,
}
