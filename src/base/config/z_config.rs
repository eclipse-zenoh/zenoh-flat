use prebindgen_proc_macro::prebindgen;

use crate::{ZConfig, ZError};

/// Build a default configuration.
#[prebindgen]
pub fn z_config_default() -> ZConfig {
    ZConfig::default()
}

/// Clone a configuration handle. Use this before passing a config to a
/// consuming call (`z_open`) when the caller needs to keep the original.
#[prebindgen]
pub fn z_config_clone(c: &ZConfig) -> ZConfig {
    c.clone()
}

/// Load a configuration from a file path. The file extension determines
/// the format (JSON, JSON5, or YAML).
#[prebindgen]
pub fn z_config_from_file(path: &str) -> Result<ZConfig, ZError> {
    Ok(ZConfig::from_file(path)?)
}

/// Parse a configuration from a JSON-formatted string. JSON is a subset
/// of JSON5, so routing through the JSON5 deserializer is sufficient.
#[prebindgen]
pub fn z_config_from_json(s: &str) -> Result<ZConfig, ZError> {
    z_config_from_json5(s)
}

/// Parse a configuration from a JSON5-formatted string.
#[prebindgen]
pub fn z_config_from_json5(s: &str) -> Result<ZConfig, ZError> {
    // Stable serde path (`Config: Deserialize`), matching zenoh-c's
    // `json5::from_str`. (`Config::from_deserializer` is an `#[unstable]` API.)
    json5::from_str::<ZConfig>(s).map_err(|e| format!("JSON error: {e}").into())
}

/// Parse a configuration from a YAML-formatted string.
#[prebindgen]
pub fn z_config_from_yaml(s: &str) -> Result<ZConfig, ZError> {
    serde_yaml::from_str::<ZConfig>(s).map_err(|e| format!("YAML error: {e}").into())
}

/// Return the JSON value associated with `key` in the configuration.
#[prebindgen]
pub fn z_config_get_json(c: &ZConfig, key: &str) -> Result<String, ZError> {
    Ok(c.get_json(key)?)
}

/// Insert a JSON5-formatted value at `key` in the configuration.
#[prebindgen]
pub fn z_config_insert_json5(c: &mut ZConfig, key: &str, value: &str) -> Result<(), ZError> {
    c.insert_json5(key, value)?;
    Ok(())
}
