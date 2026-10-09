# arch

`src/arch/` is the architecture layer: one CPU- and platform-independent interface, and behind `cfg(target_arch)` gates the concrete per-architecture backends — CPU setup, interrupts, paging, the syscall fast path, DMA/IOMMU, firmware (ACPI) and boot bring-up. x86_64 is the production backend; aarch64 and riscv64 are preview backends that build only with `nonos-arch-preview`. At 1,584 files it is the largest module, and almost all of that is x86_64.

The rest of the kernel does not name an architecture. It uses the arch-neutral surface (`crate::arch::…`), and `src/arch/mod.rs` resolves that to the one backend compiled for the target.

## One neutral surface, one compiled backend

```mermaid
flowchart TB
    neutral["crate::arch neutral surface<br/>paging, interrupt_controller, cpu, port_io, trap, console"]
    neutral --> sel{"cfg(target_arch)"}
    sel -->|x86_64 (release)| x86["arch::x86_64 -> Arch"]
    sel -->|aarch64 (preview)| arm["arch::aarch64"]
    sel -->|riscv64 (preview)| rv["arch::riscv64"]
    subgraph x86sub["x86_64 backend"]
        gdtidt["gdt / idt"]
        apic["interrupt/apic, ioapic, pic"]
        sc["syscall (syscall/sysret + MSRs)"]
        pg["paging (CR3, tagged TLB)"]
        io["iommu (VT-d / AMD-Vi)"]
        acpi["acpi (MADT, FADT, DMAR, AML)"]
        pci["pci (config, scan, DMA)"]
        cpu["cpu (CPUID, MSR, TSC, topology)"]
    end
    x86 --> x86sub
```

`src/arch/mod.rs:49-55` gates the three backends by target, and `:62-70` aliases the selected one to the crate-wide `Arch` type and re-exports it. The arch-neutral modules (`abi` with the `ArchOps` trait, `cpu`, `paging`, `interrupt_controller`, `trap`, `port_io`, `console`, `wall_clock`, `firmware_table`) are the shape; the per-arch directories are the implementation.

## The x86_64 backend

```
src/arch/x86_64/
  gdt/            GDT, TSS, per-CPU segments, FS/GS base, guard stacks
  idt/            the IDT table, entries, exception/IRQ gates, handlers
  interrupt/      apic/ (local APIC: xAPIC/x2APIC, timer, IPIs, AP bring-up), ioapic/, pic/
  interrupt_controller/  the neutral-shaped wrapper over the APIC
  syscall/        the syscall/sysret fast path: manager/ (entry, init, MSR program), msr.rs
  paging/         CR3 read/write, boot identity map, tagged-TLB/PCID invalidation
  iommu/          DMA remapping: backend/ (VT-d vs AMD-Vi), unit/, mapping/, remap/, tables/
  acpi/           firmware tables: parser/ (madt, dmar), tables/ (madt, fadt, hpet), aml/, devices/
  pci/            config space, bus scan, BAR/MSI-X, the DMA engine
  cpu/            CPUID, features, MSRs, topology, frequency, cache, TSC, xstate
  boot/ + nonos_boot.rs   the boot stage machine; multiboot/ and uefi/ entry environments
  pat/, smm/, asm/, serial/, vga/, console/, time/, watchdog/, port/
```

Note there is no x86_64 `smp/` directory: AP startup lives in `interrupt/apic/ipi_ap.rs` + `init_ap.rs` and `syscall/manager/init.rs::init_ap`, driven from the top-level [smp](process.md#smp) module.

## Key items (x86_64)

| Item | Where | What it does |
|---|---|---|
| `syscall_handler` | `src/arch/x86_64/syscall/manager/entry.rs:24` | The `#[no_mangle]` syscall C entry; redirects unknown numbers to `process::foreign`. |
| `init` / `init_ap` (syscall MSRs) | `src/arch/x86_64/syscall/manager/init.rs:31` / `:44` | Program LSTAR/STAR/SFMASK and enable `EFER.SCE`, on the BSP and APs. |
| `IA32_LSTAR` et al. | `src/arch/x86_64/syscall/msr.rs:19` | The syscall MSR constants; `read_msr`/`write_msr` at `:37`/`:53`. |
| `init` / `init_with_acpi` | `src/arch/x86_64/api.rs:19` / `:31` | Top-level arch init. |
| `init` (GDT) | `src/arch/x86_64/gdt/ops_init.rs:23` | Load the GDT and TSS. |
| `init` / `load_idt` | `src/arch/x86_64/idt/ops/init.rs:30` / `:51` | Build the IDT and issue `lidt`. |
| `init_apic` | `src/arch/x86_64/interrupt/apic/init.rs:64` | Bring up the local APIC. |
| `init_ap_lapic` | `src/arch/x86_64/interrupt/apic/init_ap.rs:29` | AP-side LAPIC bring-up. |
| `map` / `unmap` (IOMMU) | `src/arch/x86_64/iommu/backend/dispatch.rs:40` / `:54` | The vendor-neutral IOMMU op surface. |
| `select_vendor` / `detect` | `src/arch/x86_64/iommu/backend/select.rs:28` / `:43` | Choose VT-d vs AMD-Vi. |
| `init` (IOMMU unit) | `src/arch/x86_64/iommu/unit/bringup/init.rs:27` | Bring up the hardware IOMMU unit. |
| `map_range` / `unmap_range` | `src/arch/x86_64/iommu/mapping/map_range.rs:28` / `unmap_range.rs:33` | Map/unmap an IOMMU range. |
| `parse_madt` | `src/arch/x86_64/acpi/parser/madt/parse.rs:27` | Parse the MADT (CPUs, APICs, IOAPICs). |
| `local_id` / `end_of_interrupt` / `send_ipi` | `src/arch/x86_64/interrupt_controller/ops.rs:22` / `:28` / `:39` | The neutral interrupt-controller surface. |

## Wiring

- **memory/paging:** the IOMMU backend uses `crate::memory::iommu::IommuVendor` (`iommu/backend/select.rs:23`); `paging/` operates CR3 and is driven by [memory](memory.md); `arch::active_page_table_root` and `arch::remap_pci_windows` bridge the two.
- **syscall:** `syscall_handler` calls `crate::syscall`, and redirects unknown numbers to `crate::process::foreign::redirect` — arch is the trampoline into the generic [syscall](syscall.md) and [process](process.md) layers.
- **interrupts:** `interrupt/apic` and `idt` are consumed through the neutral `interrupt_controller` surface by [interrupts](interrupts.md) and the scheduler's IPI path.
- **hardware/iommu:** `pci/` and `acpi/` feed the `iommu/` backend; ACPI MADT feeds the APIC/IOAPIC and the [smp](process.md#smp) topology.
- **boot:** `nonos_boot` (re-exported as `arch::boot`), `multiboot/` and `uefi/` are the entry environments that call `api::init`.

## See also

- [x86_64](../architectures/x86_64.md), [aarch64](../architectures/aarch64.md), [riscv64](../architectures/riscv64.md): the architectures from the behavior side.
- [memory](memory.md): the paging and IOMMU types this backend drives.
- [interrupts](interrupts.md) and [syscall](syscall.md): the neutral layers arch trampolines into.
- [PCI and ACPI](../kernel/pci-and-acpi.md) and [IOMMU](../kernel/iommu.md): the firmware and DMA behavior.
