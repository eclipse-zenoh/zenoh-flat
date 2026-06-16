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

//! Flat-API port of eclipse-zenoh/zenoh `examples/examples/z_queryable.rs`.

use clap::Parser;
use zenoh_flat::{
    init_zenoh_logs_from_env_or, keyexpr_new_try_from, open, query_get_parameters,
    query_get_payload, query_reply_success, session_declare_queryable, zbytes_as_bytes,
    zbytes_new_from_slice,
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
    println!("Declaring Queryable on '{}'...", args.key);

    let key = args.key.clone();
    let payload = args.payload.clone();
    let _queryable = session_declare_queryable(
        &session,
        ke,
        Some(args.complete),
        move |query| {
            match query_get_payload(&query) {
                None => println!(
                    ">> [Queryable] Received Query '{}'",
                    query_get_parameters(&query)
                ),
                Some(p) => println!(
                    ">> [Queryable] Received Query (params: '{}') with payload '{}'",
                    query_get_parameters(&query),
                    String::from_utf8_lossy(zbytes_as_bytes(p).as_ref())
                ),
            }
            println!(">> [Queryable] Responding ('{key}': '{payload}')");
            let reply_ke = keyexpr_new_try_from(key.clone()).expect("reply key expr");
            query_reply_success(
                &query,
                &reply_ke,
                zbytes_new_from_slice(payload.as_bytes()),
                None,
                None,
                None,
                None,
            )
            .unwrap_or_else(|e| println!(">> [Queryable] Error sending reply: {e}"));
        },
        || {},
    )?;

    println!("Press CTRL-C to quit...");
    std::thread::park();

    Ok(())
}

#[derive(Parser, Clone, Debug)]
struct Args {
    /// The key expression matching queries to reply to.
    #[arg(short, long, default_value = "demo/example/zenoh-rs-queryable")]
    key: String,
    /// The payload to reply to queries.
    #[arg(short, long, default_value = "Queryable from Rust!")]
    payload: String,
    /// Declare the queryable as complete w.r.t. the key expression.
    #[arg(long)]
    complete: bool,
    #[command(flatten)]
    common: CommonArgs,
}
