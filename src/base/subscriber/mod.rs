use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[cfg(feature = "unstable")]
use crate::ZenohId;
use crate::{Error, KeyExpr, Subscriber};

/// Key expression the subscriber listens on (borrowed; valid while `subscriber`
/// lives).
#[prebindgen]
pub fn subscriber_get_keyexpr(subscriber: &Subscriber) -> &KeyExpr {
    subscriber.key_expr()
}

/// Undeclare a subscriber, stopping delivery and releasing its network
/// declaration — the flat port of `zenoh::pubsub::Subscriber::undeclare`.
/// Consumes the handle; its `on_close` callback fires as it is torn down.
#[prebindgen]
pub fn subscriber_undeclare(subscriber: Subscriber) -> Result<(), Error> {
    subscriber.undeclare().wait()
}

/// Zenoh id of the node hosting this subscriber (the `zid` of its entity global
/// id).
///
/// Unstable: `zenoh::pubsub::Subscriber::id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn subscriber_zid(subscriber: &Subscriber) -> ZenohId {
    subscriber.id().zid()
}

/// Entity id of this subscriber (the per-session part of its entity global id).
///
/// Unstable: `zenoh::pubsub::Subscriber::id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn subscriber_eid(subscriber: &Subscriber) -> i32 {
    subscriber.id().eid() as i32
}
