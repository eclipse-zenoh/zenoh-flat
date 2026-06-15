use prebindgen_proc_macro::prebindgen;

use crate::Timestamp;

/// NTP64 time component of the timestamp.
#[prebindgen]
pub fn timestamp_get_ntp64(t: &Timestamp) -> i64 {
    t.get_time().as_u64() as i64
}

/// Raw bytes of the originating `TimestampId`.
#[prebindgen]
pub fn timestamp_get_id(t: &Timestamp) -> Vec<u8> {
    let id = t.get_id();
    id.to_le_bytes()[..id.size()].to_vec()
}
