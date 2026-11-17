use std::fs;

pub fn check_for_updates(url: &str, local_version_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let remote_version = ureq::get(url)
        .call()?
        .body_mut()
        .read_to_string()?;

    let local_version = fs::read_to_string(local_version_path).unwrap_or_default();

    // Compares the local version note to the remote version note to decide whether an update can be done
    let needs_update = remote_version.trim() != local_version.trim();

    // Prints the remote version and local version
    println!("Remote: {remote_version} | Local: {local_version} | Update needed?: {needs_update}");

    // TODO: Figure out what the fuck to do from here
    Ok(needs_update)




}