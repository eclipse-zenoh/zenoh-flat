# zenoh-flat

`zenoh-flat` is a flat, FFI-friendly facade over the [`zenoh`](https://github.com/eclipse-zenoh/zenoh)
crate. It re-exports the zenoh types it operates on under their own Rust names
(`Session`, `KeyExpr`, `ZBytes`, `Sample`, `Config`, …) and exposes the entire
API as free functions whose names mirror the type they act on:

```rust
let config = config_default();
let session = open(config)?;
let ke = keyexpr_try_from("demo/example".to_string())?;
session_put(&session, &ke, bytes_from_slice(b"hello"), /* … */)?;
```

Every public function is annotated with `#[prebindgen]`, so
[`prebindgen`](../prebindgen) captures this surface and generates idiomatic
bindings for other languages (C via `lang::Cbindgen`, Kotlin/JNI via
`lang::JniGen`, …) — no hand-written FFI layer per target language.

## Conventions

- **Naming follows zenoh's Rust names, not zenoh-c.** Functions are
  `<type>_<verb>` (`session_declare_publisher`, `publisher_undeclare`,
  `keyexpr_intersects`); type aliases keep zenoh's own identifier
  (`ZBytes`, `ZenohId`).
- **Callback-based, no channels.** Subscribers, queryables, queriers, scouts and
  liveliness subscribers deliver items through an `impl Fn(..)` callback plus an
  `on_close` hook. This keeps the surface trivially FFI-exportable.
- **Errors as `Result<T, Error>`.** `Error` is zenoh's boxed error; the
  `error_message` accessor renders it to a `String` for callers that cannot carry
  a Rust error across the boundary.
- **Opaque handles vs. value types.** Handle types (`Session`, `Publisher`,
  `Sample`, …) cross as opaque pointers; enums and small structs cross by value.

## Layout

All API lives under `src/base/`, grouped by area (`keyexpr`, `config`, `bytes`,
`session`, `publisher`, `subscriber`, `query`, `sample`, `scouting`,
`liveliness`, `time`, `qos`) and re-exported flat from `zenoh_flat::*`.

## Features

Feature flags forward to `zenoh` and mirror zenoh-c's defaults. `unstable`
additionally enables the `#[unstable]` slices of the API (`Reliability`,
entity-id accessors, key-expression relations, sample source info); it is **off**
by default.
