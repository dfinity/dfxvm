# Contributing to `dfxvm`

Thank you for your interest in contributing to `dfxvm`! By participating in
this project, you agree to abide by our [Code of Conduct][code-of-conduct].

## CLA

All code contributions are subject to our [Contributor License Agreement (CLA)][cla].
When you open your first PR, the CLA bot will prompt you to agree to the CLA
before the PR can be reviewed and merged.

## Documentation

Every change to the command-line interface must contain documentation.
We use `clap`, so Rustdoc comments turn into CLI documentation. Additionally,
this in-code documentation must be mirrored by a corresponding change
in `docs/cli-reference`. Finally, any feature or notable bugfix should be
mentioned in [CHANGELOG.md](CHANGELOG.md), under the `## Unreleased` header.

## Guidelines

### anyhow

The only permitted use of anyhow::Result is in the return type of `fn main`.
Everywhere else must use `core::result::Result`.

[code-of-conduct]: https://github.com/dfinity/ic-docutrack/blob/main/.github/CODE_OF_CONDUCT.md
[cla]: https://github.com/dfinity/cla/blob/master/CLA.md
