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

//! Flat-API port of eclipse-zenoh/zenoh `examples/examples/z_delete.rs`.

use clap::Parser;
use zenoh_flat::{init_zenoh_logs_from_env_or, keyexpr_new_try_from, open, session_close, session_delete};

#[path = "common/mod.rs"]
mod common;
use common::CommonArgs;

fn main() {
    init_zenoh_logs_from_env_or("error");
    let args = Args::parse();

    println!("Opening session...");
    let session = open(args.common.into()).unwrap_or_else(|e| panic!("{e}"));

    let ke = keyexpr_new_try_from(args.key.clone()).unwrap_or_else(|e| panic!("{e}"));
    println!("Deleting resources matching '{}'...", args.key);
    session_delete(
        &session,
        &ke,
        None,
        None,
        None,
        None,
        #[cfg(feature = "unstable")]
        None,
    )
    .unwrap_or_else(|e| panic!("{e}"));

    session_close(&session).unwrap_or_else(|e| panic!("{e}"));
}

#[derive(Parser, Clone, Debug)]
struct Args {
    /// The key expression to delete.
    #[arg(short, long, default_value = "demo/example/zenoh-rs-put")]
    key: String,
    #[command(flatten)]
    common: CommonArgs,
}
