use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

use crate::{Error, MatchingListener};

/// Undeclare a matching listener.
#[prebindgen]
pub fn matching_listener_undeclare(listener: MatchingListener) -> Result<(), Error> {
    listener.undeclare().wait()
}
