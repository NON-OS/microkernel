# riscv64

What exists of the riscv64 port of NONOS, why it does not build into a kernel, and what it would take.

## Status

riscv64 is not supported. The tree holds a backend under `src/arch/riscv64`, a linker script and a capsule target file, but no kernel target file, no make target and no CI job, and the backend's entry calls a function that does not exist. No page here gives a date for the port.

The comment on `nonos-arch-preview` says the feature compiles and boots the `aarch64` and `riscv64` trees in QEMU (`Cargo.toml:53-55`). For riscv64 that is not so.
