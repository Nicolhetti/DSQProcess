use crate::shared::types::Preset;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

const GITHUB_API_URL: &str =
    "https://api.github.com/repos/Nicolhetti/DSQProcess/releases/tags/presets";
const PRESETS_FILE: &str = "presets.json";
const CUSTOM_PRESETS_FILE: &str = "presets_custom.json";
const PRESETS_METADATA_FILE: &str = "presets_metadata.json";
const APP_UA: &str = concat!("DSQProcess/", env!("CARGO_PKG_VERSION"));
const CACHE_TTL_SECONDS: u64 = 21600; // 6 horas

#[derive(Serialize, Deserialize, Default, Clone)]
struct PresetsMetadata {
    version: String,
    last_check: u64,
    hash: String,
}

/// Carga todos los presets (oficiales + personalizados)
pub fn load_presets() -> Vec<Preset> {
    let mut all_presets = Vec::new();

    // Cargar presets oficiales
    match fs::read_to_string(PRESETS_FILE) {
        Ok(data) => match serde_json::from_str::<Vec<Preset>>(&data) {
            Ok(mut presets) => {
                for preset in &mut presets {
                    preset.is_custom = false;
                }
                all_presets.extend(presets);
            }
            Err(e) => {
                log::error!("Failed to parse {}: {}", PRESETS_FILE, e);
            }
        },
        Err(e) => {
            log::warn!("Failed to read {}: {}", PRESETS_FILE, e);
        }
    }

    // Cargar presets personalizados
    match fs::read_to_string(CUSTOM_PRESETS_FILE) {
        Ok(data) => match serde_json::from_str::<Vec<Preset>>(&data) {
            Ok(mut presets) => {
                for preset in &mut presets {
                    preset.is_custom = true;
                }
                all_presets.extend(presets);
            }
            Err(e) => {
                log::error!("Failed to parse {}: {}", CUSTOM_PRESETS_FILE, e);
            }
        },
        Err(_) => {
            // Es normal que no exista al inicio
            log::debug!("Custom presets file not found (expected on first run)");
        }
    }

    all_presets
}

/// Escritura atómica de archivos para evitar corrupción
fn write_atomic(path: &str, contents: &[u8]) -> std::io::Result<()> {
    use std::{fs, io::Write, path::Path};

    let tmp = format!("{}.tmp", path);

    // Escribir a archivo temporal
    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(contents)?;
        f.sync_all()?; // Forzar flush a disco
    }

    // Renombrar atómicamente
    if let Err(e) = fs::rename(&tmp, path) {
        // En Windows, puede ser necesario eliminar el archivo existente primero
        if Path::new(path).exists() {
            fs::remove_file(path)?;
        }
        fs::rename(&tmp, path).map_err(|_| e)?;
    }

    Ok(())
}

/// Carga solo los presets personalizados
fn load_custom_presets() -> Result<Vec<Preset>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(CUSTOM_PRESETS_FILE)?;
    let presets = serde_json::from_str(&data)?;
    Ok(presets)
}

/// Guarda los presets personalizados
fn save_custom_presets(presets: &[Preset]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(presets)?;
    write_atomic(CUSTOM_PRESETS_FILE, json.as_bytes())?;
    log::info!("Saved {} custom presets", presets.len());
    Ok(())
}

/// Agrega un nuevo preset personalizado
pub fn add_preset(preset: Preset) -> Result<(), Box<dyn std::error::Error>> {
    let mut custom_presets = load_custom_presets().unwrap_or_default();

    // Verificar duplicados (case-insensitive)
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
    let mut custom_presets = load_custom_presets().unwrap_or_default();
    let original_len = custom_presets.len();

    custom_presets.retain(|p| !p.name.eq_ignore_ascii_case(preset_name));

    if custom_presets.len() == original_len {
        return Err("Preset not found".into());
    }

    save_custom_presets(&custom_presets)?;
    Ok(())
}

/// Edita un preset personalizado existente
pub fn edit_custom_preset(
    old_name: &str,
    new_preset: Preset,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut custom_presets = load_custom_presets().unwrap_or_default();

    // Verificar si el nuevo nombre ya existe (excluyendo el preset actual)
    if custom_presets.iter().any(|p| {
        p.name.eq_ignore_ascii_case(&new_preset.name) && !p.name.eq_ignore_ascii_case(old_name)
    }) {
        return Err("A preset with this name already exists".into());
    }

    // Buscar y actualizar el preset
    let preset = custom_presets
        .iter_mut()
        .find(|p| p.name.eq_ignore_ascii_case(old_name))
        .ok_or("Preset not found")?;

    *preset = new_preset;
    save_custom_presets(&custom_presets)?;
    Ok(())
}

/// Carga los metadatos de presets
fn load_metadata() -> PresetsMetadata {
    match fs::read_to_string(PRESETS_METADATA_FILE) {
        Ok(data) => match serde_json::from_str(&data) {
            Ok(metadata) => metadata,
            Err(e) => {
                log::warn!("Failed to parse metadata: {}", e);
                PresetsMetadata::default()
            }
        },
        Err(_) => PresetsMetadata::default(),
    }
}

/// Guarda los metadatos de presets
fn save_metadata(metadata: &PresetsMetadata) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(metadata)?;
    write_atomic(PRESETS_METADATA_FILE, json.as_bytes())?;
    Ok(())
}

/// Calcula el hash SHA-256 del contenido
fn calculate_hash(content: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    hex::encode(hasher.finalize())
}

/// Obtiene el timestamp actual en segundos
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Verifica si el cache ha expirado
fn is_cache_expired(last_check: u64) -> bool {
    let now = current_timestamp();
    now.saturating_sub(last_check) > CACHE_TTL_SECONDS
}

#[derive(Deserialize, Debug)]
struct GitHubRelease {
    tag_name: String,
    name: Option<String>,
    assets: Vec<GitHubAsset>,
}

#[derive(Deserialize, Debug)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
    updated_at: Option<String>,
}

/// Extrae la versión del release de forma robusta
fn extract_remote_version(rel: &GitHubRelease) -> String {
    // Intentar parsear desde el nombre del release
    if let Some(title) = &rel.name {
        // Buscar patrón vX.Y.Z o X.Y.Z
        if let Some(version) = extract_version_from_string(title) {
            return version;
        }
    }

    // Fallback: usar tag_name
    if let Some(version) = extract_version_from_string(&rel.tag_name) {
        return version;
    }

    // Último fallback: timestamp del asset
    if let Some(asset) = rel.assets.iter().find(|a| a.name == "presets.json") {
        if let Some(ts) = &asset.updated_at {
            return ts.clone();
        }
    }

    // Si todo falla, usar el tag tal cual
    rel.tag_name.clone()
}

/// Extrae versión semántica de un string
fn extract_version_from_string(s: &str) -> Option<String> {
    // Buscar patrón X.Y.Z (con o sin 'v' adelante)
    let re = regex::Regex::new(r"v?(\d+\.\d+\.\d+)").ok()?;
    re.captures(s)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

/// Verifica si los presets están desactualizados (con cache inteligente)
pub fn is_presets_outdated() -> bool {
    let metadata = load_metadata();

    // Si el cache no ha expirado, confiar en el cache
    if !is_cache_expired(metadata.last_check) {
        log::debug!("Using cached presets status");
        return false;
    }

    // Cache expirado, verificar remotamente
    log::info!("Cache expired, checking remote version");
    match check_remote_version() {
        Ok(remote_version) => {
            let is_outdated = remote_version != metadata.version;
            log::info!(
                "Local: {}, Remote: {}, Outdated: {}",
                metadata.version,
                remote_version,
                is_outdated
            );

            // Solo actualizar timestamp si NO está desactualizado
            if !is_outdated {
                let new_metadata = PresetsMetadata {
                    version: metadata.version,
                    last_check: current_timestamp(),
                    hash: metadata.hash,
                };
                let _ = save_metadata(&new_metadata);
            }

            is_outdated
        }
        Err(e) => {
            log::warn!("Failed to check remote version: {}", e);
            // En caso de error de red, asumir que está actualizado
            // para no molestar al usuario con alertas falsas
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
        .header("User-Agent", APP_UA)
        .header("Accept", "application/vnd.github+json")
        .send()?;

    let status = response.status().as_u16();

    if status == 404 {
        return Err("Presets release not found (404)".into());
    }

    if status == 403 {
        return Err("GitHub API rate limit exceeded (403)".into());
    }

    if !(200..300).contains(&status) {
        return Err(format!("Unexpected status code: {}", status).into());
    }

    let release: GitHubRelease = response.json()?;
    let version = extract_remote_version(&release);

    log::debug!("Remote version detected: {}", version);
    Ok(version)
}

/// Actualiza el archivo de presets desde GitHub Release
pub fn update_presets_file() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Starting presets update");

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // Obtener información del release
    let response = client
        .get(GITHUB_API_URL)
        .header("User-Agent", APP_UA)
        .header("Accept", "application/vnd.github+json")
        .send()?;

    if response.status().as_u16() == 404 {
        return Err("Presets release not found. Using local version.".into());
    }

    let release: GitHubRelease = response.json()?;

    // Buscar el asset presets.json
    let preset_asset = release
        .assets
        .iter()
        .find(|asset| asset.name == "presets.json")
        .ok_or("presets.json not found in release assets")?;

    log::info!(
        "Downloading presets from: {}",
        preset_asset.browser_download_url
    );

    // Descargar el archivo
    let presets_content = client
        .get(&preset_asset.browser_download_url)
        .header("User-Agent", APP_UA)
        .send()?
        .text()?;

    // Validar que sea JSON válido antes de guardar
    let validated_presets: Vec<Preset> = serde_json::from_str(&presets_content)
        .map_err(|e| format!("Downloaded presets are not valid JSON: {}", e))?;

    log::info!("Downloaded {} presets", validated_presets.len());

    // Guardar el archivo
    write_atomic(PRESETS_FILE, presets_content.as_bytes())?;

    // Actualizar metadata
    let metadata = PresetsMetadata {
        version: extract_remote_version(&release),
        last_check: current_timestamp(),
        hash: calculate_hash(&presets_content),
    };
    save_metadata(&metadata)?;

    log::info!(
        "Presets updated successfully to version: {}",
        metadata.version
    );
    Ok(())
}

/// Fuerza una verificación remota ignorando el cache
#[allow(dead_code)]
pub fn force_check_updates() -> bool {
    log::info!("Force checking for updates");

    match check_remote_version() {
        Ok(remote_version) => {
            let metadata = load_metadata();
            let is_outdated = remote_version != metadata.version;

            // Actualizar timestamp independientemente del resultado
            let new_metadata = PresetsMetadata {
                version: metadata.version,
                last_check: current_timestamp(),
                hash: metadata.hash,
            };
            let _ = save_metadata(&new_metadata);

            is_outdated
        }
        Err(e) => {
            log::error!("Force check failed: {}", e);
            false
        }
    }
}
