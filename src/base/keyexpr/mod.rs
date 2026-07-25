#[cfg(feature = "unstable")]
pub(crate) mod set_intersection_level;

use prebindgen_proc_macro::prebindgen;

// `SetIntersectionLevel` (the relation API's return type) mirrors
// `zenoh::key_expr::SetIntersectionLevel`, gated behind `unstable`.
#[cfg(feature = "unstable")]
use self::set_intersection_level::SetIntersectionLevel;
use crate::{Error, KeyExpr};

/// Validate text as a canonical Zenoh key expression.
///
/// Use [`keyexpr_new_autocanonize`] to canonicalize the text before validation.
#[prebindgen]
pub fn keyexpr_new_try_from(s: String) -> Result<KeyExpr, Error> {
    let ke = KeyExpr::try_from(s)?;
    Ok(ke)
}

/// Create an independent copy of a key expression.
#[prebindgen]
pub fn keyexpr_new_clone(ke: &KeyExpr) -> KeyExpr {
    ke.clone()
}

/// Return the canonical text of a key expression as an owned copy.
///
/// Use [`keyexpr_as_str`] to read the text without copying it.
#[prebindgen]
pub fn keyexpr_to_string(ke: &KeyExpr) -> String {
    ke.as_str().to_string()
}

/// Borrow the canonical text of a key expression.
///
/// The text is valid for as long as the key expression is. Use
/// [`keyexpr_to_string`] when an owned copy is needed.
#[prebindgen]
pub fn keyexpr_as_str(ke: &KeyExpr) -> &str {
    ke.as_str()
}

/// Convert text to canonical key-expression form and validate the result.
///
/// Unlike [`keyexpr_new_try_from`], this accepts valid non-canonical forms such
/// as redundant wildcards and normalizes them.
#[prebindgen]
pub fn keyexpr_new_autocanonize(s: String) -> Result<KeyExpr, Error> {
    let ke = KeyExpr::autocanonize(s)?;
    Ok(ke)
}

/// Return whether the two key expressions can match at least one common key.
#[prebindgen]
pub fn keyexpr_intersects(a: &KeyExpr, b: &KeyExpr) -> bool {
    a.intersects(b)
}

/// Return whether every key matched by `b` is also matched by `a`.
#[prebindgen]
pub fn keyexpr_includes(a: &KeyExpr, b: &KeyExpr) -> bool {
    a.includes(b)
}

/// Describe how the sets of keys matched by two expressions relate.
///
/// This operation is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn keyexpr_relation_to(a: &KeyExpr, b: &KeyExpr) -> SetIntersectionLevel {
    a.relation_to(b).into()
}

/// Join a key expression and a suffix with a `/` separator.
///
/// Returns an error if the result is not a valid key expression.
#[prebindgen]
pub fn keyexpr_new_join(a: &KeyExpr, b: String) -> Result<KeyExpr, Error> {
    a.join(&b)
}

/// Append text directly to a key expression and validate the result.
///
/// No separator is inserted. Prefer [`keyexpr_new_join`] when combining path
/// segments.
#[prebindgen]
pub fn keyexpr_new_concat(a: &KeyExpr, b: String) -> Result<KeyExpr, Error> {
    a.concat(&b)
}
