use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".into());
    let target_base =
        env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| format!("{}/target", out_dir));
    // Prefer target/<triple>/<profile> when it exists; fallback to target/<profile>
    let triple = env::var("TARGET").unwrap_or_default();
    let triple_dir = Path::new(&target_base).join(&triple).join(&profile);
    let default_dir = Path::new(&target_base).join(&profile);
    let target_dir = if triple_dir.exists() {
        triple_dir
    } else {
        default_dir
    };

    let lang_src = Path::new(&out_dir).join("lang");
    let lang_dst = Path::new(&target_dir).join("lang");

    if lang_dst.exists() {
        let _ = fs::remove_dir_all(&lang_dst);
    }

    fs::create_dir_all(&lang_dst).unwrap();
    if lang_src.exists() {
        for entry in fs::read_dir(&lang_src).unwrap() {
            let entry = entry.unwrap();
            let src_path = entry.path();
            let filename = entry.file_name();
            fs::copy(src_path, lang_dst.join(filename)).unwrap();
        }
    }

    let _ = fs::copy(
        Path::new(&out_dir).join("presets.json"),
        Path::new(&target_dir).join("presets.json"),
    );

    let _ = fs::copy(
        Path::new(&out_dir).join("presets_custom.json"),
        Path::new(&target_dir).join("presets_custom.json"),
    );

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("CompanyName", "Nicolhetti");
        res.set("FileDescription", "DSQProcess - Discord Quest Process Tool");
        res.set("ProductName", "DSQProcess");
        res.set("LegalCopyright", "© 2025 Nicolhetti");
        res.set("OriginalFilename", "DSQProcess.exe");

        let ver = env::var("CARGO_PKG_VERSION").unwrap();
        res.set("FileVersion", &ver);
        res.set("ProductVersion", &ver);

        res.compile().expect("Failed to compile resources");
    }
}
