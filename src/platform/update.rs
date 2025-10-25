pub const VERSION: &str = "0.4.4";

/// Checks GitHub for a newer release than the provided version and returns its download URL if one exists.
///
/// The function queries the repository's latest release, compares its semantic version to `current_version`,
/// and, if the latest is greater, returns the first asset's browser download URL.
/// Network, JSON parsing, and semantic version parsing errors are propagated to the caller.
///
/// # Parameters
///
/// * `current_version` - The current semantic version string to compare (e.g., `"0.4.4"`).
///
/// # Returns
///
/// `Ok(Some(url))` with the release asset download URL if a newer release is available, `Ok(None)` if the current version is up to date, or `Err` if an IO/HTTP/JSON/semver error occurs.
///
/// # Examples
///
/// ```
/// let current = "0.4.4";
/// match check_for_updates(current) {
///     Ok(Some(url)) => println!("Update available: {}", url),
///     Ok(None) => println!("Already up to date"),
///     Err(e) => eprintln!("Update check failed: {}", e),
/// }
/// ```
pub fn check_for_updates(
    current_version: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/Nicolhetti/DSQProcess/releases/latest";
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).header("User-Agent", "dsqprocess").send()?;

    let json: serde_json::Value = response.json()?;
    let latest_version = json["tag_name"]
        .as_str()
        .unwrap_or("v0.0.0")
        .trim_start_matches('v');
    let current = semver::Version::parse(current_version)?;
    let latest = semver::Version::parse(latest_version)?;

    if latest > current {
        Ok(Some(
            json["assets"][0]["browser_download_url"]
                .as_str()
                .unwrap_or("")
                .to_string(),
        ))
    } else {
        Ok(None)
    }
}