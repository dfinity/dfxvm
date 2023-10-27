pub fn platform() -> &'static str {
    #[cfg(target_os = "linux")]
    let platform = "linux";
    #[cfg(target_os = "macos")]
    let platform = "darwin";
    platform
}
