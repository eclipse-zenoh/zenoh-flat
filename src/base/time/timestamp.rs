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
    /// Bytes of the originating node identifier, in the same form as
    /// [`ZenohId::bytes`](crate::ZenohId::bytes): little-endian, zero-padded to
    /// the full [`ZENOH_ID_MAX_SIZE`](crate::ZENOH_ID_MAX_SIZE) width, so an
    /// identifier obtained here is byte-for-byte the one obtained from any
    /// other part of this API.
    pub id: Vec<u8>,
}

impl TryFrom<&Timestamp> for zenoh::time::Timestamp {
    type Error = crate::Error;

    fn try_from(t: &Timestamp) -> Result<Self, Self::Error> {
        // The id round-trips exactly: the `From` impl below carries the whole
        // padded width, and `TimestampId` reads the padding back as the
        // high-order zeros it is.
        let id = zenoh::time::TimestampId::try_from(t.id.as_slice())
            .map_err(|e| format!("invalid timestamp id: {e}"))?;
        Ok(zenoh::time::Timestamp::new(zenoh::time::NTP64(t.ntp64), id))
    }
}

impl From<&zenoh::time::Timestamp> for Timestamp {
    fn from(t: &zenoh::time::Timestamp) -> Self {
        Timestamp {
            ntp64: t.get_time().as_u64(),
            // The whole padded width, not `[..id.size()]`: `size()` drops
            // high-order zero bytes, which would make the same identifier a
            // different byte string here than in `ZenohId`.
            id: t.get_id().to_le_bytes().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use zenoh::time::{NTP64, TimestampId};

    use super::*;

    /// The identifier a timestamp carries is the same byte string the rest of
    /// the API carries for that node. Short identifiers are the interesting
    /// case: their high-order bytes are zero, and a representation that dropped
    /// the padding would still *render* identically while comparing unequal.
    #[test]
    fn id_matches_zenoh_id_bytes() {
        for text in ["1", "aabbcc", "112233445566778899aabbccddeeff11"] {
            let zid: zenoh::session::ZenohId = text.parse().expect("valid identifier");
            let id = TimestampId::try_from(&zid.to_le_bytes()[..]).unwrap();
            let zt = zenoh::time::Timestamp::new(NTP64(42), id);

            let flat = Timestamp::from(&zt);
            assert_eq!(
                flat.id,
                crate::ZenohId::from(zid).bytes,
                "{text} id differs from the value form of the same node"
            );
            assert_eq!(
                zenoh::time::Timestamp::try_from(&flat).unwrap(),
                zt,
                "{text} does not round-trip"
            );
        }
    }

    #[test]
    fn preserves_unsigned_ntp64_high_bit() {
        // A current-era NTP64 value has its high bit set (above `i64::MAX`); the
        // value form must carry it unsigned, not wrap it into a negative number.
        let raw = (i64::MAX as u64) + 12_345;
        let zt = zenoh::time::Timestamp::new(NTP64(raw), TimestampId::rand());
        assert_eq!(Timestamp::from(&zt).ntp64, raw);
    }
}
