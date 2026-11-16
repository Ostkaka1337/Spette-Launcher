use std::fs;

pub fn check_for_updates(url: &str, local_version_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let remote_version = ureq::get(url)
        .call()?
        .body_mut()
        .read_to_string()?;

    let local_version = fs::read_to_string(local_version_path).unwrap_or_default();

    let needs_update = remote_version.trim() != local_version.trim();
    Ok(needs_update)
}