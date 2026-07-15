use prebindgen_proc_macro::prebindgen;

/// The relationship between the sets of keys matched by two key expressions.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetIntersectionLevel {
    /// The expressions cannot match a common key.
    Disjoint = 0,
    /// The expressions can match a common key, but neither includes the other.
    Intersects = 1,
    /// The first expression matches every key matched by the second.
    Includes = 2,
    /// The expressions match the same set of keys.
    Equals = 3,
}

#[cfg(feature = "unstable")]
impl From<zenoh::key_expr::SetIntersectionLevel> for SetIntersectionLevel {
    fn from(value: zenoh::key_expr::SetIntersectionLevel) -> Self {
        match value {
            zenoh::key_expr::SetIntersectionLevel::Disjoint => SetIntersectionLevel::Disjoint,
            zenoh::key_expr::SetIntersectionLevel::Intersects => SetIntersectionLevel::Intersects,
            zenoh::key_expr::SetIntersectionLevel::Includes => SetIntersectionLevel::Includes,
            zenoh::key_expr::SetIntersectionLevel::Equals => SetIntersectionLevel::Equals,
        }
    }
}
