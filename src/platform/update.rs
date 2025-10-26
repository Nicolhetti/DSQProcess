pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn check_for_updates(
    current_version: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/Nicolhetti/DSQProcess/releases/latest";
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent(format!("DSQProcess/{}", VERSION))
        .build()?;

    let response = client
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .send()?;

    let status = response.status().as_u16();

    match status {
        200..=299 => {}
        404 => {
            log::warn!("No releases found (404)");
            return Ok(None);
        }
        403 => {
            log::warn!("GitHub API rate limit exceeded (403)");
            return Ok(None);
        }
        _ => {
            log::warn!("Unexpected status code: {}", status);
            return Ok(None);
        }
    }

    let json: serde_json::Value = response.json()?;

    // Obtener el tag_name del release
    let tag_name = json["tag_name"].as_str().unwrap_or("").trim();

    // IMPORTANTE: Filtrar releases que no sean versiones de la app
    // Los releases de presets usan el tag "presets", ignorarlos
    if tag_name.is_empty() || tag_name.eq_ignore_ascii_case("presets") {
        log::info!("Skipping non-version release: {}", tag_name);
        return Ok(None);
    }

    // Extraer la versión (eliminar 'v' si existe)
    let latest_version = tag_name.trim_start_matches('v');

    // Validar que sea una versión válida antes de parsear
    if !is_valid_semver(latest_version) {
        log::warn!("Invalid version format in release: {}", tag_name);
        return Ok(None);
    }

    // Parsear versiones
    let current = semver::Version::parse(current_version)?;
    let latest = match semver::Version::parse(latest_version) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Failed to parse version '{}': {}", latest_version, e);
            return Ok(None);
        }
    };

    // Comparar versiones
    if latest > current {
        log::info!(
            "Update available: {} -> {}",
            current_version,
            latest_version
        );

        // Obtener la URL del asset (el .zip)
        let asset_url = json
            .get("assets")
            .and_then(|a| a.as_array())
            .and_then(|assets| {
                // Buscar el asset que contenga "DSQProcess" y termine en ".zip"
                assets.iter().find(|asset| {
                    if let Some(name) = asset.get("name").and_then(|n| n.as_str()) {
                        name.contains("DSQProcess") && name.ends_with(".zip")
                    } else {
                        false
                    }
                })
            })
            .and_then(|a| a.get("browser_download_url"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        if let Some(url) = asset_url {
            return Ok(Some(url));
        } else {
            // Fallback: usar la página del release si no hay asset
            let release_url = json
                .get("html_url")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            return Ok(release_url);
        }
    }

    log::info!("Already on latest version: {}", current_version);
    Ok(None)
}

/// Valida que un string sea una versión semántica válida (X.Y.Z)
fn is_valid_semver(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();

    // Debe tener exactamente 3 partes
    if parts.len() != 3 {
        return false;
    }

    // Cada parte debe ser un número
    parts
        .iter()
        .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
}
