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

//! A querier can ask whether anything would answer it, and be told when that
//! changes — the check that lets a caller skip a query nothing will serve.

use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use zenoh_flat::{
    KeyExpr, Querier, Query, Session, config_new_default, keyexpr_new_try_from, open,
    querier_declare_matching_listener, querier_matching_status, queryable_undeclare,
    session_declare_querier, session_declare_queryable,
};

fn ke(s: &str) -> KeyExpr {
    keyexpr_new_try_from(s.to_string()).expect("key expr")
}

fn declare_querier(session: &Session, key: &str) -> Querier {
    session_declare_querier(session, ke(key), None, None, None, None, None, None, None)
        .expect("declare querier")
}

/// Matching status reflects whether a queryable exists, and flips as one comes
/// and goes. Asserting both states matters: a stub returning a constant would
/// satisfy either one alone.
#[test]
fn matching_status_tracks_queryables() {
    let session = open(config_new_default()).expect("open session");
    let key = "test/querier_matching/status";
    let querier = declare_querier(&session, key);

    assert!(
        !querier_matching_status(&querier).expect("matching status"),
        "no queryable declared yet"
    );

    let queryable = session_declare_queryable(&session, ke(key), None, |_q: Query| {}, || {})
        .expect("declare queryable");
    std::thread::sleep(Duration::from_millis(500));
    assert!(
        querier_matching_status(&querier).expect("matching status"),
        "a matching queryable exists"
    );

    queryable_undeclare(queryable).expect("undeclare queryable");
    std::thread::sleep(Duration::from_millis(500));
    assert!(
        !querier_matching_status(&querier).expect("matching status"),
        "the queryable is gone again"
    );
}

/// A matching listener is notified when the status changes.
#[test]
fn matching_listener_is_notified() {
    let session = open(config_new_default()).expect("open session");
    let key = "test/querier_matching/listener";
    let querier = declare_querier(&session, key);

    let hits = Arc::new(AtomicUsize::new(0));
    let hits_cb = hits.clone();
    let _listener = querier_declare_matching_listener(
        &querier,
        move |_matching| {
            hits_cb.fetch_add(1, Ordering::SeqCst);
        },
        || {},
    )
    .expect("declare matching listener");

    let _queryable = session_declare_queryable(&session, ke(key), None, |_q: Query| {}, || {})
        .expect("declare queryable");
    std::thread::sleep(Duration::from_millis(800));

    assert!(
        hits.load(Ordering::SeqCst) > 0,
        "listener should have been notified when a queryable appeared"
    );
}
