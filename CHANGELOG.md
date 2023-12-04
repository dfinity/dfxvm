# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.1.1] - 2023-12-04

- Added `dfx` mode, which selects a dfx version and dispatches execution to it.
- Added `dfxvm install` command, which installs a dfx version.
- Added `dfxvm default` command, which sets or displays the default dfx version.
- Added `dfxvm update` command, which sets the latest dfx version as the default.
- Added `dfxvm uninstall` command, which uninstalls a dfx version.
- Added `dfxvm list` command, which lists all installed dfx versions.
- Added `dfxvm-init` mode, which installs dfxvm and dfx.
  - does not yet source the env file in profile scripts.
  - does not yet clean up previously-installed dfx versions.

<!-- next-url -->
[Unreleased]: https://github.com/dfinity/dfxvm/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/dfinity/dfxvm/compare/828e4ed...v0.1.1
