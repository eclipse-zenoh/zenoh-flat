# zenoh-flat

`zenoh-flat` is a flat, FFI-friendly facade over the [`zenoh`](https://github.com/eclipse-zenoh/zenoh)
crate. It re-exports the zenoh types it operates on under their own Rust names
(`Session`, `KeyExpr`, `ZBytes`, `Sample`, `Config`, …) and exposes the entire
API as free functions whose names mirror the type they act on:

```rust
let config = config_new_default();
let session = open(config)?;
let ke = keyexpr_new_try_from("demo/example".to_string())?;
session_put(&session, &ke, zbytes_new_from_slice(b"hello"), /* … */)?;
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
- **Opaque handles, values, and twins.** See [Type representation](#type-representation):
  resource types (`Session`, `Publisher`, …) cross as opaque handles, pure data
  (`EntityGlobalId`, the `*Config` inputs) crosses by value, and payload-carrying types
  (`Sample`, `Reply`, `ReplyError`) offer both.

## Type representation

zenoh-flat presents every zenoh concept in one of **three shapes**. Picking the right shape for a
type is the central design decision in this crate, so the rules are written out here. (Some shapes
are already in place; others are being rolled out under these rules.)

### The three shapes

- **Handle** — a live object with identity, or a resource that must be released: a session, a
  publisher, a subscription, a key expression, a byte buffer. A handle crosses the boundary as an
  opaque pointer; you read its parts through `<type>_get_<field>` accessor functions, and you are
  responsible for closing or freeing it. Examples: `Session`, `Publisher`, `Subscriber`, `KeyExpr`,
  `ZBytes`, `Encoding`.
- **Value** — plain data with no identity and nothing to release: an entity id, a report, a small
  configuration. A value is an ordinary Rust `struct` with public fields and crosses by copying
  them. Examples: `EntityGlobalId { zid, eid }`, `Miss { source, nb }`, and the input configs
  (`HistoryConfig`, `RecoveryConfig`, …).
- **Twin** — a type worth having *both* ways. It holds real data you may want as a plain value, but
  it also carries an unbounded payload you may not want to copy on every access. Such a type gets a
  handle *and* a value form: the handle keeps zenoh's name (e.g. `Sample`) and the value form adds a
  `Struct` suffix (`SampleStruct`), reached with a `<type>_to_struct` accessor. The payload-carrying
  types are `Sample`, `Reply`, and `ReplyError` (each carries an unbounded `ZBytes`, directly or via
  the sample/error it wraps).

### Choosing a shape

Ask: **does an opaque handle earn its keep?**

- Owns a resource, or is a live network object → **Handle**.
- Small, pure data → **Value**.
- Data *and* an unbounded payload (a `ZBytes`) → **Twin**, so a caller can read one field cheaply
  off the handle *or* take the whole thing as a value, whichever suits them.

Add the value form only when it pays off: a tiny pure-data type gains nothing from a handle, and a
live resource has no meaningful value form. Don't manufacture a second shape just for symmetry.

### Naming

- Re-exported zenoh types keep zenoh's own name: `Sample`, `KeyExpr`, `ZBytes`, `ZenohId`.
- The `Struct` suffix exists *only* to tell a value apart from a same-named handle (`Sample` →
  `SampleStruct`). A value with no handle uses the plain name (`EntityGlobalId`, `Miss`).
- Functions are `<type>_<verb>`: `sample_get_payload`, `sample_new_put`, `keyexpr_new_try_from`.

### Be faithful to zenoh — the most important rule

A value form must mirror zenoh **exactly**: the same field types and the same optionality,
expressed with ordinary Rust types (`u32`, `Option<u32>`), never a wire- or binding-specific type.

- **Never fake "unknown" with a sentinel.** If zenoh returns an `Option`, flat returns an `Option`.
  For a sample's source, "no source information at all" and "source entity id is `0`" are different
  facts and must stay different — `sample_get_source_eid` returns `Option<u32>`, not a `u32` with
  `0` meaning absent. (Collapsing the two was the bug in
  [issue #10](https://github.com/ZettaScaleLabs/zenoh-flat/issues/10).)
- **Put the optionality on the right edge.** When a sample's source is known, its id and sequence
  number always exist; only the *whole* source-info is optional. So those fields are non-optional
  and the parent carries the `Option`, not a struct full of `Option` fields.

### One source of truth per field

A grouped accessor and flat shortcuts may coexist — that is encouraged. But a shortcut must
**delegate** to the same underlying path, not carry its own copy of the logic: two hand-written
bodies reading the same field eventually disagree (that is how #10 slipped in). For an optional
nested value, also expose a `<path>_defined -> bool` flag, so a caller reading only flat fields can
still tell present from absent.

### Construction mirrors zenoh

flat exposes only the constructors zenoh actually provides (`sample_new_put`,
`keyexpr_new_try_from`, …); it never invents a "build the whole struct from its fields"
constructor. If zenoh gives no public way to build a type — the things you only ever *receive*,
such as a `Reply` — flat offers no constructor either, and the type is read-only.

### Bindings choose; flat stays neutral

flat offers the menu — handle, value, or both — and each language binding selects the forms that
suit it. flat itself never names a target language or wire detail (C, JNI, Kotlin, pointer widths,
…); turning these shapes into a concrete ABI is the binding generator's job.

## Layout

All API lives under `src/base/`, grouped by area (`keyexpr`, `config`, `bytes`,
`session`, `publisher`, `subscriber`, `query`, `sample`, `scouting`,
`liveliness`, `time`, `qos`) and re-exported flat from `zenoh_flat::*`.

## Features

Feature flags forward to `zenoh` and mirror zenoh-c's defaults. `unstable`
additionally enables the `#[unstable]` slices of the API (`Reliability`,
entity-id accessors, key-expression relations, sample source info); it is **off**
by default.
