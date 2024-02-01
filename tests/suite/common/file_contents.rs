use crate::common::ReleaseAsset;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::io::Write;
use tar::Builder;

/// make a .tar.gz that looks like a dfx release tarball
/// $ tar -tvf dfx-0.15.1-x86_64-darwin.tar.gz
// -rwxr-xr-x  0 runner staff 128330472 Oct  5 00:45 dfx
/// $ tar -xvf dfx-0.15.1-x86_64-darwin.tar.gz
/// x dfx
/// $ ls -l dfx
/// -rwxr-xr-x    1 ericswanson  staff  128330472 Oct  5 00:45 dfx
pub fn binary_tar_gz(binary_name: &str, contents: &[u8]) -> Vec<u8> {
    let tar_buffer = Vec::new();
    let mut tar = Builder::new(Vec::new());

    let mut file_header = tar::Header::new_gnu();
    file_header.set_mode(0o755); // Make it executable
    file_header.set_size(contents.len() as u64);
    file_header.set_cksum();

    tar.append_data(&mut file_header, binary_name, contents)
        .unwrap();

    let mut gzipped = GzEncoder::new(tar_buffer, Compression::default());
    gzipped.write_all(&tar.into_inner().unwrap()).unwrap();

    gzipped.finish().unwrap()
}

/// generate same format as output of `shasum -a 256 <filename>`
pub fn sha256(filename: &str, contents: &[u8]) -> String {
    let hash = hex::encode(Sha256::digest(contents));
    format!("{hash}  {filename}")
}

pub fn bash_script(snippet: &str) -> String {
    format!(
        r#"#!/usr/bin/env bash

set -e

{snippet}
"#
    )
}

pub fn manifest_json(latest: &str) -> String {
    json!({
        "tags": {
            "latest": latest
        },
        "versions": [
            "0.5.0",
            "0.5.2"
        ]
    })
    .to_string()
}

pub fn dist_manifest_json(latest: &str) -> String {
    json!({
        "releases": [
            {
                "app_name": "dfxvm",
                "app_version": latest
            }
        ]
    })
    .to_string()
}

// dfxvm tarball looks like:
// $ tar -tvf dfxvm-aarch64-apple-darwin.tar.gz
// drwxr-xr-x  0 501    20          0 Dec 19 11:24 dfxvm-aarch64-apple-darwin/
// -rw-r--r--  0 501    20       1342 Dec 19 11:21 dfxvm-aarch64-apple-darwin/README.md
// -rw-r--r--  0 501    20       1277 Dec 19 11:21 dfxvm-aarch64-apple-darwin/CHANGELOG.md
// -rw-r--r--  0 501    20      11357 Dec 19 11:21 dfxvm-aarch64-apple-darwin/LICENSE
// -rwxr-xr-x  0 501    20    5747075 Dec 19 11:24 dfxvm-aarch64-apple-darwin/dfxvm

pub fn dfxvm_tarball(contents: &[u8]) -> Vec<u8> {
    let dirname = ReleaseAsset::dfxvm_tarball_basename();

    let tar_buffer = Vec::new();
    let mut tar = Builder::new(Vec::new());

    append_file(&mut tar, 0o644, &dirname, "README.md", b"the readme\n");
    append_file(
        &mut tar,
        0o644,
        &dirname,
        "CHANGELOG.md",
        b"the changelog\n",
    );
    append_file(&mut tar, 0o644, &dirname, "LICENSE", b"the license\n");
    append_file(&mut tar, 0o755, &dirname, "dfxvm", contents);

    let mut gzipped = GzEncoder::new(tar_buffer, Compression::default());
    gzipped.write_all(&tar.into_inner().unwrap()).unwrap();

    gzipped.finish().unwrap()
}

fn append_file(
    tar: &mut Builder<Vec<u8>>,
    mode: u32,
    dirname: &str,
    filename: &str,
    contents: &[u8],
) {
    let path = format!("{}/{}", dirname, filename);
    let mut file_header = tar::Header::new_gnu();
    file_header.set_mode(mode);
    file_header.set_size(contents.len() as u64);
    file_header.set_cksum();

    tar.append_data(&mut file_header, path, contents).unwrap();
}
