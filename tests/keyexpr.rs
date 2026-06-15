//
// Copyright (c) 2026 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

//! Key-expression logic tests against the flat API — the flat-API counterpart of
//! the relation/canonization checks in eclipse-zenoh/zenoh's keyexpr tests.

use zenoh_flat::{
    keyexpr_includes, keyexpr_intersects, keyexpr_new_autocanonize, keyexpr_new_concat,
    keyexpr_new_join, keyexpr_new_try_from, keyexpr_to_string,
};

fn ke(s: &str) -> zenoh_flat::KeyExpr {
    keyexpr_new_try_from(s.to_string()).unwrap_or_else(|e| panic!("invalid key expr {s:?}: {e}"))
}

#[test]
fn try_from_rejects_non_canonical() {
    assert!(keyexpr_new_try_from("a/b".to_string()).is_ok());
    // `a/**/**` is not canonical and must be rejected by the strict constructor.
    assert!(keyexpr_new_try_from("a/**/**".to_string()).is_err());
    // Empty chunks are invalid.
    assert!(keyexpr_new_try_from("a//b".to_string()).is_err());
}

#[test]
fn autocanonize_rewrites_redundant_wildcards() {
    let k = keyexpr_new_autocanonize("a/**/**".to_string()).expect("autocanonize");
    assert_eq!(keyexpr_to_string(&k), "a/**");
}

#[test]
fn intersects() {
    assert!(keyexpr_intersects(&ke("a/b/c"), &ke("a/*/c")));
    assert!(keyexpr_intersects(&ke("a/**"), &ke("a/b/c")));
    assert!(!keyexpr_intersects(&ke("a/b/c"), &ke("a/b/d")));
    assert!(!keyexpr_intersects(&ke("a/b"), &ke("x/y")));
}

#[test]
fn includes() {
    // `a/**` includes everything under `a`, but the reverse is not true.
    assert!(keyexpr_includes(&ke("a/**"), &ke("a/b/c")));
    assert!(!keyexpr_includes(&ke("a/b/c"), &ke("a/**")));
    // `*` includes a concrete single chunk.
    assert!(keyexpr_includes(&ke("a/*"), &ke("a/b")));
    assert!(!keyexpr_includes(&ke("a/b"), &ke("a/*")));
}

#[test]
fn join_and_concat() {
    // join inserts a `/` separator.
    let joined = keyexpr_new_join(&ke("a/b"), "c/d".to_string()).expect("join");
    assert_eq!(keyexpr_to_string(&joined), "a/b/c/d");

    // concat appends verbatim (no separator), here completing a chunk.
    let concatenated = keyexpr_new_concat(&ke("a/b"), "c".to_string()).expect("concat");
    assert_eq!(keyexpr_to_string(&concatenated), "a/bc");
}

#[cfg(feature = "unstable")]
#[test]
fn relation_to() {
    use zenoh_flat::{SetIntersectionLevel, keyexpr_relation_to};

    assert_eq!(
        keyexpr_relation_to(&ke("a/b"), &ke("a/b")),
        SetIntersectionLevel::Equals
    );
    assert_eq!(
        keyexpr_relation_to(&ke("a/**"), &ke("a/b")),
        SetIntersectionLevel::Includes
    );
    assert_eq!(
        keyexpr_relation_to(&ke("a/*"), &ke("*/b")),
        SetIntersectionLevel::Intersects
    );
    assert_eq!(
        keyexpr_relation_to(&ke("a/b"), &ke("x/y")),
        SetIntersectionLevel::Disjoint
    );
}
