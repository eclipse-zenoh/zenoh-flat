use prebindgen_proc_macro::prebindgen;

/// The change represented by a sample.
#[prebindgen]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleKind {
    /// The sample publishes or updates a value.
    Put = 0,
    /// The sample announces that a value was deleted.
    Delete = 1,
}

impl From<zenoh::sample::SampleKind> for SampleKind {
    fn from(k: zenoh::sample::SampleKind) -> Self {
        match k {
            zenoh::sample::SampleKind::Put => SampleKind::Put,
            zenoh::sample::SampleKind::Delete => SampleKind::Delete,
        }
    }
}
