use eframe::egui;
use super::state::DsqApp;
use crate::app::state::Tab;
use crate::app::translate::translate;
use crate::core::presets::{ load_presets, update_presets_file };
use crate::shared::config::save_config;
use crate::core::process::create_fake_process;
use crate::platform::update::{ check_for_updates, VERSION };
use crate::shared::types::Config;
use crate::platform::discord::{ is_discord_running, get_installed_discord_versions, open_discord, DiscordVersion };

pub fn render_ui(app: &mut DsqApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            if
                ui
                    .selectable_label(app.selected_tab == Tab::Main, translate(app, "tab_main"))
                    .clicked()
            {
                app.selected_tab = Tab::Main;
            }
            if
                ui
                    .selectable_label(
                        app.selected_tab == Tab::Settings,
                        translate(app, "tab_settings")
                    )
                    .clicked()
            {
                app.selected_tab = Tab::Settings;
            }
            if
                ui
                    .selectable_label(app.selected_tab == Tab::About, translate(app, "tab_about"))
                    .clicked()
            {
                app.selected_tab = Tab::About;
            }
        });

        ui.separator();

        match app.selected_tab {
            Tab::Main => render_tab_main(ui, app),
            Tab::Settings => render_tab_settings(ui, app),
            Tab::About => render_tab_about(ui, app),
        }
    });
}

fn render_tab_main(ui: &mut egui::Ui, app: &mut DsqApp) {

    if !is_discord_running() {
        ui.colored_label(egui::Color32::RED, translate(app, "discord_not_running"));

        let installed_versions = get_installed_discord_versions();
        if !installed_versions.is_empty() {
            ui.label(translate(app, "start_discord_prompt"));
            for version in installed_versions {
                let label = match version {
                    DiscordVersion::Stable => "🍱 Discord",
                    DiscordVersion::PTB => "🔎 Discord PTB",
                    DiscordVersion::Canary => "🛠 Discord Canary",
                };

                if ui.button(label).clicked() {
                    let _ = open_discord(version);
                }
            }
        } else {
            ui.label(translate(app, "discord_not_installed"));
        }

        ui.separator();
    }

    if !app.presets.is_empty() {
        egui::ComboBox
            ::from_label(translate(app, "select_preset"))
            .selected_text(&app.presets[app.selected_preset].name)
            .show_ui(ui, |ui| {
                for (i, preset) in app.presets.iter().enumerate() {
                    ui.selectable_value(&mut app.selected_preset, i, &preset.name);
                }
            });

        if ui.button(translate(app, "use_preset")).clicked() {
            let preset = &app.presets[app.selected_preset];
            app.process_name = preset.executable.clone();
            app.custom_path = preset.path.clone();
        }

        ui.separator();
    }

    if app.presets_outdated {
        ui.colored_label(egui::Color32::YELLOW, translate(app, "presets_outdated"));
        if ui.button("🔄 ".to_string() + &translate(app, "update_presets")).clicked() {
            match update_presets_file() {
                Ok(_) => {
                    app.status = translate(app, "presets_updated");
                    app.presets = load_presets();
                    app.presets_outdated = false;
                }
                Err(e) => {
                    app.status = translate(app, "error").replace("{error}", &e.to_string());
                }
            }
        }
    }

    ui.label(translate(app, "executable_name"));
    ui.text_edit_singleline(&mut app.process_name);

    ui.label(translate(app, "path_label"));
    ui.text_edit_singleline(&mut app.custom_path);

    if ui.button(translate(app, "start_process")).clicked() {
        if !app.process_name.trim().is_empty() {
            let result = create_fake_process(&app.custom_path, &app.process_name, 15);
            app.status = match result {
                Ok(_) =>
                    translate(app, "success")
                        .replace("{name}", &app.process_name)
                        .replace("{path}", &app.custom_path),
                Err(e) => translate(app, "error").replace("{error}", &e.to_string()),
            };
        } else {
            app.status = translate(app, "error_empty");
        }
    }

    ui.separator();
    ui.label(&app.status);
}

fn render_tab_settings(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.horizontal(|ui| {
        ui.label(translate(app, "lang"));
        egui::ComboBox
            ::from_id_source("language_select")
            .selected_text(&app.selected_lang)
            .show_ui(ui, |ui| {
                for key in app.langs.keys() {
                    ui.selectable_value(&mut app.selected_lang, key.clone(), key);
                }
            });
    });

    ui.separator();
    ui.label(translate(app, "settings_notice"));

    let config = Config {
        language: app.selected_lang.clone(),
        selected_preset: app.selected_preset,
        process_name: app.process_name.clone(),
        custom_path: app.custom_path.clone(),
    };
    save_config(&config);
}

fn render_tab_about(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.label(format!("DSQProcess v{}", VERSION));

    if ui.button(translate(app, "check_update")).clicked() {
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

    ui.separator();
    ui.label(&app.status);

    ui.separator();
    ui.label(translate(app, "about_credit"));
}
