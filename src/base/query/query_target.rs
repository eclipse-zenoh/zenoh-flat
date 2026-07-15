use prebindgen_proc_macro::prebindgen;

/// The set of queryables that should receive a query.
#[prebindgen]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryTarget {
    /// Let Zenoh choose the most appropriate queryable targets.
    BestMatching = 0,
    /// Send the query to every matching queryable.
    All = 1,
    /// Send the query to all complete queryables and the best matching
    /// incomplete queryable.
    AllComplete = 2,
}

impl From<zenoh::query::QueryTarget> for QueryTarget {
    fn from(t: zenoh::query::QueryTarget) -> Self {
        match t {
            zenoh::query::QueryTarget::BestMatching => QueryTarget::BestMatching,
            zenoh::query::QueryTarget::All => QueryTarget::All,
            zenoh::query::QueryTarget::AllComplete => QueryTarget::AllComplete,
        }
    }
}

impl From<QueryTarget> for zenoh::query::QueryTarget {
    fn from(t: QueryTarget) -> Self {
        match t {
            QueryTarget::BestMatching => zenoh::query::QueryTarget::BestMatching,
            QueryTarget::All => zenoh::query::QueryTarget::All,
            QueryTarget::AllComplete => zenoh::query::QueryTarget::AllComplete,
        }
    }
}
