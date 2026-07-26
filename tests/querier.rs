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

//! A querier reports the settings it was declared with.

use zenoh_flat::{
    CongestionControl, KeyExpr, Priority, Querier, Session, config_new_default,
    keyexpr_new_try_from, open, querier_get_congestion_control, querier_get_priority,
    session_declare_querier,
};
#[cfg(feature = "unstable")]
use zenoh_flat::{ReplyKeyExpr, querier_get_accept_replies};

fn ke(s: &str) -> KeyExpr {
    keyexpr_new_try_from(s.to_string()).expect("key expr")
}

fn declare(
    session: &Session,
    key: &str,
    cc: Option<CongestionControl>,
    prio: Option<Priority>,
) -> Querier {
    session_declare_querier(session, ke(key), None, None, cc, prio, None, None, None)
        .expect("declare querier")
}

/// Settings passed at declare time read back as what was asked for.
///
/// Non-default values throughout: against a defaulted querier an accessor
/// returning a hardcoded constant would still agree. Note a querier defaults
/// to `Block`, unlike a publisher which defaults to `Drop` — so `Drop` is the
/// non-default choice here. That divergence is itself why these accessors are
/// needed: the default is not something a caller can guess from the type.
#[test]
fn declared_settings_are_readable() {
    let session = open(config_new_default()).expect("open session");
    let q = declare(
        &session,
        "test/querier/qos",
        Some(CongestionControl::Drop),
        Some(Priority::InteractiveHigh),
    );

    assert_eq!(querier_get_congestion_control(&q), CongestionControl::Drop);
    assert_eq!(querier_get_priority(&q), Priority::InteractiveHigh);
}

/// A querier declared without settings reports base's defaults — the case a
/// caller cannot work out for itself, and the reason these accessors exist.
/// Asserts the defaults *differ* from the explicit values rather than pinning
/// what base chose, which would make this a change-detector for base.
#[test]
fn defaults_differ_from_explicit_settings() {
    let session = open(config_new_default()).expect("open session");
    let def = declare(&session, "test/querier/default", None, None);
    let set = declare(
        &session,
        "test/querier/set",
        Some(CongestionControl::Drop),
        Some(Priority::InteractiveHigh),
    );

    assert_ne!(
        querier_get_congestion_control(&def),
        querier_get_congestion_control(&set)
    );
    assert_ne!(querier_get_priority(&def), querier_get_priority(&set));
}

#[cfg(feature = "unstable")]
#[test]
fn accept_replies_is_readable() {
    let session = open(config_new_default()).expect("open session");
    let q = session_declare_querier(
        &session,
        ke("test/querier/anyke"),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(ReplyKeyExpr::Any),
    )
    .expect("declare querier");
    assert_eq!(querier_get_accept_replies(&q), ReplyKeyExpr::Any);
}
