#![windows_subsystem = "windows"]

mod app;
mod core;
mod platform;
mod shared;

use app::state::DsqApp;
use core::presets::{is_presets_outdated, load_presets};
use eframe::egui;
use shared::config::load_config;
use shared::lang::load_language;
use shared::richpresence::RichPresenceManager;

fn main() -> Result<(), eframe::Error> {
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "info"
        },
    ))
    .try_init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([505.0, 500.0])
            .with_resizable(false),
        ..Default::default()
    };

    let config = load_config();
    let mut app = DsqApp::default();

    app.presets = load_presets();
    app.filtered_presets = app.presets.clone();
    app.presets_outdated = is_presets_outdated();
    app.langs.insert("Español".to_string(), load_language("es"));
    app.langs.insert("English".to_string(), load_language("en"));

    app.selected_lang = config.language;
    app.selected_preset = config.selected_preset;
    app.process_name = config.process_name;
    app.custom_path = config.custom_path;
    app.rich_presence_enabled = config.rich_presence_enabled;

    // Inicializar Rich Presence si está habilitado
    if app.rich_presence_enabled {
        let mut rp = RichPresenceManager::new();
        if let Ok(()) = rp.connect() {
            let _ = rp.set_activity(None);
            app.rich_presence = Some(rp);
        }
    }

    eframe::run_native(
        "DSQProcess",
        options,
        Box::new(move |cc| {
            // Configurar estilo para mejor rendimiento
            let mut style = (*cc.egui_ctx.style()).clone();
            style.animation_time = 0.1; // Reducir tiempo de animaciones
            cc.egui_ctx.set_style(style);

            Box::new(app)
        }),
    )
}
