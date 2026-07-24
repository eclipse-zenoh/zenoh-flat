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

//! Round-trip regression test for `query_reply_sample`.
//!
//! `query_reply_sample` takes a fully-formed [`Sample`] and answers a query with
//! it. This is the only consumer of an owned `Sample`, and it is dead code from
//! the managed-language SDKs' point of view (they reply via the flat-param
//! `query_reply_success` / `_delete`), so a regression here was previously
//! invisible: the function silently turned a Delete sample into a Put reply and
//! dropped the timestamp/attachment. This test exercises it directly through a
//! single local session (a queryable answers the session's own `get`).

use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

use zenoh_flat::{
    KeyExpr, Reply, Sample, SampleKind, Selector, Session, ZBytes, config_new_default,
    keyexpr_new_try_from, open, query_reply_sample, reply_get_sample, reply_is_ok,
    sample_get_attachment, sample_get_kind, sample_get_payload, sample_get_timestamp,
    sample_new_delete, sample_new_put, session_declare_queryable, session_get, zbytes_as_bytes,
    zbytes_new_from_vec,
};

/// What the `get` callback extracted from the received reply.
struct Received {
    ok: bool,
    kind: Option<SampleKind>,
    payload: Vec<u8>,
    ntp64: Option<u64>,
    attachment: Option<Vec<u8>>,
}

// The `reliability` parameter of the sample constructors only exists with the
// `unstable` feature; wrap the calls so the test body stays feature-agnostic.

fn make_put(key_expr: KeyExpr, payload: ZBytes, ntp64: u64, attachment: ZBytes) -> Sample {
    #[cfg(not(feature = "unstable"))]
    {
        sample_new_put(
            key_expr,
            payload,
            None,
            Some(ntp64),
            Some(attachment),
            None,
            None,
            Some(true),
        )
    }
    #[cfg(feature = "unstable")]
    {
        sample_new_put(
            key_expr,
            payload,
            None,
            Some(ntp64),
            Some(attachment),
            None,
            None,
            Some(true),
            None,
        )
    }
}

fn make_delete(key_expr: KeyExpr, ntp64: u64, attachment: ZBytes) -> Sample {
    #[cfg(not(feature = "unstable"))]
    {
        sample_new_delete(
            key_expr,
            Some(ntp64),
            Some(attachment),
            None,
            None,
            Some(true),
        )
    }
    #[cfg(feature = "unstable")]
    {
        sample_new_delete(
            key_expr,
            Some(ntp64),
            Some(attachment),
            None,
            None,
            Some(true),
            None,
        )
    }
}

/// Declare a queryable on `key` that answers each query via `query_reply_sample`
/// with a Put or Delete sample, then `get` the same key and return what the
/// reply carried.
fn round_trip(
    session: &Session,
    key: &str,
    is_delete: bool,
    ntp64: u64,
    payload: &[u8],
    attachment: &[u8],
) -> Received {
    let slot: Arc<(Mutex<Option<Received>>, Condvar)> =
        Arc::new((Mutex::new(None), Condvar::new()));

    let key_owned = key.to_string();
    let payload_owned = payload.to_vec();
    let attachment_owned = attachment.to_vec();

    let _queryable = session_declare_queryable(
        session,
        keyexpr_new_try_from(key.to_string()).expect("queryable key expr"),
        None,
        move |query| {
            let ke = keyexpr_new_try_from(key_owned.clone()).expect("reply key expr");
            let att = zbytes_new_from_vec(attachment_owned.clone());
            let sample = if is_delete {
                make_delete(ke, ntp64, att)
            } else {
                make_put(ke, zbytes_new_from_vec(payload_owned.clone()), ntp64, att)
            };
            let _ = query_reply_sample(&query, sample);
        },
        || {},
    )
    .expect("declare queryable");

    // Give the queryable a moment to be registered before querying.
    std::thread::sleep(Duration::from_millis(500));

    let ke_get = keyexpr_new_try_from(key.to_string()).expect("get key expr");
    let slot_cb = slot.clone();
    session_get(
        session,
        Selector {
            key_expr: ke_get,
            parameters: String::new(),
        },
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        move |reply: Reply| {
            let mut rec = Received {
                ok: reply_is_ok(&reply),
                kind: None,
                payload: Vec::new(),
                ntp64: None,
                attachment: None,
            };
            if let Some(sample) = reply_get_sample(&reply) {
                rec.kind = Some(sample_get_kind(sample));
                rec.payload = zbytes_as_bytes(sample_get_payload(sample)).into_owned();
                rec.ntp64 = sample_get_timestamp(sample).map(|t| t.ntp64);
                rec.attachment =
                    sample_get_attachment(sample).map(|z| zbytes_as_bytes(z).into_owned());
            }
            let (lock, cv) = &*slot_cb;
            *lock.lock().unwrap() = Some(rec);
            cv.notify_all();
        },
        || {},
    )
    .expect("session get");

    let (lock, cv) = &*slot;
    let mut guard = lock.lock().unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while guard.is_none() {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            break;
        }
        let (g, _timed_out) = cv.wait_timeout(guard, remaining).unwrap();
        guard = g;
    }
    guard.take().expect("no reply received within timeout")
}

#[test]
fn put_sample_round_trip_preserves_metadata() {
    let session = open(config_new_default()).expect("open session");

    let ntp64: u64 = 0x0123_4567_89ab_cdef;
    let payload = b"hello put sample";
    let attachment = b"put-attachment";

    let rec = round_trip(
        &session,
        "test/z_sample_reply/put",
        false,
        ntp64,
        payload,
        attachment,
    );

    assert!(rec.ok, "put reply should be a success reply");
    assert_eq!(
        rec.kind,
        Some(SampleKind::Put),
        "received sample kind must be Put"
    );
    assert_eq!(rec.payload, payload, "payload must round-trip");
    assert_eq!(
        rec.ntp64,
        Some(ntp64),
        "timestamp NTP64 must be forwarded by query_reply_sample"
    );
    assert_eq!(
        rec.attachment.as_deref(),
        Some(&attachment[..]),
        "attachment must be forwarded by query_reply_sample"
    );
}

#[test]
fn delete_sample_round_trip_preserves_kind() {
    let session = open(config_new_default()).expect("open session");

    let ntp64: u64 = 0x0011_2233_4455_6677;
    let attachment = b"delete-attachment";

    let rec = round_trip(
        &session,
        "test/z_sample_reply/delete",
        true,
        ntp64,
        b"",
        attachment,
    );

    assert!(rec.ok, "delete reply should still be a success reply");
    assert_eq!(
        rec.kind,
        Some(SampleKind::Delete),
        "received sample kind must be Delete (regression: was turned into Put)"
    );
    assert!(
        rec.payload.is_empty(),
        "delete sample carries no payload, got {:?}",
        rec.payload
    );
    assert_eq!(
        rec.ntp64,
        Some(ntp64),
        "timestamp NTP64 must be forwarded for a delete reply"
    );
    assert_eq!(
        rec.attachment.as_deref(),
        Some(&attachment[..]),
        "attachment must be forwarded for a delete reply"
    );
}
