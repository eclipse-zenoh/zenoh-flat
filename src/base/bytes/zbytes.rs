use std::borrow::Cow;

use prebindgen_proc_macro::prebindgen;

use crate::ZBytes;

/// Return the payload as a contiguous sequence of bytes.
#[prebindgen]
pub fn zbytes_as_bytes(z: &ZBytes) -> Cow<'_, [u8]> {
    z.to_bytes()
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
