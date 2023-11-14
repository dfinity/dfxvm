use crate::common::{
    file_contents,
    file_contents::{bash_script, dfx_tar_gz},
    target,
};
use httptest::http::{response, Response};
use semver::Version;

pub struct ReleaseAsset {
    pub version: Version,
    pub filename: String,
    pub contents: Vec<u8>,
}

impl ReleaseAsset {
    pub fn dfx_tarball(version: &str, snippet: &str) -> ReleaseAsset {
        let filename = Self::dfx_tarball_filename(version);
        let version = Version::parse(version).unwrap();
        let contents = dfx_tar_gz(&bash_script(snippet));
        ReleaseAsset {
            version,
            filename,
            contents,
        }
    }

    pub fn sha256(asset: &ReleaseAsset) -> ReleaseAsset {
        let filename = format!("{}.sha256", asset.filename);
        let contents = file_contents::sha256(&asset.filename, &asset.contents)
            .as_bytes()
            .to_vec();
        ReleaseAsset {
            version: asset.version.clone(),
            filename,
            contents,
        }
    }

    pub fn ok_response(&self) -> Response<Vec<u8>> {
        response::Builder::new()
            .status(200)
            .body(self.contents.clone())
            .unwrap()
    }

    fn dfx_tarball_filename(version: &str) -> String {
        let platform = target::platform();
        format!("dfx-{version}-x86_64-{platform}.tar.gz")
    }
}
