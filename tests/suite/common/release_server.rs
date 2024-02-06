use crate::common::file_contents::manifest_json;
use crate::common::{ReleaseAsset, TempHomeDir};
use httptest::http::response;
use httptest::{matchers::request, responders::status_code, Expectation, Server};

pub struct ReleaseServer {
    server: Server,
}

impl ReleaseServer {
    pub fn new(home_dir: &TempHomeDir) -> Self {
        let server = Server::run();
        let download_url_template =
            server.url_str("/any/arbitrary/path/{{version}}/{{basename}}.{{archive-format}}");
        home_dir
            .settings()
            .write_download_url_template(&download_url_template);
        let manifest_url = server.url_str("/manifest.json");
        home_dir.settings().write_manifest_url(&manifest_url);
        home_dir
            .settings()
            .write_dfxvm_latest_download_root_url(&server.url_str("/dfxvm-latest-download-root"));
        Self { server }
    }

    pub fn expect_get(&self, asset: &ReleaseAsset) {
        self.server.expect(
            Expectation::matching(request::method_path("GET", asset.url_path.clone()))
                .respond_with(asset.ok_response()),
        );
    }

    pub fn expect_get_respond_not_found(&self, asset: &ReleaseAsset) {
        self.server.expect(
            Expectation::matching(request::method_path("GET", asset.url_path.clone()))
                .respond_with(status_code(404)),
        );
    }

    pub fn expect_get_manifest(&self, contents: &str) {
        self.server.expect(
            Expectation::matching(request::method_path("GET", "/manifest.json")).respond_with(
                response::Builder::new()
                    .status(200)
                    .body(contents.as_bytes().to_vec())
                    .unwrap(),
            ),
        );
    }

    pub fn expect_install_latest(&self) {
        let tarball = ReleaseAsset::dfx_tarball("0.15.0", "echo 'this is dfx 0.15.0'");
        let sha256 = ReleaseAsset::sha256(&tarball);
        self.expect_get(&tarball);
        self.expect_get(&sha256);
        self.expect_get_manifest(&manifest_json("0.15.0"));
    }

    pub fn expect_get_dist_manifest(&self, contents: &str) {
        self.server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/dfxvm-latest-download-root/dist-manifest.json",
            ))
            .respond_with(
                response::Builder::new()
                    .status(200)
                    .body(contents.as_bytes().to_vec())
                    .unwrap(),
            ),
        );
    }
}
