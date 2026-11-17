use crate::download::download_update_app;
use crate::file_exists::check_for_file;

use eframe::egui;

#[derive(Default)]
pub struct DisplayGUI {}

impl DisplayGUI{
    pub fn new(cc: &eframe::CreationContext<'_>)->Self{
        Self::default()
    }
}

impl eframe::App for DisplayGUI{
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello World!");
            ui.label("App1");
            // TODO: Each button should only pass the application name
            // Everything else will be sorted further down the line.
            if ui.button("Start").clicked(){
                check_for_file("src/applications/app1_dir/dummy_app.txt");
            }
        });
    }
}