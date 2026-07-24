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

//! Flat-API port of eclipse-zenoh/zenoh `examples/examples/z_get.rs`.
//!
//! `session_get` delivers each reply to a callback and fires `on_close` once the
//! reply stream ends; the main thread blocks on a channel until that happens.

use std::sync::mpsc;

use clap::Parser;
use zenoh_flat::{
    QueryTarget, Selector, init_zenoh_logs_from_env_or, keyexpr_get_str, keyexpr_new_try_from,
    open, reply_error_get_payload, reply_get_err, reply_get_sample, sample_get_key_expr,
    sample_get_payload, session_get, zbytes_as_bytes, zbytes_new_from_slice,
};

#[path = "common/mod.rs"]
mod common;
use common::CommonArgs;

fn main() -> Result<(), zenoh_flat::Error> {
    init_zenoh_logs_from_env_or("error");
    let args = Args::parse();

    // A selector is `key_expr[?parameters]`.
    let (key, params) = match args.selector.split_once('?') {
        Some((k, p)) => (k.to_string(), Some(p.to_string())),
        None => (args.selector.clone(), None),
    };

    println!("Opening session...");
    let session = open(args.common.try_into()?)?;

    let ke = keyexpr_new_try_from(key)?;
    println!("Sending Query '{}'...", args.selector);

    let (tx, rx) = mpsc::channel::<()>();
    session_get(
        &session,
        Selector {
            key_expr: ke,
            parameters: params.unwrap_or_default(),
        },
        Some(args.timeout as i64),
        Some(args.target.into()),
        None,
        None,
        None,
        None,
        None,
        args.payload.map(|p| zbytes_new_from_slice(p.as_bytes())),
        None,
        None,
        |reply| {
            if let Some(sample) = reply_get_sample(&reply) {
                let bytes = zbytes_as_bytes(sample_get_payload(sample));
                let payload = String::from_utf8_lossy(bytes.as_ref());
                println!(
                    ">> Received ('{}': '{}')",
                    keyexpr_get_str(sample_get_key_expr(sample)),
                    payload
                );
            } else if let Some(err) = reply_get_err(&reply) {
                let bytes = zbytes_as_bytes(reply_error_get_payload(err));
                let payload = String::from_utf8_lossy(bytes.as_ref());
                println!(">> Received (ERROR: '{payload}')");
            }
        },
        move || {
            let _ = tx.send(());
        },
    )?;

    // Block until the reply stream is exhausted.
    let _ = rx.recv();

    Ok(())
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
#[value(rename_all = "SCREAMING_SNAKE_CASE")]
enum Qt {
    BestMatching,
    All,
    AllComplete,
}

impl From<Qt> for QueryTarget {
    fn from(t: Qt) -> Self {
        match t {
            Qt::BestMatching => QueryTarget::BestMatching,
            Qt::All => QueryTarget::All,
            Qt::AllComplete => QueryTarget::AllComplete,
        }
    }
}

#[derive(Parser, Clone, Debug)]
struct Args {
    /// The selection of resources to query.
    #[arg(short, long, default_value = "demo/example/**")]
    selector: String,
    /// An optional payload to put in the query.
    #[arg(short, long)]
    payload: Option<String>,
    /// The target queryables of the query.
    #[arg(short, long, default_value = "BEST_MATCHING")]
    target: Qt,
    /// The query timeout in milliseconds.
    #[arg(short = 'o', long, default_value = "10000")]
    timeout: u64,
    #[command(flatten)]
    common: CommonArgs,
}
