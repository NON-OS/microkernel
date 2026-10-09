# nonos-panic

The native runtime's `#[panic_handler]`: it exits with 134 through `MEXT` and
prints nothing. `nonos_runtime` links it. See [the libc and std page](../../docs/handbook/userland/libc-and-std.md).
