use std::time::Duration;

use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;
use zenoh_ext::AdvancedPublisherBuilderExt;

use crate::{
    AdvancedPublisher, CongestionControl, Encoding, EntityGlobalId, Error, KeyExpr,
    MatchingListener, Priority, Reliability, Session, ZBytes, util::OnceDrop,
};

/// Configuration enabling sample-miss detection on an advanced publisher.
///
/// A `heartbeat` period lets advanced subscribers recover the last sample;
/// `None` enables miss detection without a heartbeat.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, Default)]
pub struct MissDetectionConfig {
    /// Heartbeat period; `None` = miss detection without a heartbeat.
    pub heartbeat: Option<Duration>,
    /// When a `heartbeat` period is set, whether it is sent sporadically (only
    /// when the last sample changed) rather than periodically.
    pub sporadic: bool,
}

/// Delivery quality applied to the replies served from an advanced publisher's
/// cache.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug)]
pub struct RepliesConfig {
    /// Priority of the cached replies.
    pub priority: Priority,
    /// Congestion control policy for the cached replies.
    pub congestion_control: CongestionControl,
    /// Whether cached replies are sent express (not batched).
    pub is_express: bool,
}

/// Configuration of an advanced publisher's retransmission cache.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug)]
pub struct CacheConfig {
    /// How many samples to keep for each resource.
    pub max_samples: u64,
    /// Delivery quality applied to the replies served from the cache.
    pub replies_config: RepliesConfig,
}

impl Default for RepliesConfig {
    fn default() -> Self {
        RepliesConfig {
            priority: Priority::Data,
            congestion_control: CongestionControl::Block,
            is_express: false,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig {
            max_samples: 1,
            replies_config: RepliesConfig::default(),
        }
    }
}

impl From<RepliesConfig> for zenoh_ext::RepliesConfig {
    fn from(r: RepliesConfig) -> Self {
        zenoh_ext::RepliesConfig::default()
            .priority(r.priority.into())
            .congestion_control(r.congestion_control.into())
            .express(r.is_express)
    }
}

impl From<CacheConfig> for zenoh_ext::CacheConfig {
    fn from(c: CacheConfig) -> Self {
        zenoh_ext::CacheConfig::default()
            .max_samples(c.max_samples as usize)
            .replies_config(c.replies_config.into())
    }
}

impl From<MissDetectionConfig> for zenoh_ext::MissDetectionConfig {
    fn from(m: MissDetectionConfig) -> Self {
        let cfg = zenoh_ext::MissDetectionConfig::default();
        match m.heartbeat {
            Some(period) if m.sporadic => cfg.sporadic_heartbeat(period),
            Some(period) => cfg.heartbeat(period),
            None => cfg,
        }
    }
}

/// Declare an advanced publisher for the given key expression.
///
/// An advanced publisher adds, on top of a regular publisher, optional sample
/// miss detection, publisher detection so that advanced subscribers can discover
/// it, and a retransmission cache. The standard delivery settings behave as for
/// [`crate::session_declare_publisher`].
///
/// `sample_miss_detection`, when set, enables miss detection (see
/// [`MissDetectionConfig`]); `publisher_detection` allows advanced subscribers
/// to discover this publisher; `cache`, when set, enables the retransmission
/// cache (see [`CacheConfig`]). Available only when unstable features are
/// enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[allow(clippy::too_many_arguments)]
pub fn session_declare_advanced_publisher(
    session: &Session,
    key_expr: KeyExpr,
    encoding: Option<&Encoding>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    reliability: Option<Reliability>,
    sample_miss_detection: Option<MissDetectionConfig>,
    publisher_detection: Option<bool>,
    cache: Option<CacheConfig>,
) -> Result<AdvancedPublisher, Error> {
    let mut builder = session.declare_publisher(key_expr).advanced();
    if let Some(enc) = encoding {
        builder = builder.encoding(enc.clone());
    }
    if let Some(cc) = congestion_control {
        builder = builder.congestion_control(cc.into());
    }
    if let Some(p) = priority {
        builder = builder.priority(p.into());
    }
    if let Some(v) = express {
        builder = builder.express(v);
    }
    if let Some(r) = reliability {
        builder = builder.reliability(r.into());
    }
    if let Some(md) = sample_miss_detection {
        builder = builder.sample_miss_detection(md.into());
    }
    if publisher_detection == Some(true) {
        builder = builder.publisher_detection();
    }
    if let Some(c) = cache {
        builder = builder.cache(c.into());
    }
    builder.wait()
}

/// Publish data on the advanced publisher's key expression.
///
/// The encoding applies to this publication; the attachment carries
/// user-defined metadata. Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_publisher_put(
    publisher: &AdvancedPublisher,
    payload: ZBytes,
    encoding: Option<&Encoding>,
    attachment: Option<ZBytes>,
) -> Result<(), Error> {
    let mut publication = publisher.put(payload);
    if let Some(enc) = encoding {
        publication = publication.encoding(enc.clone());
    }
    if let Some(att) = attachment {
        publication = publication.attachment(att);
    }
    publication.wait()
}

/// Publish a deletion notification on the advanced publisher's key expression.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_publisher_delete(
    publisher: &AdvancedPublisher,
    attachment: Option<ZBytes>,
) -> Result<(), Error> {
    let mut delete = publisher.delete();
    if let Some(att) = attachment {
        delete = delete.attachment(att);
    }
    delete.wait()
}

/// Return the global identifier of this advanced publisher.
///
/// This is the identifier a receiver sees as a sample's source, so it is what
/// correlates what this publisher sends with what a subscriber attributes.
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_publisher_get_id(publisher: &AdvancedPublisher) -> EntityGlobalId {
    publisher.id().into()
}

/// Return the key expression on which this advanced publisher publishes.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_publisher_get_key_expr(publisher: &AdvancedPublisher) -> &KeyExpr {
    publisher.key_expr()
}

/// Return whether the advanced publisher currently has matching subscribers.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_publisher_matching_status(publisher: &AdvancedPublisher) -> Result<bool, Error> {
    Ok(publisher.matching_status().wait()?.matching())
}

/// Declare a matching listener that is notified when the publisher's matching
/// status changes.
///
/// The callback receives the new matching status (`true` if matching
/// subscribers exist). The close callback is called when the listener ends.
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_publisher_declare_matching_listener(
    publisher: &AdvancedPublisher,
    callback: impl Fn(bool) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<MatchingListener, Error> {
    let on_close = OnceDrop::new(on_close);
    publisher
        .matching_listener()
        .callback(move |status| {
            let _ = &on_close;
            callback(status.matching());
        })
        .wait()
}

/// Declare a background matching listener that runs until the publisher is
/// undeclared.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_publisher_declare_background_matching_listener(
    publisher: &AdvancedPublisher,
    callback: impl Fn(bool) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    publisher
        .matching_listener()
        .callback(move |status| {
            let _ = &on_close;
            callback(status.matching());
        })
        .background()
        .wait()
}

/// Undeclare the advanced publisher and release its network declaration.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_publisher_undeclare(publisher: AdvancedPublisher) -> Result<(), Error> {
    publisher.undeclare().wait()
}

/// Undeclare a matching listener.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn matching_listener_undeclare(listener: MatchingListener) -> Result<(), Error> {
    listener.undeclare().wait()
}
