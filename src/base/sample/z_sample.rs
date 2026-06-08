use crate::{
    CongestionControl, Priority, SampleKind, ZEncoding, ZKeyExpr, ZSample, ZTimestamp, ZZBytes,
};
use prebindgen_proc_macro::prebindgen;
use zenoh::sample::SampleBuilder;

/// Build a Put [`ZSample`] from its key expression, payload, and encoding —
/// the flat port of zenoh's `SampleBuilder`. Its parameters are themselves
/// `ptr_class` types (`ZKeyExpr`, `ZZBytes`, `ZEncoding`), so wiring this as a
/// `ptr_class_input` for `ZSample` exercises **recursive input**: a `ZSample`
/// parameter expands to these three, each of which expands per its own
/// canonical input (key-expr String|handle, bytes ByteArray, encoding String).
#[prebindgen]
pub fn z_sample_new(key_expr: ZKeyExpr, payload: ZZBytes, encoding: ZEncoding) -> ZSample {
    SampleBuilder::put(key_expr, payload).encoding(encoding).into()
}

/// Key expression the sample was published on (borrowed; valid while `s` lives).
#[prebindgen]
pub fn z_sample_key_expr(s: &ZSample) -> &ZKeyExpr {
    s.key_expr()
}

/// Sample payload (borrowed bytes; valid while `s` lives).
#[prebindgen]
pub fn z_sample_payload(s: &ZSample) -> &ZZBytes {
    s.payload()
}

/// Encoding of the payload (borrowed; valid while `s` lives).
#[prebindgen]
pub fn z_sample_encoding(s: &ZSample) -> &ZEncoding {
    s.encoding()
}

/// Whether the sample is a PUT or a DELETE.
#[prebindgen]
pub fn z_sample_kind(s: &ZSample) -> SampleKind {
    s.kind().into()
}

/// Timestamp (borrowed), or `None` when the sample carries no timestamp.
#[prebindgen]
pub fn z_sample_timestamp(s: &ZSample) -> Option<&ZTimestamp> {
    s.timestamp()
}

/// QoS express flag.
#[prebindgen]
pub fn z_sample_express(s: &ZSample) -> bool {
    s.express()
}

/// QoS priority.
#[prebindgen]
pub fn z_sample_priority(s: &ZSample) -> Priority {
    s.priority().into()
}

/// QoS congestion-control policy.
#[prebindgen]
pub fn z_sample_congestion_control(s: &ZSample) -> CongestionControl {
    s.congestion_control().into()
}

/// Optional user attachment (borrowed bytes), or `None`.
#[prebindgen]
pub fn z_sample_attachment(s: &ZSample) -> Option<&ZZBytes> {
    s.attachment()
}
