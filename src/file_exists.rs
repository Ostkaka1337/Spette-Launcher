use std::path::Path;
use crate::update_manager::check_for_updates;

// TODO: Honestly not much. We'll see as I change other things.
pub fn check_for_file(relative_path: &str) -> bool{
    let exists = Path::new(relative_path).exists();

    if exists {
        println!("Found file: {relative_path}");
        println!("Checking for updates...");
        check_for_updates("https://raw.githubusercontent.com/Ostkaka1337/Spette-Launcher/refs/heads/versions/app1version.txt", "src/applications/app1_dir/dummy_note.txt");
    }else{
        println!("File not found. Please install the application.");
    }
    exists
}