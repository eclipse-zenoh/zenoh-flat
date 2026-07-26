pub(crate) mod consolidation_mode;
pub(crate) mod parameters;
pub(crate) mod querier;
pub(crate) mod query_target;
pub(crate) mod queryable;
pub(crate) mod reply;
pub(crate) mod reply_key_expr;
pub(crate) mod selector;

use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[cfg(feature = "unstable")]
use self::reply_key_expr::ReplyKeyExpr;
#[cfg(feature = "unstable")]
use crate::{CongestionControl, Priority, SourceInfo, TimestampStack};
use crate::{Encoding, Error, KeyExpr, Query, Sample, Timestamp, ZBytes};

/// Return the key expression targeted by the query.
#[prebindgen]
pub fn query_get_key_expr(q: &Query) -> &KeyExpr {
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

/// Return the congestion-control policy the query was sent with.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn query_get_congestion_control(q: &Query) -> CongestionControl {
    q.congestion_control().into()
}

/// Return the priority the query was sent with.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn query_get_priority(q: &Query) -> Priority {
    q.priority().into()
}

/// Return whether the query requested express delivery.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn query_get_express(q: &Query) -> bool {
    q.express()
}

/// Return information about the entity that issued the query, when known.
///
/// This is the query-side counterpart of [`crate::sample_get_source_info`], so a
/// queryable can attribute a query as a subscriber attributes a sample.
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn query_get_source_info(q: &Query) -> Option<SourceInfo> {
    q.source_info().map(SourceInfo::from)
}

/// Return the timestamps this query accumulated along its path, when
/// instrumentation recorded any.
pub fn query_get_timestamp_stack(q: &Query) -> Option<&TimestampStack> {
    q.timestamp_stack()
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
/// express delivery. When no timestamp is supplied, Zenoh assigns one; a
/// supplied one is used exactly as given, node id included. Use
/// [`query_reply_sample`] to send a complete sample instead.
#[prebindgen]
pub fn query_reply_success(
    query: &Query,
    key_expr: &KeyExpr,
    payload: ZBytes,
    encoding: Option<&Encoding>,
    timestamp: Option<Timestamp>,
    attachment: Option<ZBytes>,
    express: Option<bool>,
) -> Result<(), Error> {
    let mut b = query.reply(key_expr, payload);
    if let Some(enc) = encoding {
        b = b.encoding(enc.clone());
    }
    if let Some(t) = timestamp {
        b = b.timestamp(zenoh::time::Timestamp::try_from(&t)?);
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
/// A supplied timestamp is used exactly as given, node id included.
#[prebindgen]
pub fn query_reply_delete(
    query: &Query,
    key_expr: &KeyExpr,
    timestamp: Option<Timestamp>,
    attachment: Option<ZBytes>,
    express: Option<bool>,
) -> Result<(), Error> {
    let mut b = query.reply_del(key_expr);
    if let Some(t) = timestamp {
        b = b.timestamp(zenoh::time::Timestamp::try_from(&t)?);
    }
    if let Some(att) = attachment {
        b = b.attachment(att);
    }
    if let Some(v) = express {
        b = b.express(v);
    }
    b.wait()
}
