# Coverage: Wired Ethernet

Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,
not this page. [How to read it](coverage.md#how-to-read-it).

|---|---|---|---|---|---|---|---|---|
| P0 | Intel I219-LM/V (e1000e), onboard on most Intel desktops and laptops | 2017-2026 | most machines | `8086:15b7` `8086:15b8` `8086:15bb` `8086:15bc` `8086:15bd` `8086:15be` `8086:0d4e` `8086:0d4f` `8086:15fb` `8086:15fc` `8086:1a1e` `8086:1a1f` `8086:0dc7` `8086:550a` `8086:57a0` | 15 of 15 | listed, unsealed | B | listed in full-gui with no trust files: the release seals it; boot a real I219 |
| P1 | Realtek RTL8111/8168 GbE | 2017-2026 | most machines | `10ec:8168` `10ec:8161` `10ec:8136` | 3 of 3 | yes | B | in the image; never booted on a real card on main |
| P1 | Realtek RTL8125 2.5 GbE, Killer E3000 | 2019-2026 | most machines | `10ec:8125` `10ec:3000` | 2 of 2 | yes | B | in the image; never booted on a real card on main |
| P1 | Intel I210/I211 (igb) | 2017-2021 | common | `8086:1533` `8086:1539` `8086:157b` | 3 of 3 | listed, unsealed | B | listed in full-gui with no trust files: the release seals it |
| P2 | Realtek RTL8126 5 GbE | 2024-2026 | common | `10ec:8126` | no driver | no feature | B, hw | named by rtl8169 only to exclude it: register map not written |
| P2 | Intel I350 server GbE (igb) | 2017-2026 | common | `8086:1521` `8086:1523` | 2 of 2 | listed, unsealed | B | listed, the release seals it; copper only |
| P2 | Intel X710/XL710 (i40e) | 2017-2026 | common | `8086:1572` `8086:1583` `8086:1584` `8086:1589` `8086:15ff` | no driver | no feature | hw | no driver |
| P2 | Broadcom BCM5720/5719 server GbE (tg3) | 2017-2026 | common | `14e4:165f` `14e4:1657` | no driver | no feature | B | tg3 work covered the 57765 class only; the 5717 class is excluded |
| P2 | Intel X540/X550 10GBASE-T (ixgbe) | 2017-2024 | servers or niche | `8086:1528` `8086:1563` `8086:15d1` | 3 of 3 | no | B | out of Standard until sealed; X550EM refused |
| P3 | Intel E810 (ice) | 2020-2026 | common | `8086:1591` `8086:1592` `8086:1593` `8086:159b` | no driver | no feature | hw | no driver |
| P3 | Broadcom NetXtreme-E BCM574xx (bnxt) | 2018-2026 | servers or niche | `14e4:16d7` `14e4:16d8` | no driver | no feature | hw | no driver |
| P3 | Killer E2200/E2400, Qualcomm Atheros (alx) | 2016-2019 | servers or niche | `1969:e0a1` `1969:e091` | no driver | no feature | hw | no driver |
| P3 | Aquantia/Marvell AQC107/AQC113 10 GbE | 2018-2026 | servers or niche | `1d6a:07b1` `1d6a:04c0` `1d6a:94c0` | no driver | no feature | hw | no driver |
