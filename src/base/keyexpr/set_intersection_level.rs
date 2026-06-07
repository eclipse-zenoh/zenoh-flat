use prebindgen_proc_macro::prebindgen;

/// Mirrors `zenoh::key_expr::SetIntersectionLevel` with a stable FFI surface.
///
/// Unstable: the underlying `zenoh::key_expr::SetIntersectionLevel` is
/// `#[cfg(feature = "unstable")]`. Returned by [`crate::z_keyexpr_relation_to`].
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub enum SetIntersectionLevel {
    Disjoint = 0,
    Intersects = 1,
    Includes = 2,
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
