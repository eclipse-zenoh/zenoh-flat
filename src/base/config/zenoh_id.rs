use prebindgen_proc_macro::prebindgen;

use crate::ZenohId;

/// Serialize a Zenoh node identifier as raw bytes (16 bytes, little-endian).
#[prebindgen]
pub fn zenoh_id_to_le_bytes(z: &ZenohId) -> Vec<u8> {
    z.to_le_bytes().to_vec()
}

/// Format a Zenoh node identifier as its standard string form.
#[prebindgen]
pub fn zenoh_id_to_string(z: &ZenohId) -> String {
    z.to_string()
}
