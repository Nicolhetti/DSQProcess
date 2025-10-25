use std::env;
use std::fs;
use std::path::Path;

/// Copies language files and preset JSON files into the release target directory and, on Windows, embeds application resources.
///
/// This build script reads `CARGO_MANIFEST_DIR` to locate the project directory, constructs `<manifest>/target/release`, and synchronizes the `lang` directory from the manifest into that release directory (removing any existing destination `lang` directory first). It then attempts to copy `presets.json` and `presets_custom.json` from the manifest to the release directory, ignoring copy errors. When compiled on Windows, it sets standard PE resource fields (icon, company, product, file description, original filename, and version) and compiles those resources.
///
/// Panics:
/// - If creating the destination `lang` directory fails.
/// - If reading or copying individual language files fails.
/// - On Windows, if resource compilation fails.
///
/// # Examples
///
/// ```no_run
/// // Set up a manifest directory (path must point to a real project layout for a real run)
/// std::env::set_var("CARGO_MANIFEST_DIR", "/path/to/project");
/// // Running this `main` will synchronize `lang` and copy preset files into `/path/to/project/target/release`.
/// main();
/// ```
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
        res.set("FileVersion", "0.4.4");
        res.set("ProductVersion", "0.4.4");

        res.compile().expect("Failed to compile resources");
    }
}