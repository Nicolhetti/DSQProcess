use crate::app::ui::render_ui;
use crate::core::process::ProcessMonitor;
use crate::shared::richpresence::RichPresenceManager;
use crate::shared::types::{LangMap, Preset};
use eframe::{egui, App};
use std::collections::HashMap;
use std::time::{Duration, Instant};

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

    // Cache para optimización de Discord
    pub discord_running_cache: Option<bool>,
    pub discord_versions_cache: Option<Vec<crate::platform::discord::DiscordVersion>>,
    pub last_discord_check: Option<Instant>,

    // Monitor de procesos
    pub process_monitor: ProcessMonitor,
    pub last_process_check: Option<Instant>,
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub enum Tab {
    #[default]
    Main,
    Settings,
    About,
}

impl DsqApp {
    /// Verifica si debe actualizar el cache de Discord
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

    /// Invalida el cache de Discord para forzar verificación inmediata
    pub fn invalidate_discord_cache(&mut self) {
        self.discord_running_cache = None;
        self.discord_versions_cache = None;
        self.last_discord_check = None;
        log::debug!("Discord cache invalidated");
    }

    /// Verifica si debe revisar procesos muertos
    pub fn should_check_processes(&mut self) -> bool {
        const CHECK_INTERVAL: Duration = Duration::from_secs(2);

        match self.last_process_check {
            None => {
                self.last_process_check = Some(Instant::now());
                true
            }
            Some(last_check) => {
                if last_check.elapsed() >= CHECK_INTERVAL {
                    self.last_process_check = Some(Instant::now());
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Verifica y limpia procesos muertos
    pub fn check_dead_processes(&mut self) {
        if !self.should_check_processes() {
            return;
        }

        let dead_processes = self.process_monitor.check_and_remove_dead_processes();

        // Si hay procesos que murieron y Rich Presence está activo, resetear
        if !dead_processes.is_empty() {
            log::info!(
                "Detected {} dead process(es): {:?}",
                dead_processes.len(),
                dead_processes
            );

            if self.rich_presence_enabled {
                if let Some(ref mut rp) = self.rich_presence {
                    match rp.set_activity(None) {
                        Ok(_) => {
                            log::info!("Rich Presence reset to idle state");
                            self.current_simulated_game = None;
                        }
                        Err(e) => {
                            log::error!("Failed to reset Rich Presence: {}", e);
                        }
                    }
                }
            }
        }
    }

    /// Limpia todos los recursos antes de cerrar
    pub fn cleanup(&mut self) {
        log::info!("Starting app cleanup");

        // Limpiar Rich Presence
        if let Some(mut rp) = self.rich_presence.take() {
            log::info!("Cleaning up Rich Presence");
            let _ = rp.clear_activity();
            rp.disconnect();
        }

        // Limpiar procesos monitoreados
        self.process_monitor.cleanup_all();

        log::info!("App cleanup completed");
    }
}

impl App for DsqApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Verificar procesos muertos periódicamente
        self.check_dead_processes();

        // Renderizar UI
        render_ui(self, ctx);
    }
}

impl Drop for DsqApp {
    fn drop(&mut self) {
        self.cleanup();
    }
}
