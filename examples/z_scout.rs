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

//! Flat-API port of eclipse-zenoh/zenoh `examples/examples/z_scout.rs`.

use std::{thread::sleep, time::Duration};

use zenoh_flat::{
    WhatAmI, hello_get_locators, hello_get_whatami, hello_get_zid, init_zenoh_logs_from_env_or,
    scout, zenoh_id_to_string,
};

fn main() -> Result<(), zenoh_flat::Error> {
    init_zenoh_logs_from_env_or("error");

    println!("Scouting...");
    // whatami bitfield: Peer | Router.
    let whatami = WhatAmI::Peer as i32 | WhatAmI::Router as i32;
    let _scout = scout(
        whatami,
        None,
        |hello| {
            println!(
                "Hello {{ whatami: {:?}, zid: {}, locators: {:?} }}",
                hello_get_whatami(&hello),
                zenoh_id_to_string(&hello_get_zid(&hello)),
                hello_get_locators(&hello),
            );
        },
        || {},
    )?;

    // Scout for one second, then stop (dropping the handle).
    sleep(Duration::from_secs(1));

    Ok(())
}
