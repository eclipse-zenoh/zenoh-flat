use prebindgen_proc_macro::prebindgen;

#[cfg(feature = "unstable")]
use crate::ZenohId;
use crate::{Encoding, Reply, ReplyError, Sample, ZBytes};

/// Return the identifier of the node that answered, when known.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn reply_get_replier_zid(r: &Reply) -> Option<ZenohId> {
    r.replier_id().map(|id| id.zid())
}

/// Return the answering entity's identifier, or `0` when unknown.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn reply_get_replier_eid(r: &Reply) -> i32 {
    r.replier_id().map(|id| id.eid() as i32).unwrap_or(0)
}

/// Return whether this reply contains a sample rather than an error.
#[prebindgen]
pub fn reply_is_ok(r: &Reply) -> bool {
    r.result().is_ok()
}

/// Return the sample carried by a successful reply.
#[prebindgen]
pub fn reply_get_sample(r: &Reply) -> Option<&Sample> {
    r.result().ok()
}

/// Return the application error carried by an unsuccessful reply.
#[prebindgen]
pub fn reply_get_err(r: &Reply) -> Option<&ReplyError> {
    r.result().err()
}

/// Return the error payload.
#[prebindgen]
pub fn reply_error_get_payload(e: &ReplyError) -> &ZBytes {
    e.payload()
}

/// Return format information associated with the error payload.
#[prebindgen]
pub fn reply_error_get_encoding(e: &ReplyError) -> &Encoding {
    e.encoding()
}
