use prebindgen_proc_macro::prebindgen;

use crate::{Timestamp, TimestampStack};

/// Where along a message's path an interception timestamp was recorded.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterceptionPoint {
    /// Recorded when the message leaves the publishing or replying node.
    Send,
    /// Recorded when the message passes through a routing node.
    Route,
    /// Recorded when the message arrives at a subscribing or queryable node.
    Receive,
}

impl From<zenoh::timestamp_stack::InterceptionPoint> for InterceptionPoint {
    fn from(p: zenoh::timestamp_stack::InterceptionPoint) -> Self {
        match p {
            zenoh::timestamp_stack::InterceptionPoint::Send => InterceptionPoint::Send,
            zenoh::timestamp_stack::InterceptionPoint::Route => InterceptionPoint::Route,
            zenoh::timestamp_stack::InterceptionPoint::Receive => InterceptionPoint::Receive,
        }
    }
}

/// The timestamp carried by an interception record.
///
/// A record's timestamp is either zenoh's own hybrid logical clock or bytes in
/// a format the application's timestamp callback defined — never both, so the
/// alternatives are variants rather than parallel optional fields.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstrumentationTimestamp {
    /// A zenoh hybrid-logical-clock timestamp.
    Uhlc(Timestamp),
    /// A timestamp in an application-defined format.
    Custom(Vec<u8>),
}

/// One interception record: where the measurement was taken, and when.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimestampStackRecord {
    /// Point along the path at which this timestamp was recorded.
    pub point: InterceptionPoint,
    /// The timestamp recorded there.
    pub timestamp: InstrumentationTimestamp,
}

impl From<&zenoh::timestamp_stack::TimestampStackRecord> for TimestampStackRecord {
    fn from(r: &zenoh::timestamp_stack::TimestampStackRecord) -> Self {
        // `is_custom()` is not carried as a field: it is exactly "which variant
        // is live", and a flag beside the data would be a second way to say the
        // same thing.
        let timestamp = match r.timestamp() {
            zenoh::timestamp_stack::InstrumentationTimestamp::UHLC(t) => {
                InstrumentationTimestamp::Uhlc(Timestamp::from(t))
            }
            zenoh::timestamp_stack::InstrumentationTimestamp::Custom(bytes) => {
                InstrumentationTimestamp::Custom(bytes.clone())
            }
        };
        TimestampStackRecord {
            point: r.point().into(),
            timestamp,
        }
    }
}

/// Which interception points a message's instrumentation was configured to
/// record.
///
/// This is what was *asked for*, which is not what the records show: a stack may
/// be configured for [`InterceptionPoint::Route`] and still carry no route
/// record, because the message met no routing node.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TimestampInstrumentation {
    /// Whether the sending node was asked to record a timestamp.
    pub send: bool,
    /// Whether forwarding nodes were asked to record a timestamp.
    pub route: bool,
    /// Whether the receiving node was asked to record a timestamp.
    pub receive: bool,
}

impl From<zenoh::timestamp_stack::TimestampInstrumentation> for TimestampInstrumentation {
    fn from(i: zenoh::timestamp_stack::TimestampInstrumentation) -> Self {
        // Base keeps the configuration as a private bit mask; one bool per point
        // carries the same information without exposing a wire detail.
        use zenoh::timestamp_stack::InterceptionPoint as Point;
        TimestampInstrumentation {
            send: i.is_instrumented(Point::Send),
            route: i.is_instrumented(Point::Route),
            receive: i.is_instrumented(Point::Receive),
        }
    }
}

/// Return which interception points this stack's message was instrumented for.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn timestamp_stack_get_instrumentation(s: &TimestampStack) -> TimestampInstrumentation {
    s.instrumentation().into()
}

/// Return the interception records collected along the message's path, in
/// traversal order.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn timestamp_stack_get_records(s: &TimestampStack) -> Vec<TimestampStackRecord> {
    s.records().iter().map(TimestampStackRecord::from).collect()
}

/// A timestamp stack decomposed into its fields as a plain value.
///
/// This is the value form of [`TimestampStack`]: it materializes the whole
/// record list, which the handle lets a caller skip when only the configuration
/// is wanted.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimestampStackStruct {
    /// Which interception points the message was instrumented for.
    pub instrumentation: TimestampInstrumentation,
    /// The records collected along the message's path, in traversal order.
    pub records: Vec<TimestampStackRecord>,
}

impl From<&TimestampStack> for TimestampStackStruct {
    fn from(s: &TimestampStack) -> Self {
        // Delegate to the field accessors so each field has one definition.
        TimestampStackStruct {
            instrumentation: timestamp_stack_get_instrumentation(s),
            records: timestamp_stack_get_records(s),
        }
    }
}

/// Decompose a timestamp stack into its [`TimestampStackStruct`] value form.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn timestamp_stack_to_struct(s: &TimestampStack) -> TimestampStackStruct {
    s.into()
}

#[cfg(test)]
mod tests {
    use zenoh::timestamp_stack::TimestampInstrumentationBuilder;

    use super::*;

    /// Each configured point lands on its own field. The subject enables a
    /// **mixed** set — send and receive but not route — so a swapped or
    /// hardcoded flag is visible, which an all-on or all-off subject would hide.
    #[test]
    fn instrumentation_carries_each_point_separately() {
        let base = TimestampInstrumentationBuilder::new()
            .set_send(true)
            .set_route(false)
            .set_receive(true)
            .build()
            .expect("at least one point is enabled");

        assert_eq!(
            TimestampInstrumentation::from(base),
            TimestampInstrumentation {
                send: true,
                route: false,
                receive: true,
            }
        );
    }
}
