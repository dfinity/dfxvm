pub fn platform() -> &'static str {
    #[cfg(target_os = "linux")]
    let platform = "linux";
    #[cfg(target_os = "macos")]
    let platform = "darwin";
    platform
}

pub fn arch() -> &'static str {
    #[cfg(target_arch = "aarch64")]
    let arch = "aarch64";
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    let arch = "x86_64";
    #[cfg(target_os = "linux")]
    let arch = "x86_64";
    arch
}
