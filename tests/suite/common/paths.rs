use std::path::Path;

#[cfg(unix)]
pub const MINIMAL_PATH: &str = "/usr/bin:/bin:/usr/sbin:/sbin";

#[cfg(target_os = "windows")]
pub const PATH_ENV_SEPARATOR: char = ';';

#[cfg(unix)]
pub const PATH_ENV_SEPARATOR: char = ':';

pub fn prepend_to_minimal_path<A: AsRef<Path>>(a: A) -> String {
    format!(
        "{}{}{}",
        a.as_ref().to_str().unwrap(),
        PATH_ENV_SEPARATOR,
        MINIMAL_PATH
    )
}
