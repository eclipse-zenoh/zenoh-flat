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
    zbytes_to_bytes,
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
