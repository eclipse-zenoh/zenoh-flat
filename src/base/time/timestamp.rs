use prebindgen_proc_macro::prebindgen;

/// A time value associated with a Zenoh operation, as a plain value.
///
/// A timestamp is fully described by its NTP64 time component and the identifier
/// of the node that created it; both are small and fixed-size, so a timestamp
/// crosses whole, by value.
#[prebindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Timestamp {
    /// NTP64 time component of the timestamp. This is an unsigned value;
    /// current-era timestamps set the high bit.
    pub ntp64: u64,
    /// Raw bytes of the originating node identifier.
    pub id: Vec<u8>,
}

impl From<&zenoh::time::Timestamp> for Timestamp {
    fn from(t: &zenoh::time::Timestamp) -> Self {
        let id = t.get_id();
        Timestamp {
            ntp64: t.get_time().as_u64(),
            id: id.to_le_bytes()[..id.size()].to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use zenoh::time::{NTP64, TimestampId};

    use super::*;

    #[test]
    fn preserves_unsigned_ntp64_high_bit() {
        // A current-era NTP64 value has its high bit set (above `i64::MAX`); the
        // value form must carry it unsigned, not wrap it into a negative number.
        let raw = (i64::MAX as u64) + 12_345;
        let zt = zenoh::time::Timestamp::new(NTP64(raw), TimestampId::rand());
        assert_eq!(Timestamp::from(&zt).ntp64, raw);
    }
}
