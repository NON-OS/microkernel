# Coverage: Platform and IOMMU

Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,
not this page. [How to read it](coverage.md#how-to-read-it).

| Priority | Part | Years | How common | Ids | Named by the driver | In Standard | Owner | Gap |
|---|---|---|---|---|---|---|---|---|
| P0 | Firmware with VT-d or AMD-Vi switched off | 2017-2026 | most machines | - | - | yes | hw | a Standard boot takes the no-IOMMU posture: claims go ahead unconfined, said on serial, the splash and About (src/memory/iommu/no_iommu.rs); Hardened and Air-Gapped stop the boot; booted under QEMU (docs/release-evidence/0.9.3/gpu-469dddb58-no-iommu), no real-hardware run recorded |
| P1 | PCH root ports, 100/200-series and 7th/8th-gen U, ACS in the dword layout | 2016-2019 | most machines | - | - | kernel | A | the dword layout is read since K11b; never booted on a real 100/200-series laptop |
| P1 | AMD-Vi on AMD desktops and laptops | 2017-2026 | most machines | `acpi:IVRS` | 1 of 1 | kernel | A | QEMU's amd-iommu reads amd-vi present, enforcing=1 (docs/hardware/evidence/2026-10-25/i7-seeded-iommu, 2026-10-26/main-3fa2f94f86); no AMD machine recorded |
| P3 | AMD SMBus (FCH) | 2017-2026 | common | `1022:790b` | no driver | yes | B | smbus is Intel only (in Standard since 27 October 2026); no AMD FCH driver |
