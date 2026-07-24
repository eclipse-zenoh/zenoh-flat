use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[cfg(feature = "unstable")]
use crate::EntityGlobalId;
use crate::{Encoding, Error, KeyExpr, Querier, Reply, ZBytes, util::OnceDrop};

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
pub fn querier_get_keyexpr(querier: &Querier) -> &KeyExpr {
    querier.key_expr()
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
