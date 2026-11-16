mod display_hub;
mod download;
pub mod file_exists;
pub mod update_manager;

fn main() {
    // Window name/options
    let window_name = "Hub";
    let options = eframe::NativeOptions::default();
    let app_creator: eframe::AppCreator<'_> =
        Box::new(|cc|Ok(Box::new(display_hub::DisplayGUI::new(cc))));
    // Initialize window
    eframe::run_native(window_name, options, app_creator).unwrap();
}
