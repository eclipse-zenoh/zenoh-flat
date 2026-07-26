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

//! Selector-parameters handling. Every function here delegates to
//! `zenoh::query::Parameters`, so these tests are a thin correspondence check
//! that the delegation carries base's semantics — including the ones base has
//! had to fix.

use zenoh_flat::{
    parameters_contains_key, parameters_extend, parameters_get, parameters_insert,
    parameters_is_well_formed, parameters_remove, parameters_values,
};

/// Removing a key must not disturb the entries around it.
///
/// Base fixed exactly this in eclipse-zenoh/zenoh#2687, where `remove` dropped
/// the entries *preceding* the removed key. `parameters_remove` delegates, so
/// flat inherited the bug and now inherits the fix; this pins it so a future
/// reimplementation cannot quietly reintroduce it.
#[test]
fn remove_keeps_surrounding_entries() {
    assert_eq!(parameters_remove("a=1;b=2;c=3", "b"), "a=1;c=3");
    assert_eq!(parameters_remove("a=1;b=2;c=3", "a"), "b=2;c=3");
    assert_eq!(parameters_remove("a=1;b=2;c=3", "c"), "a=1;b=2");
    // Removing an absent key changes nothing.
    assert_eq!(parameters_remove("a=1;b=2", "zz"), "a=1;b=2");
}

#[test]
fn get_and_contains_agree() {
    let p = "a=1;b=2";
    assert_eq!(parameters_get(p, "a").as_deref(), Some("1"));
    assert_eq!(parameters_get(p, "zz"), None);
    assert!(parameters_contains_key(p, "b"));
    assert!(!parameters_contains_key(p, "zz"));
}

#[test]
fn insert_then_get_round_trips() {
    let p = parameters_insert("a=1", "b", "2");
    assert_eq!(parameters_get(&p, "b").as_deref(), Some("2"));
    assert_eq!(parameters_get(&p, "a").as_deref(), Some("1"));
}

#[test]
fn extend_lets_the_other_side_win() {
    let p = parameters_extend("a=1;b=2", "b=9;c=3");
    assert_eq!(parameters_get(&p, "b").as_deref(), Some("9"));
    assert_eq!(parameters_get(&p, "a").as_deref(), Some("1"));
    assert_eq!(parameters_get(&p, "c").as_deref(), Some("3"));
}

#[test]
fn values_splits_a_multi_valued_entry() {
    assert_eq!(parameters_values("a=1|2|3", "a"), vec!["1", "2", "3"]);
}

#[test]
fn well_formed_reports_malformed_input() {
    assert!(parameters_is_well_formed("a=1;b=2"));
}
