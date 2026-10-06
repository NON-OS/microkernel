# Architectures

Which CPU architectures NONOS runs on, how complete each port is, and how a port plugs into the kernel.

## Support level

| Architecture | Support | In short |
|---|---|---|
| [x86_64](x86_64.md) | Supported | The release target: a UEFI loader, every build profile, the CI boot check, and one hardware report. |
| [aarch64](aarch64.md) | Preview | Builds only with `nonos-arch-preview`; CI builds it and boot-tests it under QEMU `virt`. No loader, no release image, no real hardware tested. |
| [riscv64](riscv64.md) | Not supported | A backend with no kernel target file and no make target; its entry calls a function that does not exist, so it does not build into a kernel. |

A kernel for any architecture other than x86_64 stops at `compile_error!` unless the `nonos-arch-preview` feature is on, so a release cannot ship another architecture by accident (`src/lib.rs:28-35`).
