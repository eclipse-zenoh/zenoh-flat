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

//! Pub/sub round-trip over a single in-process session — the flat-API
//! counterpart of eclipse-zenoh/zenoh's basic `session`/pub-sub tests.

use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

use zenoh_flat::{
    Sample, SampleKind, config_new_default, keyexpr_as_str, keyexpr_new_try_from, open,
    sample_get_key_expr, sample_get_kind, sample_get_payload, session_declare_subscriber,
    session_put, zbytes_new_from_slice, zbytes_to_bytes,
};

struct Got {
    key: String,
    payload: Vec<u8>,
    kind: SampleKind,
}

#[test]
fn put_is_received_by_subscriber() {
    let session = open(config_new_default()).expect("open session");

    let slot: Arc<(Mutex<Option<Got>>, Condvar)> = Arc::new((Mutex::new(None), Condvar::new()));
    let slot_cb = slot.clone();

    let _subscriber = session_declare_subscriber(
        &session,
        keyexpr_new_try_from("test/pubsub/**".to_string()).expect("sub key expr"),
        move |sample: Sample| {
            let got = Got {
                key: keyexpr_as_str(sample_get_key_expr(&sample)).to_string(),
                payload: zbytes_to_bytes(sample_get_payload(&sample)).into_owned(),
                kind: sample_get_kind(&sample),
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

    let ke = keyexpr_new_try_from("test/pubsub/value".to_string()).expect("put key expr");
    session_put(
        &session,
        &ke,
        zbytes_new_from_slice(b"hello pubsub"),
        None,
        None,
        None,
        None,
        None,
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
    let got = guard.take().expect("no sample received within timeout");

    assert_eq!(got.key, "test/pubsub/value");
    assert_eq!(got.payload, b"hello pubsub");
    assert_eq!(got.kind, SampleKind::Put);
}
