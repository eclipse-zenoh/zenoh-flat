use std::time::Duration;

use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

use crate::{Error, KeyExpr, LivelinessToken, Reply, Sample, Session, Subscriber, util::OnceDrop};

/// Declare a liveliness token on the supplied key expression.
///
/// The token asserts that the associated application is alive until the token
/// is undeclared.
#[prebindgen]
pub fn liveliness_declare_token(
    session: &Session,
    key_expr: KeyExpr,
) -> Result<LivelinessToken, Error> {
    session.liveliness().declare_token(key_expr).wait()
}

/// Query liveliness tokens matching the supplied key expression.
///
/// The callback is called for each reply. The close callback is called after
/// the reply stream ends.
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

/// Subscribe to liveliness changes matching the supplied key expression.
///
/// When history is enabled, tokens that are already alive are reported when
/// the subscription starts. The close callback is called when the subscription
/// ends.
#[prebindgen]
pub fn liveliness_declare_subscriber(
    session: &Session,
    key_expr: KeyExpr,
    history: bool,
    callback: impl Fn(Sample) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<Subscriber, Error> {
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

/// Undeclare a liveliness token and stop its liveliness assertion.
#[prebindgen]
pub fn liveliness_undeclare_token(token: LivelinessToken) -> Result<(), Error> {
    token.undeclare().wait()
}
