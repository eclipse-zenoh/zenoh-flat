use prebindgen_proc_macro::prebindgen;

#[cfg(feature = "unstable")]
use crate::ZenohId;
use crate::{Encoding, Reply, ReplyError, Sample, ZBytes};

/// Zenoh id of the node that answered, or `None` when unknown (owned handle).
///
/// Unstable: `Reply::replier_id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn reply_replier_zid(r: &Reply) -> Option<ZenohId> {
    r.replier_id().map(|id| id.zid())
}

/// Entity id of the replier (0 when the replier is unknown).
///
/// Unstable: `Reply::replier_id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn reply_replier_eid(r: &Reply) -> i32 {
    r.replier_id().map(|id| id.eid() as i32).unwrap_or(0)
}

/// `true` if this reply is a success (carries a sample), `false` if an error.
#[prebindgen]
pub fn reply_is_ok(r: &Reply) -> bool {
    r.result().is_ok()
}

/// The reply's sample on success (borrowed; valid while `r` lives), `None` on error.
#[prebindgen]
pub fn reply_get_sample(r: &Reply) -> Option<&Sample> {
    r.result().ok()
}

/// The reply's error on failure (borrowed; valid while `r` lives), `None` on
/// success.
#[prebindgen]
pub fn reply_get_err(r: &Reply) -> Option<&ReplyError> {
    r.result().err()
}

/// The error's payload (borrowed bytes).
#[prebindgen]
pub fn reply_error_get_payload(e: &ReplyError) -> &ZBytes {
    e.payload()
}

/// The error's encoding (borrowed).
#[prebindgen]
pub fn reply_error_get_encoding(e: &ReplyError) -> &Encoding {
    e.encoding()
}
