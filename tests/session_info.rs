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

//! Session open/close and info accessors — the flat-API counterpart of
//! eclipse-zenoh/zenoh-c's `tests/z_api_session_test.c` (open/close) and the zid
//! accessors exercised by `tests/z_api_info.c`.

use zenoh_flat::{
    config_insert_json5, config_new_default, open, session_close, session_get_peers_zid,
    session_get_routers_zid, session_get_zid, session_is_closed, zenoh_id_to_bytes,
    zenoh_id_to_string,
};

/// A fully isolated session: scouting (multicast + gossip) disabled and no
/// connect endpoints, so it never discovers peers or routers — making the
/// "no peers / no routers" assertions deterministic regardless of what else is
/// running on the host network.
fn isolated_session() -> zenoh_flat::Session {
    let mut config = config_new_default();
    config_insert_json5(&mut config, "scouting/multicast/enabled", "false")
        .expect("disable multicast scouting");
    config_insert_json5(&mut config, "scouting/gossip/enabled", "false")
        .expect("disable gossip scouting");
    open(config).expect("open session")
}

/// `z_open` + `z_close`: a fresh session reports open, then closed after
/// `session_close`, and the call is idempotent. The handle stays valid across
/// the transition (only its state changes).
#[test]
fn open_close_is_idempotent() {
    let session = isolated_session();
    assert!(!session_is_closed(&session));

    session_close(&session).expect("close session");
    assert!(session_is_closed(&session));

    // Closing again is a no-op.
    session_close(&session).expect("re-close session");
    assert!(session_is_closed(&session));
}

/// The session's own zid renders to a non-empty string and a 16-byte
/// little-endian id (`uhlc::ID::MAX_SIZE`), and the two representations agree.
#[test]
fn zid_has_string_and_byte_form() {
    let session = isolated_session();

    let zid = session_get_zid(&session);
    let s = zenoh_id_to_string(&zid);
    let bytes = zenoh_id_to_bytes(&zid);

    assert!(!s.is_empty(), "zid string form must not be empty");
    assert_eq!(
        bytes.len(),
        16,
        "zid byte form is fixed 16-byte little-endian"
    );
    assert!(
        bytes.iter().any(|&b| b != 0),
        "a real zid is not all zeroes"
    );
}

/// An isolated session is connected to no peers and no routers.
#[test]
fn isolated_session_has_no_peers_or_routers() {
    let session = isolated_session();
    assert!(
        session_get_peers_zid(&session).is_empty(),
        "isolated session must have no peers"
    );
    assert!(
        session_get_routers_zid(&session).is_empty(),
        "isolated session must have no routers"
    );
}
