pub(crate) mod sample_kind;
#[cfg(feature = "unstable")]
pub(crate) mod source_info;

use prebindgen_proc_macro::prebindgen;
use zenoh::sample::SampleBuilder;

use self::sample_kind::SampleKind;
use crate::{CongestionControl, Encoding, Error, KeyExpr, Priority, Sample, Timestamp, ZBytes};
#[cfg(feature = "unstable")]
use crate::{Reliability, SourceInfo, TimestampStack};

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

/// Return the timestamps this sample accumulated along its path, when
/// instrumentation recorded any.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_get_timestamp_stack(s: &Sample) -> Option<&TimestampStack> {
    s.timestamp_stack()
}

/// A sample decomposed into its fields as a value form.
///
/// This is the value form of [`Sample`]: the sample's accessors gathered into
/// one struct, for callers that prefer the whole sample as data over reading
/// fields from the handle one at a time. Fields whose type is a handle stay
/// handles, so taking a sample apart never copies a nested payload.
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
    /// Timestamps accumulated along the sample's path, when instrumentation
    /// recorded any. Available only when unstable features are enabled.
    #[cfg(feature = "unstable")]
    pub timestamp_stack: Option<TimestampStack>,
}

impl From<Sample> for SampleStruct {
    fn from(s: Sample) -> Self {
        // `zenoh::sample::SampleFields` is zenoh's own by-value exit: it exists,
        // in zenoh's words, so a sample can be deconstructed to its fields
        // without cloning. Every field below is therefore a move.
        let f = zenoh::sample::SampleFields::from(s);
        SampleStruct {
            key_expr: f.key_expr,
            payload: f.payload,
            encoding: f.encoding,
            kind: f.kind.into(),
            timestamp: f.timestamp.as_ref().map(Timestamp::from),
            express: f.express,
            priority: f.priority.into(),
            congestion_control: f.congestion_control.into(),
            attachment: f.attachment,
            #[cfg(feature = "unstable")]
            reliability: f.reliability.into(),
            #[cfg(feature = "unstable")]
            source_info: f.source_info.as_ref().map(SourceInfo::from),
            #[cfg(feature = "unstable")]
            timestamp_stack: f.timestamp_stack,
        }
    }
}

impl From<&Sample> for SampleStruct {
    fn from(s: &Sample) -> Self {
        // The consuming form is the single body — README §One source of truth
        // per field. Cloning the sample costs the same field clones this form
        // used to pay one by one.
        s.clone().into()
    }
}

/// Decompose a sample into its [`SampleStruct`] value form.
#[prebindgen]
pub fn sample_to_struct(s: &Sample) -> SampleStruct {
    s.into()
}

/// Decompose a sample into its [`SampleStruct`] value form, consuming it.
///
/// Unlike [`sample_to_struct`] this destroys the sample, which lets each field
/// **move** into the value form instead of being cloned. Prefer it wherever the
/// sample is owned and not needed afterwards — a subscriber callback, which is
/// handed its sample by value.
#[prebindgen]
pub fn sample_into_struct(s: Sample) -> SampleStruct {
    s.into()
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "unstable")]
    use super::source_info::sample_get_source_info;
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
        assert_eq!(&st.encoding, sample_get_encoding(s));
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
        #[cfg(feature = "unstable")]
        assert_eq!(st.timestamp_stack.as_ref(), sample_get_timestamp_stack(s));
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
        assert_eq!(&st.encoding, encoding_const_text_plain());
        assert_eq!(st.timestamp, Some(marker_timestamp()));
        assert!(st.express);
        assert_eq!(st.priority, Priority::InteractiveHigh);
        assert_eq!(st.congestion_control, CongestionControl::Block);
        assert!(st.attachment.is_some());
        #[cfg(feature = "unstable")]
        assert_eq!(st.reliability, Reliability::BestEffort);
    }

    /// A twin-typed field is carried as its **handle**, not its value form —
    /// README §Composing a value: a `…Struct` opens the one handle it was
    /// called on and stops there.
    ///
    /// This is checked structurally rather than by reading the field's value:
    /// the annotated binding below only compiles while the field is the handle.
    /// If `SampleStruct.encoding` reverted to `EncodingStruct` — pulling the
    /// encoding's arbitrary-length schema into every `sample_to_struct` call —
    /// this test would stop compiling, which is the point.
    ///
    /// `ReplyErrorStruct.encoding` is pinned the same way by
    /// `reply_struct_mismatches` in `tests/queryable.rs`, which compares it
    /// against `reply_error_get_encoding(..)` and so equally stops compiling if
    /// that field reverts.
    #[test]
    fn twin_fields_are_carried_as_handles() {
        let s = put_sample();
        let st = sample_to_struct(&s);

        let encoding: Encoding = st.encoding;
        assert_eq!(&encoding, sample_get_encoding(&s));
    }

    #[test]
    fn delete_sample_has_delete_kind() {
        assert_eq!(sample_to_struct(&delete_sample()).kind, SampleKind::Delete);
    }
}
