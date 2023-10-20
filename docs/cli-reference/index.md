# Command-line reference

The dfxvm binary is a chimera, changing its behavior based on
the name of the binary.

When run as `dfx`, it determines which version of dfx to use and then
dispatches execution to it, passing through all command-line arguments.

When run as `dfxvm`, it provides commands for managing dfx versions
and for updating itself.

When run as `dfxvm-init`, it performs one-time installation tasks.

For more information, see the documentation
for individual commands:

- [dfx](dfx/dfx.md)
- [dfxvm](dfxvm/dfxvm.md)
  - [dfxvm default](dfxvm-default.md)
  - [dfxvm install](dfxvm/dfxvm-install.md)
  - [dfxvm list](dfxvm-list.md)
  - [dfxvm self](dfxvm/dfxvm-self.md)
    - [dfxvm self uninstall](dfxvm/dfxvm-self-uninstall.md)
    - [dfxvm self update](dfxvm/dfxvm-self-update.md)
  - [dfxvm uninstall](dfxvm/dfxvm-uninstall.md)
  - [dfxvm update](dfxvm/dfxvm-update.md)
- [dfxvm-init](dfxvm-init/dfxvm-init.md)
