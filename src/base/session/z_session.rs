#[cfg(feature = "unstable")]
use crate::Reliability;
use crate::util::OnceDrop;
use crate::{
    CongestionControl, ConsolidationMode, Priority, QueryTarget, ReplyKeyExpr, ZConfig, ZEncoding,
    ZError, ZKeyExpr, ZPublisher, ZQuerier, ZQuery, ZQueryable, ZReply, ZSample, ZSession,
    ZSubscriber, ZZBytes, ZZenohId,
};
use prebindgen_proc_macro::prebindgen;
use std::time::Duration;
use zenoh::{Wait, query::Selector};

/// Open a session with the given configuration. The config is consumed by value
/// (matching native `zenoh::open`); C callers that need to keep it should
/// `z_config_clone` first.
#[prebindgen]
pub fn z_open(config: ZConfig) -> Result<ZSession, ZError> {
    zenoh::open(config).wait()
}

// The `reliability` QoS is unstable in zenoh; gate the single parameter (and the
// `.reliability()` call) with `#[cfg(feature = "unstable")]`. prebindgen honors
// per-parameter cfg, so the captured signature — and the generated C ABI — gains
// or loses the trailing `reliability` param with the feature, from ONE definition.
#[prebindgen]
pub fn z_session_declare_publisher(
    session: &ZSession,
    key_expr: ZKeyExpr,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> Result<ZPublisher, ZError> {
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

#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn z_session_put(
    session: &ZSession,
    key_expr: &ZKeyExpr,
    payload: ZZBytes,
    encoding: Option<&ZEncoding>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    attachment: Option<ZZBytes>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> Result<(), ZError> {
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

#[prebindgen]
pub fn z_session_delete(
    session: &ZSession,
    key_expr: &ZKeyExpr,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    attachment: Option<ZZBytes>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> Result<(), ZError> {
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

/// Declare a subscriber delivering each change as an opaque [`ZSample`] handle
/// (thin surface). `on_close` fires when the subscriber is dropped.
#[prebindgen]
pub fn z_session_declare_subscriber(
    session: &ZSession,
    key_expr: ZKeyExpr,
    callback: impl Fn(ZSample) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<ZSubscriber, ZError> {
    let on_close = OnceDrop::new(on_close);
    session
        .declare_subscriber(key_expr)
        .callback(move |sample| {
            let _ = &on_close;
            callback(sample);
        })
        .wait()
}

#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn z_session_declare_querier(
    session: &ZSession,
    key_expr: ZKeyExpr,
    target: Option<QueryTarget>,
    consolidation: Option<ConsolidationMode>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    timeout_ms: Option<i64>,
    accept_replies: Option<ReplyKeyExpr>,
) -> Result<ZQuerier, ZError> {
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

/// Declare a queryable delivering each query as an opaque [`ZQuery`] handle
/// (thin surface). `on_close` fires when the queryable is dropped.
#[prebindgen]
pub fn z_session_declare_queryable(
    session: &ZSession,
    key_expr: ZKeyExpr,
    complete: Option<bool>,
    callback: impl Fn(ZQuery) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<ZQueryable, ZError> {
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

#[prebindgen]
pub fn z_session_declare_keyexpr(session: &ZSession, key_expr: String) -> Result<ZKeyExpr, ZError> {
    session.declare_keyexpr(key_expr).wait()
}

#[prebindgen]
pub fn z_session_undeclare_keyexpr(session: &ZSession, key_expr: ZKeyExpr) -> Result<(), ZError> {
    session.undeclare(key_expr).wait()
}

/// Query matching queryables, delivering each reply as an opaque [`ZReply`]
/// handle (thin surface). `on_close` fires when the reply stream ends.
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn z_session_get(
    session: &ZSession,
    key_expr: &ZKeyExpr,
    parameters: Option<String>,
    timeout_ms: Option<i64>,
    target: Option<QueryTarget>,
    consolidation: Option<ConsolidationMode>,
    accept_replies: Option<ReplyKeyExpr>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    payload: Option<ZZBytes>,
    encoding: Option<&ZEncoding>,
    attachment: Option<ZZBytes>,
    callback: impl Fn(ZReply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), ZError> {
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

#[prebindgen]
pub fn z_session_zid(session: &ZSession) -> ZZenohId {
    session.info().zid().wait()
}

#[prebindgen]
pub fn z_session_peers_zid(session: &ZSession) -> Vec<ZZenohId> {
    session.info().peers_zid().wait().collect()
}

#[prebindgen]
pub fn z_session_routers_zid(session: &ZSession) -> Vec<ZZenohId> {
    session.info().routers_zid().wait().collect()
}
