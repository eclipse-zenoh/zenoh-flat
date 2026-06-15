#[cfg(feature = "unstable")]
pub(crate) mod set_intersection_level;

use prebindgen_proc_macro::prebindgen;

// `SetIntersectionLevel` (the relation API's return type) mirrors
// `zenoh::key_expr::SetIntersectionLevel`, gated behind `unstable`.
#[cfg(feature = "unstable")]
use self::set_intersection_level::SetIntersectionLevel;
use crate::{Error, KeyExpr};

/// Validate `s` as a key expression and build an owned handle, returning an
/// error if it is not canonical (the flat port of `KeyExpr::try_from`). Use
/// [`keyexpr_autocanonize`] to accept and canonicalize non-canonical input.
#[prebindgen]
pub fn keyexpr_try_from(s: String) -> Result<KeyExpr, Error> {
    let ke = KeyExpr::try_from(s)?;
    Ok(ke)
}

/// Clone a key-expression handle. Use this before passing a handle to a
/// consuming call (e.g. `session_declare_publisher`) when the caller needs to
/// keep the original. Cheap (Arc bump for owned key expressions).
#[prebindgen]
pub fn keyexpr_clone(ke: &KeyExpr) -> KeyExpr {
    ke.clone()
}

/// Canonical string form of a key expression (owned, NUL-terminated `char*`).
#[prebindgen]
pub fn keyexpr_to_string(ke: &KeyExpr) -> String {
    ke.as_str().to_string()
}

/// Borrowed canonical string form of a key expression — zero-copy `&str` into
/// the key expression's own storage. Used as an output-expansion accessor
/// (`expand_output`) so the JNI layer converts `&str → jstring` in a single
/// copy (no intermediate owned `String`). The owned [`keyexpr_to_string`]
/// twin remains for the C / owned-`char*` tier.
#[prebindgen]
pub fn keyexpr_get_str(ke: &KeyExpr) -> &str {
    ke.as_str()
}

/// Canonicalize `s` and build an owned key-expression handle (the flat port of
/// `KeyExpr::autocanonize`). Unlike [`keyexpr_try_from`], this rewrites
/// redundant wildcards into canonical form instead of rejecting them; it still
/// errors on input that is not a valid key expression at all.
#[prebindgen]
pub fn keyexpr_autocanonize(s: String) -> Result<KeyExpr, Error> {
    let ke = KeyExpr::autocanonize(s)?;
    Ok(ke)
}

/// Whether `a` and `b` share at least one key they both match (the flat port of
/// `keyexpr::intersects`).
#[prebindgen]
pub fn keyexpr_intersects(a: &KeyExpr, b: &KeyExpr) -> bool {
    a.intersects(b)
}

/// Whether every key matched by `b` is also matched by `a`, i.e. `a` includes
/// `b` (the flat port of `keyexpr::includes`).
#[prebindgen]
pub fn keyexpr_includes(a: &KeyExpr, b: &KeyExpr) -> bool {
    a.includes(b)
}

/// The set relation of `a` to `b` (disjoint / intersects / includes / equals) —
/// the flat port of `keyexpr::relation_to`, a finer-grained result than the
/// boolean [`keyexpr_intersects`] / [`keyexpr_includes`] pair.
///
/// Unstable: `zenoh::key_expr::SetIntersectionLevel` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn keyexpr_relation_to(a: &KeyExpr, b: &KeyExpr) -> SetIntersectionLevel {
    a.relation_to(b).into()
}

/// Join `a` with `b` using `/` as separator, returning a new owned key
/// expression (the flat port of `keyexpr::join`). Errors if the result is not a
/// valid key expression.
#[prebindgen]
pub fn keyexpr_join(a: &KeyExpr, b: String) -> Result<KeyExpr, Error> {
    Ok(a.join(&b)?)
}

/// Concatenate `b` onto `a` verbatim (no separator inserted), returning a new
/// owned key expression (the flat port of `keyexpr::concat`). Errors if the
/// result is not a valid key expression.
#[prebindgen]
pub fn keyexpr_concat(a: &KeyExpr, b: String) -> Result<KeyExpr, Error> {
    Ok(a.concat(&b)?)
}
