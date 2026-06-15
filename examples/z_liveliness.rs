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

//! Flat-API port of eclipse-zenoh/zenoh `examples/examples/z_liveliness.rs`.

use clap::Parser;
use zenoh_flat::{
    init_zenoh_logs_from_env_or, keyexpr_new_try_from, liveliness_declare_token,
    liveliness_undeclare_token, open,
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
    println!("Declaring LivelinessToken on '{}'...", args.key);
    let token = liveliness_declare_token(&session, ke)?;

    println!("Press CTRL-C to undeclare LivelinessToken and quit...");
    std::thread::park();

    // Reached only if the thread is unparked; tokens also undeclare on drop.
    liveliness_undeclare_token(token)?;

    Ok(())
}

#[derive(Parser, Clone, Debug)]
struct Args {
    /// The key expression of the liveliness token.
    #[arg(short, long, default_value = "group1/zenoh-rs")]
    key: String,
    #[command(flatten)]
    common: CommonArgs,
}
