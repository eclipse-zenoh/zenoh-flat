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

//! Queryable / get round-trip — the flat-API counterpart of
//! eclipse-zenoh/zenoh-c's `tests/z_int_queryable_test.c`. The C test uses two
//! processes synced by a semaphore; here a single in-process session answers its
//! own `get` (the reply stream ends via `on_close`), and we additionally cover
//! the error-reply path that the C integration test does not.

use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

#[cfg(feature = "unstable")]
use zenoh_flat::reply_get_replier_id;
use zenoh_flat::{
    KeyExpr, Reply, ReplyResult, Selector, Session, config_new_default, encoding_to_struct,
    keyexpr_new_try_from, open, query_reply_error, query_reply_success, reply_error_get_encoding,
    reply_error_get_payload, reply_error_to_struct, reply_get_err, reply_get_sample, reply_is_ok,
    reply_to_struct, sample_get_encoding, sample_get_key_expr, sample_get_payload,
    session_declare_queryable, session_get, zbytes_new_from_slice, zbytes_to_bytes,
};

fn ke(s: &str) -> KeyExpr {
    keyexpr_new_try_from(s.to_string()).unwrap_or_else(|e| panic!("invalid key expr {s:?}: {e}"))
}

/// Check that every field of `ReplyStruct` / `ReplyErrorStruct` agrees with the
/// accessor for that same field — the guard for "one source of truth per field"
/// on the two value forms that can only be obtained from a live reply.
///
/// Returns a description per mismatch (empty = all fields agree). The checks run
/// on the reply callback's thread, where a panic would be reported far from the
/// test, so mismatches are collected and asserted on the test thread instead.
fn reply_struct_mismatches(r: &Reply) -> Vec<String> {
    let rs = reply_to_struct(r);
    let mut bad = Vec::new();

    // The live `ReplyResult` variant must agree with all three opaque-tier
    // accessors at once. Since `ReplyResult` is a sum, "both present" and
    // "neither present" are no longer representable, so the old presence checks
    // for those states are gone by construction rather than by assertion — the
    // match arms below are exhaustive over what the type can hold.
    match &rs.result {
        ReplyResult::Sample(st) => {
            if !reply_is_ok(r) {
                bad.push("result is Sample but reply_is_ok is false".to_string());
            }
            if reply_get_err(r).is_some() {
                bad.push("result is Sample but reply_get_err returned an error".to_string());
            }
            match reply_get_sample(r) {
                None => bad.push("result is Sample but reply_get_sample is None".to_string()),
                Some(sample) => {
                    if &st.key_expr != sample_get_key_expr(sample) {
                        bad.push("sample.key_expr disagrees with sample_get_key_expr".to_string());
                    }
                    if st.payload != *sample_get_payload(sample) {
                        bad.push("sample.payload disagrees with sample_get_payload".to_string());
                    }
                    if st.encoding != encoding_to_struct(sample_get_encoding(sample)) {
                        bad.push("sample.encoding disagrees with sample_get_encoding".to_string());
                    }
                }
            }
        }
        ReplyResult::Error(es) => {
            if reply_is_ok(r) {
                bad.push("result is Error but reply_is_ok is true".to_string());
            }
            if reply_get_sample(r).is_some() {
                bad.push("result is Error but reply_get_sample returned a sample".to_string());
            }
            match reply_get_err(r) {
                None => bad.push("result is Error but reply_get_err is None".to_string()),
                Some(err) => {
                    if es.payload != *reply_error_get_payload(err) {
                        bad.push(
                            "error.payload disagrees with reply_error_get_payload".to_string(),
                        );
                    }
                    if es.encoding != encoding_to_struct(reply_error_get_encoding(err)) {
                        bad.push(
                            "error.encoding disagrees with reply_error_get_encoding".to_string(),
                        );
                    }
                    // `ReplyErrorStruct`'s own value form, reached directly
                    // rather than nested inside the reply.
                    let direct = reply_error_to_struct(err);
                    if direct.payload != *reply_error_get_payload(err)
                        || direct.encoding != encoding_to_struct(reply_error_get_encoding(err))
                    {
                        bad.push("reply_error_to_struct disagrees with its accessors".to_string());
                    }
                }
            }
        }
    }

    #[cfg(feature = "unstable")]
    if rs.replier_id != reply_get_replier_id(r) {
        bad.push("replier_id disagrees with reply_get_replier_id".to_string());
    }

    bad
}

/// A collected reply: `(is_ok, payload, value-form mismatches)`.
type Entry = (bool, Vec<u8>, Vec<String>);
/// Shared state for [`collect_get_replies`]: the replies seen so far plus a flag
/// set when the reply stream ends.
type ReplySlot = Arc<(Mutex<(Vec<Entry>, bool)>, Condvar)>;

/// Issue a `get` on `key`, collect every reply as [`Entry`], and return once the
/// reply stream ends (`on_close`) or a generous timeout elapses.
fn collect_get_replies(session: &Session, key: &str) -> Vec<Entry> {
    let slot: ReplySlot = Arc::new((Mutex::new((Vec::new(), false)), Condvar::new()));

    let ke_get = ke(key);
    let slot_cb = slot.clone();
    let slot_close = slot.clone();
    session_get(
        session,
        Selector {
            key_expr: ke_get,
            parameters: String::new(),
        },
        Some(5000),
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
            let mismatches = reply_struct_mismatches(&reply);
            let entry = if reply_is_ok(&reply) {
                let sample = reply_get_sample(&reply).expect("ok reply has a sample");
                (
                    true,
                    zbytes_to_bytes(sample_get_payload(sample)).into_owned(),
                    mismatches,
                )
            } else {
                let err = reply_get_err(&reply).expect("err reply has an error");
                (
                    false,
                    zbytes_to_bytes(reply_error_get_payload(err)).into_owned(),
                    mismatches,
                )
            };
            let (lock, cv) = &*slot_cb;
            lock.lock().unwrap().0.push(entry);
            cv.notify_all();
        },
        move || {
            let (lock, cv) = &*slot_close;
            lock.lock().unwrap().1 = true;
            cv.notify_all();
        },
    )
    .expect("session get");

    let (lock, cv) = &*slot;
    let mut guard = lock.lock().unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while !guard.1 {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            break;
        }
        let (g, _) = cv.wait_timeout(guard, remaining).unwrap();
        guard = g;
    }
    guard.0.clone()
}

#[test]
fn queryable_replies_with_value() {
    let session = open(config_new_default()).expect("open session");
    let key = "test/queryable/ok";

    let _queryable = session_declare_queryable(
        &session,
        ke(key),
        None,
        move |query| {
            query_reply_success(
                &query,
                &ke(key),
                zbytes_new_from_slice(b"the-value"),
                None,
                None,
                None,
                None,
            )
            .expect("send reply");
        },
        || {},
    )
    .expect("declare queryable");

    // Let the queryable register before querying.
    std::thread::sleep(Duration::from_millis(500));

    let replies = collect_get_replies(&session, key);
    assert_eq!(replies.len(), 1, "expected exactly one reply");
    assert!(replies[0].0, "reply should be a success");
    assert_eq!(replies[0].1, b"the-value");
    assert!(
        replies[0].2.is_empty(),
        "value form disagrees with accessors: {:?}",
        replies[0].2
    );
}

#[test]
fn queryable_replies_with_error() {
    let session = open(config_new_default()).expect("open session");
    let key = "test/queryable/err";

    let _queryable = session_declare_queryable(
        &session,
        ke(key),
        None,
        move |query| {
            query_reply_error(&query, zbytes_new_from_slice(b"boom"), None).expect("send error");
        },
        || {},
    )
    .expect("declare queryable");

    std::thread::sleep(Duration::from_millis(500));

    let replies = collect_get_replies(&session, key);
    assert_eq!(replies.len(), 1, "expected exactly one reply");
    assert!(!replies[0].0, "reply should be an error");
    assert_eq!(replies[0].1, b"boom");
    assert!(
        replies[0].2.is_empty(),
        "value form disagrees with accessors: {:?}",
        replies[0].2
    );
}
