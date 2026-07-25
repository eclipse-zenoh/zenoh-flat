use prebindgen_proc_macro::prebindgen;

/// Congestion control policy used when routing data.
#[prebindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongestionControl {
    /// Drop messages when buffers are full.
    Drop = 0,
    /// Wait for buffer space instead of dropping messages.
    Block = 1,
    /// Block only while delivering the first message.
    ///
    /// This policy is available only when unstable features are enabled.
    #[cfg(feature = "unstable")]
    BlockFirst = 2,
}

impl From<zenoh::qos::CongestionControl> for CongestionControl {
    fn from(value: zenoh::qos::CongestionControl) -> Self {
        match value {
            zenoh::qos::CongestionControl::Drop => CongestionControl::Drop,
            zenoh::qos::CongestionControl::Block => CongestionControl::Block,
            #[cfg(feature = "unstable")]
            zenoh::qos::CongestionControl::BlockFirst => CongestionControl::BlockFirst,
        }
    }
}

impl From<CongestionControl> for zenoh::qos::CongestionControl {
    fn from(value: CongestionControl) -> Self {
        match value {
            CongestionControl::Drop => zenoh::qos::CongestionControl::Drop,
            CongestionControl::Block => zenoh::qos::CongestionControl::Block,
            #[cfg(feature = "unstable")]
            CongestionControl::BlockFirst => zenoh::qos::CongestionControl::BlockFirst,
        }
    }
}
