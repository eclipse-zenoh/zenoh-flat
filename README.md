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

- **Naming follows zenoh's Rust names.** Functions are
  `<type>_<verb>` (`session_declare_publisher`, `publisher_undeclare`,
  `keyexpr_intersects`); type aliases keep zenoh's own identifier
  (`ZBytes`, `Sample`).
- **Callback-based, no channels.** Subscribers, queryables, queriers, scouts and
  liveliness subscribers deliver items through an `impl Fn(..)` callback plus an
  `on_close` hook. This keeps the surface trivially FFI-exportable.
- **Errors as `Result<T, Error>`.** `Error` is zenoh's boxed error; the
  `error_get_message` accessor renders it to a `String` for callers that cannot
  carry a Rust error across the boundary.
- **Opaque handles, values, and twins.** See [Type representation](#type-representation):
  resource types (`Session`, `Publisher`, …) cross as opaque handles, pure data
  (`EntityGlobalId`, `Timestamp`, the `*Config` inputs) crosses by value, and
  payload-carrying types (`Sample`, `Encoding`, `Hello`, …) offer both.

## Type representation

zenoh-flat presents every zenoh concept in one of **three shapes**. Picking the right shape for a
type is the central design decision in this crate, so the rules are written out here.

### The three shapes

- **Handle** — a live object with identity, or a resource that must be released: a session, a
  publisher, a subscription, a key expression, a byte buffer. A handle crosses the boundary as an
  opaque pointer; you read its parts through `<type>_get_<field>` accessor functions, and you are
  responsible for closing or freeing it. Examples: `Session`, `Publisher`, `Subscriber`, `KeyExpr`,
  `ZBytes`.
- **Value** — plain data with no identity and nothing to release: an entity id, a report, a small
  configuration. A value is an ordinary Rust `struct` with public fields and crosses by copying
  them. Examples: `ZenohId`, `EntityGlobalId { zid, eid }`, `SourceInfo { source, sn }`,
  `Timestamp`, and the input configs (`HistoryConfig`, `RecoveryConfig`, …).
- **Twin** — a type worth having *both* ways. It is fully described by its fields (so it can be a
  value), but it also carries a **payload** — an unbounded string, list, or `ZBytes` — you may not
  want to copy on every access (so a handle lets you read the cheap fields without materializing it).
  Such a type gets a handle *and* a value form: the handle keeps zenoh's name (`Sample`, `Encoding`)
  and the value form adds a `Struct` suffix (`SampleStruct`, `EncodingStruct`), reached with a
  `<type>_to_struct` accessor. Examples: `Sample`, `Reply`, `ReplyError`, `Hello`, `Encoding`.

### Choosing a shape

Two independent, objective questions — no "is it big or common enough?" guesswork:

- **Give it a handle?** Yes if the type is a **live resource** (a session, a subscription —
  something with identity or a lifecycle, not just readable data), **or** if it has a **payload
  field**: a `String`, a list, a `ZBytes`, or anything containing one. Across a language boundary
  such a field is not free — every language must reallocate and re-encode it (a Rust `String`
  becomes a Java `String`, a C `char*`, …), so a handle lets a caller read the cheap fields
  (`Encoding`'s id) without paying to materialize the payload. This is an *output* concern: a type
  you only ever *build and pass in*, like `Selector`, never lazily reads, so its string fields don't
  force a handle.
- **Give it a value form?** Yes if the type is **fully defined by its readable fields** — a data
  snapshot, not an opaque resource.

A type can answer **yes to both** — that is exactly what a **twin** is (`Sample`, `Encoding`,
`Hello`): a handle *and* a value form, no separate decision to make. A type of only cheap, fixed-size
fields (an id, a timestamp) answers no to the handle question and is **value-only**. A live resource
answers no to the value question and is **handle-only**.

A **bounded**, fixed-maximum blob — a 16-byte node id — counts as cheap, not a payload: copying it
whole is trivial. Only *unbounded* data (arbitrary-length strings and lists) is a materialization
cost. So a `ZenohId` (≤16 bytes) and a `Timestamp` (a `u64` plus a ≤16-byte id) are value-only,
while an `Encoding` (which carries an arbitrary-length schema) is a twin.

Where a field is bounded, **say so in the type**, not in a comment: `ZenohId.bytes` is a
`[u8; ZENOH_ID_MAX_SIZE]`, so the bound is a fact a reader and a generator can both see, and reading
an id allocates nothing. A `Vec<u8>` would be indistinguishable from an arbitrary-length list and
would put the bound in prose only — which is why `Timestamp.id`, still spelled that way, is the
remaining exception rather than the pattern to copy.

### Sums: mutually exclusive alternatives are one enum

When zenoh says a thing is *one of* several alternatives, flat says so with a **single enum whose
variants carry their payloads** — never with parallel `Option` fields, and never with independent
fields resolved by a precedence rule.

Parallel `Option`s make invalid states representable and demote the invariant to a doc comment:
two `Option`s express four states where only two are legal, so "both set" and "neither set" become
things a caller can build and a consumer must guess about. A precedence chain is the same defect
with a silent resolution — the loser is ignored without an error. An enum removes the question:
the exclusivity is carried by the type, the conversion to base becomes an exhaustive `match`, and
no invariant has to be documented because none can be broken.

- `Reply::result()` is a `Result<&Sample, &ReplyError>`, so `ReplyStruct` carries one
  `result: ReplyResult` with `Sample(..)` / `Error(..)` variants — not a `sample` and an `error`
  `Option` alongside each other.
- `zenoh_ext::RecoveryConfig`'s builder type-state makes `periodic_queries` and `heartbeat`
  unreachable from one another, so flat's `RecoveryConfig` carries one
  `mode: Option<RecoveryMode>` — not a period and a flag with the period winning.

Two details follow from the rule rather than from taste:

- **The `Option` and the choice stay separate.** "Which alternative" and "is there one at all" are
  independent facts, so an absent choice is `Option<Mode>`, never an extra `None`-ish variant
  smuggled into the enum's own domain.
- **A payload of more than one part uses named fields** (`Variant { a, b }`), not a positional
  tuple; a single-part payload may be a tuple variant (`PeriodicQueries(Duration)`).

A field that is genuinely optional-and-independent of the others stays its own `Option` — this rule
is about *alternatives*, not about every group of optional fields. And a `Result<T, E>` field is
not the way to spell a domain sum: in the binding generators `Result` is the error channel, so a
domain sum is always a named enum.

### Naming

- Re-exported zenoh types keep zenoh's own name: `Sample`, `KeyExpr`, `ZBytes`. A type converted
  to a flat value keeps it too (`ZenohId`, `Timestamp`).
- The `Struct` suffix exists *only* to tell a value apart from a same-named handle (`Sample` →
  `SampleStruct`). A value with no handle uses the plain name (`EntityGlobalId`, `Miss`).
- Functions are `<type>_<verb>`: `sample_get_payload`, `sample_new_put`, `keyexpr_new_try_from`.
- **The `<verb>` mirrors zenoh's own method name**, so a flat name can be translated back by
  inspection: `ZBytes::to_bytes` → `zbytes_to_bytes`, `keyexpr::as_str` → `keyexpr_as_str`,
  `Sample::key_expr` →
  `sample_get_key_expr`. The same holds for value-struct fields: `CacheConfig::replies_config`
  keeps zenoh's field name. Where zenoh spells a name as one word, so does flat
  (`Session::declare_keyexpr` → `session_declare_keyexpr`).

### Be faithful to zenoh — the most important rule

A value form must mirror zenoh **exactly**: the same field types and the same optionality,
expressed with ordinary Rust types (`u32`, `Option<u32>`), never a wire- or binding-specific type.

- **Never fake "unknown" with a sentinel.** If zenoh returns an `Option`, flat returns an `Option`.
  For a sample's source, "no source information at all" and "the source's fields happen to be `0`"
  are different facts and must stay different — `sample_get_source_info` returns `Option<SourceInfo>`
  (absent ⇒ `None`), never a `SourceInfo` with zeroed fields.
- **Put the optionality on the right edge.** When a sample's source is known, its entity id and
  sequence number always exist; only the *whole* source-info is optional. So those fields are
  non-optional and the parent carries the `Option`, not a struct full of `Option` fields.

### One source of truth per field

Each field is read one way: through the value, or a grouped accessor that returns it. A convenience
shortcut for a nested field may be added, but it must **delegate** to that same path rather than
re-deriving the value — two independent bodies reading the same field eventually disagree.

### Construction mirrors zenoh

flat exposes only the constructors zenoh actually provides (`sample_new_put`,
`keyexpr_new_try_from`, …); it never invents a "build the whole struct from its fields"
constructor. If zenoh gives no public way to build a type — the things you only ever *receive*,
such as a `Reply` — flat offers no constructor either, and the type is read-only.

### Bindings choose; flat stays neutral

flat offers the menu — handle, value, or both — and each language binding selects the forms that
suit it. flat itself never names a target language or wire detail (C, JNI, Kotlin, pointer widths,
…); turning these shapes into a concrete ABI is the binding generator's job.

This is why **enums carry no `#[repr]`**. An integer width is a wire detail, so the choice belongs
to the adapter, which already declares its own representation. The one exception is an enum whose
discriminants *are* data rather than an ordinal sequence — `WhatAmI`, whose values are bit flags
combined into the bitfield `scout` takes. There the numbers are part of the type's meaning, so the
`repr` stays.

## Layout

All API lives under `src/base/`, grouped by area (`keyexpr`, `config`, `bytes`,
`session`, `publisher`, `subscriber`, `query`, `sample`, `scouting`,
`liveliness`, `time`, `qos`) and re-exported flat from `zenoh_flat::*`.

## Features

Feature flags forward to `zenoh` and mirror zenoh-c's defaults. `unstable`
additionally enables the `#[unstable]` slices of the API (`Reliability`,
entity-id accessors, key-expression relations, sample source info); it is **off**
by default.
