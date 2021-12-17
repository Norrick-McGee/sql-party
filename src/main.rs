use eframe::{egui, epi};

#[derive(Default)]
struct SqlPartyApp {}

impl epi::App for SqlPartyApp {
    fn name(&self) -> &str {
        "SQL-Party"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Time to Party!");
        });
    }
}

fn main() {
    let app = SqlPartyApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
