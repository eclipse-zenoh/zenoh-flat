pub(crate) mod sample_kind;
#[cfg(feature = "unstable")]
pub(crate) mod source_info;

use prebindgen_proc_macro::prebindgen;
use zenoh::{
    sample::SampleBuilder,
    time::{NTP64, TimestampId},
};

use self::sample_kind::SampleKind;
use crate::{CongestionControl, Encoding, KeyExpr, Priority, Sample, Timestamp, ZBytes};
#[cfg(feature = "unstable")]
use crate::{Reliability, SourceInfo};

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
    timestamp_ntp64: Option<u64>,
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
        builder = builder.timestamp(zenoh::time::Timestamp::new(NTP64(ntp), TimestampId::rand()));
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
    timestamp_ntp64: Option<u64>,
    attachment: Option<ZBytes>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> Sample {
    let mut builder = SampleBuilder::delete(key_expr);
    if let Some(ntp) = timestamp_ntp64 {
        builder = builder.timestamp(zenoh::time::Timestamp::new(NTP64(ntp), TimestampId::rand()));
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
pub fn sample_get_timestamp(s: &Sample) -> Option<Timestamp> {
    s.timestamp().map(Timestamp::from)
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

/// A sample decomposed into its fields as a plain value.
///
/// This is the value form of [`Sample`]: it owns a full copy of every field,
/// for callers that prefer the whole sample as data over reading fields from
/// the handle one at a time.
#[prebindgen]
#[derive(Clone, Debug)]
pub struct SampleStruct {
    /// Key expression on which the sample was published.
    pub key_expr: KeyExpr,
    /// Sample payload.
    pub payload: ZBytes,
    /// Format information associated with the payload.
    pub encoding: Encoding,
    /// Whether the sample publishes a value or announces a deletion.
    pub kind: SampleKind,
    /// Publication timestamp, when present.
    pub timestamp: Option<Timestamp>,
    /// Whether express delivery was requested.
    pub express: bool,
    /// Delivery priority.
    pub priority: Priority,
    /// Congestion-control policy used for the sample.
    pub congestion_control: CongestionControl,
    /// User-defined metadata associated with the sample, when present.
    pub attachment: Option<ZBytes>,
    /// Reliability policy used to deliver the sample. Available only when
    /// unstable features are enabled.
    #[cfg(feature = "unstable")]
    pub reliability: Reliability,
    /// Source information, when known. Available only when unstable features
    /// are enabled.
    #[cfg(feature = "unstable")]
    pub source_info: Option<SourceInfo>,
}

impl From<&Sample> for SampleStruct {
    fn from(s: &Sample) -> Self {
        SampleStruct {
            key_expr: s.key_expr().clone(),
            payload: s.payload().clone(),
            encoding: s.encoding().clone(),
            kind: s.kind().into(),
            timestamp: s.timestamp().map(Timestamp::from),
            express: s.express(),
            priority: s.priority().into(),
            congestion_control: s.congestion_control().into(),
            attachment: s.attachment().cloned(),
            #[cfg(feature = "unstable")]
            reliability: s.reliability().into(),
            #[cfg(feature = "unstable")]
            source_info: s.source_info().map(SourceInfo::from),
        }
    }
}

/// Decompose a sample into its [`SampleStruct`] value form.
#[prebindgen]
pub fn sample_to_struct(s: &Sample) -> SampleStruct {
    s.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{keyexpr_new_try_from, zbytes_new_from_slice};

    fn put_sample() -> Sample {
        let ke = keyexpr_new_try_from("test/ke".to_string()).unwrap();
        sample_new_put(
            ke,
            zbytes_new_from_slice(b"hello"),
            None,
            None,
            None,
            None,
            None,
            Some(true),
            #[cfg(feature = "unstable")]
            None,
        )
    }

    #[test]
    fn sample_to_struct_mirrors_accessors() {
        let s = put_sample();
        let st = sample_to_struct(&s);
        assert_eq!(st.express, sample_get_express(&s));
        assert!(st.express);
    }
}
