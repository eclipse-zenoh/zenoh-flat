use crate::util::OnceDrop;
use crate::{ZError, ZEncoding, ZQuerier, ZReply, ZZBytes};
use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

/// Perform a GET through a querier, delivering each reply as an opaque
/// [`ZReply`] handle (thin surface — cheap-FFI bindings pull fields via the
/// `z_reply_*` accessors). `on_close` fires when the reply stream ends.
#[prebindgen]
pub fn z_querier_get(
    querier: &ZQuerier,
    parameters: Option<String>,
    payload: Option<ZZBytes>,
    encoding: Option<&ZEncoding>,
    attachment: Option<ZZBytes>,
    callback: impl Fn(ZReply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), ZError> {
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
