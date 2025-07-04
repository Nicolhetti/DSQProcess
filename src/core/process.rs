use std::path::Path;

pub fn create_fake_process(folder: &str, exe_name: &str, duration_min: u64) -> std::io::Result<()> {
    let target_folder = Path::new(folder);
    std::fs::create_dir_all(target_folder)?;

    let new_exe_path = target_folder.join(exe_name);
    let current_exe = std::env::current_exe()?;
    let child_path = current_exe
        .parent()
        .unwrap()
        .join(if cfg!(windows) { "dsqchild.exe" } else { "dsqchild" });

    std::fs::copy(&child_path, &new_exe_path)?;
    std::process::Command::new(&new_exe_path).arg(duration_min.to_string()).spawn()?;

    Ok(())
}
