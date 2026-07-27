use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

use crate::{
    CongestionControl, Encoding, Error, KeyExpr, MatchingListener, Priority, Querier, Reply,
    ZBytes, util::OnceDrop,
};
#[cfg(feature = "unstable")]
use crate::{EntityGlobalId, ReplyKeyExpr};

/// Send a query through a reusable querier.
///
/// The callback is called for each reply. Optional arguments specify selector
/// parameters, payload metadata, and attachment. The close callback is called
/// after the reply stream ends.
#[prebindgen]
pub fn querier_get(
    querier: &Querier,
    parameters: Option<String>,
    payload: Option<ZBytes>,
    encoding: Option<&Encoding>,
    attachment: Option<ZBytes>,
    callback: impl Fn(Reply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    let mut builder = querier.get();
    if let Some(params) = parameters {
        builder = builder.parameters(params);
    }
    if let Some(payload) = payload {
        builder = builder.payload(payload);
        if let Some(enc) = encoding {
            builder = builder.encoding(enc.clone());
        }
    }
    if let Some(attachment) = attachment {
        builder = builder.attachment(attachment);
    }
    builder
        .callback(move |reply| {
            let _ = &on_close;
            callback(reply);
        })
        .wait()
}

/// Return the key expression targeted by this querier.
#[prebindgen]
pub fn querier_get_key_expr(querier: &Querier) -> &KeyExpr {
    querier.key_expr()
}

/// Return the congestion-control policy this querier was declared with.
///
/// The declaration takes this as an option and falls back to base's default, so
/// this is the only way to learn what the querier actually got.
#[prebindgen]
pub fn querier_get_congestion_control(querier: &Querier) -> CongestionControl {
    querier.congestion_control().into()
}

/// Return the priority this querier was declared with.
#[prebindgen]
pub fn querier_get_priority(querier: &Querier) -> Priority {
    querier.priority().into()
}

/// Return the policy for reply key expressions this querier accepts.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn querier_get_accept_replies(querier: &Querier) -> ReplyKeyExpr {
    querier.accept_replies().into()
}

/// Return whether any queryable would currently answer this querier's queries.
///
/// Asking before issuing a query is the point: there is no need to send one
/// that nothing will serve.
#[prebindgen]
pub fn querier_matching_status(querier: &Querier) -> Result<bool, Error> {
    Ok(querier.matching_status().wait()?.matching())
}

/// Declare a matching listener that is notified when the querier's matching
/// status changes.
///
/// The callback receives the new matching status (`true` if matching queryables
/// exist). The close callback is called when the listener ends.
#[prebindgen]
pub fn querier_declare_matching_listener(
    querier: &Querier,
    callback: impl Fn(bool) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<MatchingListener, Error> {
    let on_close = OnceDrop::new(on_close);
    querier
        .matching_listener()
        .callback(move |status| {
            let _ = &on_close;
            callback(status.matching());
        })
        .wait()
}

/// Declare a background matching listener that runs until the querier is
/// undeclared.
#[prebindgen]
pub fn querier_declare_background_matching_listener(
    querier: &Querier,
    callback: impl Fn(bool) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    querier
        .matching_listener()
        .callback(move |status| {
            let _ = &on_close;
            callback(status.matching());
        })
        .background()
        .wait()
}

/// Undeclare the querier and release its network declaration.
#[prebindgen]
pub fn querier_undeclare(querier: Querier) -> Result<(), Error> {
    querier.undeclare().wait()
}

/// Return the global identifier of this querier.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn querier_get_id(querier: &Querier) -> EntityGlobalId {
    querier.id().into()
}
