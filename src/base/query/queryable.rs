use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[cfg(feature = "unstable")]
use crate::ZenohId;
use crate::{Error, KeyExpr, Queryable};

/// Key expression the queryable answers on (borrowed; valid while `queryable`
/// lives).
#[prebindgen]
pub fn queryable_get_keyexpr(queryable: &Queryable) -> &KeyExpr {
    queryable.key_expr()
}

/// Undeclare a queryable, stopping query delivery and releasing its network
/// declaration — the flat port of `zenoh::query::Queryable::undeclare`. Consumes
/// the handle; its `on_close` callback fires as it is torn down.
#[prebindgen]
pub fn queryable_undeclare(queryable: Queryable) -> Result<(), Error> {
    queryable.undeclare().wait()
}

/// Zenoh id of the node hosting this queryable (the `zid` of its entity global
/// id).
///
/// Unstable: `zenoh::query::Queryable::id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn queryable_zid(queryable: &Queryable) -> ZenohId {
    queryable.id().zid()
}

/// Entity id of this queryable (the per-session part of its entity global id).
///
/// Unstable: `zenoh::query::Queryable::id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn queryable_eid(queryable: &Queryable) -> i32 {
    queryable.id().eid() as i32
}
