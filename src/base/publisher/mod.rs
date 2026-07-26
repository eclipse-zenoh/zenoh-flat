use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[cfg(feature = "unstable")]
use crate::EntityGlobalId;
#[cfg(feature = "unstable")]
use crate::Reliability;
use crate::{
    CongestionControl, Encoding, Error, KeyExpr, MatchingListener, Priority, Publisher, ZBytes,
    util::OnceDrop,
};

/// Publish data on the publisher's key expression.
///
/// The encoding applies to this publication, while the publisher's configured
/// delivery settings remain in effect. The attachment carries user-defined
/// metadata.
#[prebindgen]
pub fn publisher_put(
    publisher: &Publisher,
    payload: ZBytes,
    encoding: Option<&Encoding>,
    attachment: Option<ZBytes>,
) -> Result<(), Error> {
    let mut publication = publisher.put(payload);
    if let Some(enc) = encoding {
        publication = publication.encoding(enc.clone());
    }
    if let Some(att) = attachment {
        publication = publication.attachment(att);
    }
    publication.wait()
}

/// Publish a deletion notification on the publisher's key expression.
///
/// Matching subscribers receive a DELETE sample. The attachment carries
/// user-defined metadata.
#[prebindgen]
pub fn publisher_delete(publisher: &Publisher, attachment: Option<ZBytes>) -> Result<(), Error> {
    let mut delete = publisher.delete();
    if let Some(att) = attachment {
        delete = delete.attachment(att);
    }
    delete.wait()
}

/// Return the key expression on which this publisher publishes.
#[prebindgen]
pub fn publisher_get_key_expr(publisher: &Publisher) -> &KeyExpr {
    publisher.key_expr()
}

/// Return the congestion-control policy this publisher was declared with.
///
/// The declaration takes this as an option and falls back to base's default, so
/// this is the only way to learn what the publisher actually got.
#[prebindgen]
pub fn publisher_get_congestion_control(publisher: &Publisher) -> CongestionControl {
    publisher.congestion_control().into()
}

/// Return the priority this publisher was declared with.
#[prebindgen]
pub fn publisher_get_priority(publisher: &Publisher) -> Priority {
    publisher.priority().into()
}

/// Return the encoding applied to publications that do not supply one.
#[prebindgen]
pub fn publisher_get_encoding(publisher: &Publisher) -> &Encoding {
    publisher.encoding()
}

/// Return the reliability policy this publisher was declared with.
///
/// Available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn publisher_get_reliability(publisher: &Publisher) -> Reliability {
    publisher.reliability().into()
}

/// Return whether the publisher currently has matching subscribers.
///
/// Answering this before producing an expensive payload is the point: there is
/// no need to build what nothing is listening for.
#[prebindgen]
pub fn publisher_matching_status(publisher: &Publisher) -> Result<bool, Error> {
    Ok(publisher.matching_status().wait()?.matching())
}

/// Declare a matching listener that is notified when the publisher's matching
/// status changes.
///
/// The callback receives the new matching status (`true` if matching
/// subscribers exist). The close callback is called when the listener ends.
#[prebindgen]
pub fn publisher_declare_matching_listener(
    publisher: &Publisher,
    callback: impl Fn(bool) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<MatchingListener, Error> {
    let on_close = OnceDrop::new(on_close);
    publisher
        .matching_listener()
        .callback(move |status| {
            let _ = &on_close;
            callback(status.matching());
        })
        .wait()
}

/// Declare a background matching listener that runs until the publisher is
/// undeclared.
#[prebindgen]
pub fn publisher_declare_background_matching_listener(
    publisher: &Publisher,
    callback: impl Fn(bool) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    publisher
        .matching_listener()
        .callback(move |status| {
            let _ = &on_close;
            callback(status.matching());
        })
        .background()
        .wait()
}

/// Undeclare the publisher and release its network declaration.
#[prebindgen]
pub fn publisher_undeclare(publisher: Publisher) -> Result<(), Error> {
    publisher.undeclare().wait()
}

/// Return the global identifier of this publisher.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn publisher_get_id(publisher: &Publisher) -> EntityGlobalId {
    publisher.id().into()
}
