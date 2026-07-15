use crate::Error;
use crate::ZKeyExpr;
use prebindgen_proc_macro::prebindgen;

/// Borrow the canonical string representation of a key expression.
#[prebindgen]
pub fn z_keyexpr_get_str(key_expr: &ZKeyExpr) -> &str {
    key_expr.as_str()
}
// `SetIntersectionLevel` mirrors `zenoh::key_expr::SetIntersectionLevel`, which
// is `#[cfg(feature = "unstable")]` — gate the relation API behind `unstable`.
#[cfg(feature = "unstable")]
use crate::SetIntersectionLevel;

#[prebindgen]
pub fn z_keyexpr_try_from(s: String) -> Result<ZKeyExpr, Error> {
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

#[prebindgen]
pub fn z_keyexpr_autocanonize(s: String) -> Result<ZKeyExpr, Error> {
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
pub fn z_keyexpr_join(a: &ZKeyExpr, b: String) -> Result<ZKeyExpr, Error> {
    Ok(a.join(&b)?)
}

#[prebindgen]
pub fn z_keyexpr_concat(a: &ZKeyExpr, b: String) -> Result<ZKeyExpr, Error> {
    Ok(a.concat(&b)?)
}
