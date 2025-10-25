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

/// Load and return all presets from the official and custom presets files.
///
/// For each preset loaded from the official presets file, `is_custom` is set to `false`.
/// For each preset loaded from the custom presets file, `is_custom` is set to `true`.
/// If either file is missing or contains invalid JSON, that file is ignored and loading continues with the other.
/// The returned vector contains the combined presets (official first, then custom).
///
/// # Examples
///
/// ```no_run
/// let presets = load_presets();
/// // `presets` contains presets from the bundled `presets.json` and the user `custom_presets.json`
/// ```
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

/// Add a new custom preset, enforcing a unique name compared case-insensitively.
///
/// # Parameters
///
/// - `preset`: The preset to add; its `name` must not match (case-insensitive) any existing custom preset.
///
/// # Returns
///
/// `Ok(())` if the preset was added and persisted successfully, `Err` if a preset with the same name exists or if saving fails.
///
/// # Examples
///
/// ```
/// let preset = Preset { name: "my-custom".into(), ..Default::default() };
/// add_preset(preset).unwrap();
/// ```
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

/// Remove a custom preset by its name.
///
/// The comparison is case-sensitive. If no preset matches `preset_name`, the function does nothing
/// and still returns success. An error is returned only if persisting the updated custom presets fails.
///
/// # Arguments
///
/// * `preset_name` - The name of the custom preset to remove (case-sensitive).
///
/// # Returns
///
/// `Ok(())` on success, or an error if saving the updated custom presets fails.
///
/// # Examples
///
/// ```
/// // Attempt to delete a preset named "my-preset"
/// let res = delete_custom_preset("my-preset");
/// assert!(res.is_ok());
/// ```
pub fn delete_custom_preset(preset_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut custom_presets = load_custom_presets();
    custom_presets.retain(|p| p.name != preset_name);
    save_custom_presets(&custom_presets)?;
    Ok(())
}

/// Replaces an existing custom preset identified by `old_name` with `new_preset` and persists the change.
///
/// This searches the stored custom presets for an entry whose `name` exactly equals `old_name`.
/// If found, the entry is replaced with `new_preset` and the custom presets file is updated.
/// If no matching preset is found, an error is returned.
///
/// # Parameters
///
/// - `old_name`: The name of the custom preset to replace.
/// - `new_preset`: The new preset that will replace the existing one.
///
/// # Returns
///
/// `Ok(())` if the preset was replaced and saved successfully; `Err` if no preset with `old_name` exists
/// or if saving the updated presets fails.
///
/// # Examples
///
/// ```rust,no_run
/// let new = Preset { name: "my-preset".into(), /* other fields */ };
/// // Replaces the custom preset named "old-preset" with `new`.
/// let result = edit_custom_preset("old-preset", new);
/// assert!(result.is_ok());
/// ```
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

/// Load presets metadata from the metadata file.
///
/// Attempts to read and parse the presets metadata file (PRESETS_METADATA_FILE) into a `PresetsMetadata`.
/// If the file does not exist or cannot be parsed, returns `PresetsMetadata::default()`.
///
/// # Examples
///
/// ```
/// let meta = load_metadata();
/// // use metadata fields such as `version`
/// println!("{}", meta.version);
/// ```
fn load_metadata() -> PresetsMetadata {
    if let Ok(data) = fs::read_to_string(PRESETS_METADATA_FILE) {
        if let Ok(metadata) = serde_json::from_str(&data) {
            return metadata;
        }
    }
    PresetsMetadata::default()
}

/// Persist presets metadata to the configured metadata file.
///
/// Serializes `metadata` as pretty JSON and writes it to the `PRESETS_METADATA_FILE`, creating or truncating the file.
///
/// # Errors
///
/// Returns an `Err` if serialization fails or if creating/writing the metadata file fails.
///
/// # Examples
///
/// ```
/// let meta = PresetsMetadata { version: "1.2.3".into(), last_check: 0, hash: "".into() };
/// save_metadata(&meta).expect("failed to save presets metadata");
/// ```
fn save_metadata(metadata: &PresetsMetadata) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(metadata)?;
    let mut file = fs::File::create(PRESETS_METADATA_FILE)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

/// Compute a non-cryptographic, deterministic hash of a string.
///
/// The returned value is a hex-encoded 64-bit hash derived from the input string; it is suitable for change detection or caching but is not cryptographically secure.
///
/// # Examples
///
/// ```
/// let a = calculate_hash("example");
/// let b = calculate_hash("example");
/// assert_eq!(a, b);
/// ```
fn calculate_hash(content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Get the current UNIX timestamp in seconds.
///
/// # Examples
///
/// ```
/// let ts = current_timestamp();
/// assert!(ts > 0);
/// ```
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Determines whether the presets metadata cache has expired.
///
/// `last_check` is the UNIX timestamp in seconds when the cache was last validated.
/// The cache is considered expired when the elapsed time since `last_check` exceeds `CACHE_TTL_SECONDS`.
///
/// # Examples
///
/// ```
/// let now = current_timestamp();
/// // freshly checked -> not expired
/// assert_eq!(is_cache_expired(now), false);
/// // simulate an old timestamp far in the past -> expired
/// assert_eq!(is_cache_expired(0), true);
/// ```
fn is_cache_expired(last_check: u64) -> bool {
    let now = current_timestamp();
    now - last_check > CACHE_TTL_SECONDS
}

/// Estructura para el response de GitHub API
#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

/// Determine whether the local presets are outdated by consulting cached metadata and, when the cache has expired, comparing with the remote release version.
///
/// This updates the metadata's last-check timestamp and preserves the stored hash; if a remote version is successfully fetched and differs from the cached version, the function returns `true`. If the remote check fails, the function conservatively returns `false`.
///
/// # Examples
///
/// ```
/// # use crate::core::presets::is_presets_outdated;
/// let _ = is_presets_outdated();
/// ```
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

/// Fetches the latest presets release version from the GitHub Releases API.
///
/// On an HTTP 404 (release not found) this returns the fallback version `"1.0.0"`.
///
/// # Returns
///
/// `String` containing the release tag with a leading `v` removed (for example `"2.3.0"`).
///
/// # Examples
///
/// ```
/// let version = check_remote_version().unwrap();
/// // version is a string like "1.0.0" or "2.3.0"
/// assert!(version.chars().next().unwrap().is_ascii_digit());
/// ```
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
    Ok(release.tag_name.trim_start_matches('v').to_string())
}

/// Download and replace the local presets.json with the presets.json asset from the repository's GitHub release, then update local metadata.
///
/// On success the local presets file at PRESETS_FILE is overwritten and presets metadata (version, last_check, hash) is saved. Returns an error if the release or the presets.json asset cannot be found, if the downloaded content is not valid presets JSON, or on any network or filesystem failure.
///
/// # Examples
///
/// ```
/// // Attempt to update the local presets file from GitHub.
/// // Handle the result according to your application's error policy.
/// if let Err(e) = update_presets_file() {
///     eprintln!("Failed to update presets: {}", e);
/// }
/// ```
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
        version: release.tag_name.trim_start_matches('v').to_string(),
        last_check: current_timestamp(),
        hash: calculate_hash(&presets_content),
    };
    save_metadata(&metadata)?;

    Ok(())
}

/// Force a remote presets version check and update the metadata's last-check timestamp.
///
/// Updates the stored metadata's `last_check` to the current time and compares the remote
/// release tag to the stored `version`.
///
/// # Returns
///
/// `true` if the remote presets version differs from the stored version, `false` otherwise or if the remote check fails.
///
/// # Examples
///
/// ```no_run
/// let _ = crate::core::presets::_force_check_updates();
/// ```
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