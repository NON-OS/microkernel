# Coverage: Platform and IOMMU

Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,
not this page. [How to read it](coverage.md#how-to-read-it).

|---|---|---|---|---|---|---|---|---|
| P0 | Firmware with VT-d or AMD-Vi switched off | 2017-2026 | most machines | - | - | no feature | hw | every bus-master claim refused, boot stops at the splash; hw-1 posture pending |
| P1 | AMD-Vi on AMD desktops and laptops | 2017-2026 | most machines | `acpi:IVRS` | 1 of 1 | kernel | A | QEMU's amd-iommu reads enforcing=0 in the boot matrix; real AMD-Vi never booted |
| P3 | AMD SMBus (FCH) | 2017-2026 | common | `1022:790b` | no driver | no | B | smbus is Intel only, and in no image |
