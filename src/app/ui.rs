use eframe::egui;
use super::state::DsqApp;
use crate::app::state::Tab;
use crate::app::translate::translate;
use crate::core::presets::{ load_presets, update_presets_file, is_presets_outdated };
use crate::shared::config::save_config;
use crate::shared::richpresence::RichPresenceManager;
use crate::core::process::create_fake_process;
use crate::platform::update::{ check_for_updates, VERSION };
use crate::shared::types::Config;
use crate::platform::discord::{
    is_discord_running,
    get_installed_discord_versions,
    open_discord,
    DiscordVersion,
};

pub fn render_ui(app: &mut DsqApp, ctx: &egui::Context) {
    egui::CentralPanel
        ::default()
        .frame(egui::Frame {
            inner_margin: egui::Margin::symmetric(20.0, 20.0),
            fill: ctx.style().visuals.window_fill(),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space((ui.available_width() - 465.0) / 2.0);

                    if
                        ui
                            .selectable_label(
                                app.selected_tab == Tab::Main,
                                translate(app, "tab_main")
                            )
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
                            .selectable_label(
                                app.selected_tab == Tab::About,
                                translate(app, "tab_about")
                            )
                            .clicked()
                    {
                        app.selected_tab = Tab::About;
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                match app.selected_tab {
                    Tab::Main => render_tab_main(ui, app),
                    Tab::Settings => render_tab_settings(ui, app),
                    Tab::About => render_tab_about(ui, app),
                }
            });
        });
}

fn render_tab_main(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.vertical_centered(|ui| {
        if app.rich_presence_enabled {
            let status_text = if
                app.rich_presence.is_some() &&
                app.rich_presence.as_ref().unwrap().is_connected()
            {
                translate(app, "rich_presence_connected")
            } else {
                translate(app, "rich_presence_disconnected")
            };

            let color = if
                app.rich_presence.is_some() &&
                app.rich_presence.as_ref().unwrap().is_connected()
            {
                egui::Color32::GREEN
            } else {
                egui::Color32::RED
            };

            ui.colored_label(color, status_text);
            ui.add_space(10.0);
        }

        if !is_discord_running() {
            ui.colored_label(egui::Color32::RED, translate(app, "discord_not_running"));

            let installed_versions = get_installed_discord_versions();
            if !installed_versions.is_empty() {
                ui.add_space(5.0);
                ui.label(translate(app, "start_discord_prompt"));
                ui.vertical_centered(|ui| {
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
                });
            } else {
                ui.label(translate(app, "discord_not_installed"));
            }
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
        }

        if !app.presets.is_empty() {
            ui.group(|ui| {
                ui.set_min_width(350.0);
                ui.vertical_centered(|ui| {
                    ui.heading("🎮 Presets");
                    ui.add_space(5.0);

                    egui::ComboBox
                        ::from_label("")
                        .selected_text(&app.presets[app.selected_preset].name)
                        .width(300.0)
                        .show_ui(ui, |ui| {
                            for (i, preset) in app.presets.iter().enumerate() {
                                ui.selectable_value(&mut app.selected_preset, i, &preset.name);
                            }
                        });

                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        if ui.button("📋 ".to_string() + &translate(app, "use_preset")).clicked() {
                            let preset = &app.presets[app.selected_preset];
                            app.process_name = preset.executable.clone();
                            app.custom_path = preset.path.clone();
                        }

                        if
                            ui
                                .button("🔍 ".to_string() + &translate(app, "check_presets"))
                                .clicked()
                        {
                            app.presets_outdated = is_presets_outdated();
                            if !app.presets_outdated {
                                app.status = translate(app, "presets_up_to_date");
                            }
                        }
                    });
                });
            });
            ui.add_space(10.0);
        }

        if app.presets_outdated {
            ui.group(|ui| {
                ui.set_min_width(350.0);
                ui.vertical_centered(|ui| {
                    ui.colored_label(egui::Color32::YELLOW, translate(app, "presets_outdated"));
                    ui.add_space(5.0);
                    if ui.button("🔄 ".to_string() + &translate(app, "update_presets")).clicked() {
                        match update_presets_file() {
                            Ok(_) => {
                                app.status = translate(app, "presets_updated");
                                app.presets = load_presets();
                                app.presets_outdated = false;
                            }
                            Err(e) => {
                                app.status = translate(app, "error").replace(
                                    "{error}",
                                    &e.to_string()
                                );
                            }
                        }
                    }
                });
            });
            ui.add_space(10.0);
        }

        ui.group(|ui| {
            ui.set_min_width(350.0);
            ui.vertical_centered(|ui| {
                ui.heading(translate(app, "process_cfg"));
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label(translate(app, "executable_name"));
                    ui.text_edit_singleline(&mut app.process_name);
                });

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label(translate(app, "path_label"));
                    ui.text_edit_singleline(&mut app.custom_path);
                });

                ui.add_space(10.0);

                if ui.button("🚀 ".to_string() + &translate(app, "start_process")).clicked() {
                    if !app.process_name.trim().is_empty() {
                        let result = create_fake_process(&app.custom_path, &app.process_name, 15);
                        app.status = match result {
                            Ok(_) => {
                                if app.rich_presence_enabled {
                                    if let Some(ref mut rp) = app.rich_presence {
                                        let game_display_name = app.presets
                                            .iter()
                                            .find(|preset| preset.executable == app.process_name)
                                            .map(|preset| preset.name.clone())
                                            .unwrap_or_else(||
                                                app.process_name.replace(".exe", "")
                                            );

                                        if
                                            let Err(e) = rp.set_activity(
                                                Some(game_display_name.clone())
                                            )
                                        {
                                            app.status = translate(
                                                app,
                                                "rich_presence_error"
                                            ).replace("{error}", &e.to_string());
                                        } else {
                                            app.current_simulated_game = Some(game_display_name);
                                        }
                                    }
                                }

                                translate(app, "success")
                                    .replace("{name}", &app.process_name)
                                    .replace("{path}", &app.custom_path)
                            }
                            Err(e) => translate(app, "error").replace("{error}", &e.to_string()),
                        };
                    } else {
                        app.status = translate(app, "error_empty");
                    }
                }
            });
        });

        ui.add_space(15.0);
        ui.separator();
        ui.add_space(10.0);

        if !app.status.is_empty() {
            ui.group(|ui| {
                ui.set_min_width(350.0);
                ui.vertical_centered(|ui| {
                    ui.label(&app.status);
                });
            });
        }
    });
}

fn render_tab_settings(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.vertical_centered(|ui| {
        ui.heading(translate(app, "settings"));
        ui.add_space(20.0);

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

        ui.add_space(15.0);

        ui.group(|ui| {
            ui.set_min_width(350.0);
            ui.vertical_centered(|ui| {
                ui.heading("🎮 Discord Rich Presence");
                ui.add_space(10.0);

                let mut rich_presence_changed = false;
                let enable_rich_presence_text = translate(app, "enable_rich_presence");
                if
                    ui
                        .checkbox(&mut app.rich_presence_enabled, &enable_rich_presence_text)
                        .changed()
                {
                    rich_presence_changed = true;
                }

                if rich_presence_changed {
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
            });
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        ui.colored_label(egui::Color32::GRAY, translate(app, "settings_notice"));

        let config = Config {
            language: app.selected_lang.clone(),
            selected_preset: app.selected_preset,
            process_name: app.process_name.clone(),
            custom_path: app.custom_path.clone(),
            rich_presence_enabled: app.rich_presence_enabled,
        };
        save_config(&config);
    });
}

fn render_tab_about(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.vertical_centered(|ui| {
        ui.heading(translate(app, "info"));
        ui.add_space(20.0);

        ui.group(|ui| {
            ui.set_min_width(350.0);
            ui.vertical_centered(|ui| {
                ui.heading(format!("DSQProcess v{}", VERSION));
                ui.add_space(10.0);

                if ui.button("🔍 ".to_string() + &translate(app, "check_update")).clicked() {
                    match check_for_updates(VERSION) {
                        Ok(Some(download_url)) => {
                            app.status = translate(app, "update_available").replace(
                                "{url}",
                                &download_url
                            );
                            let _ = open::that(download_url);
                        }
                        Ok(None) => {
                            app.status = translate(app, "up_to_date");
                        }
                        Err(e) => {
                            app.status = translate(app, "update_error").replace(
                                "{error}",
                                &e.to_string()
                            );
                        }
                    }
                }
            });
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        if !app.status.is_empty() {
            ui.group(|ui| {
                ui.set_min_width(350.0);
                ui.vertical_centered(|ui| {
                    ui.label(&app.status);
                });
            });
            ui.add_space(15.0);
        }

        ui.group(|ui| {
            ui.set_min_width(350.0);
            ui.vertical_centered(|ui| {
                ui.label(translate(app, "about_credit"));
            });
        });
    });
}
