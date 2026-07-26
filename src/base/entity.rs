use prebindgen_proc_macro::prebindgen;

use crate::ZenohId;

/// Global identifier of an entity (publisher, subscriber, …) in a Zenoh system:
/// the node's [`ZenohId`] plus the entity's per-session id.
///
/// This information is available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityGlobalId {
    /// Identifier of the node the entity belongs to.
    pub zid: ZenohId,
    /// Entity identifier within its session.
    pub eid: u32,
}

impl From<zenoh::session::EntityGlobalId> for EntityGlobalId {
    fn from(id: zenoh::session::EntityGlobalId) -> Self {
        EntityGlobalId {
            zid: id.zid(),
            eid: id.eid(),
        }
    }
}
