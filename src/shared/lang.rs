use crate::shared::types::LangMap;

pub fn load_language(code: &str) -> LangMap {
    let path = format!("lang/{}.json", code);
    let data = std::fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}
