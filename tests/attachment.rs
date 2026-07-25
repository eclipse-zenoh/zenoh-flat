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

//! Pub/sub attachment round-trip — the flat-API counterpart of
//! eclipse-zenoh/zenoh-c's `tests/z_int_pub_sub_attachment_test.c`. The C test
//! serializes a key/value map into the attachment via `zenoh-ext`; `zenoh-flat`
//! carries the attachment as an opaque `ZBytes`, so this test verifies that the
//! raw attachment bytes (and their absence) survive a put → subscribe hop over a
//! single in-process session.

use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

use zenoh_flat::{
    Sample, config_new_default, keyexpr_new_try_from, open, sample_get_attachment,
    sample_get_payload, session_declare_subscriber, session_put, zbytes_new_from_slice,
    zbytes_to_bytes,
};

struct Got {
    payload: Vec<u8>,
    attachment: Option<Vec<u8>>,
}

/// Subscribe to `test/attachment/**`, publish one sample with the given optional
/// attachment, and return what the subscriber observed.
fn put_and_receive(attachment: Option<&[u8]>) -> Got {
    let session = open(config_new_default()).expect("open session");

    let slot: Arc<(Mutex<Option<Got>>, Condvar)> = Arc::new((Mutex::new(None), Condvar::new()));
    let slot_cb = slot.clone();

    let _subscriber = session_declare_subscriber(
        &session,
        keyexpr_new_try_from("test/attachment/**".to_string()).expect("sub key expr"),
        move |sample: Sample| {
            let got = Got {
                payload: zbytes_to_bytes(sample_get_payload(&sample)).into_owned(),
                attachment: sample_get_attachment(&sample).map(|z| zbytes_to_bytes(z).into_owned()),
            };
            let (lock, cv) = &*slot_cb;
            *lock.lock().unwrap() = Some(got);
            cv.notify_all();
        },
        || {},
    )
    .expect("declare subscriber");

    // Let the subscription propagate before publishing.
    std::thread::sleep(Duration::from_millis(500));

    let ke = keyexpr_new_try_from("test/attachment/value".to_string()).expect("put key expr");
    session_put(
        &session,
        &ke,
        zbytes_new_from_slice(b"payload-data"),
        None,
        None,
        None,
        None,
        attachment.map(zbytes_new_from_slice),
        #[cfg(feature = "unstable")]
        None,
    )
    .expect("session put");

    let (lock, cv) = &*slot;
    let mut guard = lock.lock().unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while guard.is_none() {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            break;
        }
        let (g, _) = cv.wait_timeout(guard, remaining).unwrap();
        guard = g;
    }
    guard.take().expect("no sample received within timeout")
}

#[test]
fn attachment_round_trips() {
    let got = put_and_receive(Some(b"attachment-data"));
    assert_eq!(got.payload, b"payload-data");
    assert_eq!(
        got.attachment.as_deref(),
        Some(&b"attachment-data"[..]),
        "attachment bytes must survive the put → subscribe hop"
    );
}

#[test]
fn no_attachment_is_none() {
    let got = put_and_receive(None);
    assert_eq!(got.payload, b"payload-data");
    assert!(
        got.attachment.is_none(),
        "a put without an attachment yields no attachment on the sample"
    );
}
