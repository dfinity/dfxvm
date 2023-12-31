# dfxvm: The dfx version manager

*dfxvm* installs the [Internet Computer SDK][sdk] (dfx), enabling you
to easily switch between different versions.

## Documentation

See the [command-line reference](docs/cli-reference/index.md) for
documentation on using dfxvm.

## Installation

You can install dfxvm via `curl`.  The URLs below are temporary,
for use during initial development.

``` bash
sh -ci "$(curl -fsSL https://raw.githubusercontent.com/dfinity/sdk/dfxvm-install-script/install.sh)"
```

You will have an opportunity to select the version,
but you can also specify it on the command line:

``` bash
DFX_VERSION=0.15.1 sh -ci "$(curl -fsSL https://raw.githubusercontent.com/dfinity/sdk/dfxvm-install-script/install.sh)"
```

## Contribution

Contributions to dfxvm are welcome! For information about contributing,
see [CONTRIBUTING.md](CONTRIBUTING.md). Contributors must agree to a [CLA][cla].

## License

Copyright 2023 DFINITY Stiftung <sdk@dfinity.org>.

dfxvm is licensed under the [Apache 2.0 License](LICENSE).

## Acknowledgements

dfxvm is inspired by, and parts are copied from and/or derived from, [rustup][rustup],
which is also licensed under the [Apache 2.0 License](LICENSE).

[sdk]: https://github.com/dfinity/sdk
[cla]: https://github.com/dfinity/cla/blob/main/CLA.md
[rustup]: https://github.com/rust-lang/rustup
