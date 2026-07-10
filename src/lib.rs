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
//! # Purpose
//!
//! `zenoh-flat` flattens zenoh's generic, builder-based Rust API into plain free
//! functions over opaque handles. Every public function is annotated with
//! `#[prebindgen]`, so [`prebindgen`] captures this surface and generates
//! idiomatic bindings for other languages (C, Kotlin/JNI, …) — no hand-written
//! FFI layer per target. The surface is **callback-based**: subscribers,
//! queryables, queriers, scouts and liveliness subscribers deliver items through
//! an `impl Fn(..)` callback plus an `on_close` hook (no channels), keeping it
//! trivially FFI-exportable. Fallible calls return `Result<T, `[`Error`]`>`;
//! [`error_get_message`] renders the error message as a `String`.
//!
//! # Structure
//!
//! Types are re-exported under their own zenoh Rust names ([`Session`],
//! [`KeyExpr`], [`ZBytes`], [`Sample`], …). Functions
//! grouped by type in the sources (keyexpr, config, bytes, session, publisher, subscriber,
//! query, sample, scouting, liveliness, time, qos) but exported flatly at the crate root,
//! so the FFI surface is a single namespace.
//!
//! # Naming
//!
//! A function name encodes both its receiver type and its role:
//!
//! - `<type>_<op>` — an operation (`session_put`, `publisher_undeclare`,
//!   `keyexpr_intersects`, `open`).
//! - `<type>_get_<member>` — read a value from an instance, by reference or value
//!   (`sample_get_payload`, `sample_get_kind`, `session_get_zid`).
//! - `<type>_new_<member>` — construct a new instance (`sample_new_put`,
//!   `config_new_default`, `keyexpr_new_try_from`).
//! - `encoding_const_<name>` — a predefined constant ([`Encoding`] presets),
//!   returned as a shared `&'static` borrow (decomposed values come from the
//!   general accessors: [`encoding_get_id`], [`encoding_to_string`]).
//!
//! Conversions keep their verb (`keyexpr_to_string`).
//!
//! # Features
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
// as the `E` of every fallible `Result`; `error_get_message` (base/error) converts
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

pub type ZError = Error;
pub type ZKeyExpr = KeyExpr;
pub type ZConfig = Config;
pub type ZZenohId = ZenohId;
pub type ZHello = Hello;
pub type ZScout = Scout;
pub type ZZBytes = ZBytes;
pub type ZEncoding = Encoding;
pub type ZPublisher = Publisher;
pub type ZSubscriber = Subscriber;
pub type ZQueryable = Queryable;
pub type ZQuerier = Querier;
pub type ZQuery = Query;
pub type ZSample = Sample;
pub type ZReply = Reply;
pub type ZReplyError = ReplyError;
pub type ZTimestamp = Timestamp;
pub type ZSession = Session;
pub type ZLivelinessToken = LivelinessToken;

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
pub use crate::base::publisher::{publisher_get_eid, publisher_get_zid};
#[cfg(feature = "unstable")]
pub use crate::base::qos::reliability::Reliability;
#[cfg(feature = "unstable")]
pub use crate::base::query::querier::{querier_get_eid, querier_get_zid};
#[cfg(feature = "unstable")]
pub use crate::base::query::query_get_accepts_replies;
#[cfg(feature = "unstable")]
pub use crate::base::query::queryable::{queryable_get_eid, queryable_get_zid};
#[cfg(feature = "unstable")]
pub use crate::base::query::reply::{reply_get_replier_eid, reply_get_replier_zid};
#[cfg(feature = "unstable")]
pub use crate::base::sample::{
    sample_get_reliability, sample_get_source_eid, sample_get_source_sn, sample_get_source_zid,
};
#[cfg(feature = "unstable")]
pub use crate::base::subscriber::{subscriber_get_eid, subscriber_get_zid};
pub use crate::base::{
    bytes::{
        encoding::{
            encoding_const_application_cbor, encoding_const_application_cdr,
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
            encoding_to_string,
        },
        zbytes::{zbytes_as_bytes, zbytes_new_clone, zbytes_new_from_slice, zbytes_new_from_vec},
    },
    config::{
        config_get_json, config_insert_json5, config_new_clone, config_new_default,
        config_new_from_file, config_new_from_json, config_new_from_json5, config_new_from_yaml,
        whatami::WhatAmI,
        zenoh_id::{zenoh_id_to_bytes, zenoh_id_to_string},
    },
    error::error_get_message,
    keyexpr::{
        keyexpr_get_str, keyexpr_includes, keyexpr_intersects, keyexpr_new_autocanonize,
        keyexpr_new_clone, keyexpr_new_concat, keyexpr_new_join, keyexpr_new_try_from,
        keyexpr_to_string,
    },
    liveliness::{
        liveliness_declare_subscriber, liveliness_declare_token, liveliness_get,
        liveliness_undeclare_token,
    },
    logger::{init_android_logs, init_zenoh_logs_from_env_or, try_init_zenoh_logs_from_env},
    publisher::{publisher_delete, publisher_get_keyexpr, publisher_put, publisher_undeclare},
    qos::{congestion_control::CongestionControl, priority::Priority},
    query::{
        consolidation_mode::ConsolidationMode,
        querier::{querier_get, querier_get_keyexpr, querier_undeclare},
        query_get_attachment, query_get_encoding, query_get_keyexpr, query_get_parameters,
        query_get_payload, query_reply_delete, query_reply_error, query_reply_sample,
        query_reply_success,
        query_target::QueryTarget,
        queryable::{queryable_get_keyexpr, queryable_undeclare},
        reply::{
            reply_error_get_encoding, reply_error_get_payload, reply_get_err, reply_get_sample,
            reply_is_ok,
        },
        reply_key_expr::ReplyKeyExpr,
    },
    sample::{
        sample_get_attachment, sample_get_congestion_control, sample_get_encoding,
        sample_get_express, sample_get_key_expr, sample_get_kind, sample_get_payload,
        sample_get_priority, sample_get_timestamp, sample_kind::SampleKind, sample_new_delete,
        sample_new_put,
    },
    scouting::{
        hello::{hello_get_locators, hello_get_whatami, hello_get_zid},
        scout::scout,
    },
    session::{
        open, session_close, session_declare_keyexpr, session_declare_publisher,
        session_declare_querier, session_declare_queryable, session_declare_subscriber,
        session_delete, session_get, session_get_peers_zid, session_get_routers_zid,
        session_get_zid, session_is_closed, session_new_timestamp, session_put,
        session_undeclare_keyexpr,
    },
    subscriber::{subscriber_get_keyexpr, subscriber_undeclare},
    time::timestamp::{timestamp_get_id, timestamp_get_ntp64},
};
