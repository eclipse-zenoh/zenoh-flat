use prebindgen_proc_macro::prebindgen;

#[cfg(feature = "unstable")]
use crate::EntityGlobalId;
use crate::{Encoding, Reply, ReplyError, Sample, SampleStruct, ZBytes};

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

/// The application error carried by an unsuccessful reply, as a plain value.
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
        ReplyErrorStruct {
            payload: e.payload().clone(),
            encoding: e.encoding().clone(),
        }
    }
}

/// Decompose a reply error into its [`ReplyErrorStruct`] value form.
#[prebindgen]
pub fn reply_error_to_struct(e: &ReplyError) -> ReplyErrorStruct {
    e.into()
}

/// A reply decomposed into a plain value.
///
/// Exactly one of `sample` (a successful reply) or `error` (an unsuccessful
/// one) is present.
#[prebindgen]
#[derive(Clone, Debug)]
pub struct ReplyStruct {
    /// The sample carried by a successful reply.
    pub sample: Option<SampleStruct>,
    /// The error carried by an unsuccessful reply.
    pub error: Option<ReplyErrorStruct>,
    /// Global identifier of the entity that answered, when known. Available
    /// only when unstable features are enabled.
    #[cfg(feature = "unstable")]
    pub replier_id: Option<EntityGlobalId>,
}

impl From<&Reply> for ReplyStruct {
    fn from(r: &Reply) -> Self {
        let (sample, error) = match r.result() {
            Ok(s) => (Some(SampleStruct::from(s)), None),
            Err(e) => (None, Some(ReplyErrorStruct::from(e))),
        };
        ReplyStruct {
            sample,
            error,
            #[cfg(feature = "unstable")]
            replier_id: r.replier_id().map(EntityGlobalId::from),
        }
    }
}

/// Decompose a reply into its [`ReplyStruct`] value form.
#[prebindgen]
pub fn reply_to_struct(r: &Reply) -> ReplyStruct {
    r.into()
}
