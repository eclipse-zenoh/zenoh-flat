use std::time::Duration;

use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

use crate::{Error, KeyExpr, LivelinessToken, Reply, Sample, Session, Subscriber, util::OnceDrop};

/// Declare a [`LivelinessToken`] on `key_expr`. The token keeps the liveliness
/// alive until its handle is dropped, which undeclares it.
#[prebindgen]
pub fn liveliness_declare_token(
    session: &Session,
    key_expr: &KeyExpr,
) -> Result<LivelinessToken, Error> {
    session.liveliness().declare_token(key_expr.clone()).wait()
}

/// Query liveliness tokens matching `key_expr`, delivering each reply as an
/// opaque [`Reply`] handle (thin surface — cheap-FFI bindings pull fields via
/// the `reply_*` accessors). `on_close` fires when the reply stream ends.
#[prebindgen]
pub fn liveliness_get(
    session: &Session,
    key_expr: &KeyExpr,
    timeout_ms: i64,
    callback: impl Fn(Reply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    session
        .liveliness()
        .get(key_expr)
        .timeout(Duration::from_millis(timeout_ms as u64))
        .callback(move |reply| {
            let _ = &on_close;
            callback(reply);
        })
        .wait()
}

/// Declare a subscriber to liveliness changes matching `key_expr`, delivering
/// each change as an opaque [`Sample`] handle (thin surface). With `history`,
/// currently-alive tokens are delivered on declaration. `on_close` fires when
/// the returned subscriber is dropped.
#[prebindgen]
pub fn liveliness_declare_subscriber(
    session: &Session,
    key_expr: &KeyExpr,
    history: bool,
    callback: impl Fn(Sample) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<Subscriber, Error> {
    let on_close = OnceDrop::new(on_close);
    session
        .liveliness()
        .declare_subscriber(key_expr.clone())
        .history(history)
        .callback(move |sample| {
            let _ = &on_close;
            callback(sample);
        })
        .wait()
}

/// Undeclare a [`LivelinessToken`], dropping the liveliness it asserted — the
/// flat port of `zenoh::liveliness::LivelinessToken::undeclare`. Consumes the
/// handle. (Dropping the handle without calling this also undeclares the token,
/// but only this variant surfaces a network error.)
#[prebindgen]
pub fn liveliness_undeclare_token(token: LivelinessToken) -> Result<(), Error> {
    token.undeclare().wait()
}
