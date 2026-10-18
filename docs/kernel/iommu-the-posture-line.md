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
mean every device is confined: a device firmware reserved memory for keeps
that memory while no capsule has claimed it. On a
machine with no remapping hardware the boot prints `[IOMMU] no DMAR remapping
unit and no IVRS table; IOMMU domains refused; DMA is unrestricted` and selects
`IommuVendor::Absent` (`src/arch/x86_64/iommu/backend/select.rs:62-65`).

## AMD-Vi

`init` for AMD-Vi first takes every unit back from firmware, because a unit left
enabled would translate every device through tables built for firmware's own
drivers (`src/arch/x86_64/iommu/amd_vi/init.rs`). `microkernel-core` builds
`nonos-iommu-amdvi`, so the kernel then drives the units through `bringup`:
one shared device table in which every requester id starts blocked, IV set
with no table so its interrupts are refused, a device with IVMD ranges keeping
those, per capsule domains, and a remapping table per device that binds an
interrupt (`src/arch/x86_64/iommu/amd_vi/intr.rs`). The boot prints
`[AMD-VI] IOMMU translation enabled, units=<n> reserved=<n>`.

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

The [proof crate](../overview/glossary-peer-list.md#proof-crate) `userland/kernel_proofs`
compiles the DMAR scope parser (`userland/kernel_proofs/src/dmar_scope/mod.rs`),
the IVRS walk (`userland/kernel_proofs/src/firmware_iommu/mod.rs`), the
confinement posture (`userland/kernel_proofs/src/confine_posture/mod.rs`) and
several VT-d and AMD-Vi register, table and remapping encoders from the kernel
sources. It passed, 493 tests, on the host at this commit. `userland/mechanism_proofs` holds
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
