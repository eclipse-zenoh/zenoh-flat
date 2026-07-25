use prebindgen_proc_macro::prebindgen;

/// The requested delivery reliability for publications and subscriptions.
///
/// This policy is available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reliability {
    /// Prefer lower overhead without retransmission guarantees.
    BestEffort = 0,
    /// Request retransmission when necessary for reliable delivery.
    Reliable = 1,
}

#[cfg(feature = "unstable")]
impl From<zenoh::qos::Reliability> for Reliability {
    fn from(value: zenoh::qos::Reliability) -> Self {
        match value {
            zenoh::qos::Reliability::BestEffort => Reliability::BestEffort,
            zenoh::qos::Reliability::Reliable => Reliability::Reliable,
        }
    }
}

#[cfg(feature = "unstable")]
impl From<Reliability> for zenoh::qos::Reliability {
    fn from(value: Reliability) -> Self {
        match value {
            Reliability::BestEffort => zenoh::qos::Reliability::BestEffort,
            Reliability::Reliable => zenoh::qos::Reliability::Reliable,
        }
    }
}
