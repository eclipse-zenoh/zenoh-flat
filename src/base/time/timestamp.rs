use prebindgen_proc_macro::prebindgen;

/// A time value associated with a Zenoh operation, as a plain value.
///
/// A timestamp is fully described by its NTP64 time component and the identifier
/// of the node that created it; both are small and fixed-size, so a timestamp
/// crosses whole, by value.
#[prebindgen]
#[derive(Clone, Debug)]
pub struct Timestamp {
    /// NTP64 time component of the timestamp.
    pub ntp64: i64,
    /// Raw bytes of the originating node identifier.
    pub id: Vec<u8>,
}

impl From<&zenoh::time::Timestamp> for Timestamp {
    fn from(t: &zenoh::time::Timestamp) -> Self {
        let id = t.get_id();
        Timestamp {
            ntp64: t.get_time().as_u64() as i64,
            id: id.to_le_bytes()[..id.size()].to_vec(),
        }
    }
}
