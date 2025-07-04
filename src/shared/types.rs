use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

pub type LangMap = HashMap<String, String>;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub language: String,
    pub selected_preset: usize,
    pub process_name: String,
    pub custom_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Preset {
    pub name: String,
    pub executable: String,
    pub path: String,
}
