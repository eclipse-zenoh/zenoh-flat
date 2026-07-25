use prebindgen_proc_macro::prebindgen;

/// Whether replies may use key expressions that do not match the query.
#[prebindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplyKeyExpr {
    /// Accept replies with any key expression.
    Any = 0,
    /// Accept only replies whose key expressions match the query.
    MatchingQuery = 1,
}

impl From<zenoh::query::ReplyKeyExpr> for ReplyKeyExpr {
    fn from(r: zenoh::query::ReplyKeyExpr) -> Self {
        match r {
            zenoh::query::ReplyKeyExpr::Any => ReplyKeyExpr::Any,
            zenoh::query::ReplyKeyExpr::MatchingQuery => ReplyKeyExpr::MatchingQuery,
        }
    }
}

impl From<ReplyKeyExpr> for zenoh::query::ReplyKeyExpr {
    fn from(r: ReplyKeyExpr) -> Self {
        match r {
            ReplyKeyExpr::Any => zenoh::query::ReplyKeyExpr::Any,
            ReplyKeyExpr::MatchingQuery => zenoh::query::ReplyKeyExpr::MatchingQuery,
        }
    }
}
