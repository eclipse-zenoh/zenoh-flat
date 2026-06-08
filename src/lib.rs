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

pub const PREBINDGEN_OUT_DIR: &str = prebindgen_proc_macro::prebindgen_out_dir!();
pub const FEATURES: &str = prebindgen_proc_macro::features!();
pub const MANIFEST_DIR: &str = prebindgen_proc_macro::manifest_dir!();

pub(crate) mod base;
pub(crate) mod util;

// reexports to make all zenoh-flat API really flat
pub use base::*;

// reexports of zenoh types with Z prefix to distiguish them from zenoh-flat types
// `ZError` is zenoh's native boxed error (`Box<dyn Error + Send + Sync>`), used
// as the `E` of every fallible `z_*` `Result`; `z_error_message` (base/error)
// converts it to a `String` for the JNI error callback.
pub type ZError = zenoh::Error;
pub type ZKeyExpr = zenoh::key_expr::KeyExpr<'static>;
pub type ZConfig = zenoh::Config;
pub type ZZenohId = zenoh::session::ZenohId;
pub type ZHello = zenoh::scouting::Hello;
pub type ZScout = zenoh::scouting::Scout<()>;
pub type ZZBytes = zenoh::bytes::ZBytes;
pub type ZEncoding = zenoh::bytes::Encoding;
pub type ZPublisher = zenoh::pubsub::Publisher<'static>;
pub type ZSubscriber = zenoh::pubsub::Subscriber<()>;
pub type ZQueryable = zenoh::query::Queryable<()>;
pub type ZQuerier = zenoh::query::Querier<'static>;
pub type ZQuery = zenoh::query::Query;
pub type ZSample = zenoh::sample::Sample;
pub type ZReply = zenoh::query::Reply;
pub type ZTimestamp = zenoh::time::Timestamp;
pub type ZSession = zenoh::Session;
pub type ZLivelinessToken = zenoh::liveliness::LivelinessToken;
