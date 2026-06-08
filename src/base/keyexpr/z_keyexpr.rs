use crate::ZError;
use crate::ZKeyExpr;
use prebindgen_proc_macro::prebindgen;
// `SetIntersectionLevel` mirrors `zenoh::key_expr::SetIntersectionLevel`, which
// is `#[cfg(feature = "unstable")]` — gate the relation API behind `unstable`.
#[cfg(feature = "unstable")]
use crate::SetIntersectionLevel;

#[prebindgen]
pub fn z_keyexpr_try_from(s: String) -> Result<ZKeyExpr, ZError> {
    let ke = ZKeyExpr::try_from(s)?;
    Ok(ke)
}

/// Clone a key-expression handle. Use this before passing a handle to a
/// consuming call (e.g. `z_session_declare_publisher`) when the caller needs to
/// keep the original. Cheap (Arc bump for owned key expressions).
#[prebindgen]
pub fn z_keyexpr_clone(ke: &ZKeyExpr) -> ZKeyExpr {
    ke.clone()
}

/// Canonical string form of a key expression (owned, NUL-terminated `char*`).
#[prebindgen]
pub fn z_keyexpr_to_string(ke: &ZKeyExpr) -> String {
    ke.as_str().to_string()
}

/// Borrowed canonical string form of a key expression — zero-copy `&str` into
/// the key expression's own storage. Used as an output-expansion accessor
/// (`expand_output`) so the JNI layer converts `&str → jstring` in a single
/// copy (no intermediate owned `String`). The owned [`z_keyexpr_to_string`]
/// twin remains for the C / owned-`char*` tier.
#[prebindgen]
pub fn z_keyexpr_as_str(ke: &ZKeyExpr) -> &str {
    ke.as_str()
}

#[prebindgen]
pub fn z_keyexpr_autocanonize(s: String) -> Result<ZKeyExpr, ZError> {
    let ke = ZKeyExpr::autocanonize(s)?;
    Ok(ke)
}

#[prebindgen]
pub fn z_keyexpr_intersects(a: &ZKeyExpr, b: &ZKeyExpr) -> bool {
    a.intersects(b)
}

#[prebindgen]
pub fn z_keyexpr_includes(a: &ZKeyExpr, b: &ZKeyExpr) -> bool {
    a.includes(b)
}

#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn z_keyexpr_relation_to(a: &ZKeyExpr, b: &ZKeyExpr) -> SetIntersectionLevel {
    a.relation_to(b).into()
}

#[prebindgen]
pub fn z_keyexpr_join(a: &ZKeyExpr, b: String) -> Result<ZKeyExpr, ZError> {
    Ok(a.join(&b)?)
}

#[prebindgen]
pub fn z_keyexpr_concat(a: &ZKeyExpr, b: String) -> Result<ZKeyExpr, ZError> {
    Ok(a.concat(&b)?)
}
