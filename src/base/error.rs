use prebindgen_proc_macro::prebindgen;

use crate::Error;

/// Format a zenoh error as its display string.
///
/// `Error` (zenoh's native `Box<dyn Error + Send + Sync>`) is the `E` of every
/// fallible `z_*` `Result`. It never crosses the FFI boundary as a value; this
/// accessor converts it to a `String` for the JNI error callback (declared as a
/// `.convert_error` / `.default()` converter in the binding crate's `build.rs`).
#[prebindgen]
pub fn error_get_message(e: &Error) -> String {
    e.to_string()
}
