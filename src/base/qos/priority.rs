use prebindgen_proc_macro::prebindgen;

/// The delivery priority of a message, from real-time to background traffic.
#[prebindgen]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    /// Highest priority for real-time data.
    RealTime = 1,
    /// High priority for interactive traffic.
    InteractiveHigh = 2,
    /// Low priority for interactive traffic.
    InteractiveLow = 3,
    /// High priority for ordinary data.
    DataHigh = 4,
    /// Standard priority for ordinary data.
    Data = 5,
    /// Low priority for ordinary data.
    DataLow = 6,
    /// Lowest priority for background traffic.
    Background = 7,
}

impl From<zenoh::qos::Priority> for Priority {
    fn from(value: zenoh::qos::Priority) -> Self {
        match value {
            zenoh::qos::Priority::RealTime => Priority::RealTime,
            zenoh::qos::Priority::InteractiveHigh => Priority::InteractiveHigh,
            zenoh::qos::Priority::InteractiveLow => Priority::InteractiveLow,
            zenoh::qos::Priority::DataHigh => Priority::DataHigh,
            zenoh::qos::Priority::Data => Priority::Data,
            zenoh::qos::Priority::DataLow => Priority::DataLow,
            zenoh::qos::Priority::Background => Priority::Background,
        }
    }
}

impl From<Priority> for zenoh::qos::Priority {
    fn from(value: Priority) -> Self {
        match value {
            Priority::RealTime => zenoh::qos::Priority::RealTime,
            Priority::InteractiveHigh => zenoh::qos::Priority::InteractiveHigh,
            Priority::InteractiveLow => zenoh::qos::Priority::InteractiveLow,
            Priority::DataHigh => zenoh::qos::Priority::DataHigh,
            Priority::Data => zenoh::qos::Priority::Data,
            Priority::DataLow => zenoh::qos::Priority::DataLow,
            Priority::Background => zenoh::qos::Priority::Background,
        }
    }
}
