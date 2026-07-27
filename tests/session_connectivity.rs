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

//! The connectivity half of `SessionInfo`: which transports a session has open,
//! which links they carry, and the notifications when either changes.
//!
//! The zid accessors (`tests/session_info.rs`) answer *who* is out there; these
//! answer *how this session reaches them*, so every test here needs two really
//! connected sessions rather than one isolated one.

#![cfg(feature = "unstable")]

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use zenoh_flat::{
    LinkEvent, LinkEventKind, Session, Transport, TransportEvent, TransportEventKind,
    config_insert_json5, config_new_default, link_events_listener_undeclare, open,
    session_declare_link_events_listener, session_declare_transport_events_listener,
    session_get_links, session_get_locators, session_get_transports, session_get_zid,
    transport_events_listener_undeclare,
};

/// How long to wait for a connection to establish and for the resulting events
/// to be delivered.
const SETTLE: Duration = Duration::from_millis(800);

/// A session with scouting off, so nothing but an explicit endpoint can connect
/// it to anything — otherwise a peer discovered on the host network would make
/// the "no transports" and per-peer assertions non-deterministic.
fn unscouted_config() -> zenoh_flat::Config {
    let mut config = config_new_default();
    config_insert_json5(&mut config, "scouting/multicast/enabled", "false")
        .expect("disable multicast scouting");
    config_insert_json5(&mut config, "scouting/gossip/enabled", "false")
        .expect("disable gossip scouting");
    config
}

fn isolated_session() -> Session {
    open(unscouted_config()).expect("open session")
}

/// A session listening on a loopback TCP port chosen by the OS, so concurrent
/// tests (and other things on the CI host) cannot collide on a fixed port. The
/// port actually bound is read back through `session_get_locators`.
fn listening_session() -> (Session, String) {
    let mut config = unscouted_config();
    config_insert_json5(&mut config, "listen/endpoints", r#"["tcp/127.0.0.1:0"]"#)
        .expect("set listen endpoint");
    let session = open(config).expect("open listening session");

    let endpoint = session_get_locators(&session)
        .into_iter()
        .find(|l| l.starts_with("tcp/"))
        .expect("the listening session reports its tcp locator");
    (session, endpoint)
}

fn connecting_session(endpoint: &str) -> Session {
    let mut config = unscouted_config();
    config_insert_json5(
        &mut config,
        "connect/endpoints",
        &format!(r#"["{endpoint}"]"#),
    )
    .expect("set connect endpoint");
    open(config).expect("open connecting session")
}

/// A session connected to nothing has no transports and no links. This is the
/// baseline the other tests are read against: without it, a report that always
/// returned a non-empty list would still satisfy them.
#[test]
fn isolated_session_has_no_transports_or_links() {
    let session = isolated_session();
    assert!(
        session_get_transports(&session).is_empty(),
        "an isolated session has no transports"
    );
    assert!(
        session_get_links(&session, None).expect("links").is_empty(),
        "an isolated session has no links"
    );
}

/// Two connected sessions each report a transport to the other, carrying the
/// other's zid, and at least one link whose endpoints are real locators.
///
/// The zid is what makes this more than a "something was returned" check: it
/// ties the reported transport to the specific session at the other end.
#[test]
fn connected_sessions_report_each_other() {
    let (listener, endpoint) = listening_session();
    let connector = connecting_session(&endpoint);
    std::thread::sleep(SETTLE);

    let listener_zid = session_get_zid(&listener);
    let connector_zid = session_get_zid(&connector);

    let transports = session_get_transports(&connector);
    assert_eq!(
        transports.len(),
        1,
        "the connecting session has exactly one transport"
    );
    assert_eq!(
        transports[0].zid, listener_zid,
        "the transport identifies the session at the other end"
    );

    let peer_transports = session_get_transports(&listener);
    assert_eq!(peer_transports.len(), 1);
    assert_eq!(peer_transports[0].zid, connector_zid);

    let links = session_get_links(&connector, None).expect("links");
    assert!(!links.is_empty(), "a connected session has links");
    for link in &links {
        assert_eq!(link.zid, listener_zid, "a link carries its transport's zid");
        // The endpoint the connection was made to is the one we asked for, so
        // this pins the rendering as well as the presence of the field.
        assert_eq!(link.dst, endpoint, "the link's destination is the endpoint");
        assert!(
            link.src.starts_with("tcp/"),
            "link source {:?} should be a rendered locator",
            link.src
        );
        assert!(link.mtu > 0, "a real link has a non-zero mtu");
    }
}

/// The transport filter is applied by zenoh, not ignored: filtering by the
/// transport that owns the links returns them, and filtering by a transport
/// that is not connected returns nothing.
///
/// The negative half is the one that matters. Without it, an implementation
/// that dropped the filter on the floor and always returned every link would
/// pass — and dropping it is exactly what an unused parameter does.
#[test]
fn links_can_be_filtered_by_transport() {
    let (listener, endpoint) = listening_session();
    let connector = connecting_session(&endpoint);
    std::thread::sleep(SETTLE);

    let all = session_get_links(&connector, None).expect("links");
    assert!(!all.is_empty());

    let transport = session_get_transports(&connector)
        .into_iter()
        .next()
        .expect("one transport");
    let filtered = session_get_links(&connector, Some(transport.clone())).expect("filtered links");
    assert_eq!(
        filtered, all,
        "filtering by the only transport returns its links"
    );

    // Same transport, but naming a node we are not connected to: our own zid.
    let stranger = Transport {
        zid: session_get_zid(&connector),
        ..transport
    };
    assert!(
        session_get_links(&connector, Some(stranger))
            .expect("filtered links")
            .is_empty(),
        "filtering by an unconnected transport returns no links"
    );

    drop(listener);
}

/// A link-events listener is notified when a link appears, and reports the same
/// link the polling accessor does.
#[test]
fn link_events_listener_reports_a_new_link() {
    let (listener, endpoint) = listening_session();

    let events: Arc<Mutex<Vec<LinkEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = events.clone();
    let handle = session_declare_link_events_listener(
        &listener,
        move |event| sink.lock().expect("lock").push(event),
        || {},
        None,
        None,
    )
    .expect("declare link events listener");

    let _connector = connecting_session(&endpoint);
    std::thread::sleep(SETTLE);

    let seen = events.lock().expect("lock").clone();
    let added: Vec<_> = seen
        .iter()
        .filter(|e| e.kind == LinkEventKind::Added)
        .collect();
    assert!(!added.is_empty(), "a new link should be reported as added");

    let polled = session_get_links(&listener, None).expect("links");
    assert!(
        added.iter().any(|e| polled.contains(&e.link)),
        "the reported link should be one the accessor also reports"
    );

    link_events_listener_undeclare(handle).expect("undeclare link events listener");
}

/// With `history`, a listener declared *after* the connection still reports the
/// links that already exist. Without it, the same declaration reports nothing —
/// which is what makes this a test of the flag rather than of the listener.
#[test]
fn link_events_listener_history_reports_existing_links() {
    let (listener, endpoint) = listening_session();
    let _connector = connecting_session(&endpoint);
    std::thread::sleep(SETTLE);

    let with_history: Arc<Mutex<Vec<LinkEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = with_history.clone();
    let handle = session_declare_link_events_listener(
        &listener,
        move |event| sink.lock().expect("lock").push(event),
        || {},
        Some(true),
        None,
    )
    .expect("declare link events listener with history");

    let without_history: Arc<Mutex<Vec<LinkEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = without_history.clone();
    let plain = session_declare_link_events_listener(
        &listener,
        move |event| sink.lock().expect("lock").push(event),
        || {},
        Some(false),
        None,
    )
    .expect("declare link events listener without history");

    std::thread::sleep(SETTLE);

    assert!(
        !with_history.lock().expect("lock").is_empty(),
        "history should replay the links already established"
    );
    assert!(
        without_history.lock().expect("lock").is_empty(),
        "without history nothing already established is replayed"
    );

    link_events_listener_undeclare(handle).expect("undeclare");
    link_events_listener_undeclare(plain).expect("undeclare");
}

/// A transport-events listener is notified when a transport opens, and names
/// the node at the other end.
#[test]
fn transport_events_listener_reports_a_new_transport() {
    let (listener, endpoint) = listening_session();

    let events: Arc<Mutex<Vec<TransportEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = events.clone();
    let handle = session_declare_transport_events_listener(
        &listener,
        move |event| sink.lock().expect("lock").push(event),
        || {},
        None,
    )
    .expect("declare transport events listener");

    let connector = connecting_session(&endpoint);
    std::thread::sleep(SETTLE);
    let connector_zid = session_get_zid(&connector);

    let seen = events.lock().expect("lock").clone();
    assert!(
        seen.iter()
            .any(|e| e.kind == TransportEventKind::Opened && e.transport.zid == connector_zid),
        "the transport to the connecting session should be reported as opened, got {seen:?}"
    );

    transport_events_listener_undeclare(handle).expect("undeclare transport events listener");
}

/// A closing transport is reported too — the other half of the two-state fact,
/// and the half a listener that only ever fired on connect would miss.
#[test]
fn transport_events_listener_reports_a_closed_transport() {
    let (listener, endpoint) = listening_session();

    let events: Arc<Mutex<Vec<TransportEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = events.clone();
    let handle = session_declare_transport_events_listener(
        &listener,
        move |event| sink.lock().expect("lock").push(event),
        || {},
        None,
    )
    .expect("declare transport events listener");

    let connector = connecting_session(&endpoint);
    std::thread::sleep(SETTLE);
    let connector_zid = session_get_zid(&connector);

    zenoh_flat::session_close(&connector).expect("close the connecting session");
    std::thread::sleep(SETTLE);

    let seen = events.lock().expect("lock").clone();
    assert!(
        seen.iter()
            .any(|e| e.kind == TransportEventKind::Closed && e.transport.zid == connector_zid),
        "the transport should be reported as closed, got {seen:?}"
    );

    transport_events_listener_undeclare(handle).expect("undeclare");
}

/// A background listener needs no handle kept alive and still delivers events.
#[test]
fn background_listeners_deliver_without_a_handle() {
    use zenoh_flat::{
        session_declare_background_link_events_listener,
        session_declare_background_transport_events_listener,
    };

    let (listener, endpoint) = listening_session();

    let links: Arc<Mutex<Vec<LinkEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = links.clone();
    session_declare_background_link_events_listener(
        &listener,
        move |event| sink.lock().expect("lock").push(event),
        || {},
        None,
        None,
    )
    .expect("declare background link events listener");

    let transports: Arc<Mutex<Vec<TransportEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = transports.clone();
    session_declare_background_transport_events_listener(
        &listener,
        move |event| sink.lock().expect("lock").push(event),
        || {},
        None,
    )
    .expect("declare background transport events listener");

    let _connector = connecting_session(&endpoint);
    std::thread::sleep(SETTLE);

    assert!(
        !links.lock().expect("lock").is_empty(),
        "the background link listener should have been notified"
    );
    assert!(
        !transports.lock().expect("lock").is_empty(),
        "the background transport listener should have been notified"
    );
}
