use prebindgen_proc_macro::prebindgen;

/// The policy used to combine replies from multiple queryables.
#[prebindgen]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsolidationMode {
    /// Let Zenoh choose an appropriate consolidation policy.
    Auto = 0,
    /// Deliver every reply without consolidation.
    None = 1,
    /// Deliver replies in monotonic timestamp order for each key expression.
    Monotonic = 2,
    /// Keep only the latest reply for each key expression.
    Latest = 3,
}

impl From<zenoh::query::ConsolidationMode> for ConsolidationMode {
    fn from(c: zenoh::query::ConsolidationMode) -> Self {
        match c {
            zenoh::query::ConsolidationMode::Auto => ConsolidationMode::Auto,
            zenoh::query::ConsolidationMode::None => ConsolidationMode::None,
            zenoh::query::ConsolidationMode::Monotonic => ConsolidationMode::Monotonic,
            zenoh::query::ConsolidationMode::Latest => ConsolidationMode::Latest,
        }
    }
}

impl From<ConsolidationMode> for zenoh::query::ConsolidationMode {
    fn from(c: ConsolidationMode) -> Self {
        match c {
            ConsolidationMode::Auto => zenoh::query::ConsolidationMode::Auto,
            ConsolidationMode::None => zenoh::query::ConsolidationMode::None,
            ConsolidationMode::Monotonic => zenoh::query::ConsolidationMode::Monotonic,
            ConsolidationMode::Latest => zenoh::query::ConsolidationMode::Latest,
        }
    }
}
