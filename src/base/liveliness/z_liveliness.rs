use std::time::Duration;

use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

use crate::{
    ZError, ZKeyExpr, ZLivelinessToken, ZReply, ZSample, ZSession, ZSubscriber, util::OnceDrop,
};

/// Declare a [`ZLivelinessToken`] on `key_expr`. The token keeps the liveliness
/// alive until its handle is dropped, which undeclares it.
#[prebindgen]
pub fn z_liveliness_declare_token(
    session: &ZSession,
    key_expr: ZKeyExpr,
) -> Result<ZLivelinessToken, ZError> {
    session.liveliness().declare_token(key_expr).wait()
}

/// Query liveliness tokens matching `key_expr`, delivering each reply as an
/// opaque [`ZReply`] handle (thin surface — cheap-FFI bindings pull fields via
/// the `z_reply_*` accessors). `on_close` fires when the reply stream ends.
#[prebindgen]
pub fn z_liveliness_get(
    session: &ZSession,
    key_expr: &ZKeyExpr,
    timeout_ms: i64,
    callback: impl Fn(ZReply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), ZError> {
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
/// each change as an opaque [`ZSample`] handle (thin surface). With `history`,
/// currently-alive tokens are delivered on declaration. `on_close` fires when
/// the returned subscriber is dropped.
#[prebindgen]
pub fn z_liveliness_declare_subscriber(
    session: &ZSession,
    key_expr: ZKeyExpr,
    history: bool,
    callback: impl Fn(ZSample) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<ZSubscriber, ZError> {
    let on_close = OnceDrop::new(on_close);
    session
        .liveliness()
        .declare_subscriber(key_expr)
        .history(history)
        .callback(move |sample| {
            let _ = &on_close;
            callback(sample);
        })
        .wait()
}
