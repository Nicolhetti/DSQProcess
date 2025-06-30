use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = format!("{}/target/release", out_dir);

    let lang_src = Path::new(&out_dir).join("lang");
    let lang_dst = Path::new(&target_dir).join("lang");

    if lang_dst.exists() {
        let _ = fs::remove_dir_all(&lang_dst);
    }

    fs::create_dir_all(&lang_dst).unwrap();
    for entry in fs::read_dir(&lang_src).unwrap() {
        let entry = entry.unwrap();
        let src_path = entry.path();
        let filename = entry.file_name();
        fs::copy(src_path, lang_dst.join(filename)).unwrap();
    }

    let _ = fs::copy(
        Path::new(&out_dir).join("presets.json"),
        Path::new(&target_dir).join("presets.json")
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
        res.set("FileVersion", "0.2.0");
        res.set("ProductVersion", "0.2.0");

        res.compile().expect("Failed to compile resources");
    }
}
