# IOMMU: Tests

Part of [IOMMU](iommu.md).

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
