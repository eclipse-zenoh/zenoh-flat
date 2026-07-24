use prebindgen_proc_macro::prebindgen;

use crate::{Sample, ZenohId};

/// Source information carried by a sample: the identity of the entity that
/// produced it and the per-source sequence number.
///
/// This is a plain value: when a sample carries source information, all of its
/// fields are known, so the optionality lives on the whole value (a sample may
/// or may not have source information) rather than on the individual fields.
///
/// This information is available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug)]
pub struct SourceInfo {
    /// Identifier of the node that produced the sample.
    pub zid: ZenohId,
    /// Entity identifier of the source within its session.
    pub eid: u32,
    /// Sequence number of the sample at its source.
    pub sn: u32,
}

impl From<&zenoh::sample::SourceInfo> for SourceInfo {
    fn from(si: &zenoh::sample::SourceInfo) -> Self {
        let id = si.source_id();
        SourceInfo {
            zid: id.zid(),
            eid: id.eid(),
            sn: si.source_sn(),
        }
    }
}

/// Return the source information of the sample, when known.
///
/// This information is available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_get_source_info(s: &Sample) -> Option<SourceInfo> {
    s.source_info().map(SourceInfo::from)
}

/// Return whether the sample carries source information.
///
/// This distinguishes "no source information" from a source whose fields
/// happen to be zero.
///
/// This information is available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_source_info_defined(s: &Sample) -> bool {
    s.source_info().is_some()
}

/// Return the identifier of the node that produced the sample, when known.
///
/// This information is available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_get_source_zid(s: &Sample) -> Option<ZenohId> {
    sample_get_source_info(s).map(|si| si.zid)
}

/// Return the entity identifier of the sample's source, when known.
///
/// This information is available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_get_source_eid(s: &Sample) -> Option<u32> {
    sample_get_source_info(s).map(|si| si.eid)
}

/// Return the source sequence number, when source information is present.
///
/// This information is available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn sample_get_source_sn(s: &Sample) -> Option<u32> {
    sample_get_source_info(s).map(|si| si.sn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{keyexpr_new_try_from, sample_new_put, zbytes_new_from_slice};

    fn put_sample() -> Sample {
        let ke = keyexpr_new_try_from("test/source".to_string()).unwrap();
        sample_new_put(
            ke,
            zbytes_new_from_slice(b"payload"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    #[test]
    fn absent_source_info_stays_none() {
        // A locally built sample carries no source information; absence must be
        // reported as such, never collapsed into a field value of 0 (#10).
        let s = put_sample();
        assert!(!sample_source_info_defined(&s));
        assert!(sample_get_source_info(&s).is_none());
        assert!(sample_get_source_zid(&s).is_none());
        assert!(sample_get_source_eid(&s).is_none());
        assert!(sample_get_source_sn(&s).is_none());
    }
}
