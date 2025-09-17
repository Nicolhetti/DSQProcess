#![windows_subsystem = "windows"]

use eframe::App;
use eframe::{ egui, Frame };
use std::{ collections::HashMap, fs, env };

type LangMap = HashMap<String, String>;

#[derive(serde::Deserialize)]
struct Config {
    language: String,
}

fn load_language_from_config() -> LangMap {
    let config: Config = fs
        ::read_to_string("config.json")
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or(Config {
            language: "Español".to_string(),
        });

    let lang_code = if config.language == "English" { "en" } else { "es" };
    let path = format!("lang/{}_child.json", lang_code);
    let data = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

struct DsqChildApp {
    closing_time: Option<std::time::Instant>,
    status: String,
    lang: LangMap,
    process_name: String,
    start_time: std::time::Instant,
    is_scheduled_to_close: bool,
}

impl Default for DsqChildApp {
    fn default() -> Self {
        let lang = load_language_from_config();
        let exe_path = env::current_exe().unwrap_or_default();
        let process_name = exe_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_else(|| "simulated_process")
            .to_string();

        Self {
            closing_time: None,
            status: String::new(),
            lang,
            process_name,
            start_time: std::time::Instant::now(),
            is_scheduled_to_close: false,
        }
    }
}

impl DsqChildApp {
    fn t(&self, key: &str) -> String {
        self.lang
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    fn format_duration(&self, duration: std::time::Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }
}

impl App for DsqChildApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel
            ::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::symmetric(20.0, 20.0),
                fill: ctx.style().visuals.window_fill(),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(&format!("🎮 {}", self.t("title")));
                    ui.colored_label(
                        egui::Color32::from_rgb(108, 117, 125),
                        format!(
                            "{} {}",
                            self.t("simulating"),
                            self.process_name.replace(".exe", "")
                        )
                    );
                });

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.set_min_width(326.0);
                    ui.vertical(|ui| {
                        let elapsed = self.start_time.elapsed();

                        ui.horizontal(|ui| {
                            ui.label("⏱");
                            ui.horizontal(|ui| {
                                ui.label(self.t("time_elapsed"));
                                ui.colored_label(
                                    egui::Color32::from_rgb(40, 167, 69),
                                    self.format_duration(elapsed)
                                );
                            });
                        });

                        if let Some(when) = self.closing_time {
                            let now = std::time::Instant::now();
                            if now >= when {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            } else {
                                let remaining = when.duration_since(now);
                                ui.add_space(5.0);
                                ui.horizontal(|ui| {
                                    ui.label("⏰");
                                    ui.horizontal(|ui| {
                                        ui.label(self.t("automatic_closing"));
                                        ui.colored_label(
                                            egui::Color32::from_rgb(255, 193, 7),
                                            self.format_duration(remaining)
                                        );
                                    });
                                });
                            }
                        }
                    });
                });

                ui.add_space(15.0);

                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(20.0);
                        if !self.is_scheduled_to_close {
                            let button_15 = ui.add_sized(
                                [140.0, 35.0],
                                egui::Button::new(&format!("⏱ {}", self.t("close_in_15")))
                            );

                            if button_15.clicked() {
                                self.closing_time = Some(
                                    std::time::Instant::now() + std::time::Duration::from_secs(900)
                                );
                                self.status = self.t("scheduled_close");
                                self.is_scheduled_to_close = true;
                            }
                        } else {
                            let button_cancel = ui.add_sized(
                                [140.0, 35.0],
                                egui::Button::new(self.t("cancel_close"))
                            );

                            if button_cancel.clicked() {
                                self.closing_time = None;
                                self.status = self.t("close_cancelled").to_string();
                                self.is_scheduled_to_close = false;
                            }
                        }

                        ui.add_space(10.0);

                        let button_now = ui.add_sized(
                            [140.0, 35.0],
                            egui::Button::new(&format!("⏹ {}", self.t("close_now")))
                        );

                        if button_now.clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });

                ui.add_space(15.0);

                if !self.status.is_empty() {
                    ui.group(|ui| {
                        ui.set_min_width(326.0);
                        ui.horizontal(|ui| {
                            ui.vertical_centered(|ui| {
                                if
                                    self.status.contains("programado") ||
                                    self.status.contains("scheduled")
                                {
                                    ui.colored_label(
                                        egui::Color32::from_rgb(40, 167, 69),
                                        &self.status
                                    );
                                } else if
                                    self.status.contains("cancelado") ||
                                    self.status.contains("cancelled")
                                {
                                    ui.colored_label(
                                        egui::Color32::from_rgb(255, 193, 7),
                                        &self.status
                                    );
                                } else {
                                    ui.label("ℹ");
                                    ui.label(&self.status);
                                }
                            });
                        });
                    });
                }

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.set_min_width(326.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("💡");
                            ui.vertical(|ui| {
                                ui.label(self.t("tips"));
                                ui.colored_label(
                                    egui::Color32::from_rgb(108, 117, 125),
                                    self.t("tip_1")
                                );
                                ui.colored_label(
                                    egui::Color32::from_rgb(108, 117, 125),
                                    self.t("tip_2")
                                );
                                ui.colored_label(
                                    egui::Color32::from_rgb(108, 117, 125),
                                    self.t("tip_3")
                                );
                            });
                        });
                    });
                });
            });

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder
            ::default()
            .with_inner_size([380.0, 420.0])
            .with_resizable(false)
            .with_decorations(true),
        ..Default::default()
    };

    let lang = load_language_from_config();
    let title = lang.get("title_child").cloned().unwrap_or("Child".to_string());

    eframe::run_native(
        &title,
        options,
        Box::new(|_cc| Box::new(DsqChildApp::default()))
    )
}
