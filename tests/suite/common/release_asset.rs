use crate::common::{
    file_contents,
    file_contents::{bash_script, dfx_tarball},
};
use httptest::http::{response, Response};
use semver::Version;

#[derive(Clone)]
pub struct ReleaseAsset {
    pub filename: String,
    pub url_path: String,
    pub contents: Vec<u8>,
}

impl ReleaseAsset {
    pub fn dfx_tarball(version: &str, snippet: &str) -> ReleaseAsset {
        Self::dfx_tarball_with_dfx_contents(version, bash_script(snippet).as_bytes())
    }

    pub fn dfx_tarball_with_dfx_contents(version: &str, executable: &[u8]) -> ReleaseAsset {
        let filename = Self::dfx_tarball_filename().to_string();
        let version = Version::parse(version).unwrap();

        // must match the download_url_template in ReleaseServer::new
        let url_path = format!("/any/arbitrary/path/{version}/{filename}");

        let contents = dfx_tarball(executable);
        ReleaseAsset {
            url_path,
            filename,
            contents,
        }
    }

    pub fn sha256(asset: &ReleaseAsset) -> ReleaseAsset {
        let filename = format!("{}.sha256", asset.filename);
        let url_path = format!("{}.sha256", asset.url_path);
        let contents = file_contents::sha256(&asset.filename, &asset.contents)
            .as_bytes()
            .to_vec();
        ReleaseAsset {
            url_path,
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

    pub fn dfx_tarball_basename() -> &'static str {
        #[cfg(target_os = "macos")]
        let basename = "dfx-x86_64-apple-darwin";
        #[cfg(target_os = "linux")]
        let basename = "dfx-x86_64-unknown-linux-gnu";
        basename
    }

    pub fn dfx_tarball_filename() -> String {
        let basename = Self::dfx_tarball_basename();
        let archive_format = "tar.gz";
        format!("{basename}.{archive_format}")
    }

    pub fn dfxvm_tarball_basename() -> String {
        #[cfg(target_arch = "aarch64")]
        let arch_and_os = "aarch64-apple-darwin";
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        let arch_and_os = "x86_64-apple-darwin";
        #[cfg(target_os = "linux")]
        let arch_and_os = "x86_64-unknown-linux-gnu";

        format!("dfxvm-{}", arch_and_os)
    }

    // tricky about testing this:
    // - we need to test that the dfxvm binary is updated
    // - we only have the current dfxvm binary to test with
    // - so we copy the binary and append a couple bytes to it.
    pub fn altered_dfxvm_binary() -> Vec<u8> {
        let mut altered_dfxvm = std::fs::read(crate::common::dfxvm_path()).unwrap();
        altered_dfxvm.push(0xCC);
        altered_dfxvm.push(0xCC);
        altered_dfxvm
    }

    pub fn altered_dfxvm_tarball() -> ReleaseAsset {
        let altered_dfxvm = Self::altered_dfxvm_binary();

        let basename = Self::dfxvm_tarball_basename();
        let filename = format!("{basename}.tar.gz");
        let url_path = format!("/dfxvm-latest-download-root/{filename}");
        let contents = file_contents::dfxvm_tarball(&altered_dfxvm);
        ReleaseAsset {
            url_path,
            filename,
            contents,
        }
    }
}
