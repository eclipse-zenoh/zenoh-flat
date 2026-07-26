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

/// How an advanced subscriber recovers missed samples.
///
/// The modes are mutually exclusive by construction, mirroring base zenoh's
/// builder, whose type-state makes `periodic_queries` and `heartbeat`
/// unreachable from one another. Because the choice is carried by the type,
/// there is no precedence rule to document and no way to ask for both.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug)]
pub enum RecoveryMode {
    /// Query for not-yet-received samples with this period.
    PeriodicQueries(Duration),
    /// Subscribe to advanced publishers' heartbeats to detect misses.
    Heartbeat,
}

/// Retransmission (missed-sample recovery) configuration for an advanced
/// subscriber.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, Default)]
pub struct RecoveryConfig {
    /// Recovery mode; `None` leaves recovery at its defaults.
    pub mode: Option<RecoveryMode>,
    /// How long a publisher's last-sample state is retained for
    /// retransmission; `None` leaves base's default (one hour).
    ///
    /// This is orthogonal to [`mode`](RecoveryConfig::mode) — base accepts it
    /// with or without a recovery mode — so it is a sibling field rather than a
    /// payload of [`RecoveryMode`].
    pub retention_period: Option<Duration>,
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
        // An exhaustive match, not a precedence chain: every mode reaches base
        // and none can be silently ignored.
        let cfg = match r.mode {
            Some(RecoveryMode::PeriodicQueries(period)) => {
                zenoh_ext::RecoveryConfig::<false>::default().periodic_queries(period)
            }
            Some(RecoveryMode::Heartbeat) => {
                zenoh_ext::RecoveryConfig::<false>::default().heartbeat()
            }
            None => zenoh_ext::RecoveryConfig::default(),
        };
        // Base takes the retention period in any mode state, so it applies to
        // every arm above.
        match r.retention_period {
            Some(period) => cfg.retention_period(period),
            None => cfg,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Render a flat [`RecoveryConfig`] as the base config it converts to.
    fn to_base(mode: Option<RecoveryMode>) -> String {
        to_base_with(mode, None)
    }

    /// As [`to_base`], with a retention period.
    fn to_base_with(mode: Option<RecoveryMode>, retention_period: Option<Duration>) -> String {
        format!(
            "{:?}",
            zenoh_ext::RecoveryConfig::from(RecoveryConfig {
                mode,
                retention_period,
            })
        )
    }

    /// Every recovery mode must arrive at base as that mode, and `None` must
    /// leave base at its defaults. This is the guard against the defect this
    /// type used to have: two independent knobs resolved by precedence, where
    /// setting both silently dropped one. With a sum there is nothing to drop,
    /// and an exhaustive `match` cannot forget a variant — but only a test
    /// proves each variant is wired to the right base builder.
    ///
    /// Base's `RecoveryConfig` has private fields and no `PartialEq`, so its
    /// `Debug` rendering is the oracle.
    #[test]
    fn every_recovery_mode_reaches_base() {
        let period = Duration::from_millis(250);

        assert_eq!(
            to_base(Some(RecoveryMode::PeriodicQueries(period))),
            format!(
                "{:?}",
                zenoh_ext::RecoveryConfig::<false>::default().periodic_queries(period)
            ),
        );
        assert_eq!(
            to_base(Some(RecoveryMode::Heartbeat)),
            format!(
                "{:?}",
                zenoh_ext::RecoveryConfig::<false>::default().heartbeat()
            ),
        );
        assert_eq!(
            to_base(None),
            format!("{:?}", zenoh_ext::RecoveryConfig::<true>::default()),
        );
    }

    /// The retention period reaches base, and does so in **every** mode state —
    /// base accepts it with or without a recovery mode, so it must not be
    /// dropped by whichever arm the mode match takes.
    #[test]
    fn retention_period_reaches_base_in_every_mode() {
        let period = Duration::from_secs(120);
        for mode in [
            None,
            Some(RecoveryMode::Heartbeat),
            Some(RecoveryMode::PeriodicQueries(Duration::from_millis(250))),
        ] {
            let with = to_base_with(mode.clone(), Some(period));
            let without = to_base_with(mode.clone(), None);
            assert_ne!(
                with, without,
                "retention period was dropped for mode {mode:?}"
            );
        }
    }

    /// Setting the retention period leaves the mode alone, and vice versa: the
    /// two are orthogonal in base, so neither may overwrite the other.
    #[test]
    fn retention_period_and_mode_are_independent() {
        let period = Duration::from_secs(120);
        let query = Duration::from_millis(250);

        // Same mode, different retention -> differs only by retention.
        assert_ne!(
            to_base_with(Some(RecoveryMode::Heartbeat), Some(period)),
            to_base_with(Some(RecoveryMode::Heartbeat), None)
        );
        // Same retention, different mode -> differs only by mode.
        assert_ne!(
            to_base_with(Some(RecoveryMode::Heartbeat), Some(period)),
            to_base_with(Some(RecoveryMode::PeriodicQueries(query)), Some(period))
        );
        // A retention period does not turn "no mode" into a mode.
        assert_eq!(
            to_base_with(None, Some(period)),
            format!(
                "{:?}",
                zenoh_ext::RecoveryConfig::<true>::default().retention_period(period)
            )
        );
    }

    /// The three outcomes above are actually distinguishable. Without this, the
    /// assertions in [`every_recovery_mode_reaches_base`] would still pass if
    /// every mode collapsed onto one base config — exactly the silent-loss bug
    /// they exist to catch.
    #[test]
    fn recovery_modes_are_distinguishable() {
        let a = to_base(Some(RecoveryMode::PeriodicQueries(Duration::from_millis(
            250,
        ))));
        let b = to_base(Some(RecoveryMode::Heartbeat));
        let c = to_base(None);
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }
}
