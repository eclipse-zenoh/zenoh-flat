use std::borrow::Cow;

use prebindgen_proc_macro::prebindgen;

use crate::ZZBytes;

/// Read the payload bytes carried by a native [`ZZBytes`]. Performs one
/// copy if the underlying buffer is non-contiguous (mirrors
/// `zenoh::bytes::ZBytes::to_bytes`).
#[prebindgen]
pub fn z_zbytes_to_bytes(z: &ZZBytes) -> Vec<u8> {
    z.to_bytes().into_owned()
}

/// Borrow the payload bytes carried by a native [`ZZBytes`] — borrowed (no
/// copy) when the underlying buffer is contiguous, owned otherwise. The
/// zero-copy sibling of [`z_zbytes_to_bytes`] for adapters that copy the
/// bytes onward exactly once anyway (e.g. into a JVM array).
#[prebindgen]
pub fn z_zbytes_as_bytes(z: &ZZBytes) -> Cow<'_, [u8]> {
    z.to_bytes()
}

/// Construct a native [`ZZBytes`] from a borrowed byte slice. Copies the
/// bytes; this is the C-facing constructor (`const uint8_t* + size`).
#[prebindgen]
pub fn z_zbytes_from_slice(bytes: &[u8]) -> ZZBytes {
    ZZBytes::from(bytes.to_vec())
}

/// Clone a payload into a new owned handle. Cheap: `ZBytes` is backed by
/// reference-counted buffers, so this bumps a refcount rather than copying.
/// Use it to hand the same payload to a consuming call repeatedly (e.g. a
/// throughput publisher loop) without re-encoding from a buffer each time.
#[prebindgen]
pub fn z_zbytes_clone(z: &ZZBytes) -> ZZBytes {
    z.clone()
}

/// Construct a native [`ZZBytes`] from an owned byte buffer, taking ownership
/// without copying. Not exported to the C layer — it exists for completeness
/// and to accept zenoh-flat's `ZBytes` payload without cloning.
#[prebindgen]
pub fn z_zbytes_from_vec(bytes: Vec<u8>) -> ZZBytes {
    ZZBytes::from(bytes)
}
