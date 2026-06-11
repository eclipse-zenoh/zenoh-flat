use prebindgen_proc_macro::prebindgen;
use zenoh::{
    sample::SampleBuilder,
    time::{NTP64, Timestamp, TimestampId},
};

use crate::{
    CongestionControl, Priority, SampleKind, ZEncoding, ZKeyExpr, ZSample, ZTimestamp, ZZBytes,
};
#[cfg(feature = "unstable")]
use crate::{Reliability, ZZenohId};

/// Build a **Put** [`ZSample`] from its key expression, payload, and optional
/// metadata — the flat port of zenoh's `SampleBuilder::put`. The required
/// `key_expr`/`payload` are themselves `ptr_class` types, so wiring this as the
/// `ptr_class_input` for `ZSample` exercises **recursive input**: a `ZSample`
/// parameter expands to this constructor's params, each of which expands per its
/// own canonical input (key-expr String|handle, bytes ByteArray, encoding String,
/// etc.).
///
/// Optional fields mirror the builder: `encoding` (default when `None`),
/// `timestamp_ntp64` (the NTP64 component; reconstructed with a random
/// [`TimestampId`], matching `z_query_reply_success`), `attachment`, and the QoS
/// knobs. `reliability` is unstable and only present with the `unstable` feature.
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn z_sample_put(
    key_expr: ZKeyExpr,
    payload: ZZBytes,
    encoding: Option<&ZEncoding>,
    timestamp_ntp64: Option<i64>,
    attachment: Option<ZZBytes>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> ZSample {
    let mut builder = SampleBuilder::put(key_expr, payload);
    if let Some(enc) = encoding {
        builder = builder.encoding(enc.clone());
    }
    if let Some(ntp) = timestamp_ntp64 {
        builder = builder.timestamp(Timestamp::new(NTP64(ntp as u64), TimestampId::rand()));
    }
    if let Some(att) = attachment {
        builder = builder.attachment(att);
    }
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
    builder.into()
}

/// Build a **Delete** [`ZSample`] from its key expression and optional metadata —
/// the flat port of zenoh's `SampleBuilder::delete`. A delete sample carries no
/// payload or encoding; the remaining fields mirror [`z_sample_put`].
#[prebindgen]
pub fn z_sample_delete(
    key_expr: ZKeyExpr,
    timestamp_ntp64: Option<i64>,
    attachment: Option<ZZBytes>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> ZSample {
    let mut builder = SampleBuilder::delete(key_expr);
    if let Some(ntp) = timestamp_ntp64 {
        builder = builder.timestamp(Timestamp::new(NTP64(ntp as u64), TimestampId::rand()));
    }
    if let Some(att) = attachment {
        builder = builder.attachment(att);
    }
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
    builder.into()
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

/// Reliability policy the sample was delivered with.
///
/// Unstable: `zenoh::sample::Sample::reliability` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn z_sample_reliability(s: &ZSample) -> Reliability {
    s.reliability().into()
}

/// Zenoh id of the source entity that produced the sample, or `None` when the
/// sample carries no source info (owned handle).
///
/// Unstable: `zenoh::sample::Sample::source_info` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn z_sample_source_zid(s: &ZSample) -> Option<ZZenohId> {
    s.source_info().map(|si| si.source_id().zid())
}

/// Entity id of the sample's source (0 when the sample carries no source info).
///
/// Unstable: `zenoh::sample::Sample::source_info` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn z_sample_source_eid(s: &ZSample) -> i32 {
    s.source_info()
        .map(|si| si.source_id().eid() as i32)
        .unwrap_or(0)
}

/// Source sequence number of the sample (0 when the sample carries no source
/// info).
///
/// Unstable: `zenoh::sample::Sample::source_info` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn z_sample_source_sn(s: &ZSample) -> i64 {
    s.source_info().map(|si| si.source_sn() as i64).unwrap_or(0)
}
