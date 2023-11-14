use flate2::write::GzEncoder;
use flate2::Compression;
use sha2::{Digest, Sha256};
use std::io::Write;
use tar::Builder;

pub fn dfx_tar_gz(script: &str) -> Vec<u8> {
    binary_tar_gz("dfx", script.as_bytes())
}

/// make a .tar.gz that looks like a dfx release tarball
/// $ tar -tvf dfx-0.15.1-x86_64-darwin.tar.gz
// -rwxr-xr-x  0 runner staff 128330472 Oct  5 00:45 dfx
/// $ tar -xvf dfx-0.15.1-x86_64-darwin.tar.gz
/// x dfx
/// $ ls -l dfx
/// -rwxr-xr-x    1 ericswanson  staff  128330472 Oct  5 00:45 dfx
fn binary_tar_gz(binary_name: &str, contents: &[u8]) -> Vec<u8> {
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
