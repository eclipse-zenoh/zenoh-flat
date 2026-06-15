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

//! Flat, FFI-friendly facade over the [`zenoh`] crate.
//!
//! `zenoh-flat` re-exports the zenoh types this crate operates on under their own
//! Rust names (e.g. [`Session`], [`KeyExpr`], [`ZBytes`], [`Sample`]) and exposes
//! the whole API as free functions whose names mirror the type they act on
//! (`session_put`, `keyexpr_intersects`, `publisher_undeclare`, …). Every public
//! function is annotated with `#[prebindgen]`, so [`prebindgen`] can capture this
//! surface and generate idiomatic bindings for other languages (C, Kotlin/JNI, …)
//! without a hand-written FFI layer per target.
//!
//! The surface is **callback-based**: subscribers, queryables, queriers, scouts,
//! and liveliness subscribers deliver their items through `impl Fn(..)` callbacks
//! plus an `on_close` hook, rather than channels. Fallible operations return
//! `Result<T, `[`Error`]`>`; [`error_message`] renders the error for callers that
//! cannot carry a Rust error across the boundary.
//!
//! Feature flags forward to `zenoh`; `unstable` additionally enables the
//! `#[unstable]` slices of the API (`Reliability`, entity-id accessors, key
//! expression relations, sample source info).

pub const PREBINDGEN_OUT_DIR: &str = prebindgen_proc_macro::prebindgen_out_dir!();
pub const FEATURES: &str = prebindgen_proc_macro::features!();
pub const MANIFEST_DIR: &str = prebindgen_proc_macro::manifest_dir!();

pub(crate) mod base;
pub(crate) mod util;

// Flat re-exports of the zenoh types this crate's functions operate on. Each
// alias keeps the underlying zenoh Rust identifier (e.g. `ZBytes`, `ZenohId`)
// so the captured FFI surface mirrors zenoh's own names one-to-one.
//
// `Error` is zenoh's native boxed error (`Box<dyn Error + Send + Sync>`), used
// as the `E` of every fallible `Result`; `error_message` (base/error) converts
// it to a `String` for the JNI error callback.
pub type Error = zenoh::Error;
pub type KeyExpr = zenoh::key_expr::KeyExpr<'static>;
pub type Config = zenoh::Config;
pub type ZenohId = zenoh::session::ZenohId;
pub type Hello = zenoh::scouting::Hello;
pub type Scout = zenoh::scouting::Scout<()>;
pub type ZBytes = zenoh::bytes::ZBytes;
pub type Encoding = zenoh::bytes::Encoding;
pub type Publisher = zenoh::pubsub::Publisher<'static>;
pub type Subscriber = zenoh::pubsub::Subscriber<()>;
pub type Queryable = zenoh::query::Queryable<()>;
pub type Querier = zenoh::query::Querier<'static>;
pub type Query = zenoh::query::Query;
pub type Sample = zenoh::sample::Sample;
pub type Reply = zenoh::query::Reply;
pub type ReplyError = zenoh::query::ReplyError;
pub type Timestamp = zenoh::time::Timestamp;
pub type Session = zenoh::Session;
pub type LivelinessToken = zenoh::liveliness::LivelinessToken;

// ─────────────────────────────────────────────────────────────────────────────
// Public API surface — the single source of truth for what `zenoh-flat` exports.
//
// Every `#[prebindgen]` function and value type is re-exported explicitly here;
// the `base` module tree carries NO glob re-exports. When adding or removing a
// `#[prebindgen]` item, update this list (and only this list).
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "unstable")]
pub use crate::base::keyexpr::keyexpr_relation_to;
#[cfg(feature = "unstable")]
pub use crate::base::keyexpr::set_intersection_level::SetIntersectionLevel;
#[cfg(feature = "unstable")]
pub use crate::base::publisher::{publisher_eid, publisher_zid};
#[cfg(feature = "unstable")]
pub use crate::base::qos::reliability::Reliability;
#[cfg(feature = "unstable")]
pub use crate::base::query::querier::{querier_eid, querier_zid};
#[cfg(feature = "unstable")]
pub use crate::base::query::query_accepts_replies;
#[cfg(feature = "unstable")]
pub use crate::base::query::queryable::{queryable_eid, queryable_zid};
#[cfg(feature = "unstable")]
pub use crate::base::query::reply::{reply_replier_eid, reply_replier_zid};
#[cfg(feature = "unstable")]
pub use crate::base::sample::{
    sample_reliability, sample_source_eid, sample_source_sn, sample_source_zid,
};
#[cfg(feature = "unstable")]
pub use crate::base::subscriber::{subscriber_eid, subscriber_zid};
pub use crate::base::{
    bytes::{
        encoding::{
            encoding_application_cbor, encoding_application_cdr, encoding_application_coap_payload,
            encoding_application_java_serialized_object, encoding_application_json,
            encoding_application_json_patch_json, encoding_application_json_seq,
            encoding_application_jsonpath, encoding_application_jwt, encoding_application_mp4,
            encoding_application_octet_stream, encoding_application_openmetrics_text,
            encoding_application_protobuf, encoding_application_python_serialized_object,
            encoding_application_soap_xml, encoding_application_sql,
            encoding_application_x_www_form_urlencoded, encoding_application_xml,
            encoding_application_yaml, encoding_application_yang, encoding_audio_aac,
            encoding_audio_flac, encoding_audio_mp4, encoding_audio_ogg, encoding_audio_vorbis,
            encoding_clone, encoding_from_id, encoding_from_string, encoding_id,
            encoding_image_bmp, encoding_image_gif, encoding_image_jpeg, encoding_image_png,
            encoding_image_webp, encoding_schema, encoding_text_css, encoding_text_csv,
            encoding_text_html, encoding_text_javascript, encoding_text_json, encoding_text_json5,
            encoding_text_markdown, encoding_text_plain, encoding_text_xml, encoding_text_yaml,
            encoding_to_string, encoding_video_h261, encoding_video_h263, encoding_video_h264,
            encoding_video_h265, encoding_video_h266, encoding_video_mp4, encoding_video_ogg,
            encoding_video_raw, encoding_video_vp8, encoding_video_vp9, encoding_with_schema,
            encoding_zenoh_bytes, encoding_zenoh_serialized, encoding_zenoh_string,
        },
        zbytes::{
            zbytes_as_bytes, zbytes_clone, zbytes_from_slice, zbytes_from_vec, zbytes_to_bytes,
        },
    },
    config::{
        config_clone, config_default, config_from_file, config_from_json, config_from_json5,
        config_from_yaml, config_get_json, config_insert_json5,
        whatami::WhatAmI,
        zenoh_id::{zenoh_id_to_bytes, zenoh_id_to_string},
    },
    error::error_message,
    keyexpr::{
        keyexpr_as_str, keyexpr_autocanonize, keyexpr_clone, keyexpr_concat, keyexpr_includes,
        keyexpr_intersects, keyexpr_join, keyexpr_to_string, keyexpr_try_from,
    },
    liveliness::{
        liveliness_declare_subscriber, liveliness_declare_token, liveliness_get,
        liveliness_undeclare_token,
    },
    logger::{init_android_logs, init_zenoh_logs_from_env_or, try_init_zenoh_logs_from_env},
    publisher::{publisher_delete, publisher_keyexpr, publisher_put, publisher_undeclare},
    qos::{congestion_control::CongestionControl, priority::Priority},
    query::{
        consolidation_mode::ConsolidationMode,
        querier::{querier_get, querier_keyexpr, querier_undeclare},
        query_attachment, query_encoding, query_keyexpr, query_parameters, query_payload,
        query_reply_delete, query_reply_error, query_reply_sample, query_reply_success,
        query_target::QueryTarget,
        queryable::{queryable_keyexpr, queryable_undeclare},
        reply::{reply_err, reply_error_encoding, reply_error_payload, reply_is_ok, reply_sample},
        reply_key_expr::ReplyKeyExpr,
    },
    sample::{
        sample_attachment, sample_congestion_control, sample_delete, sample_encoding,
        sample_express, sample_key_expr, sample_kind, sample_kind::SampleKind, sample_payload,
        sample_priority, sample_put, sample_timestamp,
    },
    scouting::{
        hello::{hello_locators, hello_whatami, hello_zid},
        scout::scout,
    },
    session::{
        open, session_close, session_declare_keyexpr, session_declare_publisher,
        session_declare_querier, session_declare_queryable, session_declare_subscriber,
        session_delete, session_get, session_is_closed, session_new_timestamp, session_peers_zid,
        session_put, session_routers_zid, session_undeclare_keyexpr, session_zid,
    },
    subscriber::{subscriber_keyexpr, subscriber_undeclare},
    time::timestamp::{timestamp_id, timestamp_ntp64},
};
