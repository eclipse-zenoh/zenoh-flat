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

/// Create a sample that publishes a value.
///
/// Optional arguments specify the payload format, timestamp, attachment, and
/// delivery quality. Reliability is available only when unstable features are
/// enabled.
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn sample_new_put(
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

/// Create a sample that announces a deletion.
///
/// A delete sample has no payload or encoding. Optional arguments specify its
/// timestamp, attachment, and delivery quality.
#[prebindgen]
pub fn sample_new_delete(
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

/// Return the key expression on which the sample was published.
#[prebindgen]
pub fn sample_get_key_expr(s: &Sample) -> &KeyExpr {
    s.key_expr()
}

/// Return the sample payload.
#[prebindgen]
pub fn sample_get_payload(s: &Sample) -> &ZBytes {
    s.payload()
}

/// Return format information associated with the payload.
#[prebindgen]
pub fn sample_get_encoding(s: &Sample) -> &Encoding {
    s.encoding()
}

/// Return whether the sample publishes a value or announces a deletion.
#[prebindgen]
pub fn sample_get_kind(s: &Sample) -> SampleKind {
    s.kind().into()
}

/// Return the publication timestamp, when present.
#[prebindgen]
pub fn sample_get_timestamp(s: &Sample) -> Option<&Timestamp> {
    s.timestamp()
}

/// Return whether express delivery was requested.
#[prebindgen]
pub fn sample_get_express(s: &Sample) -> bool {
    s.express()
}

/// Return the sample's delivery priority.
#[prebindgen]
pub fn sample_get_priority(s: &Sample) -> Priority {
    s.priority().into()
}

/// Return the congestion-control policy used for the sample.
#[prebindgen]
pub fn sample_get_congestion_control(s: &Sample) -> CongestionControl {
    s.congestion_control().into()
}

/// Return user-defined metadata associated with the sample, when present.
#[prebindgen]
pub fn sample_get_attachment(s: &Sample) -> Option<&ZBytes> {
    s.attachment()
}

/// Return the reliability policy used to deliver the sample.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_get_reliability(s: &Sample) -> Reliability {
    s.reliability().into()
}

/// Return the identifier of the node that produced the sample, when known.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_get_source_zid(s: &Sample) -> Option<ZenohId> {
    s.source_info().map(|si| si.source_id().zid())
}

/// Return the entity identifier of the sample's source, or `0` when unknown.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_get_source_eid(s: &Sample) -> i32 {
    s.source_info()
        .map(|si| si.source_id().eid() as i32)
        .unwrap_or(0)
}

/// Return the source sequence number, or `0` when source information is absent.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_get_source_sn(s: &Sample) -> i64 {
    s.source_info().map(|si| si.source_sn() as i64).unwrap_or(0)
}
