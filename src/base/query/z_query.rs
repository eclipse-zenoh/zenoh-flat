#[cfg(feature = "unstable")]
use crate::ReplyKeyExpr;
use crate::{Error, ZEncoding, ZKeyExpr, ZQuery, ZZBytes};
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

/// Query attachment (borrowed bytes), or `None` when the query carries none.
#[prebindgen]
pub fn z_query_attachment(q: &ZQuery) -> Option<&ZZBytes> {
    q.attachment()
}

/// Policy describing which key expressions this query accepts in replies.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn z_query_accepts_replies(q: &ZQuery) -> ReplyKeyExpr {
    q.accepts_replies().into()
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
    b.wait().map_err(Error::from)
}

#[prebindgen]
pub fn z_query_reply_error(
    query: &ZQuery,
    payload: ZZBytes,
    encoding: Option<&ZEncoding>,
) -> Result<(), Error> {
    let mut b = query.reply_err(payload);
    if let Some(enc) = encoding {
        b = b.encoding(enc.clone());
    }
    b.wait().map_err(Error::from)
}

#[prebindgen]
pub fn z_query_reply_delete(
    query: &ZQuery,
    key_expr: &ZKeyExpr,
    timestamp_ntp64: Option<i64>,
    attachment: Option<ZZBytes>,
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
    b.wait().map_err(Error::from)
}
