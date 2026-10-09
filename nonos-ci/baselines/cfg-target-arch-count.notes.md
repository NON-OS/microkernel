# `cfg(target_arch)` outside `src/arch` baseline notes

The number in `cfg-target-arch-count.txt` is the count of lines matching
`cfg(target_arch` under `src/`, excluding `src/arch/`, as the static gate
measures it:

```
grep -rn 'cfg(target_arch' src --include='*.rs' | grep -v '^src/arch/' | wc -l
```

The baseline is shrink-only. It must not grow without an entry here that
names the new sites and why they are not arch-leaks.

## Current value: 227

The stale value was 116. It did not grow because arch-leaks crept in; it
grew because the tree gained real aarch64 and riscv64 support (the
`nonos-arch-preview` targets) after 116 was recorded, and every portable
code path that behaves differently per arch carries a `#[cfg(target_arch)]`
arm the gate counts. This pass reduced the live count and then recorded the
true remainder.

## What this pass removed (234 -> 227)

- `src/nonos_time/` deleted: an orphaned module with no `mod nonos_time`
  declaration anywhere in the crate, so it was never compiled. It carried
  3 counted arms (`high_precision.rs` x2, `sleep.rs` x1).
- `src/interrupts/safety/guard.rs`: the `InterruptGuard` helpers were
  rewritten to route through the `crate::arch::cpu` shims
  (`<Arch as ArchOps>`), removing 3 local `#[cfg(target_arch = "x86_64")]`
  inline-asm arms. This also fixed a real bug: on aarch64 the old local
  helpers were no-ops, so the guard never masked interrupts; it now masks
  through the DAIF path.
- `src/drivers/pci/mod.rs`: deleted the unused
  `pub use ... PciStats as LegacyPciStats` re-export (no caller in the
  tree), removing 1 counted arm.
- `src/hardware/broker/irq/mod.rs`: reworded a comment that literally
  contained `#![cfg(target_arch = "x86_64")]` and so tripped the grep as a
  false positive; the meaning is unchanged.

One site was added this pass, honestly: `src/drivers/pci/config/mod.rs`
gained a `#[cfg(target_arch = "x86_64")]` on the `map_ecam_bus` re-export.
The `extended` module it comes from is already x86_64-only, and its only
caller is x86_64 ACPI code, but the re-export was ungated and broke the
aarch64 build with E0432. Gating it matches the module and the sibling
`read_extended32` re-export. Net change this pass: -8 +1 = -7.

## Breakdown of the 227 (all checkable with the grep above)

By arch arm:

- `target_arch = "x86_64"`: 187
- `target_arch = "aarch64"`: 33
- `target_arch = "riscv64"`: 7

By `src/` subsystem:

- hardware 33, memory 27, process 25, smp 23, security 14,
  kernel_core 14, interrupts 13, sys 12, syscall 11, drivers 10,
  log 8, nonos_main.rs 8, boot 7, elf 6, crypto 6, fs 4, entry 3,
  bus 2, context 1

These 227 sites are of two honest kinds, neither of which is the arch-leak
the gate guards against (generic code inlining platform behavior that
belongs behind the `Arch` trait):

1. Multi-arch dispatch arms (the 33 aarch64 + 7 riscv64 arms, each paired
   with an x86_64 arm). These select genuinely different per-arch code and
   are the correct portable form, for example: context save/restore
   (`process/context`, `process/core/*`), ELF machine and relocation
   constants (`elf/types/constants/machine.rs`, `elf/loader/core/relocate.rs`),
   speculation barriers (`security/hardening/speculation/*`), MMIO and DMA
   ordering backends (`memory/mmio/ordering/backend.rs`,
   `memory/dma/coherency/backend.rs`), IPI dispatch and topology detection
   (`smp/ipi_dispatch`, `smp/topology/detection`), the IRQ broker backend
   selector (`hardware/broker/irq/mod.rs`), and procfs cpuinfo
   (`fs/procfs/cpuinfo`). Deleting either arm would break that target.

2. x86_64-only platform gates (lone `#[cfg(target_arch = "x86_64")]`
   guarding code that only exists on the x86 platform, compiled out on the
   preview aarch64/riscv64 targets and live on the production x86_64
   target): port-I/O syscalls (`syscall/microkernel/pio/mod.rs`, 10), the
   APIC/IO-APIC IRQ broker backend (`hardware/broker/irq`, `hardware/broker`),
   the VGA log backend (`log/*`), GDT/IDT bring-up and the SMP start path
   (`smp/*`, `kernel_core/init`, `interrupts/mod.rs`), XSAVE/xstate and PAT
   (`process/userspace`, `memory`), and the x86 boot entry
   (`nonos_main.rs`, `boot/*`).

## Shrink track

The remaining reducible arch-leaks are the deferred `Arch`-trait migration
(M-ARCH-0): generic kernel code should reach the platform through
`src/arch/abi.rs` rather than naming a platform module. As each call site
moves behind the trait, drop this count and record the move here. The
multi-arch dispatch arms (category 1) are permanent and correct; they leave
the count only if the subsystem itself moves under `src/arch`.
