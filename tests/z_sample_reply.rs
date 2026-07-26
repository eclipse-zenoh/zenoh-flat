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
    KeyExpr, Reply, Sample, SampleKind, Selector, Session, Timestamp, ZBytes, config_new_default,
    keyexpr_new_try_from, open, query_reply_sample, reply_get_sample, reply_is_ok,
    sample_get_attachment, sample_get_kind, sample_get_payload, sample_get_timestamp,
    sample_new_delete, sample_new_put, session_declare_queryable, session_get,
    session_new_timestamp, zbytes_new_from_vec, zbytes_to_bytes,
};

/// What the `get` callback extracted from the received reply.
struct Received {
    ok: bool,
    kind: Option<SampleKind>,
    payload: Vec<u8>,
    timestamp: Option<Timestamp>,
    attachment: Option<Vec<u8>>,
}

// The `reliability` parameter of the sample constructors only exists with the
// `unstable` feature; wrap the calls so the test body stays feature-agnostic.

fn make_put(key_expr: KeyExpr, payload: ZBytes, ts: Timestamp, attachment: ZBytes) -> Sample {
    #[cfg(not(feature = "unstable"))]
    {
        sample_new_put(
            key_expr,
            payload,
            None,
            Some(ts.clone()),
            Some(attachment),
            None,
            None,
            Some(true),
        )
        .expect("test timestamp is valid")
    }
    #[cfg(feature = "unstable")]
    {
        sample_new_put(
            key_expr,
            payload,
            None,
            Some(ts.clone()),
            Some(attachment),
            None,
            None,
            Some(true),
            None,
        )
        .expect("test timestamp is valid")
    }
}

fn make_delete(key_expr: KeyExpr, ts: Timestamp, attachment: ZBytes) -> Sample {
    #[cfg(not(feature = "unstable"))]
    {
        sample_new_delete(
            key_expr,
            Some(ts.clone()),
            Some(attachment),
            None,
            None,
            Some(true),
        )
        .expect("test timestamp is valid")
    }
    #[cfg(feature = "unstable")]
    {
        sample_new_delete(
            key_expr,
            Some(ts.clone()),
            Some(attachment),
            None,
            None,
            Some(true),
            None,
        )
        .expect("test timestamp is valid")
    }
}

/// Declare a queryable on `key` that answers each query via `query_reply_sample`
/// with a Put or Delete sample, then `get` the same key and return what the
/// reply carried.
fn round_trip(
    session: &Session,
    key: &str,
    is_delete: bool,
    ts: Timestamp,
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
                make_delete(ke, ts.clone(), att)
            } else {
                make_put(
                    ke,
                    zbytes_new_from_vec(payload_owned.clone()),
                    ts.clone(),
                    att,
                )
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
                timestamp: None,
                attachment: None,
            };
            if let Some(sample) = reply_get_sample(&reply) {
                rec.kind = Some(sample_get_kind(sample));
                rec.payload = zbytes_to_bytes(sample_get_payload(sample)).into_owned();
                rec.timestamp = sample_get_timestamp(sample);
                rec.attachment =
                    sample_get_attachment(sample).map(|z| zbytes_to_bytes(z).into_owned());
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

    let ts = Timestamp {
        ntp64: 0x0123_4567_89ab_cdef,
        id: vec![0xde, 0xad, 0xbe, 0xef],
    };
    let payload = b"hello put sample";
    let attachment = b"put-attachment";

    let rec = round_trip(
        &session,
        "test/z_sample_reply/put",
        false,
        ts.clone(),
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
        rec.timestamp,
        Some(ts),
        "the whole timestamp — time and node id — must be forwarded by \
         query_reply_sample, not rebuilt with a fabricated id"
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

    let ts = Timestamp {
        ntp64: 0x0011_2233_4455_6677,
        id: vec![0xca, 0xfe, 0xba, 0xbe],
    };
    let attachment = b"delete-attachment";

    let rec = round_trip(
        &session,
        "test/z_sample_reply/delete",
        true,
        ts.clone(),
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
        rec.timestamp,
        Some(ts),
        "the whole timestamp — time and node id — must be forwarded for a \
         delete reply"
    );
    assert_eq!(
        rec.attachment.as_deref(),
        Some(&attachment[..]),
        "attachment must be forwarded for a delete reply"
    );
}

/// A timestamp taken from the session's own clock survives sample construction
/// **whole** — its node id included.
///
/// This is the regression this file's constructors used to have: they took a
/// bare NTP64 integer and rebuilt the timestamp with `TimestampId::rand()`, so
/// the id was fabricated. Time still looked right, which is why only an
/// id-aware check catches it — a receiver keying de-duplication or ordering on
/// the `(time, id)` pair would have been fed a meaningless id, and
/// `session_new_timestamp`'s own causal-consistency promise was void.
#[test]
fn session_timestamp_survives_sample_construction() {
    let session = open(config_new_default()).expect("open session");
    let ts = session_new_timestamp(&session);

    // The session stamps a real node id; without this the comparison below
    // could pass on two empty ids.
    assert!(!ts.id.is_empty(), "session timestamp must carry a node id");

    let ke = keyexpr_new_try_from("test/z_sample_reply/hlc".to_string()).expect("key expr");
    let put = make_put(
        ke,
        zbytes_new_from_vec(b"payload".to_vec()),
        ts.clone(),
        zbytes_new_from_vec(b"att".to_vec()),
    );
    assert_eq!(
        sample_get_timestamp(&put),
        Some(ts.clone()),
        "put sample must carry the session timestamp unchanged"
    );

    let ke = keyexpr_new_try_from("test/z_sample_reply/hlc".to_string()).expect("key expr");
    let del = make_delete(ke, ts.clone(), zbytes_new_from_vec(b"att".to_vec()));
    assert_eq!(
        sample_get_timestamp(&del),
        Some(ts),
        "delete sample must carry the session timestamp unchanged"
    );
}
