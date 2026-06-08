use crate::{ZError, ZEncoding, ZKeyExpr, ZQuery, ZSample, ZZBytes};
#[cfg(feature = "unstable")]
use crate::ReplyKeyExpr;
use prebindgen_proc_macro::prebindgen;
use zenoh::{
    Wait,
    time::{NTP64, Timestamp, TimestampId},
};

/// Key expression the query targets (borrowed; valid while `q` lives).
#[prebindgen]
pub fn z_query_keyexpr(q: &ZQuery) -> &ZKeyExpr {
    q.key_expr()
}

/// Query selector parameters as an owned string (empty when none).
#[prebindgen]
pub fn z_query_parameters(q: &ZQuery) -> String {
    q.parameters().as_str().to_string()
}

/// Query payload (borrowed bytes), or `None` when the query carries none.
#[prebindgen]
pub fn z_query_payload(q: &ZQuery) -> Option<&ZZBytes> {
    q.payload()
}

/// Encoding of the query payload (borrowed), or `None`.
#[prebindgen]
pub fn z_query_encoding(q: &ZQuery) -> Option<&ZEncoding> {
    q.encoding()
}

/// Attachment carried by the query (borrowed bytes), or `None`.
#[prebindgen]
pub fn z_query_attachment(q: &ZQuery) -> Option<&ZZBytes> {
    q.attachment()
}

/// The [`crate::ReplyKeyExpr`] policy the querier accepts for replies.
/// Unstable: `zenoh::query::Query::accepts_replies` is `#[cfg(feature = "unstable")]`.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn z_query_accepts_replies(q: &ZQuery) -> ReplyKeyExpr {
    q.accepts_replies().into()
}

/// Reply to a query with a fully-formed [`ZSample`] (key expression, payload,
/// and encoding). The flat consumer of `z_sample_new`: its `sample` parameter
/// is a by-value `ZSample`, so its canonical input (`z_sample_new`) recursively
/// expands at the binding boundary — the recursive-input demonstration.
#[prebindgen]
pub fn z_query_reply_sample(query: &ZQuery, sample: ZSample) -> Result<(), ZError> {
    query
        .reply(sample.key_expr().clone(), sample.payload().clone())
        .encoding(sample.encoding().clone())
        .wait()
}

#[prebindgen]
pub fn z_query_reply_success(
    query: &ZQuery,
    key_expr: &ZKeyExpr,
    payload: ZZBytes,
    encoding: Option<&ZEncoding>,
    timestamp_ntp64: Option<i64>,
    attachment: Option<ZZBytes>,
    express: Option<bool>,
) -> Result<(), ZError> {
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

#[prebindgen]
pub fn z_query_reply_error(
    query: &ZQuery,
    payload: ZZBytes,
    encoding: Option<&ZEncoding>,
) -> Result<(), ZError> {
    let mut b = query.reply_err(payload);
    if let Some(enc) = encoding {
        b = b.encoding(enc.clone());
    }
    b.wait()
}

#[prebindgen]
pub fn z_query_reply_delete(
    query: &ZQuery,
    key_expr: &ZKeyExpr,
    timestamp_ntp64: Option<i64>,
    attachment: Option<ZZBytes>,
    express: Option<bool>,
) -> Result<(), ZError> {
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
