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
2. It builds one root table that every unit shares, and an [identity
   domain](../overview/glossary.md#identity-domain) with `identity_domain`
   (`src/arch/x86_64/iommu/unit/bringup/run.rs:46-50`).
3. The identity domain maps physical memory one to one up to the top of the
   managed range rounded up to 1 GiB, and never less than 4 GiB, as
   `identity_limit` computes
   (`src/arch/x86_64/iommu/unit/bringup/limit.rs:17-35`).
4. `assign_enumerated` gives every function the PCI scan found a context entry
   in that domain (`src/arch/x86_64/iommu/unit/bringup/assign.rs:22-45`).
5. Each unit is brought into service with `bring_into_service`, and only after
   all of them does `set_enforcing` record that DMA is remapped
   (`src/arch/x86_64/iommu/unit/bringup/run.rs:56-66`).

The identity domain does not confine a device that was found; what it buys is
that a device absent from the scan, such as a card added later, has no context
entry and is denied. The success lines say exactly that, from `enabled`
(`src/arch/x86_64/iommu/unit/bringup/verdict.rs:38-49`):

```
[VT-D] translation enabled, levels=<hex> devices=<hex>
[VT-D] enumerated devices identity mapped; others denied
[VT-D] units programmed=<hex>
```

Only units on PCI segment 0 are programmed. DMAR units on other segments are
counted by `foreign_segment_units` and named in a warning, and the devices
behind them are unrestricted
(`src/arch/x86_64/acpi/parser/other/dmar.rs:44-48`). The kernel records at most
`MAX_REMAP_UNITS`, 8, segment 0 units, and reads only the DRHD structures of the
table (`src/arch/x86_64/acpi/parser/other/dmar.rs:26-27`).

Interrupt remapping is a separate feature, `nonos-iommu-intremap`. No feature
list in [Cargo.toml](../../Cargo.toml) turns it on, and without it bring-up
prints that interrupt remapping is not built in, in `init`
(`src/arch/x86_64/iommu/unit/bringup/init.rs:38-43`).
