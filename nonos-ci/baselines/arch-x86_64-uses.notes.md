# `crate::arch::x86_64::*` baseline notes

The number in `arch-x86_64-uses.txt` is the count of direct
`crate::arch::x86_64::` path uses under `src/`, excluding `src/arch/`, as
the static gate measures it:

```
grep -rn 'crate::arch::x86_64::' src --include='*.rs' | grep -v '^src/arch/' | wc -l
```

Generic kernel code should reach the platform through the `Arch` trait
(`src/arch/abi.rs`); a direct path import is an arch-leak. The baseline is
shrink-only and must not grow without an entry here.

## Current value: 100

The actual count had regressed to 132 while the committed baseline stayed
at 100, which is what the gate was failing on. This pass brought the actual
back down to the 100 baseline with real code changes, so the gate passes
again with no bump.

## What this pass removed (132 -> 100)

- Moved `src/memory/iommu/backend_x86_64/` to
  `src/arch/x86_64/iommu/backend/` (the x86 VT-d / AMD-Vi backend, which
  lives next to the hardware it drives). `memory::iommu::backend` still
  pulls it in by `#[path]`, so `memory::iommu` stays the single facade, but
  the 25 `crate::arch::x86_64::` uses inside those files are now internal to
  `src/arch` and no longer counted.
- Deleted `src/nonos_time/` (orphaned, never compiled): removed 4 uses
  (`high_precision.rs` x3, `mod.rs` x1).
- Deleted `src/kernel_core/init/entry/diagnostics_silenced.rs` (a
  comment-only bring-up helper whose body was entirely inside a block
  comment) and its `mod` line: removed 1 use.
- Reworded a doc comment in `src/syscall/contract/mod.rs` that literally
  contained `crate::arch::x86_64::...` and tripped the grep: removed 1.
- Deleted the unused `LegacyPciStats` re-export in
  `src/drivers/pci/mod.rs`: removed 1.

Related fix folded in: the earlier commit that moved `amd_vi` under
`arch::x86_64::iommu::amd_vi` left callers pointing at the old
`arch::x86_64::amd_vi` path. The outside-`src/arch` caller
(`kernel_core/init/entry/init_dma_protection.rs`) broke the x86_64 build
(E0433); it and the five in-tree callers were repointed at the real
`iommu::amd_vi` path. The outside caller still counts (same arm, longer
path), so this does not change the count.

## Breakdown of the 100 (all checkable with the grep above)

By `src/` subsystem:

- interrupts 21, sys 13, smp 13, hardware 12, boot 10, kernel_core 9,
  process 6, log 6, memory 5, drivers 2, syscall 1, security 1, crypto 1

By arch submodule reached:

- acpi 22, diag 17, interrupt 13, cpu 10, vga 6, iommu 6, gdt 5, boot 4,
  time 3, syscall 3, paging 3, pat 2, idt 2, interrupt_controller 1,
  cpu_random 1, context 1, asm 1

These fall into:

- Arch-coupled by definition, permanent: the trap handlers under
  `interrupts/handlers/exceptions/*` calling `diag::dump_trap` /
  `diag::emit_fatal_notice` (x86 trap frames), GDT/IDT setup, the x86 APIC
  and ACPI readers used by SMP bring-up and the IRQ broker, VGA log output,
  and the x86 IOMMU bring-up in `init_dma_protection`.
- Reducible arch-leaks, the shrink track (M-ARCH-0): call sites that could
  reach the platform through the `Arch` trait or a thin `crate::diag`
  facade instead of naming `crate::arch::x86_64` directly. The diag facade
  alone (routing the ~17 `diag::*` uses through one shim) is the next
  planned drop. As each lands, lower this number and record it here.
