use eframe::egui;

pub fn status_card(ui: &mut egui::Ui, status_text: &str) {
    ui.group(|ui| {
        ui.set_min_width(350.0);
        ui.vertical_centered(|ui| {
            ui.label(status_text);
        });
    });
}
