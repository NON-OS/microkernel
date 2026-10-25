# riscv64

What exists of the riscv64 port of NONOS, how far it builds, and what it would take.

## Status

riscv64 is not supported. The kernel type-checks for riscv64, and that is all: nothing links it into an image, nothing boots it, and no loader starts it. It needs `nonos-arch-preview`, as aarch64 does (`src/lib.rs:28-35`). No page here gives a date for the port.

```
make nonos-mk-riscv-check
```

- That target runs `cargo check` for rustc's own `riscv64gc-unknown-none-elf`, with `microkernel-core` and `nonos-arch-preview` (`nonos-mk-riscv-check`, `mk/20-build.mk`). CI runs it in `ci-build-aarch64.yml`.
- The kernel's assembly is built for RV64GC with the `lp64d` ABI, the Rust target's own (`c_target`, `build_c_target.rs`).
- There is no riscv64 kernel target file at the repository root and no link step.

## How far a boot would get

```mermaid
flowchart LR
    S[_start] --> E[kernel_entry]
    E --> I[init]
    I --> H[KernelHandoff]
    H --> K[microkernel_init]
    K --> M[no boot memory: stop]
```

- `kernel_entry` fills `BootInfo` from the device tree and runs `init`. It then arms the bootstrap heap, builds a `KernelHandoff` with the riscv64 variant of `ArchSpecificHandoff`, and calls `microkernel_init`, as aarch64 does (`src/arch/riscv64/boot/entry.rs`, `KernelHandoff::from_riscv64` in `src/boot/handoff/kernel_handoff/riscv64/from.rs`).
- No boot memory path is written for riscv64: nothing gives the frame allocator its RAM. `microkernel_init` stops at that step and prints `[MEM] riscv64: no boot memory path yet; stopping` (`init_arch_memory_and_framebuffer`, `src/kernel_core/init/entry/init_arch_memory_and_framebuffer.rs`).
- None of this has run: the kernel has not been linked for riscv64.

The shared code riscv64 needed in order to compile now has a riscv64 branch:
- the scheduler's saved kernel `Context`, which holds s0 to s11, ra and sp (`src/arch/riscv64/context/kernel.rs`);
- MMIO ordering as RISC-V FENCEs over device input and output and memory reads and writes (`src/arch/riscv64/mmio_order/`, selected in `src/arch/mmio_order.rs`);
- the DMA sync windows. These are a full FENCE, because the base ISA has no cache maintenance; a non-snooping part would need Zicbom, which is not in the tree (`src/arch/riscv64/dma_sync.rs`, selected in `src/arch/dma_sync.rs`).

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

## What the shared code answers on riscv64

The facade modules in `src/arch` have no riscv64 branch, so riscv64 gets each one's fallback:

- `time_counter_hz` answers 0, so the clock counts as not calibrated (`src/arch/time_counter.rs:39-52`).
- `unix_timestamp` answers `None`, so there is no wall clock before the network (`src/arch/wall_clock.rs:29-36`).
- `random_u64` answers `None` (`src/arch/cpu_random/read.rs:29-41`).
- `send_ipi` answers an error (`src/arch/interrupt_controller/ipi.rs:28-38`).
- `init_broker_irq_routing`, which installs the routes that carry device interrupts for the [hardware broker](../overview/glossary-hardware-broker.md#hardware-broker), answers an error (`src/arch/init_broker_irq_routing.rs:17-34`).
- Port I/O compiles its `backend` from `unsupported.rs` (`src/arch/port_io/mod.rs:35-37`).
- `set_user_tls` reports failure (`src/arch/context/tls.rs:25-39`).

## What it would take

From the code above, a riscv64 kernel still needs at least:

- a boot memory path that hands the device tree's RAM to the frame allocator, as `src/arch/aarch64/boot/memory.rs` does;
- a kernel target file, a link step and an image;
- the `__kernel_text_start` and related section symbols in `linker_riscv64.ld`;
- riscv64 branches in the facades listed above;
- a CI job that boots it, as `ci-boot-aarch64.yml` does for aarch64.

[aarch64](aarch64.md) shows how far the same steps took that port.

## See also

- [Architectures](README.md)
- [aarch64](aarch64.md)
- [x86_64](x86_64.md)
- [Boot handoff](../kernel/boot-handoff.md)
- [Contributing](../contributing/README.md)
