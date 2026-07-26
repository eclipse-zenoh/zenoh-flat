use prebindgen_proc_macro::prebindgen;

use crate::{EntityGlobalId, Sample};

/// Source information carried by a sample: which entity produced it and the
/// per-source sequence number.
///
/// This is a plain value: when a sample carries source information, all of its
/// fields are known, so the optionality lives on the whole value (a sample may
/// or may not have source information) rather than on the individual fields.
///
/// This information is available only when unstable features are enabled.
#[prebindgen(cfg = "feature = \"unstable\"")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceInfo {
    /// Global identifier of the entity that produced the sample.
    pub source: EntityGlobalId,
    /// Sequence number of the sample at its source.
    pub sn: u32,
}

impl From<&zenoh::sample::SourceInfo> for SourceInfo {
    fn from(si: &zenoh::sample::SourceInfo) -> Self {
        SourceInfo {
            source: (*si.source_id()).into(),
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
        .expect("no timestamp to validate")
    }

    #[test]
    fn absent_source_info_stays_none() {
        // A locally built sample carries no source information; absence must be
        // reported as `None`, never collapsed into a source with zero fields (#10).
        let s = put_sample();
        assert!(sample_get_source_info(&s).is_none());
    }
}
