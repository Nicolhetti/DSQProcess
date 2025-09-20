use eframe::egui;
use crate::app::state::DsqApp;
use crate::app::translate::translate;
use crate::platform::update::{ check_for_updates, VERSION };
use super::components;

pub fn render(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.vertical_centered(|ui| {
        ui.heading(translate(app, "info"));
        ui.add_space(20.0);

        render_version_info(ui, app);
        ui.add_space(20.0);
        render_status_if_exists(ui, app);
        render_credits(ui, app);
    });
}

fn render_version_info(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.group(|ui| {
        ui.set_min_width(350.0);
        ui.vertical_centered(|ui| {
            ui.heading(format!("DSQProcess v{}", VERSION));
            ui.add_space(10.0);

            if ui.button("🔍 ".to_string() + &translate(app, "check_update")).clicked() {
                handle_update_check(app);
            }
        });
    });
}

fn handle_update_check(app: &mut DsqApp) {
    match check_for_updates(VERSION) {
        Ok(Some(download_url)) => {
            app.status = translate(app, "update_available").replace("{url}", &download_url);
            let _ = open::that(download_url);
        }
        Ok(None) => {
            app.status = translate(app, "up_to_date");
        }
        Err(e) => {
            app.status = translate(app, "update_error").replace("{error}", &e.to_string());
        }
    }
}

fn render_status_if_exists(ui: &mut egui::Ui, app: &mut DsqApp) {
    if !app.status.is_empty() {
        ui.separator();
        ui.add_space(10.0);
        components::status_card(ui, &app.status);
        ui.add_space(15.0);
    }
}

fn render_credits(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.group(|ui| {
        ui.set_min_width(350.0);
        ui.vertical_centered(|ui| {
            ui.label(translate(app, "about_credit"));
        });
    });
}
