# IOMMU: What the kernel does

Part of [IOMMU](iommu.md).

## What the kernel does

- With an Intel VT-d unit that the firmware described and left off, the kernel
  turns translation on at boot. Every device found by the PCI scan starts in an
  identity mapped domain, and anything the scan did not find is denied.
- When a [driver capsule](../overview/glossary-dma-pool.md#driver-capsule) claims a
  device behind a unit in service, the device moves into that capsule's own
  [IOMMU domain](../overview/glossary-hardware-broker.md#iommu-domain), which maps only the DMA
  buffers the capsule was granted.
- AMD-Vi units are taken back from firmware but not driven by any image profile
  this tree defines, so DMA behind an AMD-Vi unit is unrestricted.
- With no unit, or a unit that did not come up, DMA is unrestricted, and the
  kernel says so on the [serial
  console](../overview/glossary-rollback-index.md#serial-console).

Everything here is read from the code. No hardware report for this release
covers whether a given machine's units come into service; the build provides the
QEMU boot described below.

## Which IOMMU

The vendor is chosen from the ACPI tables, never from CPUID: a DMAR table with
at least one remapping unit means VT-d, an IVRS table without one means AMD-Vi,
and neither means none, the three values of `IommuVendor`
(`src/memory/iommu/vendor.rs:19-31`). `detect` makes that choice once; when both
tables exist VT-d wins, because it is the one this kernel drives, and the AMD
units are named as unconfined (`src/arch/x86_64/iommu/backend/select.rs:37-66`).

The x86_64 backend is compiled with `nonos-arch-iommu`; any other build, and any
other architecture, uses the stand in `backend_unsupported`, which selects
nothing and refuses every domain call (`src/memory/iommu/backend.rs:17-27`). An
ARM board has no SMMU driver yet, so a mapping request there is refused rather
than ignored, as the note above `iommu` says (`src/memory/mod.rs:43-46`).
