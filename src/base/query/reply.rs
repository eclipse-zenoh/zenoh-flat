use prebindgen_proc_macro::prebindgen;

#[cfg(feature = "unstable")]
use crate::EntityGlobalId;
use crate::{Encoding, Reply, ReplyError, Sample, ZBytes};

/// Return the global identifier of the entity that answered, when known.
///
/// This information is available only when unstable features are enabled.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn reply_get_replier_id(r: &Reply) -> Option<EntityGlobalId> {
    r.replier_id().map(EntityGlobalId::from)
}

/// Return whether this reply contains a sample rather than an error.
#[prebindgen]
pub fn reply_is_ok(r: &Reply) -> bool {
    r.result().is_ok()
}

/// Return the sample carried by a successful reply.
#[prebindgen]
pub fn reply_get_sample(r: &Reply) -> Option<&Sample> {
    r.result().ok()
}

/// Return the application error carried by an unsuccessful reply.
#[prebindgen]
pub fn reply_get_err(r: &Reply) -> Option<&ReplyError> {
    r.result().err()
}

/// Return the error payload.
#[prebindgen]
pub fn reply_error_get_payload(e: &ReplyError) -> &ZBytes {
    e.payload()
}

/// Return format information associated with the error payload.
#[prebindgen]
pub fn reply_error_get_encoding(e: &ReplyError) -> &Encoding {
    e.encoding()
}

/// The application error carried by an unsuccessful reply, as a value form.
///
/// Like every value form it is this type's accessors gathered into one struct,
/// so `encoding` is the [`Encoding`] handle, not its value form.
#[prebindgen]
#[derive(Clone, Debug)]
pub struct ReplyErrorStruct {
    /// Error payload.
    pub payload: ZBytes,
    /// Format information associated with the error payload.
    pub encoding: Encoding,
}

impl From<&ReplyError> for ReplyErrorStruct {
    fn from(e: &ReplyError) -> Self {
        // Delegate to the field accessors so each field has one definition.
        ReplyErrorStruct {
            payload: reply_error_get_payload(e).clone(),
            encoding: reply_error_get_encoding(e).clone(),
        }
    }
}

/// Decompose a reply error into its [`ReplyErrorStruct`] value form.
#[prebindgen]
pub fn reply_error_to_struct(e: &ReplyError) -> ReplyErrorStruct {
    e.into()
}

/// The outcome of a reply, as a plain value: the sample of a successful reply,
/// or the error of an unsuccessful one.
///
/// This mirrors zenoh's own [`zenoh::query::Reply::result`], which is a
/// `Result<&Sample, &ReplyError>` — a genuine sum. A reply is one or the other,
/// never both and never neither, so the alternatives are variants of a single
/// type rather than parallel optional fields: the exclusivity is carried by the
/// type and a consumer cannot mistake an error reply for an empty success.
///
/// Both alternatives are carried as the handles the reply's own accessors hand
/// back; a caller who wants the sample as data calls
/// [`crate::sample_to_struct`] on it.
#[prebindgen]
#[derive(Clone, Debug)]
pub enum ReplyResult {
    /// The sample carried by a successful reply.
    Sample(Sample),
    /// The error carried by an unsuccessful reply.
    Error(ReplyError),
}

/// A reply decomposed into a plain value.
#[prebindgen]
#[derive(Clone, Debug)]
pub struct ReplyStruct {
    /// The reply's outcome: its sample or its error.
    pub result: ReplyResult,
    /// Global identifier of the entity that answered, when known. Available
    /// only when unstable features are enabled.
    ///
    /// This stays a sibling field rather than a [`ReplyResult`] payload: which
    /// entity answered is orthogonal to whether it succeeded, so its `Option`
    /// belongs where it actually applies.
    #[cfg(feature = "unstable")]
    pub replier_id: Option<EntityGlobalId>,
}

impl From<&Reply> for ReplyStruct {
    fn from(r: &Reply) -> Self {
        ReplyStruct {
            // `Reply::result` is the one source for the outcome. The
            // `reply_is_ok` / `reply_get_sample` / `reply_get_err` accessors are
            // projections of that same call rather than independent readings,
            // and `reply_struct_mismatches` in `tests/queryable.rs` pins this
            // field in agreement with all three.
            result: match r.result() {
                Ok(s) => ReplyResult::Sample(s.clone()),
                Err(e) => ReplyResult::Error(e.clone()),
            },
            #[cfg(feature = "unstable")]
            replier_id: reply_get_replier_id(r),
        }
    }
}

/// Decompose a reply into its [`ReplyStruct`] value form.
#[prebindgen]
pub fn reply_to_struct(r: &Reply) -> ReplyStruct {
    r.into()
}
