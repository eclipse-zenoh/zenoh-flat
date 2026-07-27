#[cfg(feature = "unstable")]
pub(crate) mod info;

use std::time::Duration;

use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[cfg(feature = "unstable")]
use crate::Reliability;
use crate::{
    Config, CongestionControl, ConsolidationMode, Encoding, Error, KeyExpr, Priority, Publisher,
    Querier, Query, QueryTarget, Queryable, Reply, ReplyKeyExpr, Sample, Selector, Session,
    Subscriber, Timestamp, ZBytes, ZenohId, util::OnceDrop,
};

/// Open a session with the given configuration.
///
/// The configuration is consumed by this operation.
#[prebindgen]
pub fn open(config: Config) -> Result<Session, Error> {
    zenoh::open(config).wait()
}

// The `reliability` QoS is unstable in zenoh; gate the single parameter (and the
// `.reliability()` call) with `#[cfg(feature = "unstable")]`. prebindgen honors
// per-parameter cfg, so the captured signature — and the generated C ABI — gains
// or loses the trailing `reliability` param with the feature, from ONE definition.
/// Declare a publisher for the given key expression.
///
/// Optional delivery settings become defaults for publications made through
/// the returned publisher; the optional `encoding` becomes the publisher's
/// default payload encoding, applied natively to every publication that does
/// not override it. Reliability is available only when unstable features are
/// enabled.
#[prebindgen]
pub fn session_declare_publisher(
    session: &Session,
    key_expr: KeyExpr,
    encoding: Option<&Encoding>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> Result<Publisher, Error> {
    #[allow(unused_mut)]
    let mut builder = session.declare_publisher(key_expr);
    if let Some(enc) = encoding {
        builder = builder.encoding(enc.clone());
    }
    if let Some(cc) = congestion_control {
        builder = builder.congestion_control(cc.into());
    }
    if let Some(p) = priority {
        builder = builder.priority(p.into());
    }
    if let Some(v) = express {
        builder = builder.express(v);
    }
    #[cfg(feature = "unstable")]
    {
        if let Some(r) = reliability {
            builder = builder.reliability(r.into());
        }
    }
    builder.wait()
}

/// Publish data on a key expression.
///
/// Matching subscribers receive a PUT sample. Optional arguments specify the
/// payload format, delivery quality, and attachment. Reliability is available
/// only when unstable features are enabled.
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn session_put(
    session: &Session,
    key_expr: &KeyExpr,
    payload: ZBytes,
    encoding: Option<&Encoding>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    attachment: Option<ZBytes>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> Result<(), Error> {
    let mut builder = session.put(key_expr, payload);
    if let Some(cc) = congestion_control {
        builder = builder.congestion_control(cc.into());
    }
    if let Some(enc) = encoding {
        builder = builder.encoding(enc.clone());
    }
    if let Some(v) = express {
        builder = builder.express(v);
    }
    if let Some(p) = priority {
        builder = builder.priority(p.into());
    }
    #[cfg(feature = "unstable")]
    {
        if let Some(r) = reliability {
            builder = builder.reliability(r.into());
        }
    }
    if let Some(att) = attachment {
        builder = builder.attachment(att);
    }
    builder.wait()
}

/// Publish a deletion notification on a key expression.
///
/// Matching subscribers receive a DELETE sample. Optional arguments specify
/// delivery quality and attachment. Reliability is available only when
/// unstable features are enabled.
#[prebindgen]
pub fn session_delete(
    session: &Session,
    key_expr: &KeyExpr,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    attachment: Option<ZBytes>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> Result<(), Error> {
    let mut builder = session.delete(key_expr);
    if let Some(cc) = congestion_control {
        builder = builder.congestion_control(cc.into());
    }
    if let Some(v) = express {
        builder = builder.express(v);
    }
    if let Some(p) = priority {
        builder = builder.priority(p.into());
    }
    #[cfg(feature = "unstable")]
    {
        if let Some(r) = reliability {
            builder = builder.reliability(r.into());
        }
    }
    if let Some(att) = attachment {
        builder = builder.attachment(att);
    }
    builder.wait()
}

/// Subscribe to samples whose key expressions match the supplied expression.
///
/// The callback is called for each sample. The close callback is called when
/// the subscription ends.
#[prebindgen]
pub fn session_declare_subscriber(
    session: &Session,
    key_expr: KeyExpr,
    callback: impl Fn(Sample) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<Subscriber, Error> {
    let on_close = OnceDrop::new(on_close);
    session
        .declare_subscriber(key_expr)
        .callback(move |sample| {
            let _ = &on_close;
            callback(sample);
        })
        .wait()
}

/// Declare a reusable querier for the given key expression.
///
/// Optional arguments set defaults for query targets, reply consolidation,
/// delivery quality, timeout, and accepted reply key expressions.
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn session_declare_querier(
    session: &Session,
    key_expr: KeyExpr,
    target: Option<QueryTarget>,
    consolidation: Option<ConsolidationMode>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    timeout_ms: Option<i64>,
    accept_replies: Option<ReplyKeyExpr>,
) -> Result<Querier, Error> {
    let mut builder = session.declare_querier(key_expr);
    if let Some(cc) = congestion_control {
        builder = builder.congestion_control(cc.into());
    }
    if let Some(c) = consolidation {
        let c: zenoh::query::ConsolidationMode = c.into();
        builder = builder.consolidation(c);
    }
    if let Some(v) = express {
        builder = builder.express(v);
    }
    if let Some(t) = target {
        builder = builder.target(t.into());
    }
    if let Some(p) = priority {
        builder = builder.priority(p.into());
    }
    if let Some(ms) = timeout_ms {
        builder = builder.timeout(Duration::from_millis(ms as u64));
    }
    if let Some(ar) = accept_replies {
        builder = builder.accept_replies(ar.into());
    }
    builder.wait()
}

/// Declare a queryable for queries matching the supplied key expression.
///
/// The callback is called for each query. A complete queryable indicates that
/// it can provide every available result for matching queries. The close
/// callback is called when the queryable ends.
#[prebindgen]
pub fn session_declare_queryable(
    session: &Session,
    key_expr: KeyExpr,
    complete: Option<bool>,
    callback: impl Fn(Query) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<Queryable, Error> {
    let on_close = OnceDrop::new(on_close);
    let mut builder = session.declare_queryable(key_expr);
    if let Some(v) = complete {
        builder = builder.complete(v);
    }
    builder
        .callback(move |query| {
            let _ = &on_close;
            callback(query);
        })
        .wait()
}

/// Register a key expression with the session for efficient repeated use.
///
/// Release the registration with [`session_undeclare_keyexpr`].
#[prebindgen]
pub fn session_declare_keyexpr(session: &Session, key_expr: String) -> Result<KeyExpr, Error> {
    session.declare_keyexpr(key_expr).wait()
}

/// Release a key expression previously registered with
/// [`session_declare_keyexpr`].
#[prebindgen]
pub fn session_undeclare_keyexpr(session: &Session, key_expr: KeyExpr) -> Result<(), Error> {
    session.undeclare(key_expr).wait()
}

/// Send a query to matching queryables.
///
/// The callback is called for each reply. Optional arguments control timeout,
/// targets, reply consolidation, accepted reply keys, delivery quality, payload
/// metadata, and attachment. The close callback is called after the reply
/// stream ends.
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn session_get(
    session: &Session,
    selector: Selector,
    timeout_ms: Option<i64>,
    target: Option<QueryTarget>,
    consolidation: Option<ConsolidationMode>,
    accept_replies: Option<ReplyKeyExpr>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    payload: Option<ZBytes>,
    encoding: Option<&Encoding>,
    attachment: Option<ZBytes>,
    callback: impl Fn(Reply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    let mut builder = session.get(zenoh::query::Selector::from(selector));
    if let Some(cc) = congestion_control {
        builder = builder.congestion_control(cc.into());
    }
    if let Some(p) = priority {
        builder = builder.priority(p.into());
    }
    if let Some(v) = express {
        builder = builder.express(v);
    }
    if let Some(t) = target {
        builder = builder.target(t.into());
    }
    if let Some(ms) = timeout_ms {
        builder = builder.timeout(Duration::from_millis(ms as u64));
    }
    if let Some(c) = consolidation {
        let c: zenoh::query::ConsolidationMode = c.into();
        builder = builder.consolidation(c);
    }
    if let Some(ar) = accept_replies {
        builder = builder.accept_replies(ar.into());
    }
    if let Some(payload) = payload {
        builder = builder.payload(payload);
        if let Some(enc) = encoding {
            builder = builder.encoding(enc.clone());
        }
    }
    if let Some(att) = attachment {
        builder = builder.attachment(att);
    }
    builder
        .callback(move |reply| {
            let _ = &on_close;
            callback(reply);
        })
        .wait()
}

/// Return the identifier of this session.
#[prebindgen]
pub fn session_get_zid(session: &Session) -> ZenohId {
    session.info().zid().wait().into()
}

/// Return the identifiers of peers currently connected to this session.
#[prebindgen]
pub fn session_get_peers_zid(session: &Session) -> Vec<ZenohId> {
    session
        .info()
        .peers_zid()
        .wait()
        .map(ZenohId::from)
        .collect()
}

/// Return the locators this session is reachable at.
///
/// A locator is rendered as its standard string form, matching
/// [`crate::hello_get_locators`].
///
/// Available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn session_get_locators(session: &Session) -> Vec<String> {
    session
        .info()
        .locators()
        .wait()
        .iter()
        .map(|l| l.to_string())
        .collect()
}

/// Return the identifiers of routers currently connected to this session.
#[prebindgen]
pub fn session_get_routers_zid(session: &Session) -> Vec<ZenohId> {
    session
        .info()
        .routers_zid()
        .wait()
        .map(ZenohId::from)
        .collect()
}

/// Close the session and terminate its active declarations.
///
/// Closing an already closed session has no effect. Use [`session_is_closed`]
/// to inspect its state.
#[prebindgen]
pub fn session_close(session: &Session) -> Result<(), Error> {
    session.close().wait()
}

/// Return whether the session has been closed.
#[prebindgen]
pub fn session_is_closed(session: &Session) -> bool {
    session.is_closed()
}

/// Create a timestamp using the session's hybrid logical clock.
///
/// Such timestamps remain causally consistent with the session's operations.
#[prebindgen]
pub fn session_new_timestamp(session: &Session) -> Timestamp {
    (&session.new_timestamp()).into()
}
