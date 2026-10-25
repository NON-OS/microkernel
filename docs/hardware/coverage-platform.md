# Coverage: Platform and IOMMU

Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,
not this page. [How to read it](coverage.md#how-to-read-it).

| Priority | Part | Years | How common | Ids | Named by the driver | In Standard | Owner | Gap |
|---|---|---|---|---|---|---|---|---|
| P0 | Firmware with VT-d or AMD-Vi switched off | 2017-2026 | most machines | - | - | no feature | hw | every bus-master claim refused, boot stops at the splash; hw-1 posture pending |
| P1 | PCH root ports, 100/200-series and 7th/8th-gen U, ACS in the dword layout | 2016-2019 | most machines | - | - | kernel | A | the dword layout is read since K11b; never booted on a real 100/200-series laptop |
| P1 | AMD-Vi on AMD desktops and laptops | 2017-2026 | most machines | `acpi:IVRS` | 1 of 1 | kernel | A | QEMU's amd-iommu reads enforcing=0 in the boot matrix; real AMD-Vi never booted |
| P3 | AMD SMBus (FCH) | 2017-2026 | common | `1022:790b` | no driver | no | B | smbus is Intel only, and in no image |
