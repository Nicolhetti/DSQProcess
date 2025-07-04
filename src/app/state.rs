use std::collections::HashMap;
use eframe::{ egui, App };
use crate::shared::types::{ LangMap, Preset };
use crate::app::ui::render_ui;

#[derive(Default)]
pub struct DsqApp {
    pub process_name: String,
    pub custom_path: String,
    pub status: String,
    pub presets: Vec<Preset>,
    pub selected_preset: usize,
    pub langs: HashMap<String, LangMap>,
    pub selected_lang: String,
    pub presets_outdated: bool,
    pub selected_tab: Tab,
}

#[derive(PartialEq)]
pub enum Tab {
    Main,
    Settings,
    About,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Main
    }
}

impl App for DsqApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        render_ui(self, ctx);
    }
}
