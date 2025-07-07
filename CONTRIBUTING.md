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

## Release Process

### Prerequisites

Prerequisites to make a release:

```bash
cargo install cargo-release --version 0.25.17
cargo install cargo-dist --version 0.28.0 --locked
```

- Use `cargo-release` version `0.25.17 `as the versions above require rustc `1.85` or newer, but the Rust toolchain for dfxvm is still `1.84.0` which is defined [here](./rust-toolchain.toml)
- Use `--locked` to install `cargo-dist` to avoid upgrading some dependencies to the latest versions which have compatibility problems.

You can run `cargo install --list` to make sure if you have the correct `cargo-release` and `cargo-dist` installed.

### Making a release

To make a release, follow these steps:

1. Run the [Prepare Release][prepare-release-workflow] workflow.

   Make sure you choose the right option to specify the correct [SemVer version](https://github.com/crate-ci/cargo-release/blob/master/docs/reference.md#bump-level).

1. Obtain approval and merge the PR that the workflow generated.

   The PR created by a workflow won't trigger other workflows according to this [document](https://docs.github.com/en/actions/how-tos/writing-workflows/choosing-when-your-workflow-runs/triggering-a-workflow#triggering-a-workflow-from-a-workflow).You can simply use `git commit --allow-empty` to create an empty commit and push to trigger the jobs.

1. Run the following locally:

    ```bash
    git checkout main
    git pull
    dist plan
    cargo release --execute
    ```

    - The `dist plan` command will get a plan of what to build, and check project status.
    - The `cargo release --execute` will push the tag according to the version created by the PR in previsou step, and execute the [Release][release-workflow] workflow.

[code-of-conduct]: https://github.com/dfinity/ic-docutrack/blob/main/.github/CODE_OF_CONDUCT.md
[cla]: https://github.com/dfinity/cla/blob/master/CLA.md
[prepare-release-workflow]: https://github.com/dfinity/dfxvm/actions/workflows/prepare-release.yml
[release-workflow]: https://github.com/dfinity/dfxvm/actions/workflows/release.yml
