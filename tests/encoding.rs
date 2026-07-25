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

//! Encoding construction, schema handling and predefined constants — the
//! flat-API counterpart of eclipse-zenoh/zenoh-c's `tests/z_api_encoding_test.c`.
//! `zenoh-flat` carries encodings as `(id, schema)` and renders them through the
//! upstream `Display`, so these tests mirror the C `*_to_string` round-trips.

use zenoh_flat::{
    encoding_const_zenoh_bytes, encoding_const_zenoh_serialized, encoding_const_zenoh_string,
    encoding_get_id, encoding_get_schema, encoding_new_clone, encoding_new_from_id,
    encoding_new_from_string, encoding_new_with_schema, encoding_to_string,
};

/// `test_encoding_without_id`: a free-form string with no known id round-trips
/// verbatim.
#[test]
fn from_string_without_id() {
    let e = encoding_new_from_string("my_encoding".to_string());
    assert_eq!(encoding_to_string(&e), "my_encoding");
}

/// `test_encoding_with_id`: strings that name a known id (or carry a `;schema`
/// suffix) round-trip to their canonical form.
#[test]
fn from_string_with_id_and_schema() {
    let e1 = encoding_new_from_string("zenoh/string;utf8".to_string());
    assert_eq!(encoding_to_string(&e1), "zenoh/string;utf8");

    let e2 = encoding_new_from_string("custom_id;custom_schema".to_string());
    assert_eq!(encoding_to_string(&e2), "custom_id;custom_schema");
}

/// `test_constants`: predefined constants render to their canonical names.
#[test]
fn constants_render_canonically() {
    assert_eq!(
        encoding_to_string(encoding_const_zenoh_bytes()),
        "zenoh/bytes"
    );
    // Wire-id stability for the sample presets (the composed `(id, string)`
    // form is what bindings' expression constants read).
    assert_eq!(encoding_get_id(encoding_const_zenoh_bytes()), 0);
    assert_eq!(encoding_get_id(encoding_const_zenoh_string()), 1);
    assert_eq!(
        encoding_to_string(encoding_const_zenoh_string()),
        "zenoh/string"
    );
    assert_eq!(
        encoding_to_string(encoding_const_zenoh_serialized()),
        "zenoh/serialized"
    );
}

/// `test_with_schema`: attaching a schema to a base encoding yields
/// `<id>;<schema>` and is read back through `encoding_get_schema`.
#[test]
fn with_schema_appends_schema() {
    let e = encoding_new_with_schema(encoding_const_zenoh_bytes(), "my_schema".to_string());
    assert_eq!(encoding_to_string(&e), "zenoh/bytes;my_schema");
    assert_eq!(encoding_get_schema(&e).as_deref(), Some(&b"my_schema"[..]));

    let e2 = encoding_new_with_schema(encoding_const_zenoh_string(), "my_schema".to_string());
    assert_eq!(encoding_to_string(&e2), "zenoh/string;my_schema");
}

/// The `(id, schema)` pair round-trips through `encoding_new_from_id` —
/// the inverse of `encoding_get_id` / `encoding_get_schema`.
#[test]
fn id_round_trips() {
    let string_enc = encoding_const_zenoh_string();
    let id = encoding_get_id(string_enc);
    let rebuilt = encoding_new_from_id(id, None);
    assert_eq!(encoding_to_string(&rebuilt), encoding_to_string(string_enc));
    assert_eq!(encoding_get_schema(&rebuilt), None);
}

/// A schema carried alongside the id round-trips too, and renders as
/// `<id>;<schema>` when it is text.
#[test]
fn id_and_schema_round_trip() {
    let id = encoding_get_id(encoding_const_zenoh_string());
    let schema = b"utf8".to_vec();
    let e = encoding_new_from_id(id, Some(schema.clone()));
    assert_eq!(encoding_get_id(&e), id);
    assert_eq!(encoding_get_schema(&e), Some(schema));
    assert_eq!(encoding_to_string(&e), "zenoh/string;utf8");
}

/// A schema is raw bytes on the wire, so one that is not valid UTF-8 survives
/// the `(id, schema)` round-trip unchanged. No `encoding_to_string` assertion
/// here: the base rendering deliberately replaces such a schema with
/// `unknown(non-utf8)`, so the textual form is not a round-trip path.
#[test]
fn non_utf8_schema_round_trips() {
    let original = encoding_new_from_id(1, Some(vec![0xff, 0xfe, 0xfd]));
    let rebuilt = encoding_new_from_id(encoding_get_id(&original), encoding_get_schema(&original));
    assert_eq!(encoding_get_id(&rebuilt), encoding_get_id(&original));
    assert_eq!(
        encoding_get_schema(&rebuilt),
        encoding_get_schema(&original)
    );
    assert_eq!(encoding_get_schema(&rebuilt), Some(vec![0xff, 0xfe, 0xfd]));
}

#[test]
fn clone_preserves_value() {
    let e = encoding_new_from_string("zenoh/string;v1".to_string());
    let cloned = encoding_new_clone(&e);
    assert_eq!(encoding_to_string(&cloned), encoding_to_string(&e));
    assert_eq!(encoding_get_id(&cloned), encoding_get_id(&e));
}
