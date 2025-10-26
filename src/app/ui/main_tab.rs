use super::components;
use crate::app::state::DsqApp;
use crate::app::translate::translate;
use crate::core::presets::{is_presets_outdated, load_presets, update_presets_file};
use crate::core::process::create_fake_process;
use crate::platform::discord::{
    get_installed_discord_versions, is_discord_running, open_discord, DiscordVersion,
};
use eframe::egui;

pub fn render(ui: &mut egui::Ui, app: &mut DsqApp) {
    // Solo actualizar cache de Discord cuando sea necesario
    if app.should_check_discord() {
        app.discord_running_cache = Some(is_discord_running());
        if app.discord_running_cache == Some(false) {
            app.discord_versions_cache = Some(get_installed_discord_versions());
        }
    }

    ui.vertical_centered(|ui| {
        render_rich_presence_status(ui, app);
        render_discord_detection(ui, app);
        render_presets_section(ui, app);
        render_outdated_presets_warning(ui, app);
        render_process_configuration(ui, app);
        render_status_section(ui, app);
    });
}

fn render_rich_presence_status(ui: &mut egui::Ui, app: &mut DsqApp) {
    if !app.rich_presence_enabled {
        return;
    }

    let (status_text, color) = if let Some(ref rp) = app.rich_presence {
        if rp.is_connected() {
            (
                translate(app, "rich_presence_connected"),
                egui::Color32::GREEN,
            )
        } else {
            (
                translate(app, "rich_presence_disconnected"),
                egui::Color32::RED,
            )
        }
    } else {
        (
            translate(app, "rich_presence_disconnected"),
            egui::Color32::RED,
        )
    };

    ui.colored_label(color, status_text);
    ui.add_space(10.0);
}

fn render_discord_detection(ui: &mut egui::Ui, app: &mut DsqApp) {
    // Usar cache para evitar verificaciones constantes
    let discord_is_running = app.discord_running_cache.unwrap_or(true);

    if !discord_is_running {
        ui.colored_label(egui::Color32::RED, translate(app, "discord_not_running"));

        let installed_versions = app
            .discord_versions_cache
            .as_ref()
            .cloned()
            .unwrap_or_default();

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
                        match open_discord(version) {
                            Ok(_) => {
                                log::info!("Discord opened successfully");
                                // Invalidar cache para verificar inmediatamente
                                app.invalidate_discord_cache();
                            }
                            Err(e) => {
                                log::error!("Failed to open Discord: {}", e);
                                app.status = format!("Error al abrir Discord: {}", e);
                            }
                        }
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
}

fn render_presets_section(ui: &mut egui::Ui, app: &mut DsqApp) {
    if app.presets.is_empty() {
        ui.colored_label(egui::Color32::YELLOW, "⚠ No hay presets disponibles");
        return;
    }

    ui.group(|ui| {
        ui.set_min_width(350.0);
        ui.vertical_centered(|ui| {
            ui.heading("🎮 Presets");
            ui.add_space(5.0);

            // Validar que selected_preset esté en rango
            if app.selected_preset >= app.presets.len() {
                app.selected_preset = 0;
                log::warn!("Selected preset out of bounds, reset to 0");
            }

            egui::ComboBox::from_label("")
                .selected_text(&app.presets[app.selected_preset].name)
                .width(300.0)
                .show_ui(ui, |ui| {
                    for (i, preset) in app.presets.iter().enumerate() {
                        let label = if preset.is_custom {
                            format!("⭐ {}", preset.name)
                        } else {
                            preset.name.clone()
                        };
                        ui.selectable_value(&mut app.selected_preset, i, label);
                    }
                });

            ui.add_space(5.0);

            // Primera fila de botones
            ui.horizontal(|ui| {
                if ui
                    .button("➕ ".to_string() + &translate(app, "add_preset"))
                    .clicked()
                {
                    app.show_add_preset_dialog = true;
                }

                if ui
                    .button("📋 ".to_string() + &translate(app, "use_preset"))
                    .clicked()
                {
                    let preset = &app.presets[app.selected_preset];
                    app.process_name = preset.executable.clone();
                    app.custom_path = preset.path.clone();
                    log::info!("Using preset: {}", preset.name);
                }

                if ui
                    .button("🔍 ".to_string() + &translate(app, "check_presets"))
                    .clicked()
                {
                    log::info!("Manual presets check triggered");
                    app.presets_outdated = is_presets_outdated();
                    if !app.presets_outdated {
                        app.status = translate(app, "presets_up_to_date");
                    } else {
                        app.status = translate(app, "presets_outdated");
                    }
                }
            });

            // Segunda fila (solo para presets personalizados)
            if app.presets[app.selected_preset].is_custom {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    if ui
                        .button("✏ ".to_string() + &translate(app, "edit_preset"))
                        .clicked()
                    {
                        let preset = &app.presets[app.selected_preset];
                        app.preset_to_edit = Some(preset.name.clone());
                        app.new_preset_name = preset.name.clone();
                        app.new_preset_executable = preset.executable.clone();
                        app.new_preset_path = preset.path.clone();
                        app.show_edit_preset_dialog = true;
                    }

                    if ui
                        .button("🗑 ".to_string() + &translate(app, "delete_preset"))
                        .clicked()
                    {
                        let preset = &app.presets[app.selected_preset];
                        app.preset_to_delete = Some(preset.name.clone());
                        app.show_delete_confirmation = true;
                    }
                });
            }
        });
    });
    ui.add_space(10.0);
}

fn render_outdated_presets_warning(ui: &mut egui::Ui, app: &mut DsqApp) {
    if !app.presets_outdated {
        return;
    }

    ui.group(|ui| {
        ui.set_min_width(350.0);
        ui.vertical_centered(|ui| {
            ui.colored_label(egui::Color32::YELLOW, translate(app, "presets_outdated"));
            ui.add_space(5.0);
            if ui
                .button("🔄 ".to_string() + &translate(app, "update_presets"))
                .clicked()
            {
                log::info!("Starting presets update");
                match update_presets_file() {
                    Ok(_) => {
                        app.status = translate(app, "presets_updated");
                        app.presets = load_presets();
                        app.filtered_presets = app.presets.clone();
                        app.presets_outdated = false;

                        // Validar selected_preset después de actualizar
                        if app.selected_preset >= app.presets.len() {
                            app.selected_preset = 0;
                        }

                        log::info!("Presets updated successfully");
                    }
                    Err(e) => {
                        log::error!("Failed to update presets: {}", e);
                        app.status = translate(app, "error").replace("{error}", &e.to_string());
                    }
                }
            }
        });
    });
    ui.add_space(10.0);
}

fn render_process_configuration(ui: &mut egui::Ui, app: &mut DsqApp) {
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

            // Mostrar preview de la ruta completa
            if !app.custom_path.is_empty() {
                let full_path = if app.custom_path.starts_with("Games/")
                    || app.custom_path.starts_with("Games\\")
                {
                    app.custom_path.clone()
                } else {
                    format!(
                        "Games/{}",
                        app.custom_path
                            .trim_start_matches('/')
                            .trim_start_matches('\\')
                    )
                };
                ui.add_space(3.0);
                ui.colored_label(
                    egui::Color32::from_rgb(108, 117, 125),
                    format!("📁 {}", full_path),
                );
            }

            ui.add_space(10.0);

            if ui
                .button("🚀 ".to_string() + &translate(app, "start_process"))
                .clicked()
            {
                handle_start_process(app);
            }
        });
    });
}

fn handle_start_process(app: &mut DsqApp) {
    let process_name = app.process_name.trim();

    if process_name.is_empty() {
        app.status = translate(app, "error_empty");
        return;
    }

    // Calcular ruta completa para el mensaje
    let full_path =
        if app.custom_path.starts_with("Games/") || app.custom_path.starts_with("Games\\") {
            app.custom_path.clone()
        } else {
            format!(
                "Games/{}",
                app.custom_path
                    .trim_start_matches('/')
                    .trim_start_matches('\\')
            )
        };

    log::info!("Starting fake process: {} at {}", process_name, full_path);

    match create_fake_process(&app.custom_path, process_name, 15) {
        Ok((pid, exe_path)) => {
            log::info!("Fake process created successfully: PID {}", pid);

            // Registrar proceso en el monitor
            app.process_monitor
                .add_process(pid, process_name.to_string(), exe_path);

            // Actualizar Rich Presence si está habilitado
            if app.rich_presence_enabled {
                if let Some(ref mut rp) = app.rich_presence {
                    // Buscar nombre del juego en presets
                    let game_display_name = app
                        .presets
                        .iter()
                        .find(|preset| preset.executable == process_name)
                        .map(|preset| preset.name.clone())
                        .unwrap_or_else(|| process_name.replace(".exe", ""));

                    match rp.set_activity(Some(game_display_name.clone())) {
                        Ok(_) => {
                            app.current_simulated_game = Some(game_display_name);
                            log::info!("Rich Presence updated successfully");
                        }
                        Err(e) => {
                            log::error!("Failed to update Rich Presence: {}", e);
                            app.status = translate(app, "rich_presence_error")
                                .replace("{error}", &e.to_string());
                            return;
                        }
                    }
                }
            }

            app.status = translate(app, "success")
                .replace("{name}", process_name)
                .replace("{path}", &full_path);
        }
        Err(e) => {
            log::error!("Failed to create fake process: {}", e);
            app.status = translate(app, "error").replace("{error}", &e.to_string());
        }
    }
}

fn render_status_section(ui: &mut egui::Ui, app: &mut DsqApp) {
    if app.status.is_empty() {
        return;
    }

    ui.add_space(15.0);
    ui.separator();
    ui.add_space(10.0);
    components::status_card(ui, &app.status);
}
