pub mod main_tab;
pub mod settings_tab;
pub mod about_tab;
pub mod components;
pub mod preset_dialog;

use eframe::egui;
use crate::app::state::{ DsqApp, Tab };
use crate::app::translate::translate;

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
                render_tab_navigation(ui, app);

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                match app.selected_tab {
                    Tab::Main => main_tab::render(ui, app),
                    Tab::Settings => settings_tab::render(ui, app),
                    Tab::About => about_tab::render(ui, app),
                }
            });
        });
    preset_dialog::render_add_dialog(ctx, app);
    preset_dialog::render_edit_dialog(ctx, app);
    preset_dialog::render_delete_confirmation(ctx, app);
}

fn render_tab_navigation(ui: &mut egui::Ui, app: &mut DsqApp) {
    ui.horizontal(|ui| {
        ui.add_space((ui.available_width() - 465.0) / 2.0);

        if ui.selectable_label(app.selected_tab == Tab::Main, translate(app, "tab_main")).clicked() {
            app.selected_tab = Tab::Main;
        }

        if
            ui
                .selectable_label(app.selected_tab == Tab::Settings, translate(app, "tab_settings"))
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
}
