use std::path::Path;

pub fn create_fake_process(folder: &str, exe_name: &str, duration_min: u64) -> std::io::Result<()> {
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
    std::process::Command::new(&new_exe_path)
        .arg(duration_min.to_string())
        .spawn()?;

    Ok(())
}
