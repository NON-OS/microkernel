# riscv64

What exists of the riscv64 port of NONOS, how far it boots, and what it would take.

## Status

riscv64 is not supported. The kernel builds and links for riscv64 and boots under QEMU `virt` with OpenSBI as far as the VM layer, which is not ported yet. It stops there by name. No loader starts it, and it has not run on real hardware. It needs `nonos-arch-preview`, as aarch64 does (`src/lib.rs:28-35`). No page here gives a date for the port.

```
make nonos-mk-riscv-boot-check
```

- `nonos-mk-riscv` builds the kernel for rustc's own `riscv64gc-unknown-none-elf` with `microkernel-core` and `nonos-arch-preview`.
- `nonos-mk-riscv-boot-check` boots it with `-M virt -cpu rv64 -m 512 -bios default` and holds the serial log to the markers below, with no trap, panic or fatal line on the way (`scripts/check_riscv64_boot.py`). CI runs it in `ci-build-aarch64.yml`.
- The kernel's assembly is built for RV64GC with the `lp64d` ABI, the Rust target's own (`c_target`, `build_c_target.rs`).

## How far a boot gets

The serial log after OpenSBI's banner, at `-m 512`:

```
[BENCH] 0 microkernel_init_start
[BOOT-ENTROPY] nonce drawn, hardware generator absent
[MEM] phys init OK
[DMA] display pool pages=8192
[DMA] low32 pool unavailable (no low region)
[fbconsole] on-screen log disabled; serial only
[NONOS] Microkernel init
[RNG] seeded from: firmware-seed
[TIMER] Initialized, generic timer freq=10 MHz
[VM] riscv64: the VM layer is not ported (no Sv48 direct map, no page descriptors); stopping
```

- OpenSBI starts the kernel in S-mode at `0x80200000` with the device tree in a1. The kernel touches no PMP CSR: those are M-mode, the firmware's own (`src/arch/riscv64/security/init.rs`).
- The device tree gives RAM, the console UART, the PLIC, the harts and the timebase. Its blob, its reservation block and `/reserved-memory`, which holds OpenSBI's own range, go into `BootInfo.reserved` (`src/arch/riscv64/boot/dtb_adapter/`). A tree that cannot be read stops the boot.
- The boot tables map RAM as data, and the image by section: text read and execute, read-only data read only, the rest read and write. They also map the UART and the PLIC (`src/arch/riscv64/mmu/image_map.rs`).
- The entry takes the shared path, as aarch64 does: the bootstrap heap, `KernelHandoff::from_riscv64`, then `microkernel_init` (`src/arch/riscv64/boot/entry.rs`).
- The frame allocator takes the widest RAM region, past the image, with every reserved range out (`init_boot_memory`, `src/arch/riscv64/boot/memory.rs`). There is no low 32-bit DMA pool: no riscv64 device is given DMA yet.
- The RNG is seeded from the tree's `/chosen` `rng-seed`, which QEMU `virt` provides; the seed is zeroed in the blob once kept (`src/arch/common/boot_entropy.rs`, `src/arch/common/boot_seed.rs`).
- The VM layer needs a page-descriptor backend and a direct map at `DIRECTMAP_BASE`, which Sv39 cannot address. riscv64 has neither yet, so `require_vm_port` stops the boot before the first write through the direct map (`src/arch/paging/ported.rs`).

The shared code riscv64 needed has a riscv64 branch:
- the scheduler's saved kernel `Context`, which holds s0 to s11, ra and sp (`src/arch/riscv64/context/kernel.rs`);
- MMIO ordering as RISC-V FENCEs over device input and output and memory reads and writes (`src/arch/riscv64/mmio_order/`, selected in `src/arch/common/mmio_order.rs`);
- the DMA sync windows, a full FENCE, because the base ISA has no cache maintenance. A non-snooping part would need Zicbom, which is not in the tree (`src/arch/riscv64/dma_sync.rs`, selected in `src/arch/common/dma_sync.rs`).

## What exists

`src/arch/riscv64` holds 217 files and 8608 lines, headers included.

| Module | Size | What it holds |
|---|---|---|
| `abi` (`src/arch/riscv64/mod.rs:17`) | 10 files, 295 lines | the `Riscv64` implementation of `ArchOps` |
| `asm` (`src/arch/riscv64/mod.rs:18`) | 12 files, 508 lines | `start.S`, the trap entry with its register save and restore, user entry and resume, FP save and restore, secondary hart start |
| `boot` (`src/arch/riscv64/mod.rs:19`) | 27 files, 1048 lines | entry, the device tree adapter, the hart id, `BootInfo`, `init`, secondary harts, stacks |
| `context` (`src/arch/riscv64/mod.rs:21`) | 15 files, 695 lines | first entry to user mode, saving and resuming user frames, the switch, the scheduler's saved kernel `Context` |
| `cpu` (`src/arch/riscv64/mod.rs:22`) | 22 files, 874 lines | CSR access, extension and capability queries, fences, hart and vendor ids, interrupt masking |
| `fpu` (`src/arch/riscv64/mod.rs:23`) | 10 files, 355 lines | lazy floating point state per task |
| `interrupts` (`src/arch/riscv64/mod.rs:24`) | 18 files, 696 lines | trap causes, the trap frame, handlers, the `stvec` install |
| `mmu` (`src/arch/riscv64/mod.rs:25`) | 20 files, 1128 lines | Sv39 and Sv48 tables, `satp`, page attributes, TLB fences |
| `plic` (`src/arch/riscv64/mod.rs:26`) | 18 files, 687 lines | the PLIC: claim, complete, priorities, per-hart contexts, interrupt handlers including ones bound for capsules |
| `sbi` (`src/arch/riscv64/mod.rs:27`) | 29 files, 1025 lines | SBI calls: base, hart start, stop and suspend, IPIs, remote fences, the timer, the console, reset |
| `security` (`src/arch/riscv64/mod.rs:28`) | 7 files, 246 lines | control-flow integrity; PMP is the M-mode firmware's |
| `timer` (`src/arch/riscv64/mod.rs:29`) | 13 files, 445 lines | the CLINT, tick conversion, deadlines, delays |
| `uart` (`src/arch/riscv64/mod.rs:30`) | 10 files, 361 lines | the NS16550 console |

As written, the boot would go like this:

- `_start` takes the hart id in a0 and the device tree in a1 from SBI firmware, and `_riscv64_secondary_start` is the address handed to SBI to start the other harts (`src/arch/riscv64/asm/start.S:1-9`).
- `init` installs `stvec`, runs the security setup, `init_mmu`, the PLIC when the device tree names one, and the timer, then starts the other harts. A PLIC or timer that does not start stops the boot with a line that names it (`src/arch/riscv64/boot/init.rs`).
- Secondary harts start through SBI `hart_start` (`src/arch/riscv64/boot/multicore/start_harts.rs:38`).
- The kernel builds every page table for Sv39, `KERNEL_MMU_MODE` (`src/arch/riscv64/mmu/mode.rs:26-28`).
- `BootInfo` starts from QEMU's `virt` layout: 4 GB of RAM at `0x8000_0000`, the kernel at `0x8020_0000`, the UART at `0x1000_0000`, the CLINT at `0x0200_0000`, no PLIC and one hart (`src/arch/riscv64/boot/info/types.rs`). Those values only open the console: without a device tree the boot stops (`src/arch/riscv64/boot/entry.rs`).

`linker_riscv64.ld` places the image at `0x80200000`, above the 2 MB its comment leaves to the SBI firmware, in three `PT_LOAD` segments, and sets the global pointer for gp-relative addressing (`PHDRS`, `linker_riscv64.ld:9-36`). It defines the `__kernel_text_start` to `__kernel_bss_end` pairs the other two scripts define, which the boot tables and the kernel's W^X check read. `userland/riscv64-nonos-user.json` builds capsules for `generic-rv64` with the M, A, F, D and C extensions in its `features` and the `lp64d` ABI (`userland/riscv64-nonos-user.json:19-21`).

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

- Sv48 boot tables with the direct map at `DIRECTMAP_BASE`, and a riscv64 page-descriptor backend in `src/arch/paging/descriptor/`, with the root and TLB facades;
- riscv64 branches in the facades listed above;
- a loader, an image and a boot cell that reaches Core ready.

[aarch64](aarch64.md) shows how far the same steps took that port.

## See also

- [Architectures](README.md)
- [aarch64](aarch64.md)
- [x86_64](x86_64.md)
- [Boot handoff](../kernel/boot-handoff.md)
- [Contributing](../contributing/README.md)
