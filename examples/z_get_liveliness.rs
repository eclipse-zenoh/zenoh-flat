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

//! Flat-API port of eclipse-zenoh/zenoh `examples/examples/z_get_liveliness.rs`.

use std::sync::mpsc;

use clap::Parser;
use zenoh_flat::{
    init_zenoh_logs_from_env_or, keyexpr_get_str, keyexpr_new_try_from, liveliness_get, open,
    reply_error_get_payload, reply_get_err, reply_get_sample, sample_get_key_expr, zbytes_as_bytes,
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
    println!("Sending Liveliness Query '{}'...", args.key);

    let (tx, rx) = mpsc::channel::<()>();
    liveliness_get(
        &session,
        &ke,
        args.timeout as i64,
        |reply| {
            if let Some(sample) = reply_get_sample(&reply) {
                println!(
                    ">> Alive token ('{}')",
                    keyexpr_get_str(sample_get_key_expr(sample))
                );
            } else if let Some(err) = reply_get_err(&reply) {
                let bytes = zbytes_as_bytes(reply_error_get_payload(err));
                println!(
                    ">> Received (ERROR: '{}')",
                    String::from_utf8_lossy(bytes.as_ref())
                );
            }
        },
        move || {
            let _ = tx.send(());
        },
    )?;

    let _ = rx.recv();

    Ok(())
}

#[derive(Parser, Clone, Debug)]
struct Args {
    /// The key expression matching liveliness tokens to query.
    #[arg(short, long, default_value = "group1/**")]
    key: String,
    /// The query timeout in milliseconds.
    #[arg(short = 'o', long, default_value = "10000")]
    timeout: u64,
    #[command(flatten)]
    common: CommonArgs,
}
