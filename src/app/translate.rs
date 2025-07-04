use super::state::DsqApp;

pub fn translate(app: &DsqApp, key: &str) -> String {
    app.langs
        .get(&app.selected_lang)
        .and_then(|map| map.get(key))
        .cloned()
        .unwrap_or_else(|| key.to_string())
}
