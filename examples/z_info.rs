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

//! Flat-API port of eclipse-zenoh/zenoh `examples/examples/z_info.rs`.
//!
//! The flat API exposes the session's own/peer/router ids; the unstable
//! transport/link listings of the upstream example have no flat counterpart.

use clap::Parser;
use zenoh_flat::{
    init_zenoh_logs_from_env_or, open, session_get_peers_zid, session_get_routers_zid,
    session_get_zid, zenoh_id_to_string,
};

#[path = "common/mod.rs"]
mod common;
use common::CommonArgs;

fn main() -> Result<(), zenoh_flat::Error> {
    init_zenoh_logs_from_env_or("error");
    let args = Args::parse();

    println!("Opening session...");
    let session = open(args.common.try_into()?)?;

    println!("zid: {}", zenoh_id_to_string(&session_get_zid(&session)));
    let routers: Vec<String> = session_get_routers_zid(&session)
        .iter()
        .map(zenoh_id_to_string)
        .collect();
    println!("routers zid: {routers:?}");
    let peers: Vec<String> = session_get_peers_zid(&session)
        .iter()
        .map(zenoh_id_to_string)
        .collect();
    println!("peers zid: {peers:?}");

    Ok(())
}

#[derive(Parser, Clone, Debug)]
struct Args {
    #[command(flatten)]
    common: CommonArgs,
}
