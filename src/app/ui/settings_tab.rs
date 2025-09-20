use eframe::egui;
use crate::app::state::DsqApp;
use crate::app::translate::translate;
use crate::shared::config::save_config;
use crate::shared::richpresence::RichPresenceManager;
use crate::shared::types::Config;

pub fn render(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.vertical_centered(|ui| {
        ui.heading(translate(app, "settings"));
        ui.add_space(20.0);

        render_language_settings(ui, app);
        ui.add_space(15.0);
        render_rich_presence_settings(ui, app);
        ui.add_space(20.0);
        render_settings_notice(ui, app);

        save_settings_config(app);
    });
}

fn render_language_settings(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.group(|ui| {
        ui.set_min_width(350.0);
        ui.vertical_centered(|ui| {
            ui.heading(translate(app, "language"));
            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(150.0);
                    egui::ComboBox
                        ::from_id_source("language_select")
                        .selected_text(&app.selected_lang)
                        .width(150.0)
                        .show_ui(ui, |ui| {
                            for key in app.langs.keys() {
                                ui.selectable_value(&mut app.selected_lang, key.clone(), key);
                            }
                        });
                });
            });
        });
    });
}

fn render_rich_presence_settings(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.group(|ui| {
        ui.set_min_width(350.0);
        ui.vertical_centered(|ui| {
            ui.heading("🎮 Discord Rich Presence");
            ui.add_space(10.0);

            let mut rich_presence_changed = false;
            let enable_rich_presence_text = translate(app, "enable_rich_presence");

            if ui.checkbox(&mut app.rich_presence_enabled, &enable_rich_presence_text).changed() {
                rich_presence_changed = true;
            }

            if rich_presence_changed {
                handle_rich_presence_toggle(app);
            }
        });
    });
}

fn handle_rich_presence_toggle(app: &mut DsqApp) {
    if app.rich_presence_enabled {
        if app.rich_presence.is_none() {
            let mut rp = RichPresenceManager::new();
            if let Ok(()) = rp.connect() {
                let activity = if let Some(ref game) = app.current_simulated_game {
                    Some(game.clone())
                } else {
                    None
                };
                let _ = rp.set_activity(activity);
                app.rich_presence = Some(rp);
            } else {
                app.status = translate(app, "rich_presence_error").replace(
                    "{error}",
                    "No se pudo conectar"
                );
                app.rich_presence_enabled = false;
            }
        }
    } else {
        if let Some(mut rp) = app.rich_presence.take() {
            let _ = rp.clear_activity();
            rp.disconnect();
        }
    }
}

fn render_settings_notice(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.separator();
    ui.add_space(10.0);
    ui.colored_label(egui::Color32::GRAY, translate(app, "settings_notice"));
}

fn save_settings_config(app: &DsqApp) {
    let config = Config {
        language: app.selected_lang.clone(),
        selected_preset: app.selected_preset,
        process_name: app.process_name.clone(),
        custom_path: app.custom_path.clone(),
        rich_presence_enabled: app.rich_presence_enabled,
    };
    save_config(&config);
}
