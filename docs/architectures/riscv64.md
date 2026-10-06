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

## What exists

`src/arch/riscv64` holds 218 files and 8423 lines, headers included.

| Module | Size | What it holds |
|---|---|---|
| `abi` (`src/arch/riscv64/mod.rs:17`) | 10 files, 295 lines | the `Riscv64` implementation of `ArchOps` |
| `asm` (`src/arch/riscv64/mod.rs:18`) | 12 files, 508 lines | `start.S`, the trap entry with its register save and restore, user entry and resume, FP save and restore, secondary hart start |
| `boot` (`src/arch/riscv64/mod.rs:19`) | 23 files, 839 lines | entry, the device tree adapter, the hart id, `BootInfo`, `init`, secondary harts, stacks |
| `context` (`src/arch/riscv64/mod.rs:21`) | 12 files, 513 lines | first entry to user mode, saving and resuming user frames, the switch |
| `cpu` (`src/arch/riscv64/mod.rs:22`) | 22 files, 874 lines | CSR access, extension and capability queries, fences, hart and vendor ids, interrupt masking |
| `fpu` (`src/arch/riscv64/mod.rs:23`) | 10 files, 355 lines | lazy floating point state per task |
| `interrupts` (`src/arch/riscv64/mod.rs:24`) | 18 files, 692 lines | trap causes, the trap frame, handlers, the `stvec` install |
| `mmu` (`src/arch/riscv64/mod.rs:25`) | 19 files, 1046 lines | Sv39 and Sv48 tables, `satp`, page attributes, TLB fences |
| `plic` (`src/arch/riscv64/mod.rs:26`) | 18 files, 687 lines | the PLIC: claim, complete, priorities, per-hart contexts, interrupt handlers including ones bound for capsules |
| `sbi` (`src/arch/riscv64/mod.rs:27`) | 29 files, 1026 lines | SBI calls: base, hart start, stop and suspend, IPIs, remote fences, the timer, the console, reset |
| `security` (`src/arch/riscv64/mod.rs:28`) | 20 files, 743 lines | control-flow integrity and PMP setup |
| `timer` (`src/arch/riscv64/mod.rs:29`) | 13 files, 432 lines | the CLINT, tick conversion, deadlines, delays |
| `uart` (`src/arch/riscv64/mod.rs:30`) | 10 files, 353 lines | the NS16550 console |

As written, the boot would go like this:

- `_start` takes the hart id in a0 and the device tree in a1 from SBI firmware, and `_riscv64_secondary_start` is the address handed to SBI to start the other harts (`src/arch/riscv64/asm/start.S:1-9`).
- `init` installs `stvec`, runs the security setup, `init_mmu`, the PLIC when the device tree names one, and the timer, then starts the other harts. Any failure calls `halt` with no message (`src/arch/riscv64/boot/init.rs:20-39`), where aarch64 says why before it stops.
- Secondary harts start through SBI `hart_start` (`src/arch/riscv64/boot/multicore/start_harts.rs:38`).
- The kernel builds every page table for Sv39, `KERNEL_MMU_MODE` (`src/arch/riscv64/mmu/mode.rs:26-28`).
- Without a device tree, `BootInfo` assumes QEMU's `virt` layout: 4 GB of RAM at `0x8000_0000`, the kernel at `0x8020_0000`, the UART at `0x1000_0000`, the CLINT at `0x0200_0000`, no PLIC and one hart (`ram_size`, `src/arch/riscv64/boot/info/types.rs:42-60`).

`linker_riscv64.ld` places the image at `0x80200000`, above the 2 MB its comment reserves for OpenSBI, in three `PT_LOAD` segments, and sets the global pointer for gp-relative addressing (`PHDRS`, `linker_riscv64.ld:11-32`). Unlike the other two scripts it defines no `__kernel_text_start` pairs, which the kernel's W^X check reads. `userland/riscv64-nonos-user.json` builds capsules for `generic-rv64` with the M, A, F, D and C extensions in its `features` and the `lp64d` ABI (`userland/riscv64-nonos-user.json:19-21`).

Seven extraction crates lower small riscv64 functions to Lean, among them `riscv64_boot_hart_id_store`, `riscv64_cpu_caps_query`, `riscv64_cpu_extensions_query` and `riscv64_sbi_extensions_extension` (`verification/extraction/crates.json:3460-3515`), each with a refinement module in `verification/extraction/lean/NonosExtraction/`. They reason about those functions in Lean; they do not make the port build.
