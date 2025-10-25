pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn check_for_updates(
    current_version: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/Nicolhetti/DSQProcess/releases/latest";
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent(format!("DSQProcess/{}", VERSION))
        .build()?;
    let response = client.get(url).send()?;
    if !response.status().is_success() {
        return Ok(None);
    }

    let json: serde_json::Value = response.json()?;
    let latest_version = json["tag_name"]
        .as_str()
        .unwrap_or("v0.0.0")
        .trim_start_matches('v');
    let current = semver::Version::parse(current_version)?;
    let latest = semver::Version::parse(latest_version)?;

    if latest > current {
        let asset_url = json
            .get("assets")
            .and_then(|a| a.as_array())
            .and_then(|a| a.first())
            .and_then(|a| a.get("browser_download_url"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        return Ok(asset_url);
    }
    Ok(None)
}
