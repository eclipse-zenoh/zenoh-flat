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
//! answer *how this session reaches them*, so every test here needs really
//! connected sessions rather than one isolated one.

#![cfg(feature = "unstable")]

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use zenoh_flat::{
    Link, LinkEvent, LinkEventKind, Session, Transport, TransportEvent, TransportEventKind,
    ZenohId, config_insert_json5, config_new_default, link_events_listener_undeclare, link_get_dst,
    link_get_mtu, link_get_src, link_get_zid, link_to_struct, open,
    session_declare_link_events_listener, session_declare_transport_events_listener,
    session_get_links, session_get_locators, session_get_transports, session_get_zid,
    transport_events_listener_undeclare, transport_get_zid, transport_to_struct,
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

/// The transport of `session` that reaches the node `zid`.
fn transport_to(session: &Session, zid: &ZenohId) -> Transport {
    session_get_transports(session)
        .into_iter()
        .find(|t| &transport_get_zid(t) == zid)
        .expect("a transport to that node")
}

fn zids_of(links: &[Link]) -> Vec<ZenohId> {
    links.iter().map(link_get_zid).collect()
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
        session_get_links(&session, None).is_empty(),
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
        transport_get_zid(&transports[0]),
        listener_zid,
        "the transport identifies the session at the other end"
    );

    let peer_transports = session_get_transports(&listener);
    assert_eq!(peer_transports.len(), 1);
    assert_eq!(transport_get_zid(&peer_transports[0]), connector_zid);

    let links = session_get_links(&connector, None);
    assert!(!links.is_empty(), "a connected session has links");
    for link in &links {
        assert_eq!(
            link_get_zid(link),
            listener_zid,
            "a link carries its transport's zid"
        );
        // The endpoint the connection was made to is the one we asked for, so
        // this pins the rendering as well as the presence of the field.
        assert_eq!(
            link_get_dst(link),
            endpoint,
            "the link's destination is the endpoint"
        );
        assert!(
            link_get_src(link).starts_with("tcp/"),
            "link source {:?} should be a rendered locator",
            link_get_src(link)
        );
        assert!(link_get_mtu(link) > 0, "a real link has a non-zero mtu");
    }
}

/// Every field of a value form equals the accessor for that same field — the
/// guard for "one source of truth per field", on subjects that carry real data
/// rather than the defaults an `empty()` constructor would give.
#[test]
fn value_forms_mirror_accessors() {
    let (listener, endpoint) = listening_session();
    let _connector = connecting_session(&endpoint);
    std::thread::sleep(SETTLE);

    let transport = session_get_transports(&listener)
        .into_iter()
        .next()
        .expect("one transport");
    let ts = transport_to_struct(&transport);
    assert_eq!(ts.zid, transport_get_zid(&transport));
    assert_eq!(
        ts.whatami,
        zenoh_flat::transport_get_whatami(&transport),
        "whatami"
    );
    assert_eq!(ts.is_qos, zenoh_flat::transport_is_qos(&transport));
    assert_eq!(
        ts.is_multicast,
        zenoh_flat::transport_is_multicast(&transport)
    );

    let link = session_get_links(&listener, None)
        .into_iter()
        .next()
        .expect("one link");
    let ls = link_to_struct(&link);
    assert_eq!(ls.zid, link_get_zid(&link));
    assert_eq!(ls.src, link_get_src(&link));
    assert_eq!(ls.dst, link_get_dst(&link));
    assert_eq!(ls.group, zenoh_flat::link_get_group(&link));
    assert_eq!(ls.mtu, link_get_mtu(&link));
    assert_eq!(ls.is_streamed, zenoh_flat::link_is_streamed(&link));
    assert_eq!(ls.interfaces, zenoh_flat::link_get_interfaces(&link));
    assert_eq!(
        ls.auth_identifier,
        zenoh_flat::link_get_auth_identifier(&link)
    );
    assert_eq!(ls.priorities, zenoh_flat::link_get_priorities(&link));
    assert_eq!(ls.reliability, zenoh_flat::link_get_reliability(&link));
}

/// With two peers connected to the same session, filtering that session's links
/// by one peer's transport returns that peer's links and not the other's.
///
/// Two peers are what makes this a test of the filter rather than of the call:
/// with one peer, an implementation that ignored the argument and returned
/// every link would be indistinguishable from one that honoured it.
#[test]
fn links_can_be_filtered_by_transport() {
    let (hub, endpoint) = listening_session();
    let first = connecting_session(&endpoint);
    let second = connecting_session(&endpoint);
    std::thread::sleep(SETTLE);

    let first_zid = session_get_zid(&first);
    let second_zid = session_get_zid(&second);

    let all = session_get_links(&hub, None);
    assert!(
        zids_of(&all).contains(&first_zid) && zids_of(&all).contains(&second_zid),
        "unfiltered links cover both peers"
    );

    for zid in [first_zid, second_zid] {
        let transport = transport_to(&hub, &zid);
        let filtered = session_get_links(&hub, Some(&transport));
        assert!(!filtered.is_empty(), "the peer's links are reported");
        assert!(
            zids_of(&filtered).iter().all(|z| z == &zid),
            "filtering by one peer's transport must exclude the other's links"
        );
        assert!(
            filtered.len() < all.len(),
            "a filtered list is a strict subset when two peers are connected"
        );
    }
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

    let connector = connecting_session(&endpoint);
    std::thread::sleep(SETTLE);
    let connector_zid = session_get_zid(&connector);

    let seen = events.lock().expect("lock");
    let added: Vec<_> = seen
        .iter()
        .filter(|e| e.kind == LinkEventKind::Added)
        .collect();
    assert!(!added.is_empty(), "a new link should be reported as added");
    assert!(
        added.iter().any(|e| link_get_zid(&e.link) == connector_zid),
        "the reported link names the node that connected"
    );

    let polled = session_get_links(&listener, None);
    let polled_dsts: Vec<_> = polled.iter().map(link_get_dst).collect();
    assert!(
        added
            .iter()
            .any(|e| polled_dsts.contains(&link_get_dst(&e.link))),
        "the reported link should be one the accessor also reports"
    );
    drop(seen);

    link_events_listener_undeclare(handle).expect("undeclare link events listener");
}

/// The link listener takes the same transport filter the accessor does, and it
/// is applied: with two peers connected, a listener filtered to one peer's
/// transport is told about that peer's links only.
#[test]
fn link_events_listener_can_be_filtered_by_transport() {
    let (hub, endpoint) = listening_session();
    let first = connecting_session(&endpoint);
    let _second = connecting_session(&endpoint);
    std::thread::sleep(SETTLE);

    let first_zid = session_get_zid(&first);
    let transport = transport_to(&hub, &first_zid);

    // `history` replays the links that already exist, so the filter can be
    // observed without racing a fresh connection.
    let events: Arc<Mutex<Vec<LinkEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = events.clone();
    let handle = session_declare_link_events_listener(
        &hub,
        move |event| sink.lock().expect("lock").push(event),
        || {},
        Some(true),
        Some(&transport),
    )
    .expect("declare filtered link events listener");
    std::thread::sleep(SETTLE);

    let seen = events.lock().expect("lock");
    assert!(!seen.is_empty(), "history should replay the peer's links");
    assert!(
        seen.iter().all(|e| link_get_zid(&e.link) == first_zid),
        "a filtered listener must not be told about the other peer's links"
    );
    drop(seen);

    link_events_listener_undeclare(handle).expect("undeclare");
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

    let seen = events.lock().expect("lock");
    assert!(
        seen.iter().any(|e| e.kind == TransportEventKind::Opened
            && transport_get_zid(&e.transport) == connector_zid),
        "the transport to the connecting session should be reported as opened"
    );
    drop(seen);

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

    let seen = events.lock().expect("lock");
    assert!(
        seen.iter().any(|e| e.kind == TransportEventKind::Closed
            && transport_get_zid(&e.transport) == connector_zid),
        "the transport should be reported as closed"
    );
    drop(seen);

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
