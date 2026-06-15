pub(crate) mod sample_kind;

use prebindgen_proc_macro::prebindgen;
use zenoh::{
    sample::SampleBuilder,
    time::{NTP64, TimestampId},
};

use self::sample_kind::SampleKind;
use crate::{CongestionControl, Encoding, KeyExpr, Priority, Sample, Timestamp, ZBytes};
#[cfg(feature = "unstable")]
use crate::{Reliability, ZenohId};

/// Build a **Put** [`Sample`] from its key expression, payload, and optional
/// metadata — the flat port of zenoh's `SampleBuilder::put`. The required
/// `key_expr`/`payload` are themselves `ptr_class` types, so wiring this as the
/// `ptr_class_input` for `Sample` exercises **recursive input**: a `Sample`
/// parameter expands to this constructor's params, each of which expands per its
/// own canonical input (key-expr String|handle, bytes ByteArray, encoding String,
/// etc.).
///
/// Optional fields mirror the builder: `encoding` (default when `None`),
/// `timestamp_ntp64` (the NTP64 component; reconstructed with a random
/// [`TimestampId`], matching `query_reply_success`), `attachment`, and the QoS
/// knobs. `reliability` is unstable and only present with the `unstable` feature.
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn sample_put(
    key_expr: KeyExpr,
    payload: ZBytes,
    encoding: Option<&Encoding>,
    timestamp_ntp64: Option<i64>,
    attachment: Option<ZBytes>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> Sample {
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

/// Build a **Delete** [`Sample`] from its key expression and optional metadata —
/// the flat port of zenoh's `SampleBuilder::delete`. A delete sample carries no
/// payload or encoding; the remaining fields mirror [`sample_put`].
#[prebindgen]
pub fn sample_delete(
    key_expr: KeyExpr,
    timestamp_ntp64: Option<i64>,
    attachment: Option<ZBytes>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> Sample {
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
pub fn sample_key_expr(s: &Sample) -> &KeyExpr {
    s.key_expr()
}

/// Sample payload (borrowed bytes; valid while `s` lives).
#[prebindgen]
pub fn sample_payload(s: &Sample) -> &ZBytes {
    s.payload()
}

/// Encoding of the payload (borrowed; valid while `s` lives).
#[prebindgen]
pub fn sample_encoding(s: &Sample) -> &Encoding {
    s.encoding()
}

/// Whether the sample is a PUT or a DELETE.
#[prebindgen]
pub fn sample_kind(s: &Sample) -> SampleKind {
    s.kind().into()
}

/// Timestamp (borrowed), or `None` when the sample carries no timestamp.
#[prebindgen]
pub fn sample_timestamp(s: &Sample) -> Option<&Timestamp> {
    s.timestamp()
}

/// QoS express flag.
#[prebindgen]
pub fn sample_express(s: &Sample) -> bool {
    s.express()
}

/// QoS priority.
#[prebindgen]
pub fn sample_priority(s: &Sample) -> Priority {
    s.priority().into()
}

/// QoS congestion-control policy.
#[prebindgen]
pub fn sample_congestion_control(s: &Sample) -> CongestionControl {
    s.congestion_control().into()
}

/// Optional user attachment (borrowed bytes), or `None`.
#[prebindgen]
pub fn sample_attachment(s: &Sample) -> Option<&ZBytes> {
    s.attachment()
}

/// Reliability policy the sample was delivered with.
///
/// Unstable: `zenoh::sample::Sample::reliability` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_reliability(s: &Sample) -> Reliability {
    s.reliability().into()
}

/// Zenoh id of the source entity that produced the sample, or `None` when the
/// sample carries no source info (owned handle).
///
/// Unstable: `zenoh::sample::Sample::source_info` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_source_zid(s: &Sample) -> Option<ZenohId> {
    s.source_info().map(|si| si.source_id().zid())
}

/// Entity id of the sample's source (0 when the sample carries no source info).
///
/// Unstable: `zenoh::sample::Sample::source_info` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_source_eid(s: &Sample) -> i32 {
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
pub fn sample_source_sn(s: &Sample) -> i64 {
    s.source_info().map(|si| si.source_sn() as i64).unwrap_or(0)
}
