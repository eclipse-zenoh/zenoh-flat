use prebindgen_proc_macro::prebindgen;

use crate::Error;

/// Width of a [`ZenohId`] in bytes.
///
/// Mirrors zenoh's own `ZenohId::MAX_SIZE`.
#[prebindgen]
pub const ZENOH_ID_MAX_SIZE: usize = 16;

/// Identifier of a Zenoh node, as a plain value.
///
/// A node identifier is a **bounded** blob — exactly [`ZENOH_ID_MAX_SIZE`] bytes
/// wide — so it is fully described by its bytes, costs nothing to copy whole,
/// and needs no allocation. It is a value, not a handle: there is no lifecycle
/// to manage and nothing to release.
///
/// The bound is carried by the type rather than by this comment: the field is a
/// fixed-size array, so no reader has to be told how long an identifier may be.
#[prebindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ZenohId {
    /// Bytes of the identifier, little-endian, zero-padded to the full width.
    ///
    /// This is zenoh's own representation: an identifier is a 128-bit value, so
    /// an identifier that needs fewer bytes simply has zeros in its high-order
    /// ones. The padding carries no information — it neither changes the
    /// identifier nor shows up in its rendered form.
    pub bytes: [u8; ZENOH_ID_MAX_SIZE],
}

impl From<zenoh::session::ZenohId> for ZenohId {
    fn from(z: zenoh::session::ZenohId) -> Self {
        ZenohId {
            bytes: z.to_le_bytes(),
        }
    }
}

impl TryFrom<&ZenohId> for zenoh::session::ZenohId {
    type Error = Error;

    fn try_from(z: &ZenohId) -> Result<Self, Self::Error> {
        zenoh::session::ZenohId::try_from(&z.bytes[..])
    }
}

/// Format a Zenoh node identifier as its standard string form.
///
/// Fails only for all-zero bytes, which are not an identifier — every other
/// value of the field is one, since the type already fixes the width. Rendering
/// is zenoh's own, so an identifier obtained from zenoh always renders exactly
/// as zenoh renders it.
#[prebindgen]
pub fn zenoh_id_to_string(z: &ZenohId) -> Result<String, Error> {
    Ok(zenoh::session::ZenohId::try_from(z)?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The value form agrees with zenoh: it carries zenoh's own bytes, renders
    /// exactly as zenoh renders the same identifier, and converts back to it.
    /// zenoh's own `Display` is the oracle — flat must never reimplement the
    /// rendering.
    ///
    /// Identifiers of several widths are used deliberately. A short identifier
    /// is zero-padded in its high-order bytes, so the widths are where a
    /// representation that mishandled padding would show up.
    #[test]
    fn value_form_matches_base() {
        // Hex, little-endian; zenoh rejects leading zeros, since those would be
        // non-significant bytes.
        for text in [
            "1",
            "a1",
            "1234",
            "aabbcc",
            "1122334455667788",
            "112233445566778899aabbccddeeff11",
        ] {
            let base: zenoh::session::ZenohId = text.parse().expect("valid identifier");
            let flat = ZenohId::from(base);

            assert_eq!(flat.bytes, base.to_le_bytes(), "{text} bytes differ");
            assert_eq!(
                zenoh_id_to_string(&flat).unwrap(),
                base.to_string(),
                "{text} renders differently from base"
            );
            assert_eq!(zenoh::session::ZenohId::try_from(&flat).unwrap(), base);
        }
    }

    /// A short identifier really is zero-padded in the field, and that padding
    /// does not leak into the rendered form. This is what makes the fixed-width
    /// field lossless: without it, `value_form_matches_base` would not
    /// distinguish a padded representation from a truncated one.
    #[test]
    fn padding_is_present_but_invisible() {
        let base: zenoh::session::ZenohId = "a1".parse().unwrap();
        let flat = ZenohId::from(base);

        assert_eq!(flat.bytes[0], 0xa1);
        assert!(
            flat.bytes[1..].iter().all(|&b| b == 0),
            "high-order bytes of a short identifier must be zero"
        );
        assert_eq!(zenoh_id_to_string(&flat).unwrap(), "a1");
    }

    /// All-zero bytes are not an identifier and are reported rather than
    /// rendered as some arbitrary string.
    #[test]
    fn all_zero_is_rejected() {
        let zero = ZenohId {
            bytes: [0u8; ZENOH_ID_MAX_SIZE],
        };
        assert!(zenoh_id_to_string(&zero).is_err());
    }
}
