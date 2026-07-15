use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[cfg(feature = "unstable")]
use crate::ZenohId;
use crate::{Encoding, Error, KeyExpr, Publisher, ZBytes};

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
pub fn publisher_get_keyexpr(publisher: &Publisher) -> &KeyExpr {
    publisher.key_expr()
}

/// Undeclare the publisher and release its network declaration.
#[prebindgen]
pub fn publisher_undeclare(publisher: Publisher) -> Result<(), Error> {
    publisher.undeclare().wait()
}

/// Return the identifier of the node hosting this publisher.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn publisher_get_zid(publisher: &Publisher) -> ZenohId {
    publisher.id().zid()
}

/// Return the publisher's entity identifier within its session.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn publisher_get_eid(publisher: &Publisher) -> i32 {
    publisher.id().eid() as i32
}
