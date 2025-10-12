use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

pub type LangMap = HashMap<String, String>;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub language: String,
    pub selected_preset: usize,
    pub process_name: String,
    pub custom_path: String,
    pub rich_presence_enabled: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            language: "Español".to_string(),
            selected_preset: 0,
            process_name: String::new(),
            custom_path: String::new(),
            rich_presence_enabled: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Preset {
    pub name: String,
    pub executable: String,
    pub path: String,
    #[serde(default)]
    pub is_custom: bool,
}
