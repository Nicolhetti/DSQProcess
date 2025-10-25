use crate::shared::types::Preset;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

// Usar GitHub Releases API
const GITHUB_API_URL: &str =
    "https://api.github.com/repos/Nicolhetti/DSQProcess/releases/tags/presets";
const PRESETS_FILE: &str = "presets.json";
const CUSTOM_PRESETS_FILE: &str = "presets_custom.json";
const PRESETS_METADATA_FILE: &str = "presets_metadata.json";

// Cache de 6 horas para evitar demasiadas peticiones
const CACHE_TTL_SECONDS: u64 = 21600;

#[derive(Serialize, Deserialize, Default)]
struct PresetsMetadata {
    version: String,
    last_check: u64,
    hash: String,
}

/// Carga todos los presets (oficiales + personalizados)
pub fn load_presets() -> Vec<Preset> {
    let mut all_presets = Vec::new();

    // Cargar presets oficiales
    if let Ok(data) = fs::read_to_string(PRESETS_FILE) {
        if let Ok(mut presets) = serde_json::from_str::<Vec<Preset>>(&data) {
            for preset in &mut presets {
                preset.is_custom = false;
            }
            all_presets.extend(presets);
        }
    }

    // Cargar presets personalizados
    if let Ok(data) = fs::read_to_string(CUSTOM_PRESETS_FILE) {
        if let Ok(mut presets) = serde_json::from_str::<Vec<Preset>>(&data) {
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

    if custom_presets
        .iter()
        .any(|p| p.name.eq_ignore_ascii_case(&preset.name))
    {
        return Err("A preset with this name already exists".into());
    }

    custom_presets.push(preset);
    save_custom_presets(&custom_presets)?;
    Ok(())
}

/// Elimina un preset personalizado por nombre
pub fn delete_custom_preset(preset_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut custom_presets = load_custom_presets();
    custom_presets.retain(|p| p.name != preset_name);
    save_custom_presets(&custom_presets)?;
    Ok(())
}

/// Edita un preset personalizado existente
pub fn edit_custom_preset(
    old_name: &str,
    new_preset: Preset,
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

/// Carga los metadatos de presets
fn load_metadata() -> PresetsMetadata {
    if let Ok(data) = fs::read_to_string(PRESETS_METADATA_FILE) {
        if let Ok(metadata) = serde_json::from_str(&data) {
            return metadata;
        }
    }
    PresetsMetadata::default()
}

/// Guarda los metadatos de presets
fn save_metadata(metadata: &PresetsMetadata) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(metadata)?;
    let mut file = fs::File::create(PRESETS_METADATA_FILE)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/// Calcula el hash SHA-256 criptográfico de un string
///
/// Usa SHA-256 en lugar de DefaultHasher porque:
/// - Es determinístico (mismo contenido = mismo hash siempre)
/// - Es criptográficamente seguro (resistente a colisiones)
/// - Es estándar de la industria para verificación de integridad
///
/// Retorna el hash como string hexadecimal en minúsculas (64 caracteres)
fn calculate_hash(content: &str) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();

    hex::encode(result)
}

/// Obtiene el timestamp actual
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Verifica si el cache ha expirado
fn is_cache_expired(last_check: u64) -> bool {
    let now = current_timestamp();
    // Usar saturating_sub para evitar underflow si el reloj del sistema retrocede
    now.saturating_sub(last_check) > CACHE_TTL_SECONDS
}

/// Estructura para el response de GitHub API
#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    name: Option<String>,
    assets: Vec<GitHubAsset>,
}

#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
    updated_at: Option<String>, // fallback marker
}

fn extract_remote_version(rel: &GitHubRelease) -> String {
    if let Some(title) = &rel.name {
        // naive parse: last 'v' followed by digits/dots (vX.Y.Z)
        if let Some(i) = title.rfind('v') {
            let cand = title[i + 1..].trim();
            if cand.chars().all(|c| c.is_ascii_digit() || c == '.') && cand.split('.').count() == 3
            {
                return cand.to_string();
            }
        }
    }
    if let Some(a) = rel.assets.iter().find(|a| a.name == "presets.json") {
        if let Some(ts) = &a.updated_at {
            return ts.clone();
        }
    }
    rel.tag_name.clone()
}

/// Verifica si los presets están desactualizados (con cache)
pub fn is_presets_outdated() -> bool {
    let metadata = load_metadata();

    // Si el cache no ha expirado, usar el valor cacheado
    if !is_cache_expired(metadata.last_check) {
        return false;
    }

    // Si el cache expiró, verificar remotamente
    match check_remote_version() {
        Ok(remote_version) => {
            let is_outdated = remote_version != metadata.version;

            // Actualizar metadata con el timestamp actual
            let new_metadata = PresetsMetadata {
                version: if is_outdated {
                    remote_version
                } else {
                    metadata.version.clone()
                },
                last_check: current_timestamp(),
                hash: metadata.hash.clone(),
            };
            let _ = save_metadata(&new_metadata);

            is_outdated
        }
        Err(_) => {
            // En caso de error, asumir que no está desactualizado
            // para evitar mostrar alertas innecesarias
            false
        }
    }
}

/// Verifica la versión remota de presets
fn check_remote_version() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let response = client
        .get(GITHUB_API_URL)
        .header("User-Agent", "DSQProcess")
        .send()?;

    if response.status().as_u16() == 404 {
        // Si no existe el release de presets, usar fallback
        return Ok("1.0.0".to_string());
    }

    let release: GitHubRelease = response.json()?;
    Ok(extract_remote_version(&release))
}

/// Actualiza el archivo de presets desde GitHub Release
pub fn update_presets_file() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let response = client
        .get(GITHUB_API_URL)
        .header("User-Agent", "DSQProcess")
        .send()?;

    if response.status().as_u16() == 404 {
        return Err("Presets release not found. Using local version.".into());
    }

    let release: GitHubRelease = response.json()?;

    // Buscar el asset de presets.json
    let preset_asset = release
        .assets
        .iter()
        .find(|asset| asset.name == "presets.json")
        .ok_or("presets.json not found in release")?;

    // Descargar el archivo
    let presets_content = client
        .get(&preset_asset.browser_download_url)
        .header("User-Agent", "DSQProcess")
        .send()?
        .text()?;

    // Verificar que sea JSON válido
    let _: Vec<Preset> = serde_json::from_str(&presets_content)?;

    // Guardar el archivo
    let mut file = fs::File::create(PRESETS_FILE)?;
    file.write_all(presets_content.as_bytes())?;

    // Actualizar metadata
    let metadata = PresetsMetadata {
        version: extract_remote_version(&release),
        last_check: current_timestamp(),
        hash: calculate_hash(&presets_content),
    };
    save_metadata(&metadata)?;

    Ok(())
}

/// Fuerza una verificación remota ignorando el cache
pub fn _force_check_updates() -> bool {
    match check_remote_version() {
        Ok(remote_version) => {
            let metadata = load_metadata();
            let is_outdated = remote_version != metadata.version;

            // Actualizar el timestamp de última verificación
            let new_metadata = PresetsMetadata {
                version: metadata.version.clone(),
                last_check: current_timestamp(),
                hash: metadata.hash.clone(),
            };
            let _ = save_metadata(&new_metadata);

            is_outdated
        }
        Err(_) => false,
    }
}
