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

//! Flat-API port of eclipse-zenoh/zenoh `examples/examples/z_pub.rs`.

use std::{thread::sleep, time::Duration};

use clap::Parser;
use zenoh_flat::{
    encoding_const_text_plain, init_zenoh_logs_from_env_or, keyexpr_new_try_from, open,
    publisher_put, session_declare_publisher, zbytes_new_from_slice,
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
    println!("Declaring Publisher on '{}'...", args.key);
    let publisher = session_declare_publisher(
        &session,
        ke,
        None,
        None,
        None,
        None,
        #[cfg(feature = "unstable")]
        None,
    )?;

    println!("Press CTRL-C to quit...");
    for idx in 0..u32::MAX {
        sleep(Duration::from_secs(1));
        let buf = format!("[{idx:4}] {}", args.payload);
        println!("Putting Data ('{}': '{}')...", args.key, buf);
        let attachment = args
            .attach
            .as_ref()
            .map(|a| zbytes_new_from_slice(a.as_bytes()));
        publisher_put(
            &publisher,
            zbytes_new_from_slice(buf.as_bytes()),
            Some(encoding_const_text_plain()),
            attachment,
        )?;
    }

    Ok(())
}

#[derive(Parser, Clone, Debug)]
struct Args {
    /// The key expression to write to.
    #[arg(short, long, default_value = "demo/example/zenoh-rs-pub")]
    key: String,
    /// The payload to write.
    #[arg(short, long, default_value = "Pub from Rust!")]
    payload: String,
    /// The attachment to add to each put.
    #[arg(short, long)]
    attach: Option<String>,
    #[command(flatten)]
    common: CommonArgs,
}
