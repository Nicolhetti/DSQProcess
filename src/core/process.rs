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
    /// Creates a new ProcessMonitor with an empty, thread-safe process list.
    ///
    /// # Examples
    ///
    /// ```
    /// let _pm = ProcessMonitor::new();
    /// ```
    pub fn new() -> Self {
        Self {
            processes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Adds a process record to the monitor's internal list.
    ///
    /// The record stores the process id, the executable name, and the executable path.
    /// If the internal mutex is poisoned or cannot be acquired, the call is a no-op.
    ///
    /// # Parameters
    ///
    /// - `pid`: the platform process identifier to track.
    /// - `exe_name`: the executable file name associated with the process.
    /// - `exe_path`: the filesystem path to the executable file on disk.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// let monitor = crate::core::process::ProcessMonitor::new();
    /// monitor.add_process(12345, "example_game.exe".to_string(), PathBuf::from("Games/example_game.exe"));
    /// ```
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

    /// Removes tracked processes that are no longer running and deletes their executables when present.
    ///
    /// This scans the current system processes, retains only those still alive in the monitor's internal list,
    /// and attempts to remove the executable file for each process that is no longer running.
    ///
    /// # Returns
    ///
    /// `Vec<String>` with the executable names of the processes that were removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// let monitor = crate::core::process::ProcessMonitor::new();
    /// monitor.add_process(0, "dead_exe".to_string(), PathBuf::from("dead_exe"));
    /// let removed = monitor.check_and_remove_dead_processes();
    /// assert_eq!(removed, vec!["dead_exe".to_string()]);
    /// ```
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
                            eprintln!("Error al eliminar {}: {}", proc_info.exe_path.display(), e);
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

/// Creates a copy of the bundled child executable inside a Games/ subfolder, launches it with
/// the specified runtime in minutes, and returns the spawned process id and the path to the
/// copied executable.
///
/// The function ensures the target folder is rooted under `Games/` (adds that prefix if absent),
/// creates any missing directories, copies the bundled `DSQChild` executable into the target
/// folder with the provided `exe_name`, and spawns it with `duration_min` passed as a single
/// argument. On success, returns the child process id and the `PathBuf` to the copied executable.
/// Returns an `Err` if any filesystem or process spawning operation fails.
///
/// # Examples
///
/// ```
/// # use std::fs;
/// # use std::path::PathBuf;
/// let (pid, path): (u32, PathBuf) = create_fake_process("MyTest", "test_child", 1).unwrap();
/// assert!(pid > 0);
/// assert!(fs::metadata(path).is_ok());
/// ```
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