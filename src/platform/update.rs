pub const VERSION: &str = "0.4.0";

pub fn check_for_updates(
    current_version: &str
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/Nicolhetti/DSQProcess/releases/latest";
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).header("User-Agent", "dsqprocess").send()?;

    let json: serde_json::Value = response.json()?;
    let latest_version = json["tag_name"].as_str().unwrap_or("v0.0.0").trim_start_matches('v');
    let current = semver::Version::parse(current_version)?;
    let latest = semver::Version::parse(latest_version)?;

    if latest > current {
        Ok(Some(json["assets"][0]["browser_download_url"].as_str().unwrap_or("").to_string()))
    } else {
        Ok(None)
    }
}
