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

//! A plain publisher can ask whether anything is listening, and be told when
//! that changes — the check that lets a caller skip producing a payload nobody
//! wants.

use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use zenoh_flat::{
    KeyExpr, Sample, config_new_default, keyexpr_new_try_from, open,
    publisher_declare_matching_listener, publisher_matching_status, session_declare_publisher,
    session_declare_subscriber, subscriber_undeclare,
};

fn ke(s: &str) -> KeyExpr {
    keyexpr_new_try_from(s.to_string()).expect("key expr")
}

fn declare_pub(session: &zenoh_flat::Session, key: &str) -> zenoh_flat::Publisher {
    session_declare_publisher(
        session,
        ke(key),
        None,
        None,
        None,
        None,
        #[cfg(feature = "unstable")]
        None,
    )
    .expect("declare publisher")
}

/// Matching status reflects whether a subscriber exists, and flips as one comes
/// and goes. Asserting both states matters: a stub returning a constant would
/// satisfy either one alone.
#[test]
fn matching_status_tracks_subscribers() {
    let session = open(config_new_default()).expect("open session");
    let key = "test/publisher_matching/status";
    let publisher = declare_pub(&session, key);

    assert!(
        !publisher_matching_status(&publisher).expect("matching status"),
        "no subscriber declared yet"
    );

    let subscriber = session_declare_subscriber(&session, ke(key), |_s: Sample| {}, || {})
        .expect("declare subscriber");
    std::thread::sleep(Duration::from_millis(500));
    assert!(
        publisher_matching_status(&publisher).expect("matching status"),
        "a matching subscriber exists"
    );

    subscriber_undeclare(subscriber).expect("undeclare subscriber");
    std::thread::sleep(Duration::from_millis(500));
    assert!(
        !publisher_matching_status(&publisher).expect("matching status"),
        "the subscriber is gone again"
    );
}

/// A matching listener is notified when the status changes.
#[test]
fn matching_listener_is_notified() {
    let session = open(config_new_default()).expect("open session");
    let key = "test/publisher_matching/listener";
    let publisher = declare_pub(&session, key);

    let hits = Arc::new(AtomicUsize::new(0));
    let hits_cb = hits.clone();
    let _listener = publisher_declare_matching_listener(
        &publisher,
        move |_matching| {
            hits_cb.fetch_add(1, Ordering::SeqCst);
        },
        || {},
    )
    .expect("declare matching listener");

    let _subscriber = session_declare_subscriber(&session, ke(key), |_s: Sample| {}, || {})
        .expect("declare subscriber");
    std::thread::sleep(Duration::from_millis(800));

    assert!(
        hits.load(Ordering::SeqCst) > 0,
        "listener should have been notified when a subscriber appeared"
    );
}
