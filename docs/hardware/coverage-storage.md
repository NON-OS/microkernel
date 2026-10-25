# Coverage: Storage

Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,
not this page. [How to read it](coverage.md#how-to-read-it).

| Priority | Part | Years | How common | Ids | Named by the driver | In Standard | Owner | Gap |
|---|---|---|---|---|---|---|---|---|
| P0 | Intel VMD, the default on 11th-gen and later laptops | 2020-2026 | most machines | `8086:9a0b` `8086:467f` `8086:a77f` `8086:7d0b` `8086:ad0b` | 5 of 5 | kernel | B, A | kernel enumerates the VMD domain; inventory still says enumerate-only; never booted on a real VMD machine |
| P1 | AHCI SATA, any vendor (Intel PCH, AMD FCH and Promontory, ASMedia) | 2017-2026 | most machines | `class:01/06` | by class | yes | B | class match; booted in QEMU only on main |
| P1 | Intel RST RAID mode, AHCI personality | 2017-2026 | common | `class:01/04` | by class | yes | B | taken as AHCI; RST-remapped NVMe drives are detected and named, not driven |
| P1 | eMMC on Intel SDHCI (cheap laptops) | 2017-2022 | common | `8086:31cc` `8086:5acc` `8086:4dc4` `8086:9dc4` | 4 of 4 | listed, unsealed | B | listed with no trust files (the owner seals it); never booted on main |
| P2 | UFS (Intel Lakefield/ADL-N, Samsung) | 2020-2026 | servers or niche | `8086:54ff` `8086:98fa` | 2 of 2 | listed, unsealed | B | listed in full-gui with no trust files: the owner seals it |
| P2 | Hyper-V synthetic SCSI (storvsc), the disk of a Generation 2 Hyper-V or Azure guest | 2016-2026 | servers or niche | `1414:5343` | 1 of 1 | no | D | host-proved, not booted: QEMU offers no storage VSP and TCG no SynIC, so only a Hyper-V Gen 2 guest can show it; in no release profile. A VMBus channel carries no requester id, so its DMA grants are physical addresses, open only under the no-IOMMU posture or the development opt-in |
| - | NVMe, any vendor | 2017-2026 | most machines | `class:01/08` | by class | yes | B | class match; installer on a real NVMe disk works (6 Oct report) |
