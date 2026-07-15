use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[cfg(feature = "unstable")]
use crate::ZenohId;
use crate::{Error, KeyExpr, Queryable};

/// Return the key expression on which this queryable answers.
#[prebindgen]
pub fn queryable_get_keyexpr(queryable: &Queryable) -> &KeyExpr {
    queryable.key_expr()
}

/// Undeclare the queryable and stop query delivery.
///
/// The close callback registered at declaration is called when the queryable
/// ends.
#[prebindgen]
pub fn queryable_undeclare(queryable: Queryable) -> Result<(), Error> {
    queryable.undeclare().wait()
}

/// Return the identifier of the node hosting this queryable.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn queryable_get_zid(queryable: &Queryable) -> ZenohId {
    queryable.id().zid()
}

/// Return the queryable's entity identifier within its session.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn queryable_get_eid(queryable: &Queryable) -> i32 {
    queryable.id().eid() as i32
}
