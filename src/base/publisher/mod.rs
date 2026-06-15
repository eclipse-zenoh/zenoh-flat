use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[cfg(feature = "unstable")]
use crate::ZenohId;
use crate::{Encoding, Error, KeyExpr, Publisher, ZBytes};

/// Publish a payload on the publisher's key expression — the flat port of
/// `zenoh::pubsub::Publisher::put`. `encoding` overrides the publisher's default
/// for this message only; `attachment` carries optional user metadata. The
/// publisher's configured QoS (priority, congestion control, express,
/// reliability) applies automatically.
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

/// Publish a delete (tombstone) on the publisher's key expression — the flat
/// port of `zenoh::pubsub::Publisher::delete`. Subscribers receive it as a
/// `SampleKind::Delete` sample. `attachment` carries optional user metadata.
#[prebindgen]
pub fn publisher_delete(publisher: &Publisher, attachment: Option<ZBytes>) -> Result<(), Error> {
    let mut delete = publisher.delete();
    if let Some(att) = attachment {
        delete = delete.attachment(att);
    }
    delete.wait()
}

/// Key expression the publisher publishes on (borrowed; valid while `publisher`
/// lives).
#[prebindgen]
pub fn publisher_keyexpr(publisher: &Publisher) -> &KeyExpr {
    publisher.key_expr()
}

/// Undeclare a publisher, releasing its network declaration — the flat port of
/// `zenoh::pubsub::Publisher::undeclare`. Consumes the handle.
#[prebindgen]
pub fn publisher_undeclare(publisher: Publisher) -> Result<(), Error> {
    publisher.undeclare().wait()
}

/// Zenoh id of the node hosting this publisher (the `zid` of its entity global
/// id).
///
/// Unstable: `zenoh::pubsub::Publisher::id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn publisher_zid(publisher: &Publisher) -> ZenohId {
    publisher.id().zid()
}

/// Entity id of this publisher (the per-session part of its entity global id).
///
/// Unstable: `zenoh::pubsub::Publisher::id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn publisher_eid(publisher: &Publisher) -> i32 {
    publisher.id().eid() as i32
}
