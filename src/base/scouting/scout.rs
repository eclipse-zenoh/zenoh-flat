use prebindgen_proc_macro::prebindgen;
use zenoh::{Wait, config::WhatAmIMatcher};

use crate::{Config, Error, Hello, Scout, util::OnceDrop};

/// Discover Zenoh nodes and report each received hello message.
///
/// `whatami` combines the node kinds to discover: router (`1`), peer (`2`),
/// and client (`4`). A raw bitfield is used here deliberately: zenoh's
/// `WhatAmIMatcher` is a Rust-specific type that does not translate cleanly to
/// other languages. When no configuration is supplied, the default scouting
/// configuration is used.
///
/// The close callback is called when scouting ends.
#[prebindgen]
pub fn scout(
    whatami: i32,
    config: Option<&Config>,
    callback: impl Fn(Hello) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<Scout, Error> {
    let bits = u8::try_from(whatami)
        .map_err(|_| -> Error { format!("invalid whatami bitfield: {whatami}").into() })?;
    let matcher: WhatAmIMatcher = bits
        .try_into()
        .map_err(|_| -> Error { format!("invalid whatami bitfield: 0b{bits:b}").into() })?;
    let config = config.cloned().unwrap_or_default();
    let on_close = OnceDrop::new(on_close);
    zenoh::scout(matcher, config)
        .callback(move |hello| {
            let _ = &on_close;
            callback(hello);
        })
        .wait()
}
