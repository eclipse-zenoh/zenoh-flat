use prebindgen_proc_macro::prebindgen;

#[cfg(feature = "unstable")]
use crate::ZZenohId;
use crate::{ZEncoding, ZReply, ZReplyError, ZSample, ZZBytes};

/// Zenoh id of the node that answered, or `None` when unknown (owned handle).
///
/// Unstable: `Reply::replier_id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn z_reply_replier_zid(r: &ZReply) -> Option<ZZenohId> {
    r.replier_id().map(|id| id.zid())
}

/// Entity id of the replier (0 when the replier is unknown).
///
/// Unstable: `Reply::replier_id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn z_reply_replier_eid(r: &ZReply) -> i32 {
    r.replier_id().map(|id| id.eid() as i32).unwrap_or(0)
}

/// `true` if this reply is a success (carries a sample), `false` if an error.
#[prebindgen]
pub fn z_reply_is_ok(r: &ZReply) -> bool {
    r.result().is_ok()
}

/// The reply's sample on success (borrowed; valid while `r` lives), `None` on error.
#[prebindgen]
pub fn z_reply_sample(r: &ZReply) -> Option<&ZSample> {
    r.result().ok()
}

/// The reply's error on failure (borrowed; valid while `r` lives), `None` on
/// success.
#[prebindgen]
pub fn z_reply_err(r: &ZReply) -> Option<&ZReplyError> {
    r.result().err()
}

/// The error's payload (borrowed bytes).
#[prebindgen]
pub fn z_reply_error_payload(e: &ZReplyError) -> &ZZBytes {
    e.payload()
}

/// The error's encoding (borrowed).
#[prebindgen]
pub fn z_reply_error_encoding(e: &ZReplyError) -> &ZEncoding {
    e.encoding()
}
