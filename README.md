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
- **Value** — plain data with no identity: an entity id, a report, a small configuration. A value is
  an ordinary Rust `struct` with public fields and crosses by copying them. Examples: `ZenohId`,
  `EntityGlobalId { zid, eid }`, `SourceInfo { source, sn }`, `Timestamp`, and the input configs
  (`HistoryConfig`, `RecoveryConfig`, …). A value has no identity of its own, but it is **not
  necessarily release-free**: it may own a handle for a field whose type has no value form — see
  [Composing a value](#composing-a-value).
- **Twin** — a type worth having *both* ways. It is fully described by its fields (so it can be a
  value), but it also carries a **payload** — an unbounded string, list, or `ZBytes` — you may not
  want to copy on every access (so a handle lets you read the cheap fields without materializing it).
  Such a type gets a handle *and* a value form: the handle keeps zenoh's name (`Sample`, `Encoding`)
  and the value form adds a `Struct` suffix (`SampleStruct`, `EncodingStruct`), reached with a
  `<type>_to_struct` accessor. Examples: `Sample`, `Reply`, `ReplyError`, `Hello`, `Encoding`.
  Where zenoh offers a **by-value exit** for the type, the twin also gets
  `<type>_into_struct`, which consumes the handle and *moves* each field into the value form
  instead of cloning it — see [One source of truth per field](#one-source-of-truth-per-field).
  Today that is `Sample` (zenoh's `SampleFields`) and `Reply` (`Reply::into_result`); the other
  twins have no such exit, so a consuming form would clone exactly as the borrowing one does and
  none is offered.
  A type is *not* a twin when it hides state a caller cannot read back (`KeyExpr`, `Error`), nor when
  its value form would hold a **single** field, since then the accessor already is that value form
  (`ZBytes`) — see [Choosing a shape](#choosing-a-shape).

### Composing a value

A value form is the type's own accessors gathered into one struct: `sample_to_struct` returns
exactly what `sample_get_key_expr`, `sample_get_payload`, `sample_get_encoding`, … return, in one
call instead of one per field. So there is no separate rule for composing it — **a `…Struct` opens
the one handle you called it on, and stops there.** Whatever an accessor hands back, the field
holds; a field whose type is a handle stays that handle.

- `SampleStruct.encoding` is an `Encoding`, `SampleStruct.key_expr` a `KeyExpr`,
  `SampleStruct.payload` a `ZBytes`.
- `ReplyResult`'s variants carry a `Sample` and a `ReplyError`.

Unwrapping further would defeat the shape. A nested value form drags in that type's payload — an
encoding's arbitrary-length schema, a whole timestamp stack — on every call, read or not, which is
the cost the twin shape exists to let a caller avoid. A caller who wants a nested twin as data calls
that twin's own `<type>_to_struct`, and pays for the payload where they ask for it.

So a value form **carries handles**, and a consumer may still have handles to release after taking
one apart. This is the normal case, not an exception; only a value whose fields all lack handle
forms is release-free.

**Input-only structs are exempt.** A type flat only ever *receives* from a caller, never hands back —
`Selector`, the `*Config` inputs — is not a twin's value form and does not follow this rule.
`Selector.key_expr` stays a `KeyExpr` because passing an already-declared key expression is a real
capability: it avoids re-resolving the expression on every query, and a `String` field would remove
it.

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

  Read "has a payload field" **prospectively and by family**, not as a snapshot of today's struct:
  it counts if the type has one, if it may plausibly gain one as zenoh grows, **or** if a sibling it
  is used alongside has one. A type's shape is the hardest thing to change once bindings ship, so
  the question to ask is not "what fields does it have right now" but "would a handle ever have been
  the right answer". Answering yes early costs a caller nothing — the value form is right there —
  while answering no early has to be undone in every binding at once.

  `Transport` is why the rule is written this way. Its fields today are all cheap (a zid, an enum,
  three flags), so read literally it is value-only. It is a **twin** anyway: it is handed out beside
  `Link` by the same calls, consumed by the same callers, and passed *back* to zenoh to select which
  links to report. Splitting the pair — one a handle, one a value — would make the two halves of one
  API behave differently for no reason a caller can see, and would force flat to rebuild zenoh's own
  transport from its fields in order to name it, which
  [Construction mirrors zenoh](#construction-mirrors-zenoh) says flat does not do.
- **Give it a value form?** Yes if the type is **fully defined by its readable fields** — a data
  snapshot, not an opaque resource.

A type can answer **yes to both** — that is exactly what a **twin** is (`Sample`, `Encoding`,
`Hello`): a handle *and* a value form, no separate decision to make. A type of only cheap, fixed-size
fields (an id, a timestamp) answers no to the handle question — unless the prospective reading above
answers it for them — and is **value-only**. A live resource answers no to the value question and is
**handle-only**.

**"Fully defined by its readable fields" is a strict test.** A type fails it when it carries state a
caller cannot read back, even if what *is* readable looks like the whole thing:

- `KeyExpr` — a **declared** key expression holds a wire declaration bound to the session that
  declared it (an id plus a reference to that session). `keyexpr_as_str` returns only the
  expression, so rebuilding from that string yields an *undeclared* key expression: same text,
  different object, and the declaration's optimisation silently gone. A declared key expression is a
  live resource, so `KeyExpr` is **handle-only**.
- `Error` — `error_get_message` renders the error; it does not decompose it. The concrete error type
  and its `source()` chain are not recoverable from that string, so `Error` is **handle-only** too.

**A value form of one field is just an accessor.** `ZBytes` does pass both questions — it carries a
payload and it *is* its bytes — so the rules above would make it a twin. It is not, because its
value form would hold a single field, and a one-field struct is a function in disguise: the accessor
*is* the value form. So `zbytes_to_bytes` is `ZBytes`'s value form, and no `…Struct` /
`<type>_to_struct` pair is emitted. The `…Struct` machinery exists for types with **more than one**
readable field, where a struct is what saves a caller a call per field.

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
  `result: ReplyResult` with `Sample(Sample)` / `Error(ReplyError)` variants — not a `sample` and an
  `error` `Option` alongside each other.
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
- **`into_` marks a reader that consumes its subject.** `sample_into_struct` takes the sample by
  value and destroys it; `sample_to_struct` borrows it and leaves it intact. Until these, every
  by-value function in flat was an `_undeclare` or a `_new_`, where destruction is already in the
  verb — a *reader* that eats its argument has to say so.

### Be faithful to zenoh — the most important rule

A value form must preserve zenoh's **information and optionality exactly**, expressed in ordinary
Rust types (`u32`, `Option<u32>`). Where zenoh wraps a value in a newtype or an opaque id — `NTP64`,
`TimestampId`, `ZSlice` — lower it to the plain Rust type carrying the same information without
loss (`u64`, `Vec<u8>`), and never to a wire- or binding-specific type.

The test is what a reader can still recover, not what the field is spelled as: lowering `NTP64` to
`u64` loses nothing, while widening a `u16` id to `i32` or narrowing it back does.

- **Never fake "unknown" with a sentinel.** If zenoh returns an `Option`, flat returns an `Option`.
  For a sample's source, "no source information at all" and "the source's fields happen to be `0`"
  are different facts and must stay different — `sample_get_source_info` returns `Option<SourceInfo>`
  (absent ⇒ `None`), never a `SourceInfo` with zeroed fields.
- **Put the optionality on the right edge.** When a sample's source is known, its entity id and
  sequence number always exist; only the *whole* source-info is optional. So those fields are
  non-optional and the parent carries the `Option`, not a struct full of `Option` fields.

### One source of truth per field

Each field has **one implementation**. This is a rule about bodies, not about the surface: a twin
deliberately makes every field readable two ways — `sample_get_payload(&s)` and
`sample_to_struct(&s).payload` — and that is the point of the shape, not a violation.

What the rule forbids is those two routes being *computed* independently. Where a field is reachable
both through a value form and through an accessor, one must **delegate** to the other; likewise a
convenience shortcut for a nested field must delegate to the same path rather than re-deriving the
value. Two independent bodies reading the same field eventually disagree.

**The delegation runs towards the consuming form.** A twin with a `<type>_into_struct` has two
value forms, and the **consuming** one is the single body: the borrowing form is
`<type>_to_struct(x) = <type>_into_struct(x.clone())`, and any accessor that only projects the
value form (`reply_get_result`) delegates to that in turn. The clone is not a new cost — it clones
the same fields the borrowing form was cloning one by one.

This does put the moved fields' definition in zenoh's by-value exit rather than in flat's own
per-field accessors, which is the one place the two routes are genuinely separate bodies (both
zenoh's). That is what `assert_struct_mirrors_accessors` in `src/base/sample/mod.rs` and
`reply_struct_mismatches` in `tests/queryable.rs` exist to pin: they compare every field of the
value form against the accessor for that same field, and fail the moment the two disagree.

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
