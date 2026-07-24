use prebindgen_proc_macro::prebindgen;

use crate::KeyExpr;

/// A key expression together with the parameters that refine a selection.
///
/// Mirrors [`zenoh::query::Selector`]: `key_expr` identifies which keys are part
/// of the selection and `parameters` refines which values are of interest. An
/// empty `parameters` string selects everything matched by `key_expr`. Process
/// `parameters` with the `parameters_*` functions.
#[prebindgen]
#[derive(Clone, Debug)]
pub struct Selector {
    /// The key expression identifying which keys are part of the selection.
    pub key_expr: KeyExpr,
    /// The parameters refining the selection; empty selects everything matched
    /// by `key_expr` (see the `parameters_*` functions).
    pub parameters: String,
}

impl From<Selector> for zenoh::query::Selector<'static> {
    fn from(s: Selector) -> Self {
        zenoh::query::Selector::owned(s.key_expr, s.parameters)
    }
}
