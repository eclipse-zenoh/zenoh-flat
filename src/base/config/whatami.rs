use prebindgen_proc_macro::prebindgen;

/// The role of a node in a Zenoh network.
///
/// The discriminants are bit flags, not an ordinal sequence: they are combined
/// with a bitwise or into the bitfield that [`crate::scout`] accepts. The
/// `repr` is therefore part of what this type means and is kept, unlike the
/// crate's ordinary enumerations, whose lowering each binding chooses.
#[prebindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatAmI {
    /// A node that routes traffic between other nodes.
    Router = 1,
    /// A node that communicates directly with peers and routers.
    Peer = 2,
    /// A node that connects through a router or peer.
    Client = 4,
}

impl From<zenoh::config::WhatAmI> for WhatAmI {
    fn from(w: zenoh::config::WhatAmI) -> Self {
        match w {
            zenoh::config::WhatAmI::Router => WhatAmI::Router,
            zenoh::config::WhatAmI::Peer => WhatAmI::Peer,
            zenoh::config::WhatAmI::Client => WhatAmI::Client,
        }
    }
}
