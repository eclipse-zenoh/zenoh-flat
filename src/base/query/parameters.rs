//! Selector-parameters processing.
//!
//! The translatable form of [`zenoh::query::Parameters`]: the parameters
//! travel as a plain `a=b;c=d|e;f=g` string, and every operation takes and
//! returns strings, so any language binding can process parameters with the
//! exact zenoh semantics:
//!
//! - a string is split into entries on `;` (empty chunks are skipped), and an
//!   entry into key and value on its FIRST `=` (no `=` means an empty value);
//!   values are taken verbatim (no percent-decoding),
//! - any input is accepted; as in [`zenoh::query::Parameters`] construction,
//!   trailing `;`, `=`, `|` characters are ignored,
//! - a duplicated key is allowed: lookups return the first occurrence,
//! - mutating operations rebuild the string in normalized form (empty keys
//!   dropped, `=` omitted for empty values, insertion order preserved).
//!
//! [`parameters_insert`] and [`parameters_remove`] return the resulting
//! string; when the replaced/removed value is needed, read it with
//! [`parameters_get`] before mutating (the tuple return of the Rust API does
//! not translate).

use prebindgen_proc_macro::prebindgen;

/// Get the value for a key according to the parameters format: entries split
/// on `;`, key/value on the first `=`, the first matching key wins.
#[prebindgen]
pub fn parameters_get(s: &str, k: &str) -> Option<String> {
    zenoh::query::Parameters::from(s).get(k).map(str::to_string)
}

/// Get the `|`-separated list of values for a key; empty when the key is
/// absent.
#[prebindgen]
pub fn parameters_values(s: &str, k: &str) -> Vec<String> {
    zenoh::query::Parameters::from(s)
        .values(k)
        .map(str::to_string)
        .collect()
}

/// Return `true` if the parameters string contains the key.
#[prebindgen]
pub fn parameters_contains_key(s: &str, k: &str) -> bool {
    zenoh::query::Parameters::from(s).contains_key(k)
}

/// Insert a key-value pair into a parameters string: every existing entry
/// for the key is removed and the new pair appended at the end. Returns the
/// resulting string.
#[prebindgen]
pub fn parameters_insert(s: &str, k: &str, v: &str) -> String {
    let mut p = zenoh::query::Parameters::from(s);
    p.insert(k, v);
    p.as_str().to_string()
}

/// Remove every entry for a key from a parameters string. Returns the
/// resulting string.
#[prebindgen]
pub fn parameters_remove(s: &str, k: &str) -> String {
    let mut p = zenoh::query::Parameters::from(s);
    p.remove(k);
    p.as_str().to_string()
}

/// Extend a parameters string with the entries of another: each of `other`'s
/// pairs is inserted, so on conflicting keys `other`'s values win. Returns
/// the resulting string.
#[prebindgen]
pub fn parameters_extend(s: &str, other: &str) -> String {
    let mut p = zenoh::query::Parameters::from(s);
    p.extend(&zenoh::query::Parameters::from(other));
    p.as_str().to_string()
}

/// Return `true` if the parameters string contains at least one entry and
/// none of its keys are empty.
///
/// This predicate is **not** part of the public `zenoh` API: base zenoh exposes
/// well-formedness only as an internal helper. It is provided here as a
/// correspondence-test oracle — bindings that reimplement parameters processing
/// natively (for example on the JVM, where crossing the native/managed boundary
/// per operation is expensive) can check their implementation against it.
#[prebindgen]
pub fn parameters_is_well_formed(s: &str) -> bool {
    let p = zenoh::query::Parameters::from(s);
    p.iter().next().is_some() && p.iter().all(|(k, _)| !k.is_empty())
}
