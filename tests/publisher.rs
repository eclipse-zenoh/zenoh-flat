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

//! A publisher reports the settings it was declared with.

use zenoh_flat::{
    CongestionControl, Encoding, KeyExpr, Priority, Publisher, Session, config_new_default,
    encoding_const_text_plain, encoding_get_id, keyexpr_new_try_from, open,
    publisher_get_congestion_control, publisher_get_encoding, publisher_get_priority,
    session_declare_publisher,
};
#[cfg(feature = "unstable")]
use zenoh_flat::{Reliability, publisher_get_reliability};

fn ke(s: &str) -> KeyExpr {
    keyexpr_new_try_from(s.to_string()).expect("key expr")
}

fn declare(
    session: &Session,
    key: &str,
    encoding: Option<&Encoding>,
    cc: Option<CongestionControl>,
    prio: Option<Priority>,
) -> Publisher {
    session_declare_publisher(
        session,
        ke(key),
        encoding,
        cc,
        prio,
        None,
        #[cfg(feature = "unstable")]
        None,
    )
    .expect("declare publisher")
}

/// Every setting passed at declare time is readable back, and reads back as
/// what was asked for rather than as a default.
///
/// The values are deliberately non-default: against a publisher left at its
/// defaults an accessor that returned a hardcoded constant would still agree.
#[test]
fn declared_settings_are_readable() {
    let session = open(config_new_default()).expect("open session");
    let p = declare(
        &session,
        "test/publisher/qos",
        Some(encoding_const_text_plain()),
        Some(CongestionControl::Block),
        Some(Priority::InteractiveHigh),
    );

    assert_eq!(
        publisher_get_congestion_control(&p),
        CongestionControl::Block
    );
    assert_eq!(publisher_get_priority(&p), Priority::InteractiveHigh);
    assert_eq!(
        encoding_get_id(publisher_get_encoding(&p)),
        encoding_get_id(encoding_const_text_plain())
    );
}

/// A publisher declared without settings reports base's defaults — which is the
/// case a caller cannot work out for itself, and so the reason these accessors
/// exist. The assertion is that the defaults *differ* from the values above,
/// not what they happen to be: pinning base's chosen defaults here would make
/// this a change-detector for base rather than for flat.
#[test]
fn defaults_are_reported_and_differ_from_explicit_settings() {
    let session = open(config_new_default()).expect("open session");
    let def = declare(&session, "test/publisher/default", None, None, None);
    let set = declare(
        &session,
        "test/publisher/set",
        Some(encoding_const_text_plain()),
        Some(CongestionControl::Block),
        Some(Priority::InteractiveHigh),
    );

    assert_ne!(
        publisher_get_congestion_control(&def),
        publisher_get_congestion_control(&set)
    );
    assert_ne!(publisher_get_priority(&def), publisher_get_priority(&set));
}

#[cfg(feature = "unstable")]
#[test]
fn reliability_is_readable() {
    let session = open(config_new_default()).expect("open session");
    let p = session_declare_publisher(
        &session,
        ke("test/publisher/reliability"),
        None,
        None,
        None,
        None,
        Some(Reliability::BestEffort),
    )
    .expect("declare publisher");
    assert_eq!(publisher_get_reliability(&p), Reliability::BestEffort);
}
