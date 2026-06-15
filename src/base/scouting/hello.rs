use prebindgen_proc_macro::prebindgen;

use crate::{Hello, WhatAmI, ZenohId};

/// Node type that emitted this hello message.
#[prebindgen]
pub fn hello_get_whatami(h: &Hello) -> WhatAmI {
    h.whatami().into()
}

/// Zenoh id of the node that emitted this hello message.
#[prebindgen]
pub fn hello_get_zid(h: &Hello) -> ZenohId {
    h.zid()
}

/// Locators advertised in this hello message.
#[prebindgen]
pub fn hello_get_locators(h: &Hello) -> Vec<String> {
    h.locators().iter().map(|l| l.to_string()).collect()
}
