# IOMMU: When VT-d comes into service

Part of [IOMMU](iommu.md).

## When VT-d comes into service

`init_dma_protection` runs right after paging is up, because reaching a unit
means mapping its register window
(`src/kernel_core/init/entry/microkernel_init.rs:49-52`). `init_dma_protection`
selects the vendor, brings VT-d up where DMAR described a unit, and prints the
posture line last (`src/kernel_core/init/entry/init_dma_protection.rs:43-61`).

Bring-up runs only in a kernel built with `nonos-iommu-enforce`; without it
`init` prints that enforcement is not built in and leaves the hardware alone
(`src/arch/x86_64/iommu/unit/bringup/init.rs:27-34`). `microkernel-core` in
[Cargo.toml](../../Cargo.toml) includes that feature, and the kernel feature
lists of every image profile in `tools/nix/config.nix` build on
`microkernel-core`.

`bring_up` then does the work in an order chosen so the disk and network keep
working (`src/arch/x86_64/iommu/unit/bringup/run.rs:26-67`):

1. It refuses to start if firmware left any unit translating, with
   `FirmwareOwnsUnit`, rather than take over tables nobody reasons about
   (`src/arch/x86_64/iommu/unit/bringup/run.rs:38-43`).
2. It builds one root table that every unit shares, and a blocked domain that
   maps nothing (`blocked_domain` in
   `src/arch/x86_64/iommu/unit/bringup/domain.rs`).
3. `assign_enumerated` gives every function the PCI scan found a context entry
   in the blocked domain, or, for one firmware listed RMRRs for, in a domain
   of its own holding those ranges alone (`reserved_domain`, `start_for` in
   `src/arch/x86_64/iommu/unit/bringup/start.rs`). No device is [identity
   mapped](../overview/glossary-hardware-broker.md#identity-domain).
4. Each unit is brought into service with `bring_into_service`, and only after
   all of them does `set_enforcing` record that DMA is remapped
   (`src/arch/x86_64/iommu/unit/bringup/run.rs`).

A device absent from the scan, such as a card added later, has no context entry
and is denied. The success lines say so, from `enabled`
(`src/arch/x86_64/iommu/unit/bringup/verdict.rs`):

```
[VT-D] translation enabled, levels=<hex> devices=<hex>
[VT-D] enumerated devices blocked until claimed; with reserved memory=<n>; others denied
[VT-D] units programmed=<hex>
[VT-D] WARNING units left off=<hex>; not programmed, devices behind them are unrestricted
```

Only units on PCI segment 0 are programmed, at most `MAX_REMAP_UNITS`, 8.
A unit on another segment, a ninth, or one past a DMAR structure that does not
parse is left off: `units_left_off` counts it, the boot log names the count in
a warning, the devices behind it are unrestricted, and the IOMMU then claims
no enforcement at all (`src/arch/x86_64/acpi/parser/other/dmar_keep.rs`,
`fn keep`; `src/arch/x86_64/iommu/backend/capabilities.rs`, `fn vtd`).

DMAR's flags and host address width are read. Without `INTR_REMAP` interrupt
remapping stays off; the reported width is cut to the host address width; and
ATSR and SATC structures are counted in an `[VT-D] ATS left off` line, since no
context entry takes a device's own translations
(`src/arch/x86_64/acpi/parser/other/dmar_flags.rs`, `fn dmar_intr_remap`;
`src/arch/x86_64/acpi/parser/other/dmar_ats.rs`, `fn ats_structures`).

Interrupt remapping is the `nonos-iommu-intremap` feature, in
`microkernel-core`. Every unit gets one shared table with CFI clear, so a
compatibility format interrupt is blocked: each MSI, MSI-X and IO-APIC line
names an entry for its own requester id, and a claim is refused where
remapping is not on (`src/arch/x86_64/iommu/remap/`,
`src/hardware/broker/confine/interrupts.rs`).
