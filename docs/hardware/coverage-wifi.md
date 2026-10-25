# Coverage: Wi-Fi

Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,
not this page. [How to read it](coverage.md#how-to-read-it).

|---|---|---|---|---|---|---|---|---|
| P0 | Intel AX200 | 2019-2022 | most machines | `8086:2723` | 0 of 1 | yes | E | recognised, never claimed: no firmware path for this family |
| P0 | Intel AX201 / 9560 (CNVi) | 2018-2022 | most machines | `8086:a0f0` `8086:02f0` `8086:06f0` `8086:43f0` `8086:34f0` `8086:9df0` `8086:a370` | 0 of 7 | yes | E | recognised, never claimed |
| P1 | Intel AX210/AX211/AX411/BE200 | 2021-2026 | most machines | `8086:2725` `8086:51f0` `8086:51f1` `8086:54f0` `8086:7af0` `8086:7a70` `8086:7f70` `8086:272b` | 7 of 8 | yes | E | SO-family CNVi ids claimed; AX210 (2725) has no firmware committed; BE200 (272b) unknown |
| P1 | Realtek RTL8822CE, RTL8852AE/BE | 2019-2026 | most machines | `10ec:c822` `10ec:8852` `10ec:b852` | no driver | no feature | E | no driver |
| P1 | MediaTek MT7921/MT7922 | 2021-2026 | most machines | `14c3:7961` `14c3:0616` | no driver | no feature | E | no driver; most AMD laptops since 2021 |
| P2 | Intel 8265/8260/7265 | 2015-2019 | common | `8086:24fd` `8086:24f3` `8086:095a` | 0 of 3 | yes | E | recognised, never claimed |
| P2 | Qualcomm QCA6174, WCN685x | 2016-2026 | common | `168c:003e` `17cb:1103` | no driver | no feature | E | no driver |
| - | Realtek RTL8821CE | 2017-2022 | common | `10ec:c821` | 1 of 1 | yes | E | works on the bench laptop (6 Oct report) |
