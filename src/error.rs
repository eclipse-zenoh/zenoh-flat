use prebindgen_proc_macro::prebindgen;

#[prebindgen]
#[derive(Clone)]
pub struct Error {
    pub message: String,
}

/// Return the error message for bindings that route failures through a
/// language-specific error callback.
#[prebindgen]
pub fn error_get_message(error: &Error) -> String {
    error.message.clone()
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
