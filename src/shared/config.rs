use crate::shared::types::Config;
use std::fs;

pub fn save_config(config: &Config) {
    let _ = fs::write("config.json", serde_json::to_string_pretty(config).unwrap_or_default());
}

pub fn load_config() -> Config {
    let data = fs::read_to_string("config.json").unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_else(|_| Config::new())
}
