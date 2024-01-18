use crate::error::dfxvm::ListError;
use crate::locations::Locations;
use crate::settings::Settings;
use itertools::Itertools;
use semver::Version;

pub fn list(locations: &Locations) -> Result<(), ListError> {
    let settings = Settings::load_or_default(&locations.settings_path())?;
    let default_version = settings.default_version;

    for version in installed_versions(locations)? {
        let default_indicator = if default_version.as_ref() == Some(&version) {
            " (default)"
        } else {
            ""
        };
        println!("{}{}", version, default_indicator);
    }
    Ok(())
}

fn installed_versions(locations: &Locations) -> Result<Vec<Version>, ListError> {
    let versions_dir = locations.versions_dir();

    if !versions_dir.exists() {
        return Ok(vec![]);
    }

    let versions = versions_dir
        .read_dir()
        .map_err(|source| ListError::ReadDir {
            path: versions_dir.to_path_buf(),
            source,
        })?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .filter_map(|entry| {
            entry
                .file_name()
                .to_str()
                .and_then(|filename| Version::parse(filename).ok())
        })
        .sorted()
        .collect();
    Ok(versions)
}
