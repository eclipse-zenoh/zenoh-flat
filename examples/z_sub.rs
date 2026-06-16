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

//! Flat-API port of eclipse-zenoh/zenoh `examples/examples/z_sub.rs`.
//!
//! The flat API is callback-based: the subscriber delivers each sample to the
//! closure passed to `session_declare_subscriber`. The returned handle owns the
//! subscription, so it is kept alive while the main thread parks.

use clap::Parser;
use zenoh_flat::{
    init_zenoh_logs_from_env_or, keyexpr_new_try_from, open, sample_get_attachment,
    sample_get_key_expr, sample_get_kind, sample_get_payload, session_declare_subscriber,
    zbytes_as_bytes,
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
    println!("Declaring Subscriber on '{}'...", args.key);
    let _subscriber = session_declare_subscriber(
        &session,
        ke,
        |sample| {
            let bytes = zbytes_as_bytes(sample_get_payload(&sample));
            let payload = String::from_utf8_lossy(bytes.as_ref());
            print!(
                ">> [Subscriber] Received {:?} ('{}': '{}')",
                sample_get_kind(&sample),
                zenoh_flat::keyexpr_get_str(sample_get_key_expr(&sample)),
                payload
            );
            if let Some(att) = sample_get_attachment(&sample) {
                print!(
                    " ({})",
                    String::from_utf8_lossy(zbytes_as_bytes(att).as_ref())
                );
            }
            println!();
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
