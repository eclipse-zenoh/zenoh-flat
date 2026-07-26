pub(crate) mod sample_kind;
#[cfg(feature = "unstable")]
pub(crate) mod source_info;

use prebindgen_proc_macro::prebindgen;
use zenoh::sample::SampleBuilder;

use self::sample_kind::SampleKind;
#[cfg(feature = "unstable")]
use self::source_info::sample_get_source_info;
use crate::{
    CongestionControl, Encoding, EncodingStruct, Error, KeyExpr, Priority, Sample, Timestamp,
    ZBytes, encoding_to_struct,
};
#[cfg(feature = "unstable")]
use crate::{Reliability, SourceInfo};

/// Create a sample that publishes a value.
///
/// Optional arguments specify the payload format, timestamp, attachment, and
/// delivery quality. Reliability is available only when unstable features are
/// enabled.
///
/// A supplied timestamp is used exactly as given, node id included; take one
/// from [`crate::session_new_timestamp`] to stay causally consistent with the
/// session.
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn sample_new_put(
    key_expr: KeyExpr,
    payload: ZBytes,
    encoding: Option<&Encoding>,
    timestamp: Option<Timestamp>,
    attachment: Option<ZBytes>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> Result<Sample, Error> {
    let mut builder = SampleBuilder::put(key_expr, payload);
    if let Some(enc) = encoding {
        builder = builder.encoding(enc.clone());
    }
    if let Some(t) = timestamp {
        builder = builder.timestamp(zenoh::time::Timestamp::try_from(&t)?);
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
    Ok(builder.into())
}

/// Create a sample that announces a deletion.
///
/// A delete sample has no payload or encoding. Optional arguments specify its
/// timestamp, attachment, and delivery quality.
///
/// A supplied timestamp is used exactly as given, node id included; take one
/// from [`crate::session_new_timestamp`] to stay causally consistent with the
/// session.
#[prebindgen]
pub fn sample_new_delete(
    key_expr: KeyExpr,
    timestamp: Option<Timestamp>,
    attachment: Option<ZBytes>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    #[cfg(feature = "unstable")] reliability: Option<Reliability>,
) -> Result<Sample, Error> {
    let mut builder = SampleBuilder::delete(key_expr);
    if let Some(t) = timestamp {
        builder = builder.timestamp(zenoh::time::Timestamp::try_from(&t)?);
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
    Ok(builder.into())
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
    pub encoding: EncodingStruct,
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
        // Delegate to the field accessors so each field has one definition.
        // The accessors that lend a field (`&KeyExpr`, `&ZBytes`, `&Encoding`)
        // are cloned here, since the value form owns its fields.
        SampleStruct {
            key_expr: sample_get_key_expr(s).clone(),
            payload: sample_get_payload(s).clone(),
            encoding: encoding_to_struct(sample_get_encoding(s)),
            kind: sample_get_kind(s),
            timestamp: sample_get_timestamp(s),
            express: sample_get_express(s),
            priority: sample_get_priority(s),
            congestion_control: sample_get_congestion_control(s),
            attachment: sample_get_attachment(s).cloned(),
            #[cfg(feature = "unstable")]
            reliability: sample_get_reliability(s),
            #[cfg(feature = "unstable")]
            source_info: sample_get_source_info(s),
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
    use crate::{encoding_const_text_plain, keyexpr_new_try_from, zbytes_new_from_slice};

    /// NTP64 with the high bit set, so a value that silently round-trips
    /// through `i64` is visible.
    const NTP64_MARKER: u64 = (i64::MAX as u64) + 12_345;

    /// A distinctive node id, so a timestamp whose id was fabricated rather
    /// than carried through is visible.
    const ID_MARKER: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];

    fn marker_timestamp() -> Timestamp {
        Timestamp {
            ntp64: NTP64_MARKER,
            id: ID_MARKER.to_vec(),
        }
    }

    /// Assert that every field of the value form equals the accessor for that
    /// same field. This is the guard for "one source of truth per field": it
    /// fails the moment a field's value form stops delegating and starts
    /// re-deriving something different.
    fn assert_struct_mirrors_accessors(s: &Sample) {
        let st = sample_to_struct(s);
        assert_eq!(&st.key_expr, sample_get_key_expr(s));
        assert_eq!(&st.payload, sample_get_payload(s));
        assert_eq!(st.encoding, encoding_to_struct(sample_get_encoding(s)));
        assert_eq!(st.kind, sample_get_kind(s));
        assert_eq!(st.timestamp, sample_get_timestamp(s));
        assert_eq!(st.express, sample_get_express(s));
        assert_eq!(st.priority, sample_get_priority(s));
        assert_eq!(st.congestion_control, sample_get_congestion_control(s));
        assert_eq!(st.attachment.as_ref(), sample_get_attachment(s));
        #[cfg(feature = "unstable")]
        assert_eq!(st.reliability, sample_get_reliability(s));
        #[cfg(feature = "unstable")]
        assert_eq!(st.source_info, sample_get_source_info(s));
    }

    /// A put sample with every settable field carrying a **distinctive,
    /// non-default** value. This is what gives
    /// [`assert_struct_mirrors_accessors`] its teeth: against a sample left at
    /// its defaults, a value form that re-derived a field as a hardcoded
    /// default would still agree with the accessor and the test would pass.
    fn put_sample() -> Sample {
        let ke = keyexpr_new_try_from("test/ke".to_string()).unwrap();
        sample_new_put(
            ke,
            zbytes_new_from_slice(b"hello"),
            // Not the default ZENOH_BYTES.
            Some(encoding_const_text_plain()),
            Some(marker_timestamp()),
            Some(zbytes_new_from_slice(b"attachment")),
            // Not the default Drop.
            Some(CongestionControl::Block),
            // Not the default Data.
            Some(Priority::InteractiveHigh),
            // Not the default false.
            Some(true),
            // Not the default Reliable.
            #[cfg(feature = "unstable")]
            Some(Reliability::BestEffort),
        )
        .expect("marker timestamp is valid")
    }

    /// The delete counterpart, so `kind` is exercised as something other than
    /// `Put`.
    fn delete_sample() -> Sample {
        let ke = keyexpr_new_try_from("test/ke".to_string()).unwrap();
        sample_new_delete(
            ke,
            Some(marker_timestamp()),
            Some(zbytes_new_from_slice(b"attachment")),
            Some(CongestionControl::Block),
            Some(Priority::InteractiveHigh),
            Some(true),
            #[cfg(feature = "unstable")]
            Some(Reliability::BestEffort),
        )
        .expect("marker timestamp is valid")
    }

    #[test]
    fn put_struct_mirrors_accessors() {
        assert_struct_mirrors_accessors(&put_sample());
    }

    #[test]
    fn delete_struct_mirrors_accessors() {
        assert_struct_mirrors_accessors(&delete_sample());
    }

    /// The fields really do carry the distinctive values above — otherwise
    /// `assert_struct_mirrors_accessors` would be comparing defaults against
    /// defaults and could not catch a hardcoding regression.
    #[test]
    fn put_sample_fields_are_non_default() {
        let st = sample_to_struct(&put_sample());
        assert_eq!(st.kind, SampleKind::Put);
        assert_eq!(st.encoding, encoding_to_struct(encoding_const_text_plain()));
        assert_eq!(st.timestamp, Some(marker_timestamp()));
        assert!(st.express);
        assert_eq!(st.priority, Priority::InteractiveHigh);
        assert_eq!(st.congestion_control, CongestionControl::Block);
        assert!(st.attachment.is_some());
        #[cfg(feature = "unstable")]
        assert_eq!(st.reliability, Reliability::BestEffort);
    }

    /// A twin-typed field is carried as its **value form**, not its handle —
    /// the composition rule from README §Composing a value.
    ///
    /// This is checked structurally rather than by reading the field's value:
    /// `encoding_to_struct` is the only way to reach an `EncodingStruct` from an
    /// `Encoding`, so the assignment below only compiles while the field is the
    /// value form. If `SampleStruct.encoding` reverted to `Encoding`, this test
    /// would stop compiling — which is the point, since the previous defect was
    /// exactly one nesting level disagreeing with another.
    ///
    /// `ReplyErrorStruct.encoding` is pinned the same way by
    /// `reply_struct_mismatches` in `tests/queryable.rs`, which compares it
    /// against `encoding_to_struct(..)` and so equally stops compiling if that
    /// field reverts.
    #[test]
    fn twin_fields_are_carried_as_value_forms() {
        let s = put_sample();
        let st = sample_to_struct(&s);

        let encoding: EncodingStruct = st.encoding;
        assert_eq!(encoding, encoding_to_struct(sample_get_encoding(&s)));
    }

    #[test]
    fn delete_sample_has_delete_kind() {
        assert_eq!(sample_to_struct(&delete_sample()).kind, SampleKind::Delete);
    }
}
