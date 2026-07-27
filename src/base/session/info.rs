//! Connectivity information: the transports a session has open, the links
//! within them, and notifications when either changes.
//!
//! The three zid accessors on [`crate::Session`] answer *who* is out there;
//! everything here answers *how this session is connected to them*. All of it
//! is `unstable` in base zenoh, so all of it is gated the same way in flat.

use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

use crate::{
    Error, Link, LinkEventsListener, Reliability, Session, Transport, TransportEventsListener,
    WhatAmI, ZenohId, util::OnceDrop,
};

/// The range of priorities a link carries, as the raw numbers zenoh reports.
///
/// The numbers correspond to [`crate::Priority`], but zenoh also uses the value
/// `0` (control traffic), which that enum does not name. Converting to it would
/// have to fail on a number zenoh considers perfectly legal, so the raw numbers
/// are kept.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PriorityRange {
    /// First priority of the range, as zenoh reports it.
    pub start: u8,
    /// Last priority of the range, as zenoh reports it.
    pub end: u8,
}

/// Identifier of the node at the other end of this transport.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn transport_get_zid(transport: &Transport) -> ZenohId {
    (*transport.zid()).into()
}

/// Type of the node at the other end of this transport.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn transport_get_whatami(transport: &Transport) -> WhatAmI {
    transport.whatami().into()
}

/// Whether this transport supports QoS.
///
/// When it does not, its links report no priorities and no reliability — zenoh
/// has nothing to report there rather than a default to report.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn transport_is_qos(transport: &Transport) -> bool {
    transport.is_qos()
}

/// Whether this transport is multicast.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn transport_is_multicast(transport: &Transport) -> bool {
    transport.is_multicast()
}

/// Whether this transport supports shared memory.
///
/// Available only when the `shared-memory` feature is enabled, mirroring zenoh:
/// without it zenoh does not track the fact, so flat has none to report and
/// says so by having no accessor rather than by reporting `false`.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[cfg(feature = "shared-memory")]
pub fn transport_is_shm(transport: &Transport) -> bool {
    transport.is_shm()
}

/// A transport decomposed into a plain value.
///
/// Every field is what the matching `transport_*` accessor returns.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportStruct {
    /// Identifier of the node at the other end.
    pub zid: ZenohId,
    /// Type of the node at the other end.
    pub whatami: WhatAmI,
    /// Whether this transport supports QoS.
    pub is_qos: bool,
    /// Whether this transport is multicast.
    pub is_multicast: bool,
    /// Whether this transport supports shared memory. Present only when the
    /// `shared-memory` feature is enabled.
    #[cfg(feature = "shared-memory")]
    pub is_shm: bool,
}

impl From<&Transport> for TransportStruct {
    fn from(t: &Transport) -> Self {
        // Delegate to the field accessors so each field has one definition.
        TransportStruct {
            zid: transport_get_zid(t),
            whatami: transport_get_whatami(t),
            is_qos: transport_is_qos(t),
            is_multicast: transport_is_multicast(t),
            #[cfg(feature = "shared-memory")]
            is_shm: transport_is_shm(t),
        }
    }
}

/// Decompose a transport into its [`TransportStruct`] value form.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn transport_to_struct(transport: &Transport) -> TransportStruct {
    transport.into()
}

/// Identifier of the node this link's transport connects to.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn link_get_zid(link: &Link) -> ZenohId {
    (*link.zid()).into()
}

/// Local endpoint of this link, rendered as in [`crate::session_get_locators`].
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn link_get_src(link: &Link) -> String {
    link.src().to_string()
}

/// Remote endpoint of this link, rendered as in
/// [`crate::session_get_locators`].
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn link_get_dst(link: &Link) -> String {
    link.dst().to_string()
}

/// Group endpoint of this link, present when the link is multicast.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn link_get_group(link: &Link) -> Option<String> {
    link.group().map(|g| g.to_string())
}

/// Maximum transmission unit of this link, in bytes.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn link_get_mtu(link: &Link) -> u16 {
    link.mtu()
}

/// Whether this link is streamed.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn link_is_streamed(link: &Link) -> bool {
    link.is_streamed()
}

/// Network interfaces this link is bound to.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn link_get_interfaces(link: &Link) -> Vec<String> {
    link.interfaces().to_vec()
}

/// Authentication identifier of this link, for protocols that carry one.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn link_get_auth_identifier(link: &Link) -> Option<String> {
    link.auth_identifier().map(str::to_string)
}

/// Priorities this link carries, absent when its transport has no QoS.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn link_get_priorities(link: &Link) -> Option<PriorityRange> {
    link.priorities()
        .map(|(start, end)| PriorityRange { start, end })
}

/// Reliability of this link, absent when its transport has no QoS.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn link_get_reliability(link: &Link) -> Option<Reliability> {
    link.reliability().map(Reliability::from)
}

/// A link decomposed into a plain value.
///
/// Every field is what the matching `link_*` accessor returns. Reading the
/// whole struct materializes the endpoints and the interface list; a caller who
/// wants only the cheap fields reads those accessors instead, which is the
/// reason the handle exists.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkStruct {
    /// Identifier of the node this link's transport connects to.
    pub zid: ZenohId,
    /// Local endpoint.
    pub src: String,
    /// Remote endpoint.
    pub dst: String,
    /// Group endpoint, present when the link is multicast.
    pub group: Option<String>,
    /// Maximum transmission unit, in bytes.
    pub mtu: u16,
    /// Whether the link is streamed.
    pub is_streamed: bool,
    /// Network interfaces the link is bound to.
    pub interfaces: Vec<String>,
    /// Authentication identifier, for protocols that carry one.
    pub auth_identifier: Option<String>,
    /// Priorities the link carries, absent when its transport has no QoS.
    pub priorities: Option<PriorityRange>,
    /// Reliability of the link, absent when its transport has no QoS.
    pub reliability: Option<Reliability>,
}

impl From<&Link> for LinkStruct {
    fn from(l: &Link) -> Self {
        // Delegate to the field accessors so each field has one definition.
        LinkStruct {
            zid: link_get_zid(l),
            src: link_get_src(l),
            dst: link_get_dst(l),
            group: link_get_group(l),
            mtu: link_get_mtu(l),
            is_streamed: link_is_streamed(l),
            interfaces: link_get_interfaces(l),
            auth_identifier: link_get_auth_identifier(l),
            priorities: link_get_priorities(l),
            reliability: link_get_reliability(l),
        }
    }
}

/// Decompose a link into its [`LinkStruct`] value form.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn link_to_struct(link: &Link) -> LinkStruct {
    link.into()
}

/// What happened to a link.
///
/// zenoh spells this with `SampleKind` (`Put` for added, `Delete` for removed),
/// reusing an unrelated type; flat carries the same two-state fact under a name
/// that says what it means.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkEventKind {
    /// The link was established.
    Added = 0,
    /// The link went away.
    Removed = 1,
}

/// What happened to a transport.
///
/// A separate type from [`LinkEventKind`] because zenoh describes the two
/// events with different words — a link is *added* or *removed*, a transport is
/// *opened* or *closed* — and each name belongs with the thing it describes.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportEventKind {
    /// The transport was opened.
    Opened = 0,
    /// The transport was closed.
    Closed = 1,
}

/// A link appearing or disappearing.
///
/// Both kinds carry the same link, so this is a tag beside a value, not a sum:
/// there are no alternatives with different payloads to choose between. The
/// link is carried as the handle, since a value form carries handles rather
/// than unwrapping them.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug)]
pub struct LinkEvent {
    /// What happened to the link.
    pub kind: LinkEventKind,
    /// The link it happened to.
    pub link: Link,
}

/// A transport opening or closing.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug)]
pub struct TransportEvent {
    /// What happened to the transport.
    pub kind: TransportEventKind,
    /// The transport it happened to.
    pub transport: Transport,
}

impl From<zenoh::sample::SampleKind> for LinkEventKind {
    fn from(kind: zenoh::sample::SampleKind) -> Self {
        match kind {
            zenoh::sample::SampleKind::Put => LinkEventKind::Added,
            zenoh::sample::SampleKind::Delete => LinkEventKind::Removed,
        }
    }
}

impl From<zenoh::sample::SampleKind> for TransportEventKind {
    fn from(kind: zenoh::sample::SampleKind) -> Self {
        match kind {
            zenoh::sample::SampleKind::Put => TransportEventKind::Opened,
            zenoh::sample::SampleKind::Delete => TransportEventKind::Closed,
        }
    }
}

impl From<&zenoh::session::LinkEvent> for LinkEvent {
    fn from(e: &zenoh::session::LinkEvent) -> Self {
        LinkEvent {
            kind: e.kind().into(),
            link: e.link().clone(),
        }
    }
}

impl From<&zenoh::session::TransportEvent> for TransportEvent {
    fn from(e: &zenoh::session::TransportEvent) -> Self {
        TransportEvent {
            kind: e.kind().into(),
            transport: e.transport().clone(),
        }
    }
}

/// Return the transports this session currently has open.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn session_get_transports(session: &Session) -> Vec<Transport> {
    session.info().transports().wait().collect()
}

/// Return the links this session currently has established.
///
/// Links of every transport are returned unless `transport` selects one, in
/// which case only that transport's links are. The transport passed is one
/// obtained from [`session_get_transports`]: zenoh selects on the object it
/// handed out, so nothing has to be rebuilt from parts in order to name it.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn session_get_links(session: &Session, transport: Option<&Transport>) -> Vec<Link> {
    let info = session.info();
    let mut builder = info.links();
    if let Some(t) = transport {
        builder = builder.transport(t.clone());
    }
    builder.wait().collect()
}

/// Declare a listener notified when a link is added or removed.
///
/// With `history` set, the links already established are reported before live
/// events; with `transport` set, only that transport's links are reported. The
/// close callback is called when the listener ends.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn session_declare_link_events_listener(
    session: &Session,
    callback: impl Fn(LinkEvent) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
    history: Option<bool>,
    transport: Option<&Transport>,
) -> Result<LinkEventsListener, Error> {
    let on_close = OnceDrop::new(on_close);
    let info = session.info();
    let mut builder = info.link_events_listener();
    if let Some(v) = history {
        builder = builder.history(v);
    }
    if let Some(t) = transport {
        builder = builder.transport(t.clone());
    }
    builder
        .callback(move |event| {
            let _ = &on_close;
            callback((&event).into());
        })
        .wait()
}

/// Declare a background link-events listener that runs until the session is
/// closed.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn session_declare_background_link_events_listener(
    session: &Session,
    callback: impl Fn(LinkEvent) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
    history: Option<bool>,
    transport: Option<&Transport>,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    let info = session.info();
    let mut builder = info.link_events_listener();
    if let Some(v) = history {
        builder = builder.history(v);
    }
    if let Some(t) = transport {
        builder = builder.transport(t.clone());
    }
    builder
        .callback(move |event| {
            let _ = &on_close;
            callback((&event).into());
        })
        .background()
        .wait()
}

/// Undeclare a link-events listener.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn link_events_listener_undeclare(listener: LinkEventsListener) -> Result<(), Error> {
    listener.undeclare().wait()
}

/// Declare a listener notified when a transport is opened or closed.
///
/// With `history` set, the transports already open are reported before live
/// events. The close callback is called when the listener ends.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn session_declare_transport_events_listener(
    session: &Session,
    callback: impl Fn(TransportEvent) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
    history: Option<bool>,
) -> Result<TransportEventsListener, Error> {
    let on_close = OnceDrop::new(on_close);
    let info = session.info();
    let mut builder = info.transport_events_listener();
    if let Some(v) = history {
        builder = builder.history(v);
    }
    builder
        .callback(move |event| {
            let _ = &on_close;
            callback((&event).into());
        })
        .wait()
}

/// Declare a background transport-events listener that runs until the session
/// is closed.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn session_declare_background_transport_events_listener(
    session: &Session,
    callback: impl Fn(TransportEvent) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
    history: Option<bool>,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    let info = session.info();
    let mut builder = info.transport_events_listener();
    if let Some(v) = history {
        builder = builder.history(v);
    }
    builder
        .callback(move |event| {
            let _ = &on_close;
            callback((&event).into());
        })
        .background()
        .wait()
}

/// Undeclare a transport-events listener.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn transport_events_listener_undeclare(listener: TransportEventsListener) -> Result<(), Error> {
    listener.undeclare().wait()
}
