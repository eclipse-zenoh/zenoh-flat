//! Connectivity information: the transports a session has open, the links
//! within them, and notifications when either changes.
//!
//! The three zid accessors on [`crate::Session`] answer *who* is out there;
//! everything here answers *how this session is connected to them*. All of it
//! is `unstable` in base zenoh, so all of it is gated the same way in flat.

use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

use crate::{
    Error, LinkEventsListener, Reliability, Session, TransportEventsListener, WhatAmI, ZenohId,
    util::OnceDrop,
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

/// A transport established to another Zenoh node, as a plain value.
///
/// A transport is a connection to a peer; several may exist to the same peer
/// (a unicast and a multicast one, for instance), and each carries one or more
/// [`Link`]s, which are the actual protocol-level connections.
///
/// This is a value, not a handle: zenoh hands out an owned snapshot of every
/// field, there is nothing left to defer, and there is no lifecycle to manage.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transport {
    /// Identifier of the node at the other end.
    pub zid: ZenohId,
    /// Type of the node at the other end.
    pub whatami: WhatAmI,
    /// Whether this transport supports QoS.
    ///
    /// When it does not, a link of this transport reports no
    /// [`Link::priorities`] and no [`Link::reliability`] — zenoh has nothing to
    /// report rather than a default to report.
    pub is_qos: bool,
    /// Whether this transport is multicast.
    pub is_multicast: bool,
    /// Whether this transport supports shared memory.
    ///
    /// Present only when the `shared-memory` feature is enabled, mirroring
    /// zenoh: without it zenoh does not track the fact, so flat has none to
    /// report and says so by omitting the field rather than by reporting
    /// `false`.
    #[cfg(feature = "shared-memory")]
    pub is_shm: bool,
}

/// A protocol-level connection within a [`Transport`], as a plain value.
///
/// Several links may exist to the same node within one transport, using
/// different protocols (TCP, UDP, QUIC, …).
///
/// This is a value even though it carries lists (`interfaces`) and strings:
/// see the note on snapshots in the crate README's *Choosing a shape*.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link {
    /// Identifier of the node this link's transport connects to.
    pub zid: ZenohId,
    /// Local endpoint, rendered as in [`crate::session_get_locators`].
    pub src: String,
    /// Remote endpoint, rendered as in [`crate::session_get_locators`].
    pub dst: String,
    /// Group endpoint, present when the link is multicast.
    pub group: Option<String>,
    /// Maximum transmission unit of the link, in bytes.
    pub mtu: u16,
    /// Whether the link is streamed.
    pub is_streamed: bool,
    /// Network interfaces this link is bound to.
    pub interfaces: Vec<String>,
    /// Authentication identifier of the link, for protocols that carry one.
    pub auth_identifier: Option<String>,
    /// Priorities the link carries, absent when its transport has no QoS.
    pub priorities: Option<PriorityRange>,
    /// Reliability of the link, absent when its transport has no QoS.
    pub reliability: Option<Reliability>,
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
/// Both kinds carry the same [`Link`], so this is a tag beside a value, not a
/// sum: there are no alternatives with different payloads to choose between.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkEvent {
    /// What happened to the link.
    pub kind: LinkEventKind,
    /// The link it happened to.
    pub link: Link,
}

/// A transport opening or closing.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportEvent {
    /// What happened to the transport.
    pub kind: TransportEventKind,
    /// The transport it happened to.
    pub transport: Transport,
}

impl From<&zenoh::session::Transport> for Transport {
    fn from(t: &zenoh::session::Transport) -> Self {
        Transport {
            zid: (*t.zid()).into(),
            whatami: t.whatami().into(),
            is_qos: t.is_qos(),
            is_multicast: t.is_multicast(),
            #[cfg(feature = "shared-memory")]
            is_shm: t.is_shm(),
        }
    }
}

impl TryFrom<&Transport> for zenoh::session::Transport {
    type Error = Error;

    /// Rebuild zenoh's own transport, so a transport read back from a session
    /// can be handed to the calls that filter by one.
    ///
    /// Every field round-trips unchanged; the only way this fails is an
    /// identifier that is not one (all-zero bytes), which cannot come from a
    /// transport zenoh reported.
    fn try_from(t: &Transport) -> Result<Self, Self::Error> {
        Ok(zenoh::session::Transport::new_from_fields(
            (&t.zid).try_into()?,
            t.whatami.into(),
            t.is_qos,
            t.is_multicast,
            #[cfg(feature = "shared-memory")]
            t.is_shm,
        ))
    }
}

impl From<&zenoh::session::Link> for Link {
    fn from(l: &zenoh::session::Link) -> Self {
        Link {
            zid: (*l.zid()).into(),
            src: l.src().to_string(),
            dst: l.dst().to_string(),
            group: l.group().map(|g| g.to_string()),
            mtu: l.mtu(),
            is_streamed: l.is_streamed(),
            interfaces: l.interfaces().to_vec(),
            auth_identifier: l.auth_identifier().map(str::to_string),
            priorities: l
                .priorities()
                .map(|(start, end)| PriorityRange { start, end }),
            reliability: l.reliability().map(Reliability::from),
        }
    }
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
            link: e.link().into(),
        }
    }
}

impl From<&zenoh::session::TransportEvent> for TransportEvent {
    fn from(e: &zenoh::session::TransportEvent) -> Self {
        TransportEvent {
            kind: e.kind().into(),
            transport: e.transport().into(),
        }
    }
}

/// Return the transports this session currently has open.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn session_get_transports(session: &Session) -> Vec<Transport> {
    session
        .info()
        .transports()
        .wait()
        .map(|t| Transport::from(&t))
        .collect()
}

/// Return the links this session currently has established.
///
/// Links of every transport are returned unless `transport` selects one, in
/// which case only that transport's links are. Pass a transport obtained from
/// [`session_get_transports`]; the error is reported only for a transport whose
/// identifier is not one, which cannot happen for a transport zenoh reported.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn session_get_links(
    session: &Session,
    transport: Option<Transport>,
) -> Result<Vec<Link>, Error> {
    let info = session.info();
    let mut builder = info.links();
    if let Some(t) = transport {
        builder = builder.transport((&t).try_into()?);
    }
    Ok(builder.wait().map(|l| Link::from(&l)).collect())
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
    transport: Option<Transport>,
) -> Result<LinkEventsListener, Error> {
    let on_close = OnceDrop::new(on_close);
    let info = session.info();
    let mut builder = info.link_events_listener();
    if let Some(v) = history {
        builder = builder.history(v);
    }
    if let Some(t) = transport {
        builder = builder.transport((&t).try_into()?);
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
    transport: Option<Transport>,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    let info = session.info();
    let mut builder = info.link_events_listener();
    if let Some(v) = history {
        builder = builder.history(v);
    }
    if let Some(t) = transport {
        builder = builder.transport((&t).try_into()?);
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
