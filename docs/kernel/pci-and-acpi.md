# PCI and ACPI

How the NONOS kernel learns what hardware a machine has on x86_64: which ACPI tables it reads and what it takes from each, how it scans PCI and reaches configuration space, and what it leaves alone.

## Order at boot

```mermaid
flowchart TD
    A[init_acpi_tables] --> B["bus::pci::init"]
    B --> C[assign_unassigned]
    C --> D[seed_hardware_broker]
    D --> E[init_dma_protection]
    E --> F[init_broker_irq_routing]
```

`init_core_systems` reads the ACPI tables with `init_acpi_tables`, scans PCI with `bus::pci::init`, then runs the platform baseline (`src/boot/main/core_init/init_core_systems.rs:35-68`). The baseline maps the PCI windows again, assigns any BAR firmware left empty with `assign_unassigned`, and fills the [hardware broker](hardware-broker.md)'s table with `seed_hardware_broker` (`src/kernel_core/init/platform/baseline.rs:39-57`). Later `microkernel_init` brings the IOMMU up with `init_dma_protection` once paging exists (`src/kernel_core/init/entry/microkernel_init.rs:49-52`), and `init_device_routing` programs the interrupt routes with `init_broker_irq_routing` before any [capsule](../overview/glossary.md#capsule) starts (`src/kernel_core/init/entry/init_runtime.rs:23-41`).

## Finding the tables

`init_acpi_tables` takes the RSDP address from the bootloader's handoff when there is one and prints `[NONOS] ACPI tables parsed` or `[NONOS] ACPI init failed; legacy fallbacks engaged` (`src/boot/main/core_init/acpi_tables.rs:19-33`). Without a handoff address, or when nothing valid sits at it, `find_rsdp` searches the EBDA and then the BIOS ROM range (`src/arch/x86_64/acpi/parser/rsdp.rs:29-55`).

`init` reads the XSDT with `parse_xsdt` when the RSDP has one and falls back to `parse_rsdt` only when there is no XSDT or it cannot be read (`src/arch/x86_64/acpi/parser/init.rs:41-55`). A missing or unreadable FADT fails the whole parse in `parse_fadt`; the other tables are optional (`src/arch/x86_64/acpi/parser/init.rs:61-70`).
