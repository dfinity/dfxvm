use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Tags {
    pub latest: Version,
}

#[derive(Deserialize)]
pub struct Manifest {
    pub tags: Tags,
    pub versions: Vec<Version>,
}
