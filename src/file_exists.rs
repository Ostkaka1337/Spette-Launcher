use std::path::Path;
pub fn check_for_file(relative_path: &str) -> bool{
    let exists = Path::new(relative_path).exists();

    if exists {
        println!("Found file: {relative_path}");
        println!("Checking for updates...");
        // More here later
    }else{
        println!("File not found. Please install the application.");
    }
    exists
}