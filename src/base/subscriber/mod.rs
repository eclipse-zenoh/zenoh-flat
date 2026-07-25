use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[cfg(feature = "unstable")]
use crate::EntityGlobalId;
use crate::{Error, KeyExpr, Subscriber};

/// Return the key expression on which this subscriber listens.
#[prebindgen]
pub fn subscriber_get_key_expr(subscriber: &Subscriber) -> &KeyExpr {
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

/// Return the global identifier of this subscriber.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn subscriber_get_id(subscriber: &Subscriber) -> EntityGlobalId {
    subscriber.id().into()
}
