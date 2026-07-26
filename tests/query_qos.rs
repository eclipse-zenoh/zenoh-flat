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

//! A queryable can read the delivery settings an incoming query was sent with.
//! Available only when unstable features are enabled.
#![cfg(feature = "unstable")]

use std::{
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

use zenoh_flat::{
    CongestionControl, KeyExpr, Priority, Selector, config_new_default, keyexpr_new_try_from, open,
    query_get_congestion_control, query_get_express, query_get_priority, session_declare_queryable,
    session_get,
};

fn ke(s: &str) -> KeyExpr {
    keyexpr_new_try_from(s.to_string()).expect("key expr")
}

/// The QoS a querier sends with arrives at the queryable intact.
///
/// Sent with deliberately **non-default** settings: against a query left at its
/// defaults, accessors returning hardcoded constants would still agree.
#[test]
fn queryable_reads_the_query_delivery_settings() {
    let session = open(config_new_default()).expect("open session");
    let key = "test/query_qos/qos";

    type Seen = Arc<(Mutex<Option<(CongestionControl, Priority, bool)>>, Condvar)>;
    let seen: Seen = Arc::new((Mutex::new(None), Condvar::new()));
    let seen_cb = seen.clone();

    let _queryable = session_declare_queryable(
        &session,
        ke(key),
        None,
        move |query| {
            let got = (
                query_get_congestion_control(&query),
                query_get_priority(&query),
                query_get_express(&query),
            );
            let (lock, cv) = &*seen_cb;
            *lock.lock().unwrap() = Some(got);
            cv.notify_all();
        },
        || {},
    )
    .expect("declare queryable");

    std::thread::sleep(Duration::from_millis(500));

    session_get(
        &session,
        Selector {
            key_expr: ke(key),
            parameters: String::new(),
        },
        Some(5000),
        None,
        None,
        None,
        Some(CongestionControl::Block),
        Some(Priority::InteractiveHigh),
        Some(true),
        None,
        None,
        None,
        |_reply| {},
        || {},
    )
    .expect("session get");

    let (lock, cv) = &*seen;
    let mut guard = lock.lock().unwrap();
    while guard.is_none() {
        let (g, t) = cv
            .wait_timeout(guard, Duration::from_secs(10))
            .expect("wait");
        guard = g;
        if t.timed_out() {
            break;
        }
    }
    let got = guard.take().expect("queryable never saw the query");

    assert_eq!(got.0, CongestionControl::Block, "congestion control");
    assert_eq!(got.1, Priority::InteractiveHigh, "priority");
    assert!(got.2, "express");
}
