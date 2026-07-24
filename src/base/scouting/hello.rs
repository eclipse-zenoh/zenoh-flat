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

/// A discovery announcement decomposed into a plain value.
#[prebindgen]
#[derive(Clone, Debug)]
pub struct HelloStruct {
    /// Node type that emitted this hello message.
    pub whatami: WhatAmI,
    /// Zenoh id of the node that emitted this hello message.
    pub zid: ZenohId,
    /// Locators advertised in this hello message.
    pub locators: Vec<String>,
}

impl From<&Hello> for HelloStruct {
    fn from(h: &Hello) -> Self {
        HelloStruct {
            whatami: h.whatami().into(),
            zid: h.zid(),
            locators: h.locators().iter().map(|l| l.to_string()).collect(),
        }
    }
}

/// Decompose a hello message into its [`HelloStruct`] value form.
#[prebindgen]
pub fn hello_to_struct(h: &Hello) -> HelloStruct {
    h.into()
}
