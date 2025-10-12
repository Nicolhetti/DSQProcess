use crate::shared::types::Preset;
use std::fs;
use std::io::Write;

pub const PRESETS_URL: &str =
    "https://raw.githubusercontent.com/Nicolhetti/DSQProcess/refs/heads/master/presets.json";

const PRESETS_FILE: &str = "presets.json";
const CUSTOM_PRESETS_FILE: &str = "presets_custom.json";

/// Carga todos los presets (oficiales + personalizados)
pub fn load_presets() -> Vec<Preset> {
    let mut all_presets = Vec::new();

    // Cargar presets oficiales
    if let Ok(data) = fs::read_to_string(PRESETS_FILE) {
        if let Ok(mut presets) = serde_json::from_str::<Vec<Preset>>(&data) {
            // Marcar como no personalizados
            for preset in &mut presets {
                preset.is_custom = false;
            }
            all_presets.extend(presets);
        }
    }

    // Cargar presets personalizados
    if let Ok(data) = fs::read_to_string(CUSTOM_PRESETS_FILE) {
        if let Ok(mut presets) = serde_json::from_str::<Vec<Preset>>(&data) {
            // Marcar como personalizados
            for preset in &mut presets {
                preset.is_custom = true;
            }
            all_presets.extend(presets);
        }
    }

    all_presets
}

/// Carga solo los presets personalizados
fn load_custom_presets() -> Vec<Preset> {
    if let Ok(data) = fs::read_to_string(CUSTOM_PRESETS_FILE) {
        if let Ok(presets) = serde_json::from_str(&data) {
            return presets;
        }
    }
    vec![]
}

/// Guarda los presets personalizados
fn save_custom_presets(presets: &[Preset]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(presets)?;
    let mut file = fs::File::create(CUSTOM_PRESETS_FILE)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/// Agrega un nuevo preset personalizado
pub fn add_preset(preset: Preset) -> Result<(), Box<dyn std::error::Error>> {
    let mut custom_presets = load_custom_presets();

    // Verificar que no exista un preset con el mismo nombre
    if custom_presets.iter().any(|p| p.name.eq_ignore_ascii_case(&preset.name)) {
        return Err("A preset with this name already exists".into());
    }

    custom_presets.push(preset);
    save_custom_presets(&custom_presets)?;
    Ok(())
}

/// Elimina un preset personalizado por índice
pub fn delete_custom_preset(preset_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut custom_presets = load_custom_presets();
    custom_presets.retain(|p| p.name != preset_name);
    save_custom_presets(&custom_presets)?;
    Ok(())
}

/// Edita un preset personalizado existente
pub fn edit_custom_preset(
    old_name: &str,
    new_preset: Preset
) -> Result<(), Box<dyn std::error::Error>> {
    let mut custom_presets = load_custom_presets();

    if let Some(preset) = custom_presets.iter_mut().find(|p| p.name == old_name) {
        *preset = new_preset;
        save_custom_presets(&custom_presets)?;
        Ok(())
    } else {
        Err("Preset not found".into())
    }
}

/// Verifica si los presets oficiales están desactualizados
pub fn is_presets_outdated() -> bool {
    let remote = reqwest::blocking::get(PRESETS_URL).and_then(|r| r.text());
    let local = fs::read_to_string(PRESETS_FILE);

    match (remote, local) {
        (Ok(remote), Ok(local)) => remote.trim() != local.trim(),
        _ => false,
    }
}

/// Actualiza solo el archivo de presets oficiales (no toca los personalizados)
pub fn update_presets_file() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(PRESETS_URL)?.text()?;
    let mut file = fs::File::create(PRESETS_FILE)?;
    file.write_all(response.as_bytes())?;
    Ok(())
}
