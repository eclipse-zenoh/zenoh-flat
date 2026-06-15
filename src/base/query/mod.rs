pub(crate) mod consolidation_mode;
pub(crate) mod querier;
pub(crate) mod query_target;
pub(crate) mod queryable;
pub(crate) mod reply;
pub(crate) mod reply_key_expr;

use prebindgen_proc_macro::prebindgen;
use zenoh::{
    Wait,
    time::{NTP64, Timestamp, TimestampId},
};

#[cfg(feature = "unstable")]
use self::reply_key_expr::ReplyKeyExpr;
use crate::{Encoding, Error, KeyExpr, Query, Sample, ZBytes};

/// Key expression the query targets (borrowed; valid while `q` lives).
#[prebindgen]
pub fn query_get_keyexpr(q: &Query) -> &KeyExpr {
    q.key_expr()
}

/// Query selector parameters as an owned string (empty when none).
#[prebindgen]
pub fn query_get_parameters(q: &Query) -> String {
    q.parameters().as_str().to_string()
}

/// Query payload (borrowed bytes), or `None` when the query carries none.
#[prebindgen]
pub fn query_get_payload(q: &Query) -> Option<&ZBytes> {
    q.payload()
}

/// Encoding of the query payload (borrowed), or `None`.
#[prebindgen]
pub fn query_get_encoding(q: &Query) -> Option<&Encoding> {
    q.encoding()
}

/// Attachment carried by the query (borrowed bytes), or `None`.
#[prebindgen]
pub fn query_get_attachment(q: &Query) -> Option<&ZBytes> {
    q.attachment()
}

/// The [`crate::ReplyKeyExpr`] policy the querier accepts for replies.
/// Unstable: `zenoh::query::Query::accepts_replies` is `#[cfg(feature = "unstable")]`.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn query_get_accepts_replies(q: &Query) -> ReplyKeyExpr {
    q.accepts_replies().into()
}

/// Reply to a query with a fully-formed [`Sample`] — the flat port of zenoh's
/// `Query::reply_sample`. The sample is sent as-is, preserving its kind (Put or
/// Delete) and all carried metadata (payload, encoding, timestamp, attachment,
/// QoS, source info).
///
/// The flat consumer of `sample_new_put`: its `sample` parameter is a by-value
/// `Sample`, so its canonical input (`sample_new_put`) recursively expands at the
/// binding boundary — the recursive-input demonstration.
#[prebindgen]
pub fn query_reply_sample(query: &Query, sample: Sample) -> Result<(), Error> {
    query.reply_sample(sample).wait()
}

/// Reply to a query with a successful PUT sample built from its parts — the flat
/// port of `zenoh::query::Query::reply`. `encoding`, `timestamp_ntp64`,
/// `attachment`, and `express` are optional; omitting `timestamp_ntp64` lets the
/// network assign one. Use [`query_reply_sample`] to forward a ready-made
/// [`Sample`] instead.
#[prebindgen]
pub fn query_reply_success(
    query: &Query,
    key_expr: &KeyExpr,
    payload: ZBytes,
    encoding: Option<&Encoding>,
    timestamp_ntp64: Option<i64>,
    attachment: Option<ZBytes>,
    express: Option<bool>,
) -> Result<(), Error> {
    let mut b = query.reply(key_expr, payload);
    if let Some(enc) = encoding {
        b = b.encoding(enc.clone());
    }
    if let Some(ntp) = timestamp_ntp64 {
        b = b.timestamp(Timestamp::new(NTP64(ntp as u64), TimestampId::rand()));
    }
    if let Some(att) = attachment {
        b = b.attachment(att);
    }
    if let Some(v) = express {
        b = b.express(v);
    }
    b.wait()
}

/// Reply to a query with an error instead of a sample — the flat port of
/// `zenoh::query::Query::reply_err`. The `payload`/`encoding` carry the error
/// value the querier sees as a [`crate::ReplyError`].
#[prebindgen]
pub fn query_reply_error(
    query: &Query,
    payload: ZBytes,
    encoding: Option<&Encoding>,
) -> Result<(), Error> {
    let mut b = query.reply_err(payload);
    if let Some(enc) = encoding {
        b = b.encoding(enc.clone());
    }
    b.wait()
}

/// Reply to a query with a DELETE sample (tombstone) on `key_expr` — the flat
/// port of `zenoh::query::Query::reply_del`. `timestamp_ntp64`, `attachment`,
/// and `express` are optional.
#[prebindgen]
pub fn query_reply_delete(
    query: &Query,
    key_expr: &KeyExpr,
    timestamp_ntp64: Option<i64>,
    attachment: Option<ZBytes>,
    express: Option<bool>,
) -> Result<(), Error> {
    let mut b = query.reply_del(key_expr);
    if let Some(ntp) = timestamp_ntp64 {
        b = b.timestamp(Timestamp::new(NTP64(ntp as u64), TimestampId::rand()));
    }
    if let Some(att) = attachment {
        b = b.attachment(att);
    }
    if let Some(v) = express {
        b = b.express(v);
    }
    b.wait()
}
