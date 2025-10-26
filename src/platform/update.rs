pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn check_for_updates(
    current_version: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/Nicolhetti/DSQProcess/releases";
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

    let releases: Vec<serde_json::Value> = response.json()?;

    // Filtrar los releases válidos (tag que empiece con 'v' y tenga formato semver)
    let mut valid_releases: Vec<_> = releases
        .into_iter()
        .filter_map(|r| {
            let tag = r["tag_name"].as_str()?.trim();
            if !tag.starts_with('v') {
                return None;
            }

            let version_str = tag.trim_start_matches('v');
            if semver::Version::parse(version_str).is_ok() {
                Some((r.clone(), version_str.to_string()))
            } else {
                None
            }
        })
        .collect();

    if valid_releases.is_empty() {
        log::warn!("No valid versioned releases found");
        return Ok(None);
    }

    // Ordenar por versión descendente
    valid_releases.sort_by(|a, b| {
        let va = semver::Version::parse(&a.1).unwrap();
        let vb = semver::Version::parse(&b.1).unwrap();
        vb.cmp(&va)
    });

    let latest_release = &valid_releases[0];
    let latest_version = &latest_release.1;
    let current = semver::Version::parse(current_version)?;
    let latest = semver::Version::parse(latest_version)?;

    if latest > current {
        log::info!(
            "Update available: {} -> {}",
            current_version,
            latest_version
        );
        let release_json = &latest_release.0;

        // Obtener URL del asset
        let asset_url = release_json["assets"].as_array().and_then(|assets| {
            assets
                .iter()
                .find_map(|a| a["browser_download_url"].as_str().map(|s| s.to_string()))
        });

        if let Some(url) = asset_url {
            return Ok(Some(url));
        } else {
            let html_url = release_json["html_url"].as_str().map(|s| s.to_string());
            return Ok(html_url);
        }
    }

    log::info!("Already on latest version: {}", current_version);
    Ok(None)
}
