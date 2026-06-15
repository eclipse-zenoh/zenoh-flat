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

//! Configuration insert/get and parsing — the flat-API counterpart of
//! eclipse-zenoh/zenoh-c's `tests/z_api_config_test.c` (`insert_get`), extended
//! with the JSON5/JSON/YAML constructors `zenoh-flat` exposes.

use zenoh_flat::{
    config_get_json, config_insert_json5, config_new_clone, config_new_default,
    config_new_from_json, config_new_from_json5, config_new_from_yaml,
};

/// Direct port of zenoh-c's `insert_get`: insert a scalar and a list value via
/// JSON5 and read them back as canonical JSON (whitespace-normalized).
#[test]
fn insert_and_get_json5() {
    let mut config = config_new_default();
    config_insert_json5(&mut config, "mode", "\"client\"").expect("insert mode");
    config_insert_json5(
        &mut config,
        "connect/endpoints",
        "[\"tcp/127.0.0.1\", \"tcp/192.168.0.1\", \"tcp/10.0.0.1\"]",
    )
    .expect("insert endpoints");

    assert_eq!(
        config_get_json(&config, "mode").expect("get mode"),
        "\"client\""
    );
    // `get_json` returns canonical JSON, so the inserted whitespace is dropped.
    assert_eq!(
        config_get_json(&config, "connect/endpoints").expect("get endpoints"),
        "[\"tcp/127.0.0.1\",\"tcp/192.168.0.1\",\"tcp/10.0.0.1\"]"
    );
}

#[test]
fn parse_from_json5() {
    let config = config_new_from_json5("{ mode: \"peer\" }").expect("parse json5");
    assert_eq!(
        config_get_json(&config, "mode").expect("get mode"),
        "\"peer\""
    );
}

#[test]
fn parse_from_json() {
    let config = config_new_from_json("{ \"mode\": \"peer\" }").expect("parse json");
    assert_eq!(
        config_get_json(&config, "mode").expect("get mode"),
        "\"peer\""
    );
}

#[test]
fn parse_from_yaml() {
    let config = config_new_from_yaml("mode: peer\n").expect("parse yaml");
    assert_eq!(
        config_get_json(&config, "mode").expect("get mode"),
        "\"peer\""
    );
}

#[test]
fn invalid_input_is_rejected() {
    assert!(config_new_from_json("{ this is not valid json }").is_err());
    assert!(config_insert_json5(&mut config_new_default(), "mode", "not json5").is_err());
}

/// A clone is an independent snapshot: mutating the original after cloning must
/// not be visible through the clone.
#[test]
fn clone_is_independent() {
    let mut config = config_new_default();
    config_insert_json5(&mut config, "mode", "\"client\"").expect("insert");

    let cloned = config_new_clone(&config);
    config_insert_json5(&mut config, "mode", "\"peer\"").expect("reinsert");

    assert_eq!(
        config_get_json(&cloned, "mode").expect("get clone"),
        "\"client\""
    );
    assert_eq!(
        config_get_json(&config, "mode").expect("get original"),
        "\"peer\""
    );
}
