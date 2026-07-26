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

//! Advanced pub/sub round-trip over a single in-process session — the
//! flat-API counterpart of zenoh-ext's `z_advanced_{pub,sub}` examples.
//! Gated on `unstable`, which is what exposes the advanced surface.

#![cfg(feature = "unstable")]

use std::{
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use zenoh_flat::{
    CacheConfig, HistoryConfig, MissDetectionConfig, RecoveryConfig, RecoveryMode, Sample,
    SampleKind, advanced_publisher_declare_matching_listener, advanced_publisher_matching_status,
    advanced_publisher_put, advanced_subscriber_declare_detect_publishers_subscriber,
    advanced_subscriber_declare_sample_miss_listener, config_new_default, keyexpr_as_str,
    keyexpr_new_try_from, open, sample_get_key_expr, sample_get_kind, sample_get_payload,
    session_declare_advanced_publisher, session_declare_advanced_subscriber, zbytes_new_from_slice,
    zbytes_to_bytes,
};

struct Got {
    key: String,
    payload: Vec<u8>,
    kind: SampleKind,
}

/// An advanced publisher's put reaches an advanced subscriber, the publisher
/// reports a matching subscriber, and its matching listener fires.
#[test]
fn advanced_put_is_received_and_matching_detected() {
    let session = open(config_new_default()).expect("open session");

    // Advanced subscriber first, so it is already matching when the publisher's
    // matching listener is declared.
    let slot: Arc<(Mutex<Option<Got>>, Condvar)> = Arc::new((Mutex::new(None), Condvar::new()));
    let slot_cb = slot.clone();
    let _subscriber = session_declare_advanced_subscriber(
        &session,
        keyexpr_new_try_from("test/adv/**".to_string()).expect("sub key expr"),
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
        Some(HistoryConfig {
            detect_late_publishers: true,
            ..Default::default()
        }),
        Some(RecoveryConfig {
            mode: Some(RecoveryMode::Heartbeat),
            // Exercise the retention period too: base must accept it alongside
            // a recovery mode.
            retention_period: Some(Duration::from_secs(60)),
        }),
        None,       // query_timeout
        Some(true), // subscriber_detection
    )
    .expect("declare advanced subscriber");

    // Advanced publisher with cache + heartbeat miss detection + detection.
    let publisher = session_declare_advanced_publisher(
        &session,
        keyexpr_new_try_from("test/adv/value".to_string()).expect("pub key expr"),
        None, // encoding
        None, // congestion_control
        None, // priority
        None, // express
        None, // reliability
        Some(MissDetectionConfig {
            heartbeat: Some(Duration::from_millis(200)),
            sporadic: false,
        }),
        Some(true), // publisher_detection
        Some(CacheConfig {
            max_samples: 10,
            ..Default::default()
        }),
    )
    .expect("declare advanced publisher");

    let matched = Arc::new(AtomicBool::new(false));
    let matched_cb = matched.clone();
    let _matching_listener = advanced_publisher_declare_matching_listener(
        &publisher,
        move |m: bool| {
            if m {
                matched_cb.store(true, Ordering::SeqCst);
            }
        },
        || {},
    )
    .expect("declare matching listener");

    // Let the declarations propagate.
    std::thread::sleep(Duration::from_millis(500));

    // Synchronous matching-status query: a matching subscriber exists.
    assert!(
        advanced_publisher_matching_status(&publisher).expect("matching status"),
        "advanced publisher should report a matching subscriber",
    );

    // Publish and verify receipt.
    advanced_publisher_put(
        &publisher,
        zbytes_new_from_slice(b"hello advanced"),
        None,
        None,
    )
    .expect("advanced put");

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

    assert_eq!(got.key, "test/adv/value");
    assert_eq!(got.payload, b"hello advanced");
    assert_eq!(got.kind, SampleKind::Put);

    // The matching listener fired for the already-present subscriber.
    assert!(
        matched.load(Ordering::SeqCst),
        "matching listener should have reported a matching subscriber",
    );
}

/// The advanced subscriber's sample-miss and detect-publishers listeners
/// declare without error (smoke test of those API paths).
#[test]
fn advanced_subscriber_listeners_declare() {
    let session = open(config_new_default()).expect("open session");

    let subscriber = session_declare_advanced_subscriber(
        &session,
        keyexpr_new_try_from("test/adv2/**".to_string()).expect("sub key expr"),
        |_sample: Sample| {},
        || {},
        None,
        None,
        None,
        Some(true),
    )
    .expect("declare advanced subscriber");

    let _miss_listener =
        advanced_subscriber_declare_sample_miss_listener(&subscriber, |_miss| {}, || {})
            .expect("declare sample-miss listener");

    let _detect = advanced_subscriber_declare_detect_publishers_subscriber(
        &subscriber,
        |_sample: Sample| {},
        || {},
        Some(false),
    )
    .expect("declare detect-publishers subscriber");
}
