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

//! Shared CLI helper for the zenoh-flat examples — the flat-API counterpart of
//! upstream zenoh's `zenoh_examples::CommonArgs`. Included by each example via
//! `#[path = "common/mod.rs"] mod common;`.

#![allow(dead_code)]

use clap::Parser;
use zenoh_flat::{Config, config_insert_json5, config_new_default, config_new_from_file};

/// Connection/config flags shared by every example (same names as upstream
/// zenoh's `CommonArgs` and the zenoh-flat-c examples).
#[derive(Parser, Clone, Debug)]
pub struct CommonArgs {
    /// A configuration file.
    #[arg(short = 'c', long)]
    config: Option<String>,
    /// The zenoh session mode [peer|client|router].
    #[arg(short = 'm', long)]
    mode: Option<String>,
    /// Endpoint to connect to (repeatable).
    #[arg(short = 'e', long)]
    connect: Vec<String>,
    /// Locator to listen on (repeatable).
    #[arg(short = 'l', long)]
    listen: Vec<String>,
    /// Disable multicast scouting.
    #[arg(long = "no-multicast-scouting")]
    no_multicast_scouting: bool,
    /// Arbitrary config changes as KEY:VALUE (repeatable).
    #[arg(long = "cfg")]
    cfg: Vec<String>,
}

fn json_list(items: &[String]) -> String {
    let quoted: Vec<String> = items.iter().map(|e| format!("\"{e}\"")).collect();
    format!("[{}]", quoted.join(","))
}

impl TryFrom<CommonArgs> for Config {
    type Error = zenoh_flat::Error;

    fn try_from(a: CommonArgs) -> Result<Config, Self::Error> {
        let mut c = match &a.config {
            Some(path) => config_new_from_file(path)?,
            None => config_new_default(),
        };
        if let Some(m) = &a.mode {
            config_insert_json5(&mut c, "mode", &format!("\"{m}\""))?;
        }
        if !a.connect.is_empty() {
            config_insert_json5(&mut c, "connect/endpoints", &json_list(&a.connect))?;
        }
        if !a.listen.is_empty() {
            config_insert_json5(&mut c, "listen/endpoints", &json_list(&a.listen))?;
        }
        if a.no_multicast_scouting {
            config_insert_json5(&mut c, "scouting/multicast/enabled", "false")?;
        }
        for kv in &a.cfg {
            let (k, v) = kv.split_once(':').ok_or_else(|| -> zenoh_flat::Error {
                format!("--cfg expects KEY:VALUE, got {kv:?}").into()
            })?;
            config_insert_json5(&mut c, k, v)?;
        }
        Ok(c)
    }
}
