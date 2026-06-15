use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[cfg(feature = "unstable")]
use crate::ZenohId;
use crate::{Encoding, Error, KeyExpr, Querier, Reply, ZBytes, util::OnceDrop};

/// Perform a GET through a querier, delivering each reply as an opaque
/// [`Reply`] handle (thin surface — cheap-FFI bindings pull fields via the
/// `reply_*` accessors). `on_close` fires when the reply stream ends.
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

/// Key expression the querier queries on (borrowed; valid while `querier` lives).
#[prebindgen]
pub fn querier_get_keyexpr(querier: &Querier) -> &KeyExpr {
    querier.key_expr()
}

/// Undeclare a querier, releasing its network declaration — the flat port of
/// `zenoh::query::Querier::undeclare`. Consumes the handle.
#[prebindgen]
pub fn querier_undeclare(querier: Querier) -> Result<(), Error> {
    querier.undeclare().wait()
}

/// Zenoh id of the node hosting this querier (the `zid` of its entity global id).
///
/// Unstable: `zenoh::query::Querier::id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn querier_get_zid(querier: &Querier) -> ZenohId {
    querier.id().zid()
}

/// Entity id of this querier (the per-session part of its entity global id).
///
/// Unstable: `zenoh::query::Querier::id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn querier_get_eid(querier: &Querier) -> i32 {
    querier.id().eid() as i32
}
