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

use zenoh_flat::{
    KeyExpr, Reply, Selector, Session, config_new_default, keyexpr_new_try_from, open,
    query_reply_error, query_reply_success, reply_error_get_payload, reply_get_err,
    reply_get_sample, reply_is_ok, sample_get_payload, session_declare_queryable, session_get,
    zbytes_as_bytes, zbytes_new_from_slice,
};

fn ke(s: &str) -> KeyExpr {
    keyexpr_new_try_from(s.to_string()).unwrap_or_else(|e| panic!("invalid key expr {s:?}: {e}"))
}

/// A collected reply: `(is_ok, payload)`.
type Entry = (bool, Vec<u8>);
/// Shared state for [`collect_get_replies`]: the replies seen so far plus a flag
/// set when the reply stream ends.
type ReplySlot = Arc<(Mutex<(Vec<Entry>, bool)>, Condvar)>;

/// Issue a `get` on `key`, collect every reply as `(is_ok, payload)`, and return
/// once the reply stream ends (`on_close`) or a generous timeout elapses.
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
            let entry = if reply_is_ok(&reply) {
                let sample = reply_get_sample(&reply).expect("ok reply has a sample");
                (
                    true,
                    zbytes_as_bytes(sample_get_payload(sample)).into_owned(),
                )
            } else {
                let err = reply_get_err(&reply).expect("err reply has an error");
                (
                    false,
                    zbytes_as_bytes(reply_error_get_payload(err)).into_owned(),
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
}
