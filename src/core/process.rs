use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use sysinfo::{Pid, System};

pub struct ProcessMonitor {
    processes: Arc<Mutex<Vec<ProcessInfo>>>,
}

#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub exe_name: String,
    pub exe_path: PathBuf,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        Self {
            processes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_process(&self, pid: u32, exe_name: String, exe_path: PathBuf) {
        if let Ok(mut procs) = self.processes.lock() {
            procs.push(ProcessInfo {
                pid,
                exe_name,
                exe_path,
            });
        }
    }

    pub fn _get_active_processes(&self) -> Vec<ProcessInfo> {
        self.processes
            .lock()
            .ok()
            .map(|p| p.clone())
            .unwrap_or_default()
    }

    pub fn check_and_remove_dead_processes(&self) -> Vec<String> {
        let mut system = System::new_all();
        system.refresh_all();

        let mut removed = Vec::new();

        if let Ok(mut procs) = self.processes.lock() {
            procs.retain(|proc_info| {
                let pid = Pid::from_u32(proc_info.pid);
                let is_alive = system.process(pid).is_some();

                if !is_alive {
                    removed.push(proc_info.exe_name.clone());

                    // Eliminar el ejecutable cuando el proceso muere
                    if proc_info.exe_path.exists() {
                        if let Err(e) = std::fs::remove_file(&proc_info.exe_path) {
                            log::warn!("Failed to delete executable {}: {}", proc_info.exe_path.display(), e);
                        }
                    }
                }

                is_alive
            });
        }

        removed
    }
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
}

pub fn create_fake_process(
    folder: &str,
    exe_name: &str,
    duration_min: u64,
) -> std::io::Result<(u32, PathBuf)> {
    // Agregar automáticamente "Games/" si la ruta no empieza con ella
    let full_path = if folder.starts_with("Games/") || folder.starts_with("Games\\") {
        folder.to_string()
    } else {
        format!(
            "Games/{}",
            folder.trim_start_matches('/').trim_start_matches('\\')
        )
    };

    let target_folder = Path::new(&full_path);
    std::fs::create_dir_all(target_folder)?;

    let new_exe_path = target_folder.join(exe_name);
    let current_exe = std::env::current_exe()?;
    let child_path = current_exe.parent().unwrap().join(if cfg!(windows) {
        "DSQChild.exe"
    } else {
        "DSQChild"
    });

    std::fs::copy(&child_path, &new_exe_path)?;
    let child = std::process::Command::new(&new_exe_path)
        .arg(duration_min.to_string())
        .spawn()?;

    Ok((child.id(), new_exe_path))
}
