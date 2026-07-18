pub(crate) mod consolidation_mode;
pub(crate) mod parameters;
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

/// Return the key expression targeted by the query.
#[prebindgen]
pub fn query_get_keyexpr(q: &Query) -> &KeyExpr {
    q.key_expr()
}

/// Return the parameters that refine the query selector.
///
/// Process the returned string with the `parameters_*` functions.
#[prebindgen]
pub fn query_get_parameters(q: &Query) -> String {
    q.parameters().as_str().to_string()
}

/// Return the query payload, when present.
#[prebindgen]
pub fn query_get_payload(q: &Query) -> Option<&ZBytes> {
    q.payload()
}

/// Return format information for the query payload, when present.
#[prebindgen]
pub fn query_get_encoding(q: &Query) -> Option<&Encoding> {
    q.encoding()
}

/// Return user-defined metadata associated with the query, when present.
#[prebindgen]
pub fn query_get_attachment(q: &Query) -> Option<&ZBytes> {
    q.attachment()
}

/// Return the policy for accepted reply key expressions.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn query_get_accepts_replies(q: &Query) -> ReplyKeyExpr {
    q.accepts_replies().into()
}

/// Reply to a query with a complete sample.
///
/// The sample's kind, payload, encoding, timestamp, attachment, delivery
/// quality, and source information are preserved.
#[prebindgen]
pub fn query_reply_sample(query: &Query, sample: Sample) -> Result<(), Error> {
    query.reply_sample(sample).wait()
}

/// Reply to a query with a value.
///
/// Optional arguments specify the payload format, timestamp, attachment, and
/// express delivery. When no timestamp is supplied, Zenoh assigns one. Use
/// [`query_reply_sample`] to send a complete sample instead.
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

/// Reply to a query with an application error.
///
/// The payload and its format describe the error returned to the querier.
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

/// Reply to a query with a deletion notification.
///
/// Optional arguments specify the timestamp, attachment, and express delivery.
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
