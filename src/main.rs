use eframe::{egui, epi};

#[derive(Default)]
struct SqlPartyApp {}

impl epi::App for SqlPartyApp {
    fn name(&self) -> &str {
        "SQL-Party"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.add(egui::Label::new("SQL Party!"));

            let mut mystr = String::from("mystr");
            let text_edit = egui::TextEdit::multiline(&mut mystr);
            let code_editor = ui.add(egui::TextEdit::code_editor(text_edit));

            // if code_editor.changed() {}
            // if code_editor.lost_focus() && ui.input().key_pressed(egui::Key::Enter) { }
            if ui.button("Exec SQL").clicked() { /* take some action here */ }
        });
    }
}

fn main() {
    let app = SqlPartyApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
