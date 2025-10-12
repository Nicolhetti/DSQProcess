use std::collections::HashMap;
use std::time::{ Duration, Instant };
use eframe::{ egui, App };
use crate::shared::types::{ LangMap, Preset };
use crate::shared::richpresence::RichPresenceManager;
use crate::app::ui::render_ui;

#[derive(Default)]
pub struct DsqApp {
    pub process_name: String,
    pub custom_path: String,
    pub status: String,
    pub presets: Vec<Preset>,
    pub filtered_presets: Vec<Preset>,
    pub selected_preset: usize,
    pub langs: HashMap<String, LangMap>,
    pub selected_lang: String,
    pub presets_outdated: bool,
    pub selected_tab: Tab,
    pub rich_presence_enabled: bool,
    pub rich_presence: Option<RichPresenceManager>,
    pub current_simulated_game: Option<String>,

    // Gestión de presets
    pub show_add_preset_dialog: bool,
    pub show_edit_preset_dialog: bool,
    pub show_delete_confirmation: bool,
    pub new_preset_name: String,
    pub new_preset_executable: String,
    pub new_preset_path: String,
    pub preset_to_delete: Option<String>,
    pub preset_to_edit: Option<String>,

    // Cache para optimización
    pub discord_running_cache: Option<bool>,
    pub discord_versions_cache: Option<Vec<crate::platform::discord::DiscordVersion>>,
    pub last_discord_check: Option<Instant>,
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

impl DsqApp {
    pub fn should_check_discord(&mut self) -> bool {
        const CHECK_INTERVAL: Duration = Duration::from_secs(5);

        match self.last_discord_check {
            None => {
                self.last_discord_check = Some(Instant::now());
                true
            }
            Some(last_check) => {
                if last_check.elapsed() >= CHECK_INTERVAL {
                    self.last_discord_check = Some(Instant::now());
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn invalidate_discord_cache(&mut self) {
        self.discord_running_cache = None;
        self.discord_versions_cache = None;
        self.last_discord_check = None;
    }
}

impl App for DsqApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        render_ui(self, ctx);
    }
}

impl Drop for DsqApp {
    fn drop(&mut self) {
        if let Some(mut rp) = self.rich_presence.take() {
            let _ = rp.clear_activity();
            rp.disconnect();
        }
    }
}
