pub(crate) mod whatami;
pub(crate) mod zenoh_id;

use prebindgen_proc_macro::prebindgen;

use crate::{Config, Error};

/// Create a configuration with default settings.
#[prebindgen]
pub fn config_new_default() -> Config {
    Config::default()
}

/// Create an independent copy of a configuration.
#[prebindgen]
pub fn config_new_clone(c: &Config) -> Config {
    c.clone()
}

/// Load a configuration from a file path. The file extension determines
/// the format (JSON, JSON5, or YAML).
#[prebindgen]
pub fn config_new_from_file(path: &str) -> Result<Config, Error> {
    Config::from_file(path)
}

/// Parse a configuration from JSON text.
#[prebindgen]
pub fn config_new_from_json(s: &str) -> Result<Config, Error> {
    config_new_from_json5(s)
}

/// Parse a configuration from a JSON5-formatted string.
#[prebindgen]
pub fn config_new_from_json5(s: &str) -> Result<Config, Error> {
    // Stable serde path (`Config: Deserialize`), matching zenoh-c's
    // `json5::from_str`. (`Config::from_deserializer` is an `#[unstable]` API.)
    json5::from_str::<Config>(s).map_err(|e| format!("JSON error: {e}").into())
}

/// Parse a configuration from a YAML-formatted string.
#[prebindgen]
pub fn config_new_from_yaml(s: &str) -> Result<Config, Error> {
    serde_yaml::from_str::<Config>(s).map_err(|e| format!("YAML error: {e}").into())
}

/// Return the JSON value associated with `key` in the configuration.
#[prebindgen]
pub fn config_get_json(c: &Config, key: &str) -> Result<String, Error> {
    c.get_json(key)
}

/// Insert a JSON5-formatted value at `key` in the configuration.
#[prebindgen]
pub fn config_insert_json5(c: &mut Config, key: &str, value: &str) -> Result<(), Error> {
    c.insert_json5(key, value)?;
    Ok(())
}
