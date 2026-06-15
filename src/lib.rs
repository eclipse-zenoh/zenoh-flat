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

// reexports to make all zenoh-flat API really flat
pub use base::*;

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
