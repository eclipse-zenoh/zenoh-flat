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

//! Payload (`ZBytes`) round-trips — the flat-API counterpart of the
//! buffer round-trip checks in eclipse-zenoh/zenoh-c's
//! `tests/z_api_payload_test.c` (`test_slice`). `zenoh-flat` exposes `ZBytes` as
//! a contiguous-bytes value (no reader/writer/serializer surface), so these
//! tests cover the slice/vec constructors and the borrowed/owned accessors.

use zenoh_flat::{
    zbytes_is_empty, zbytes_len, zbytes_new_clone, zbytes_new_from_slice, zbytes_new_from_vec,
    zbytes_to_bytes, zbytes_try_to_string,
};

/// `test_slice`: bytes built from a borrowed slice come back byte-identical
/// through the borrowed-or-owned (`as_bytes`) accessor.
#[test]
fn slice_round_trip() {
    let data: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let payload = zbytes_new_from_slice(data);

    assert_eq!(zbytes_to_bytes(&payload).as_ref(), data);
}

/// The owned-vec constructor takes ownership without copying and round-trips.
#[test]
fn vec_round_trip() {
    let data = vec![10u8, 20, 30, 40, 50];
    let payload = zbytes_new_from_vec(data.clone());
    assert_eq!(zbytes_to_bytes(&payload).as_ref(), data);
}

/// Cloning a payload shares the underlying buffer; the bytes compare equal.
#[test]
fn clone_shares_payload() {
    let data: &[u8] = b"hello payload";
    let payload = zbytes_new_from_slice(data);
    let cloned = zbytes_new_clone(&payload);

    assert_eq!(zbytes_to_bytes(&cloned).as_ref(), data);
    assert_eq!(zbytes_to_bytes(&payload), zbytes_to_bytes(&cloned));
}

#[test]
fn empty_payload_round_trips() {
    let payload = zbytes_new_from_slice(&[]);
    assert!(zbytes_to_bytes(&payload).is_empty());
}

/// Text payloads decode through base, and non-UTF-8 input is reported rather
/// than lossily replaced.
///
/// The invalid case is the point: a binding that re-implemented the check and
/// skipped it would turn corrupt input into a string full of replacement
/// characters instead of an error.
#[test]
fn try_to_string_decodes_text_and_rejects_invalid_utf8() {
    for text in ["", "hello", "héllo — ünicode", "日本語"] {
        let z = zbytes_new_from_slice(text.as_bytes());
        assert_eq!(zbytes_try_to_string(&z).expect("valid utf-8"), text);
    }

    // An invalid UTF-8 byte sequence (0xff/0xfe are never valid UTF-8, and 0x80 is a continuation byte).
    let bad = zbytes_new_from_slice(&[0xff, 0xfe, 0x80]);
    assert!(
        zbytes_try_to_string(&bad).is_err(),
        "invalid utf-8 must be reported, not replaced"
    );
    // The bytes themselves are still readable.
    assert_eq!(zbytes_to_bytes(&bad).as_ref(), &[0xff, 0xfe, 0x80]);
}

/// The size of a payload is readable without materializing it, and agrees with
/// the materialized form. `zbytes_len` exists precisely so a caller does not
/// have to call `zbytes_to_bytes` just to learn how big the payload is.
#[test]
fn len_agrees_with_materialized_payload() {
    for bytes in [b"".as_slice(), b"x".as_slice(), b"hello world".as_slice()] {
        let z = zbytes_new_from_slice(bytes);
        assert_eq!(zbytes_len(&z), bytes.len());
        assert_eq!(zbytes_len(&z), zbytes_to_bytes(&z).len());
        assert_eq!(zbytes_is_empty(&z), bytes.is_empty());
    }
}
