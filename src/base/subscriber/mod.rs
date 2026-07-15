use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[cfg(feature = "unstable")]
use crate::ZenohId;
use crate::{Error, KeyExpr, Subscriber};

/// Return the key expression on which this subscriber listens.
#[prebindgen]
pub fn subscriber_get_keyexpr(subscriber: &Subscriber) -> &KeyExpr {
    subscriber.key_expr()
}

/// Undeclare the subscriber and stop sample delivery.
///
/// The close callback registered at declaration is called when the subscriber
/// ends.
#[prebindgen]
pub fn subscriber_undeclare(subscriber: Subscriber) -> Result<(), Error> {
    subscriber.undeclare().wait()
}

/// Return the identifier of the node hosting this subscriber.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn subscriber_get_zid(subscriber: &Subscriber) -> ZenohId {
    subscriber.id().zid()
}

/// Return the subscriber's entity identifier within its session.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn subscriber_get_eid(subscriber: &Subscriber) -> i32 {
    subscriber.id().eid() as i32
}
