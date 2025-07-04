#![windows_subsystem = "windows"]

mod app;
mod core;
mod platform;
mod shared;

use eframe::egui;
use app::state::DsqApp;
use shared::config::load_config;
use shared::lang::load_language;
use core::presets::{load_presets, is_presets_outdated};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([420.0, 400.0]),
        ..Default::default()
    };

    let config = load_config();
    let mut app = DsqApp::default();

    app.presets = load_presets();
    app.presets_outdated = is_presets_outdated();
    app.langs.insert("Español".to_string(), load_language("es"));
    app.langs.insert("English".to_string(), load_language("en"));

    app.selected_lang = config.language;
    app.selected_preset = config.selected_preset;
    app.process_name = config.process_name;
    app.custom_path = config.custom_path;

    eframe::run_native("DSQProcess", options, Box::new(|_cc| Box::new(app)))
}
