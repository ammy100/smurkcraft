//! Windows only: embed the app icon and version info (VERSIONINFO) into `smurkcraft.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `SMURKCRAFT_REQUIRE_WINRES=1` (set by the
//! release workflow) turns it into an error.

fn main() {
    let icon_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/app-icon/smurkcraft.ico");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", icon_path.display());
    println!("cargo:rerun-if-env-changed=SMURKCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let icon_path = icon_path.to_string_lossy();
    let mut res = winresource::WindowsResource::new();
    res.set_icon(&icon_path)
        .set("ProductName", "Smurkcraft")
        .set("FileDescription", "Smurkcraft image editor")
        .set("CompanyName", "Smurkcraft contributors")
        .set("LegalCopyright", "Smurkcraft contributors; based on PhotoCraft. MIT OR Apache-2.0.")
        .set("OriginalFilename", "smurkcraft.exe")
        .set("InternalName", "smurkcraft");
    if let Err(e) = res.compile() {
        if std::env::var_os("SMURKCRAFT_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=smurkcraft.exe built without icon/version resources: {e}");
    }
}
