# IOMMU: The posture line

Part of [IOMMU](iommu.md).

## The posture line

`report_posture` prints the state once DMA protection has been set up, right
after paging, and prints the first line again whenever the count changes
(`src/memory/iommu/posture.rs:38-79`):

```
[IOMMU] <none|intel-vt-d|amd-vi> present, enforcing=<0|1>, unconfined grants=<n>
[IOMMU] capabilities aw=<bits> ir=<0|1> snoop=<0|1> pages=<mask> domains=<n>
```

`enforcing=1` together with `unconfined grants=0` means that every DMA buffer
the broker has granted is confined, because `unconfined_grants` counts every
grant made without a domain (`src/memory/iommu/posture.rs:17-28`). It does not
mean every device is confined: a device found at boot that no capsule has
claimed stays in the identity domain and can still reach all memory. On a
machine with no remapping hardware the boot prints `[IOMMU] no DMAR remapping
unit and no IVRS table; IOMMU domains refused; DMA is unrestricted` and selects
`IommuVendor::Absent` (`src/arch/x86_64/iommu/backend/select.rs:62-65`).

## AMD-Vi

`init` for AMD-Vi first takes every unit back from firmware, because a unit left
enabled would translate every device through tables built for firmware's own
drivers (`src/arch/x86_64/amd_vi/init.rs:24-45`). A kernel built with
`nonos-iommu-amdvi` would then drive the units through `bringup`, with one
shared device table and per capsule domains
(`src/arch/x86_64/amd_vi/mod.rs:17-25`). No feature list in
[Cargo.toml](../../Cargo.toml) turns that feature on, so on such a machine the
boot prints `[AMD-VI] IOMMU driver not built in; DMA is unrestricted`.

## Trying it in QEMU

The build has a boot with an emulated Intel IOMMU.
`nonos-mk-run-iommu-serial-log` builds the desktop image and runs QEMU under
TCG, without hardware acceleration, so it is slow; it writes the serial log to
`QEMU_IOMMU_SERIAL_LOG`, the path it prints (`mk/40-run.mk:489-501`):

```
make nonos-mk-run-iommu-serial-log
```

No run of this target is recorded for this release, so no log from it is quoted
here.

## Tests

The [proof crate](../overview/glossary.md#proof-crate) `userland/kernel_proofs`
compiles the DMAR scope parser (`userland/kernel_proofs/src/dmar_scope/mod.rs`),
the IVRS walk (`userland/kernel_proofs/src/firmware_iommu/mod.rs`), the
confinement posture (`userland/kernel_proofs/src/confine_posture/mod.rs`) and
several VT-d register and queue helpers from the kernel sources. It passed, 388
tests, in the flake check run on this commit. `userland/mechanism_proofs` holds
the VT-d page table entry and context entry encodings
(`userland/mechanism_proofs/src/iommu/mod.rs`); it passed, 56 tests, in the same
run.

## See also

- [Hardware broker](hardware-broker.md): claims and DMA grants.
- [PCI and ACPI](pci-and-acpi.md): the DMAR and IVRS tables and the PCI scan.
- [Platform drivers](../drivers/platform.md): VT-d and AMD-Vi from the driver
  section.
- [Protections and limits](../security/protections-and-limits.md): what DMA
  protection does and does not cover.
- [Threat model](../overview/threat-model.md): DMA attacks in context.
