use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;
use zenoh_ext::AdvancedSubscriberBuilderExt;

use crate::{
    AdvancedSubscriber, Duration, EntityGlobalId, Error, KeyExpr, Sample, SampleMissListener,
    Session, Subscriber, util::OnceDrop,
};

/// Query configuration for an advanced subscriber's historical data.
///
/// History can only be retransmitted by advanced publishers that enable a
/// [`crate::CacheConfig`]; late-joiner detection additionally requires their
/// publisher detection.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, Default)]
pub struct HistoryConfig {
    /// Detect late-joiner publishers and query their historical data.
    pub detect_late_publishers: bool,
    /// How many samples to query for each resource; `None` leaves it unbounded.
    pub max_samples: Option<u64>,
    /// Maximum age, in seconds, of the samples to query; `None` leaves it
    /// unbounded.
    pub max_age: Option<f64>,
}

/// Retransmission (missed-sample recovery) configuration for an advanced
/// subscriber.
///
/// At most one recovery mode applies: a `periodic_queries` period, or
/// `heartbeat` subscription. If neither is set, recovery uses its defaults.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, Default)]
pub struct RecoveryConfig {
    /// Period of the queries for not-yet-received samples; takes precedence over
    /// `heartbeat`.
    pub periodic_queries: Option<Duration>,
    /// Subscribe to advanced publishers' heartbeats to detect misses.
    pub heartbeat: bool,
}

/// A report of samples missed from one source, delivered to a sample-miss
/// listener.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug)]
pub struct Miss {
    /// Source of the missed samples.
    pub source: EntityGlobalId,
    /// Number of missed samples.
    pub nb: u32,
}

impl From<HistoryConfig> for zenoh_ext::HistoryConfig {
    fn from(h: HistoryConfig) -> Self {
        let mut cfg = zenoh_ext::HistoryConfig::default();
        if h.detect_late_publishers {
            cfg = cfg.detect_late_publishers();
        }
        if let Some(n) = h.max_samples {
            cfg = cfg.max_samples(n as usize);
        }
        if let Some(secs) = h.max_age {
            cfg = cfg.max_age(secs);
        }
        cfg
    }
}

impl From<RecoveryConfig> for zenoh_ext::RecoveryConfig {
    fn from(r: RecoveryConfig) -> Self {
        if let Some(period) = r.periodic_queries {
            zenoh_ext::RecoveryConfig::<false>::default().periodic_queries(period)
        } else if r.heartbeat {
            zenoh_ext::RecoveryConfig::<false>::default().heartbeat()
        } else {
            zenoh_ext::RecoveryConfig::default()
        }
    }
}

impl From<zenoh_ext::Miss> for Miss {
    fn from(m: zenoh_ext::Miss) -> Self {
        Miss {
            source: m.source().into(),
            nb: m.nb(),
        }
    }
}

/// Declare an advanced subscriber for the given key expression.
///
/// On top of a regular subscriber, an advanced subscriber can query historical
/// data (see [`HistoryConfig`]), recover missed samples (see [`RecoveryConfig`]),
/// and be discovered by advanced publishers (`subscriber_detection`). The
/// callback is called for each sample; the close callback is called when the
/// subscription ends. `query_timeout` bounds the history/retransmission queries.
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[allow(clippy::too_many_arguments)]
pub fn session_declare_advanced_subscriber(
    session: &Session,
    key_expr: KeyExpr,
    callback: impl Fn(Sample) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
    history: Option<HistoryConfig>,
    recovery: Option<RecoveryConfig>,
    query_timeout: Option<Duration>,
    subscriber_detection: Option<bool>,
) -> Result<AdvancedSubscriber, Error> {
    let on_close = OnceDrop::new(on_close);
    let mut builder = session.declare_subscriber(key_expr).advanced();
    if let Some(h) = history {
        builder = builder.history(h.into());
    }
    if let Some(r) = recovery {
        builder = builder.recovery(r.into());
    }
    if let Some(t) = query_timeout {
        builder = builder.query_timeout(t);
    }
    if subscriber_detection == Some(true) {
        builder = builder.subscriber_detection();
    }
    builder
        .callback(move |sample| {
            let _ = &on_close;
            callback(sample);
        })
        .wait()
}

/// Declare a sample-miss listener that reports samples missed from advanced
/// publishers with sample-miss detection enabled.
///
/// The callback is called for each miss; the close callback is called when the
/// listener ends. Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_subscriber_declare_sample_miss_listener(
    subscriber: &AdvancedSubscriber,
    callback: impl Fn(Miss) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<SampleMissListener, Error> {
    let on_close = OnceDrop::new(on_close);
    subscriber
        .sample_miss_listener()
        .callback(move |miss| {
            let _ = &on_close;
            callback(miss.into());
        })
        .wait()
}

/// Declare a background sample-miss listener that runs until the subscriber is
/// undeclared.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_subscriber_declare_background_sample_miss_listener(
    subscriber: &AdvancedSubscriber,
    callback: impl Fn(Miss) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    subscriber
        .sample_miss_listener()
        .callback(move |miss| {
            let _ = &on_close;
            callback(miss.into());
        })
        .background()
        .wait()
}

/// Declare a subscriber that detects the advanced publishers matching this
/// advanced subscriber's key expression.
///
/// Only advanced publishers that enable publisher detection are detected. When
/// `history` is set, already-present publishers are reported too. The callback
/// receives a sample per detection event; the close callback is called when the
/// subscription ends. Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_subscriber_declare_detect_publishers_subscriber(
    subscriber: &AdvancedSubscriber,
    callback: impl Fn(Sample) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
    history: Option<bool>,
) -> Result<Subscriber, Error> {
    let on_close = OnceDrop::new(on_close);
    let mut builder = subscriber.detect_publishers();
    if history == Some(true) {
        builder = builder.history(true);
    }
    builder
        .callback(move |sample| {
            let _ = &on_close;
            callback(sample);
        })
        .wait()
}

/// Declare a background detect-publishers subscriber that runs until the
/// advanced subscriber is undeclared.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_subscriber_declare_background_detect_publishers_subscriber(
    subscriber: &AdvancedSubscriber,
    callback: impl Fn(Sample) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
    history: Option<bool>,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    let mut builder = subscriber.detect_publishers();
    if history == Some(true) {
        builder = builder.history(true);
    }
    builder
        .callback(move |sample| {
            let _ = &on_close;
            callback(sample);
        })
        .background()
        .wait()
}

/// Return the key expression this advanced subscriber matches.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_subscriber_get_key_expr(subscriber: &AdvancedSubscriber) -> &KeyExpr {
    subscriber.key_expr()
}

/// Undeclare the advanced subscriber.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn advanced_subscriber_undeclare(subscriber: AdvancedSubscriber) -> Result<(), Error> {
    subscriber.undeclare().wait()
}

/// Undeclare a sample-miss listener.
///
/// Available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_miss_listener_undeclare(listener: SampleMissListener) -> Result<(), Error> {
    listener.undeclare().wait()
}
