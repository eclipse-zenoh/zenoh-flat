use std::time::Duration;

use prebindgen_proc_macro::prebindgen;
use zenoh::{Wait, query::Selector};

#[cfg(feature = "unstable")]
use crate::Reliability;
use crate::{
    CongestionControl, ConsolidationMode, Priority, QueryTarget, ReplyKeyExpr, Config, Encoding,
    Error, KeyExpr, Publisher, Querier, Query, Queryable, Reply, Sample, Session,
    Subscriber, Timestamp, ZBytes, ZenohId, util::OnceDrop,
};

/// Open a session with the given configuration. The config is consumed by value
/// (matching native `zenoh::open`); C callers that need to keep it should
/// `config_clone` first.
#[prebindgen]
pub fn open(config: Config) -> Result<Session, Error> {
    zenoh::open(config).wait()
}

// The `reliability` QoS is unstable in zenoh; gate the single parameter (and the
// `.reliability()` call) with `#[cfg(feature = "unstable")]`. prebindgen honors
// per-parameter cfg, so the captured signature — and the generated C ABI — gains
// or loses the trailing `reliability` param with the feature, from ONE definition.
/// Declare a publisher for `key_expr` with optional default QoS — the flat port
/// of `zenoh::Session::declare_publisher`. The returned handle publishes via
/// [`crate::publisher_put`] / [`crate::publisher_delete`]; the QoS set here
/// (congestion control, priority, express, and `reliability` when `unstable`)
/// becomes the per-message default.
#[prebindgen]
pub fn session_declare_publisher(
    session: &Session,
    key_expr: KeyExpr,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> Result<Publisher, Error> {
    #[allow(unused_mut)]
    let mut builder = session.declare_publisher(key_expr);
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

/// Publish `payload` on `key_expr` in one shot, without declaring a publisher —
/// the flat port of `zenoh::Session::put`. `encoding`, `attachment`, and the QoS
/// knobs are per-message overrides (`reliability` only with `unstable`).
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

/// Publish a delete (tombstone) on `key_expr` in one shot — the flat port of
/// `zenoh::Session::delete`. Subscribers receive a `SampleKind::Delete` sample.
/// `attachment` and the QoS knobs are per-message overrides (`reliability` only
/// with `unstable`).
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

/// Declare a subscriber delivering each change as an opaque [`Sample`] handle
/// (thin surface). `on_close` fires when the subscriber is dropped.
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

/// Declare a querier for `key_expr` with optional default query settings — the
/// flat port of `zenoh::Session::declare_querier`. A querier amortizes routing
/// across repeated GETs; issue them via [`crate::querier_get`]. The target,
/// consolidation, timeout, QoS, and reply-key-expr policy set here become the
/// per-GET defaults.
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

/// Declare a queryable delivering each query as an opaque [`Query`] handle
/// (thin surface). `on_close` fires when the queryable is dropped.
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

/// Declare `key_expr` with the network, returning an optimized handle bound to
/// this session — the flat port of `zenoh::Session::declare_keyexpr`. Reusing
/// the returned handle lets the protocol elide the full string on the wire.
/// Release it with [`session_undeclare_keyexpr`].
#[prebindgen]
pub fn session_declare_keyexpr(session: &Session, key_expr: String) -> Result<KeyExpr, Error> {
    session.declare_keyexpr(key_expr).wait()
}

/// Undeclare a previously [`session_declare_keyexpr`]'d key expression, releasing
/// its network optimization. Consumes the handle.
#[prebindgen]
pub fn session_undeclare_keyexpr(session: &Session, key_expr: KeyExpr) -> Result<(), Error> {
    session.undeclare(key_expr).wait()
}

/// Query matching queryables, delivering each reply as an opaque [`Reply`]
/// handle (thin surface). `on_close` fires when the reply stream ends.
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn session_get(
    session: &Session,
    key_expr: &KeyExpr,
    parameters: Option<String>,
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
    let selector = Selector::owned(key_expr, parameters.unwrap_or_default());
    let on_close = OnceDrop::new(on_close);
    let mut builder = session.get(selector);
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

/// This session's own Zenoh id (the flat port of `SessionInfo::zid`).
#[prebindgen]
pub fn session_zid(session: &Session) -> ZenohId {
    session.info().zid().wait()
}

/// Zenoh ids of the peers currently connected to this session (the flat port of
/// `SessionInfo::peers_zid`).
#[prebindgen]
pub fn session_peers_zid(session: &Session) -> Vec<ZenohId> {
    session.info().peers_zid().wait().collect()
}

/// Zenoh ids of the routers this session is connected to (the flat port of
/// `SessionInfo::routers_zid`).
#[prebindgen]
pub fn session_routers_zid(session: &Session) -> Vec<ZenohId> {
    session.info().routers_zid().wait().collect()
}

/// Close the session, terminating all its declarations and releasing transport
/// resources (the flat port of `zenoh::Session::close`). Idempotent: closing an
/// already-closed session is a no-op. The handle stays valid afterwards — use
/// [`session_is_closed`] to test its state — and is freed when dropped.
#[prebindgen]
pub fn session_close(session: &Session) -> Result<(), Error> {
    session.close().wait()
}

/// Whether the session has been closed (explicitly via [`session_close`] or
/// because its last clone was dropped).
#[prebindgen]
pub fn session_is_closed(session: &Session) -> bool {
    session.is_closed()
}

/// Mint a new [`Timestamp`] from the session's Hybrid Logical Clock — the flat
/// port of `zenoh::Session::new_timestamp`. Use it to stamp a [`Sample`] or
/// reply with a clock that stays consistent across the session's publications.
#[prebindgen]
pub fn session_new_timestamp(session: &Session) -> Timestamp {
    session.new_timestamp()
}
