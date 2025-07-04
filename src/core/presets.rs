use crate::shared::types::Preset;
use std::fs;
use std::io::Write;

pub const PRESETS_URL: &str =
    "https://raw.githubusercontent.com/Nicolhetti/DSQProcess/refs/heads/master/presets.json";

pub fn load_presets() -> Vec<Preset> {
    if let Ok(data) = fs::read_to_string("presets.json") {
        if let Ok(presets) = serde_json::from_str(&data) {
            return presets;
        }
    }
    vec![]
}

pub fn is_presets_outdated() -> bool {
    let remote = reqwest::blocking::get(PRESETS_URL).and_then(|r| r.text());
    let local = fs::read_to_string("presets.json");

    match (remote, local) {
        (Ok(remote), Ok(local)) => remote.trim() != local.trim(),
        _ => false,
    }
}

pub fn update_presets_file() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(PRESETS_URL)?.text()?;
    let mut file = fs::File::create("presets.json")?;
    file.write_all(response.as_bytes())?;
    Ok(())
}
