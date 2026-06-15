use prebindgen_proc_macro::prebindgen;
use zenoh::{Wait, config::WhatAmIMatcher};

use crate::{Config, Error, Hello, Scout, util::OnceDrop};

/// Start a scout, invoking `callback` for each hello message received.
///
/// `whatami` is a bitfield over the [`crate::WhatAmI`] variants
/// (`Router=1 | Peer=2 | Client=4`); only the low 3 bits are
/// significant, the wider type matches the JVM/`Int` matcher
/// representation. When `config` is `None` the default configuration is
/// used.
///
/// `on_close` is dropped — and therefore invoked — when the returned
/// [`Scout`] is dropped: callers wanting to be notified of scout
/// teardown should attach behavior to that drop.
///
/// Returns an opaque scout handle whose lifetime owns the running scout;
/// dropping it stops the scout and triggers `on_close`.
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
