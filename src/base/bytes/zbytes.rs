use std::borrow::Cow;

use prebindgen_proc_macro::prebindgen;

use crate::{Error, ZBytes};

/// Return the number of bytes in the payload.
///
/// Reading the size does not materialize the payload: this is the cheap field a
/// caller can read off the handle without paying to copy the buffer out.
#[prebindgen]
pub fn zbytes_len(z: &ZBytes) -> usize {
    z.len()
}

/// Return whether the payload is empty.
#[prebindgen]
pub fn zbytes_is_empty(z: &ZBytes) -> bool {
    z.is_empty()
}

/// Return the payload as a contiguous sequence of bytes.
#[prebindgen]
pub fn zbytes_to_bytes(z: &ZBytes) -> Cow<'_, [u8]> {
    z.to_bytes()
}

/// Return the payload decoded as UTF-8 text.
///
/// Fails if the payload is not valid UTF-8. Decoding is base's, so a payload
/// that zenoh considers text decodes exactly as zenoh decodes it — a caller
/// must not re-implement the check, since a skipped one turns invalid input
/// into silent corruption rather than an error.
#[prebindgen]
pub fn zbytes_try_to_string(z: &ZBytes) -> Result<String, Error> {
    Ok(z.try_to_string()?.into_owned())
}

/// Create a payload from a sequence of bytes.
#[prebindgen]
pub fn zbytes_new_from_slice(bytes: &[u8]) -> ZBytes {
    ZBytes::from(bytes.to_vec())
}

/// Create an independent copy of a payload.
#[prebindgen]
pub fn zbytes_new_clone(z: &ZBytes) -> ZBytes {
    z.clone()
}

/// Create a payload from a byte sequence.
#[prebindgen]
pub fn zbytes_new_from_vec(bytes: Vec<u8>) -> ZBytes {
    ZBytes::from(bytes)
}
