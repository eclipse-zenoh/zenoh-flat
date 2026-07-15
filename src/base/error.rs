use prebindgen_proc_macro::prebindgen;

use crate::Error;

/// Return a human-readable description of an error.
#[prebindgen]
pub fn error_get_message(e: &Error) -> String {
    e.to_string()
}
