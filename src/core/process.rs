use std::path::{Path, PathBuf};
use sysinfo::{Pid, ProcessesToUpdate, System};

/// Normaliza la ruta de un preset bajo "Games/"
pub fn games_path(folder: &str) -> String {
    if folder.starts_with("Games/") || folder.starts_with("Games\\") {
        folder.to_string()
    } else {
        format!(
            "Games/{}",
            folder.trim_start_matches('/').trim_start_matches('\\')
        )
    }
}

pub struct ProcessMonitor {
    processes: Vec<ProcessInfo>,
    system: System,
}

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub exe_name: String,
    pub exe_path: PathBuf,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            system: System::new_all(),
        }
    }

    pub fn add_process(&mut self, pid: u32, exe_name: String, exe_path: PathBuf) {
        self.processes.push(ProcessInfo {
            pid,
            exe_name: exe_name.clone(),
            exe_path,
        });
        log::info!("Added process to monitor: {} (PID: {})", exe_name, pid);
    }

    pub fn check_and_remove_dead_processes(&mut self) -> Vec<String> {
        self.system.refresh_processes(ProcessesToUpdate::All, true);

        let mut removed = Vec::new();

        self.processes.retain(|proc_info| {
            let pid = Pid::from_u32(proc_info.pid);
            let is_alive = self.system.process(pid).is_some();

            if !is_alive {
                log::info!(
                    "Process {} (PID: {}) has terminated",
                    proc_info.exe_name,
                    proc_info.pid
                );
                removed.push(proc_info.exe_name.clone());

                // Intentar eliminar el ejecutable de forma segura
                if proc_info.exe_path.exists() {
                    // Esperar un poco para asegurar que el proceso se liberó
                    std::thread::sleep(std::time::Duration::from_millis(100));

                    match std::fs::remove_file(&proc_info.exe_path) {
                        Ok(_) => {
                            log::info!(
                                "Successfully deleted executable: {}",
                                proc_info.exe_path.display()
                            );
                        }
                        Err(e) => {
                            log::warn!(
                                "Failed to delete executable {} (will retry later): {}",
                                proc_info.exe_path.display(),
                                e
                            );
                        }
                    }
                }
            }

            is_alive
        });

        removed
    }

    /// Limpia todos los procesos y ejecutables pendientes (útil para shutdown)
    pub fn cleanup_all(&mut self) {
        for proc_info in self.processes.drain(..) {
            if proc_info.exe_path.exists() {
                if let Err(e) = std::fs::remove_file(&proc_info.exe_path) {
                    log::warn!(
                        "Failed to cleanup executable on shutdown {}: {}",
                        proc_info.exe_path.display(),
                        e
                    );
                }
            }
        }
    }
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ProcessMonitor {
    fn drop(&mut self) {
        self.cleanup_all();
    }
}

/// Crea un proceso falso copiando DSQChild.exe
pub fn create_fake_process(
    folder: &str,
    exe_name: &str,
    duration_min: u64,
) -> std::io::Result<(u32, PathBuf)> {
    // Validación de entrada
    if exe_name.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Executable name cannot be empty",
        ));
    }

    // Agregar automáticamente "Games/" si la ruta no empieza con ella
    let full_path = games_path(folder);
    let target_folder = Path::new(&full_path);

    // Crear directorio con manejo de errores apropiado
    if let Err(e) = std::fs::create_dir_all(target_folder) {
        log::error!(
            "Failed to create directory {}: {}",
            target_folder.display(),
            e
        );
        return Err(e);
    }

    let new_exe_path = target_folder.join(exe_name);

    // Verificar si el archivo ya existe y eliminarlo primero
    if new_exe_path.exists() {
        log::warn!(
            "Executable {} already exists, removing old copy",
            new_exe_path.display()
        );
        std::fs::remove_file(&new_exe_path)?;
    }

    // Obtener ruta de DSQChild.exe
    let current_exe = std::env::current_exe()?;
    let parent = current_exe
        .parent()
        .ok_or_else(|| std::io::Error::other("Executable has no parent directory"))?;

    let child_path = parent.join(if cfg!(windows) {
        "DSQChild.exe"
    } else {
        "DSQChild"
    });

    // Verificar que DSQChild existe
    if !child_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("DSQChild not found at: {}", child_path.display()),
        ));
    }

    // Copiar ejecutable
    if let Err(e) = std::fs::copy(&child_path, &new_exe_path) {
        log::error!(
            "Failed to copy {} to {}: {}",
            child_path.display(),
            new_exe_path.display(),
            e
        );
        return Err(e);
    }

    // Spawn proceso con manejo de errores
    let child = std::process::Command::new(&new_exe_path)
        .arg(duration_min.to_string())
        .spawn()
        .map_err(|e| {
            log::error!("Failed to spawn process {}: {}", new_exe_path.display(), e);
            // Limpiar el ejecutable copiado si falla el spawn
            let _ = std::fs::remove_file(&new_exe_path);
            e
        })?;

    let pid = child.id();
    log::info!(
        "Successfully created fake process {} (PID: {})",
        exe_name,
        pid
    );

    Ok((pid, new_exe_path))
}
