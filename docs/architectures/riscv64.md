# riscv64

What exists of the riscv64 port of NONOS, why it does not build into a kernel, and what it would take.

## Status

riscv64 is not supported. The tree holds a backend under `src/arch/riscv64`, a linker script and a capsule target file, but no kernel target file, no make target and no CI job, and the backend's entry calls a function that does not exist. No page here gives a date for the port.

The comment on `nonos-arch-preview` says the feature compiles and boots the `aarch64` and `riscv64` trees in QEMU (`Cargo.toml:53-55`). For riscv64 that is not so.

## Why it does not build

```mermaid
flowchart LR
    S[_start] --> E[kernel_entry]
    E --> I[init]
    I --> K[kernel_main]
```

- `kernel_entry` fills `BootInfo` from the device tree, runs `init`, and then calls `crate::kernel_main` (`src/arch/riscv64/boot/entry.rs:19-27`). No file under `src/` defines `kernel_main`; x86_64 and aarch64 reach the shared kernel through `microkernel_init` instead.
- The shared kernel takes its boot facts as a `KernelHandoff`, and `ArchSpecificHandoff` has variants for x86_64 and aarch64 only; its comment says a new architecture adds one when its boot tree lands (`src/boot/handoff/kernel_handoff/arch.rs:17-34`).
- The repository root holds `x86_64-nonos.json` and `aarch64-nonos.json` and no riscv64 kernel target file. The `build` check of `nonos-verify` records the same gap for riscv64 in its loop over each `arch`: no kernel target file and no make build target, only the capsule target file (`nonos-verify/src/build.rs:103-116`).

To confirm the missing function on your own checkout:

```
grep -rn "fn kernel_main" src/
```

It prints nothing.
