use eframe::egui;
use crate::app::state::DsqApp;
use crate::app::translate::translate;
use crate::shared::types::Preset;
use crate::core::presets::{ add_preset, edit_custom_preset, delete_custom_preset, load_presets };

pub fn render_add_dialog(ctx: &egui::Context, app: &mut DsqApp) {
    if !app.show_add_preset_dialog {
        return;
    }

    egui::Window
        ::new(translate(app, "add_preset_title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            render_preset_form(ui, app, false);
        });
}

pub fn render_edit_dialog(ctx: &egui::Context, app: &mut DsqApp) {
    if !app.show_edit_preset_dialog {
        return;
    }

    egui::Window
        ::new(translate(app, "edit_preset_title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            render_preset_form(ui, app, true);
        });
}

pub fn render_delete_confirmation(ctx: &egui::Context, app: &mut DsqApp) {
    if !app.show_delete_confirmation {
        return;
    }

    egui::Window
        ::new(translate(app, "delete_preset_title"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.set_min_width(300.0);

                ui.label(translate(app, "delete_preset_confirm"));

                if let Some(ref name) = app.preset_to_delete {
                    ui.colored_label(egui::Color32::YELLOW, name);
                }

                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    if ui.button("✔ ".to_string() + &translate(app, "delete")).clicked() {
                        handle_delete_preset(app);
                    }

                    if ui.button("✖ ".to_string() + &translate(app, "cancel")).clicked() {
                        close_delete_dialog(app);
                    }
                });
            });
        });
}

fn render_preset_form(ui: &mut egui::Ui, app: &mut DsqApp, is_edit: bool) {
    ui.vertical(|ui| {
        ui.set_min_width(400.0);

        ui.horizontal(|ui| {
            ui.label(translate(app, "preset_name"));
            ui.text_edit_singleline(&mut app.new_preset_name);
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label(translate(app, "executable_name"));
            ui.text_edit_singleline(&mut app.new_preset_executable);
        });

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label(translate(app, "path_label"));
            ui.text_edit_singleline(&mut app.new_preset_path);
        });

        ui.add_space(15.0);

        ui.horizontal(|ui| {
            if ui.button("✔ ".to_string() + &translate(app, "save_preset")).clicked() {
                if is_edit {
                    handle_edit_preset(app);
                } else {
                    handle_save_preset(app);
                }
            }

            if ui.button("✖ ".to_string() + &translate(app, "cancel")).clicked() {
                if is_edit {
                    close_edit_dialog(app);
                } else {
                    close_add_dialog(app);
                }
            }
        });
    });
}

fn handle_save_preset(app: &mut DsqApp) {
    if app.new_preset_name.trim().is_empty() || app.new_preset_executable.trim().is_empty() {
        app.status = translate(app, "preset_fields_empty");
        return;
    }

    let new_preset = Preset {
        name: app.new_preset_name.clone(),
        executable: app.new_preset_executable.clone(),
        path: app.new_preset_path.clone(),
        is_custom: true,
    };

    match add_preset(new_preset) {
        Ok(_) => {
            app.presets = load_presets();
            app.filtered_presets = app.presets.clone();
            app.status = translate(app, "preset_added_success");
            close_add_dialog(app);
        }
        Err(e) => {
            app.status = translate(app, "error").replace("{error}", &e.to_string());
        }
    }
}

fn handle_edit_preset(app: &mut DsqApp) {
    if app.new_preset_name.trim().is_empty() || app.new_preset_executable.trim().is_empty() {
        app.status = translate(app, "preset_fields_empty");
        return;
    }

    if let Some(ref old_name) = app.preset_to_edit {
        let edited_preset = Preset {
            name: app.new_preset_name.clone(),
            executable: app.new_preset_executable.clone(),
            path: app.new_preset_path.clone(),
            is_custom: true,
        };

        match edit_custom_preset(old_name, edited_preset) {
            Ok(_) => {
                app.presets = load_presets();
                app.filtered_presets = app.presets.clone();
                app.status = translate(app, "preset_edited_success");
                close_edit_dialog(app);
            }
            Err(e) => {
                app.status = translate(app, "error").replace("{error}", &e.to_string());
            }
        }
    }
}

fn handle_delete_preset(app: &mut DsqApp) {
    if let Some(ref name) = app.preset_to_delete {
        match delete_custom_preset(name) {
            Ok(_) => {
                app.presets = load_presets();
                app.filtered_presets = app.presets.clone();
                app.status = translate(app, "preset_deleted_success");
                close_delete_dialog(app);
            }
            Err(e) => {
                app.status = translate(app, "error").replace("{error}", &e.to_string());
            }
        }
    }
}

fn close_add_dialog(app: &mut DsqApp) {
    app.show_add_preset_dialog = false;
    app.new_preset_name.clear();
    app.new_preset_executable.clear();
    app.new_preset_path.clear();
}

fn close_edit_dialog(app: &mut DsqApp) {
    app.show_edit_preset_dialog = false;
    app.preset_to_edit = None;
    app.new_preset_name.clear();
    app.new_preset_executable.clear();
    app.new_preset_path.clear();
}

fn close_delete_dialog(app: &mut DsqApp) {
    app.show_delete_confirmation = false;
    app.preset_to_delete = None;
}
