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
        // Delegate to the field accessors so each field has one definition.
        HelloStruct {
            whatami: hello_get_whatami(h),
            zid: hello_get_zid(h),
            locators: hello_get_locators(h),
        }
    }
}

/// Decompose a hello message into its [`HelloStruct`] value form.
#[prebindgen]
pub fn hello_to_struct(h: &Hello) -> HelloStruct {
    h.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every field of the value form equals the accessor for that same field —
    /// the guard for "one source of truth per field".
    ///
    /// A `Hello` only ever arrives from the network, and the only constructor
    /// base zenoh offers is `Hello::empty()`, so the subject carries default
    /// values. That limits what this can catch compared with the sample guard
    /// (which builds a fully non-default subject): a re-derivation that
    /// hardcoded a default would still agree here. It does still pin the three
    /// fields to their accessors, which is what the rule is about.
    #[test]
    fn struct_mirrors_accessors() {
        let h = Hello::empty();
        let hs = hello_to_struct(&h);
        assert_eq!(hs.whatami, hello_get_whatami(&h));
        assert_eq!(hs.zid, hello_get_zid(&h));
        assert_eq!(hs.locators, hello_get_locators(&h));
    }
}
