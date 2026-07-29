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

//! A language-neutral Zenoh API surface for generated bindings.
//!
//! # Purpose
//!
//! `zenoh-flat` presents Zenoh sessions, publications, subscriptions, queries,
//! scouting, liveliness, and related data types through a consistent set of
//! operations suitable for language bindings. Asynchronous streams report
//! values through callbacks and provide a close callback that signals the end
//! of the stream.
//!
//! # Structure
//!
//! Operations are grouped by their subject: key expressions, configuration,
//! payloads, sessions, publishers, subscribers, queries, samples, scouting,
//! liveliness, time, and delivery quality.
//!
//! # Naming
//!
//! A function name encodes both its receiver type and its role:
//!
//! - `<type>_<op>` — an operation (`session_put`, `publisher_undeclare`,
//!   `keyexpr_intersects`, `open`).
//! - `<type>_get_<member>` — read information from an instance
//!   (`sample_get_payload`, `sample_get_kind`, `session_get_zid`).
//! - `<type>_new_<member>` — construct a new instance (`sample_new_put`,
//!   `config_new_default`, `keyexpr_new_try_from`).
//! - `encoding_const_<name>` — return a predefined [`Encoding`].
//!
//! Conversions keep their verb (`keyexpr_to_string`).
//!
//! # Features
//!
//! The `unstable` feature enables experimental Zenoh capabilities such as
//! reliability selection, entity identifiers, detailed key-expression
//! relations, and sample source information.

/// Directory containing the binding metadata captured while compiling this crate.
pub const PREBINDGEN_OUT_DIR: &str = prebindgen_proc_macro::prebindgen_out_dir!();
/// Features enabled while capturing the binding metadata.
pub const FEATURES: &str = prebindgen_proc_macro::features!();
/// Directory containing this crate's manifest.
pub const MANIFEST_DIR: &str = prebindgen_proc_macro::manifest_dir!();

pub(crate) mod base;
pub(crate) mod util;

// Public names for the Zenoh types used by this API.
/// An error reported by a Zenoh operation.
pub type Error = zenoh::Error;
/// A validated expression that identifies one or more keys.
pub type KeyExpr = zenoh::key_expr::KeyExpr<'static>;
/// Settings used to configure Zenoh operations and sessions.
pub type Config = zenoh::Config;
/// A discovery announcement received while scouting.
pub type Hello = zenoh::scouting::Hello;
/// An active node-discovery operation.
pub type Scout = zenoh::scouting::Scout<()>;
/// A payload or attachment carried by Zenoh.
pub type ZBytes = zenoh::bytes::ZBytes;
/// Format information associated with a payload.
pub type Encoding = zenoh::bytes::Encoding;
/// A reusable declaration for publishing on a key expression.
pub type Publisher = zenoh::pubsub::Publisher<'static>;
/// A subscription that receives matching samples.
pub type Subscriber = zenoh::pubsub::Subscriber<()>;
/// A declaration that receives and answers matching queries.
pub type Queryable = zenoh::query::Queryable<()>;
/// A reusable declaration for sending queries.
pub type Querier = zenoh::query::Querier<'static>;
/// A request received by a queryable.
pub type Query = zenoh::query::Query;
/// A published value or deletion notification.
pub type Sample = zenoh::sample::Sample;
/// A response to a query.
pub type Reply = zenoh::query::Reply;
/// An application error carried by a query reply.
pub type ReplyError = zenoh::query::ReplyError;
/// A connection to the Zenoh network.
pub type Session = zenoh::Session;
/// A declaration that asserts application liveliness.
pub type LivelinessToken = zenoh::liveliness::LivelinessToken;
/// A publisher with advanced features (caching, sample-miss detection,
/// publisher detection). Available only when unstable features are enabled.
#[cfg(feature = "unstable")]
pub type AdvancedPublisher = zenoh_ext::AdvancedPublisher<'static>;
/// A listener notified when the matching status of a publisher or a querier
/// changes.
pub type MatchingListener = zenoh::matching::MatchingListener<()>;
/// A subscription with advanced features (history query, missed-sample
/// recovery, subscriber detection). Available only when unstable features are
/// enabled.
#[cfg(feature = "unstable")]
pub type AdvancedSubscriber = zenoh_ext::AdvancedSubscriber<()>;
/// A listener reporting samples missed from advanced publishers. Available only
/// when unstable features are enabled.
#[cfg(feature = "unstable")]
pub type SampleMissListener = zenoh_ext::SampleMissListener<()>;
/// A transport established to another Zenoh node. Available only when unstable
/// features are enabled.
#[cfg(feature = "unstable")]
pub type Transport = zenoh::session::Transport;
/// A protocol-level connection within a [`Transport`]. Available only when
/// unstable features are enabled.
#[cfg(feature = "unstable")]
pub type Link = zenoh::session::Link;
/// A listener reporting links being added to and removed from a session.
/// Available only when unstable features are enabled.
#[cfg(feature = "unstable")]
pub type LinkEventsListener = zenoh::session::LinkEventsListener<()>;
/// A listener reporting transports being opened and closed by a session.
/// Available only when unstable features are enabled.
#[cfg(feature = "unstable")]
pub type TransportEventsListener = zenoh::session::TransportEventsListener<()>;
/// The interception timestamps a message accumulated along its path. Available
/// only when unstable features are enabled.
#[cfg(feature = "unstable")]
pub type TimestampStack = zenoh::timestamp_stack::TimestampStack;
/// A span of time, used by configuration options such as heartbeat periods.
pub use std::time::Duration;

// ─────────────────────────────────────────────────────────────────────────────
// Public API surface — the single source of truth for what `zenoh-flat` exports.
//
// Every `#[prebindgen]` function and value type is re-exported explicitly here;
// the `base` module tree carries NO glob re-exports. When adding or removing a
// `#[prebindgen]` item, update this list (and only this list).
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "unstable")]
pub use crate::base::advanced_publisher::{
    CacheConfig, MissDetectionConfig, RepliesConfig,
    advanced_publisher_declare_background_matching_listener,
    advanced_publisher_declare_matching_listener, advanced_publisher_delete,
    advanced_publisher_get_id, advanced_publisher_get_key_expr, advanced_publisher_matching_status,
    advanced_publisher_put, advanced_publisher_undeclare, session_declare_advanced_publisher,
};
#[cfg(feature = "unstable")]
pub use crate::base::advanced_subscriber::{
    HistoryConfig, Miss, RecoveryConfig, RecoveryMode,
    advanced_subscriber_declare_background_detect_publishers_subscriber,
    advanced_subscriber_declare_background_sample_miss_listener,
    advanced_subscriber_declare_detect_publishers_subscriber,
    advanced_subscriber_declare_sample_miss_listener, advanced_subscriber_get_id,
    advanced_subscriber_get_key_expr, advanced_subscriber_undeclare,
    sample_miss_listener_undeclare, session_declare_advanced_subscriber,
};
#[cfg(feature = "unstable")]
pub use crate::base::entity::EntityGlobalId;
#[cfg(feature = "unstable")]
pub use crate::base::keyexpr::keyexpr_relation_to;
#[cfg(feature = "unstable")]
pub use crate::base::keyexpr::set_intersection_level::SetIntersectionLevel;
#[cfg(feature = "unstable")]
pub use crate::base::publisher::{publisher_get_id, publisher_get_reliability};
#[cfg(feature = "unstable")]
pub use crate::base::qos::reliability::Reliability;
#[cfg(feature = "unstable")]
pub use crate::base::query::querier::{querier_get_accept_replies, querier_get_id};
#[cfg(feature = "unstable")]
pub use crate::base::query::query_get_timestamp_stack;
#[cfg(feature = "unstable")]
pub use crate::base::query::queryable::queryable_get_id;
#[cfg(feature = "unstable")]
pub use crate::base::query::reply::{reply_error_get_timestamp_stack, reply_get_replier_id};
#[cfg(feature = "unstable")]
pub use crate::base::query::{
    query_get_accepts_replies, query_get_congestion_control, query_get_express, query_get_priority,
    query_get_source_info,
};
#[cfg(feature = "unstable")]
pub use crate::base::sample::{
    sample_get_reliability, sample_get_timestamp_stack,
    source_info::{SourceInfo, sample_get_source_info},
};
#[cfg(all(feature = "unstable", feature = "shared-memory"))]
pub use crate::base::session::info::transport_is_shm;
#[cfg(feature = "unstable")]
pub use crate::base::session::info::{
    LinkEvent, LinkEventKind, LinkStruct, PriorityRange, TransportEvent, TransportEventKind,
    TransportStruct, link_events_listener_undeclare, link_get_auth_identifier, link_get_dst,
    link_get_group, link_get_interfaces, link_get_mtu, link_get_priorities, link_get_reliability,
    link_get_src, link_get_zid, link_is_streamed, link_to_struct,
    session_declare_background_link_events_listener,
    session_declare_background_transport_events_listener, session_declare_link_events_listener,
    session_declare_transport_events_listener, session_get_links, session_get_transports,
    transport_events_listener_undeclare, transport_get_whatami, transport_get_zid,
    transport_is_multicast, transport_is_qos, transport_to_struct,
};
#[cfg(feature = "unstable")]
pub use crate::base::session::session_get_locators;
#[cfg(feature = "unstable")]
pub use crate::base::subscriber::subscriber_get_id;
#[cfg(feature = "unstable")]
pub use crate::base::time::timestamp_stack::{
    InstrumentationTimestamp, InterceptionPoint, TimestampInstrumentation, TimestampStackRecord,
    TimestampStackStruct, timestamp_stack_get_instrumentation, timestamp_stack_get_records,
    timestamp_stack_to_struct,
};
pub use crate::base::{
    bytes::{
        encoding::{
            EncodingStruct, encoding_const_application_cbor, encoding_const_application_cdr,
            encoding_const_application_coap_payload,
            encoding_const_application_java_serialized_object, encoding_const_application_json,
            encoding_const_application_json_patch_json, encoding_const_application_json_seq,
            encoding_const_application_jsonpath, encoding_const_application_jwt,
            encoding_const_application_mp4, encoding_const_application_octet_stream,
            encoding_const_application_openmetrics_text, encoding_const_application_protobuf,
            encoding_const_application_python_serialized_object,
            encoding_const_application_soap_xml, encoding_const_application_sql,
            encoding_const_application_x_www_form_urlencoded, encoding_const_application_xml,
            encoding_const_application_yaml, encoding_const_application_yang,
            encoding_const_audio_aac, encoding_const_audio_flac, encoding_const_audio_mp4,
            encoding_const_audio_ogg, encoding_const_audio_vorbis, encoding_const_image_bmp,
            encoding_const_image_gif, encoding_const_image_jpeg, encoding_const_image_png,
            encoding_const_image_webp, encoding_const_text_css, encoding_const_text_csv,
            encoding_const_text_html, encoding_const_text_javascript, encoding_const_text_json,
            encoding_const_text_json5, encoding_const_text_markdown, encoding_const_text_plain,
            encoding_const_text_xml, encoding_const_text_yaml, encoding_const_video_h261,
            encoding_const_video_h263, encoding_const_video_h264, encoding_const_video_h265,
            encoding_const_video_h266, encoding_const_video_mp4, encoding_const_video_ogg,
            encoding_const_video_raw, encoding_const_video_vp8, encoding_const_video_vp9,
            encoding_const_zenoh_bytes, encoding_const_zenoh_serialized,
            encoding_const_zenoh_string, encoding_get_id, encoding_get_schema, encoding_new_clone,
            encoding_new_from_id, encoding_new_from_string, encoding_new_with_schema,
            encoding_to_string, encoding_to_struct,
        },
        zbytes::{
            zbytes_is_empty, zbytes_len, zbytes_new_clone, zbytes_new_from_slice,
            zbytes_new_from_vec, zbytes_to_bytes, zbytes_try_to_string,
        },
    },
    config::{
        config_get_json, config_insert_json5, config_new_clone, config_new_default,
        config_new_from_file, config_new_from_json5, config_new_from_yaml,
        whatami::WhatAmI,
        zenoh_id::{ZENOH_ID_MAX_SIZE, ZenohId, zenoh_id_to_string},
    },
    error::error_get_message,
    keyexpr::{
        keyexpr_as_str, keyexpr_includes, keyexpr_intersects, keyexpr_new_autocanonize,
        keyexpr_new_clone, keyexpr_new_concat, keyexpr_new_join, keyexpr_new_try_from,
        keyexpr_to_string,
    },
    liveliness::{
        liveliness_declare_subscriber, liveliness_declare_token, liveliness_get,
        liveliness_undeclare_token,
    },
    logger::{init_android_logs, init_zenoh_logs_from_env_or, try_init_zenoh_logs_from_env},
    matching::matching_listener_undeclare,
    publisher::{
        publisher_declare_background_matching_listener, publisher_declare_matching_listener,
        publisher_delete, publisher_get_congestion_control, publisher_get_encoding,
        publisher_get_key_expr, publisher_get_priority, publisher_matching_status, publisher_put,
        publisher_undeclare,
    },
    qos::{congestion_control::CongestionControl, priority::Priority},
    query::{
        consolidation_mode::ConsolidationMode,
        parameters::{
            parameters_contains_key, parameters_extend, parameters_get, parameters_insert,
            parameters_is_well_formed, parameters_remove, parameters_values,
        },
        querier::{
            querier_declare_background_matching_listener, querier_declare_matching_listener,
            querier_get, querier_get_congestion_control, querier_get_key_expr,
            querier_get_priority, querier_matching_status, querier_undeclare,
        },
        query_get_attachment, query_get_encoding, query_get_key_expr, query_get_parameters,
        query_get_payload, query_reply_delete, query_reply_error, query_reply_sample,
        query_reply_success,
        query_target::QueryTarget,
        queryable::{queryable_get_key_expr, queryable_undeclare},
        reply::{
            ReplyErrorStruct, ReplyResult, ReplyStruct, reply_error_get_encoding,
            reply_error_get_payload, reply_error_to_struct, reply_get_err, reply_get_result,
            reply_get_sample, reply_into_struct, reply_is_ok, reply_to_struct,
        },
        reply_key_expr::ReplyKeyExpr,
        selector::Selector,
    },
    sample::{
        SampleStruct, sample_get_attachment, sample_get_congestion_control, sample_get_encoding,
        sample_get_express, sample_get_key_expr, sample_get_kind, sample_get_payload,
        sample_get_priority, sample_get_timestamp, sample_into_struct, sample_kind::SampleKind,
        sample_new_delete, sample_new_put, sample_to_struct,
    },
    scouting::{
        hello::{
            HelloStruct, hello_get_locators, hello_get_whatami, hello_get_zid, hello_to_struct,
        },
        scout::scout,
    },
    session::{
        open, session_close, session_declare_keyexpr, session_declare_publisher,
        session_declare_querier, session_declare_queryable, session_declare_subscriber,
        session_delete, session_get, session_get_peers_zid, session_get_routers_zid,
        session_get_zid, session_is_closed, session_new_timestamp, session_put,
        session_undeclare_keyexpr,
    },
    subscriber::{subscriber_get_key_expr, subscriber_undeclare},
    time::timestamp::Timestamp,
};
