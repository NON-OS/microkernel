# Coverage: Wi-Fi

Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,
not this page. [How to read it](coverage.md#how-to-read-it).

| Priority | Part | Years | How common | Ids | Named by the driver | In Standard | Owner | Gap |
|---|---|---|---|---|---|---|---|---|
| P0 | Intel AX200 | 2019-2022 | most machines | `8086:2723` | 1 of 1 | yes | E | host-proven on the 22000-family transport (cc-a0-77); never run on the card |
| P0 | Intel AX201 / 9560 (CNVi) | 2018-2022 | most machines | `8086:a0f0` `8086:02f0` `8086:06f0` `8086:43f0` `8086:34f0` `8086:9df0` `8086:a370` | 5 of 7 | yes | E | AX201 (Qu, QuZ with an HR radio) host-proven on the 22000-family transport; 9560 (JF radio, 9df0, a370) refused by name; never run on a card |
| P1 | Intel AX210/AX211/AX411/BE200 | 2021-2026 | most machines | `8086:2725` `8086:51f0` `8086:51f1` `8086:54f0` `8086:7af0` `8086:7a70` `8086:7f70` `8086:272b` | 8 of 8 | yes | E | SO-family CNVi ids claimed; AX210 (2725, ty-a0-gf-a0-86) and BE200 (272b) host-proven, never run on the card |
| P1 | Realtek RTL8822CE | 2019-2026 | most machines | `10ec:c822` `10ec:c82f` | 2 of 2 | listed, unsealed | E | host-proven only; never run on the card |
| P1 | Realtek RTL8852BE | 2021-2026 | most machines | `10ec:b852` `10ec:b85b` | 2 of 2 | listed, unsealed | E | host-proven only (WPA2 and WPA3 join and data in the model); 20 MHz legacy rates, TX power open loop; never run on the card |
| P1 | Realtek RTL8852AE/CE | 2019-2026 | most machines | `10ec:8852` `10ec:c852` | no driver | no feature | E | no driver |
| P1 | MediaTek MT7921/MT7922 | 2021-2026 | most machines | `14c3:7961` `14c3:0616` | 2 of 2 | listed, unsealed | E | host-proven only (WPA2 join and data in the model); firmware from the package beside the image; never run on a card |
| P2 | Intel 8265/8260/7265 | 2015-2019 | common | `8086:24fd` `8086:24f3` `8086:095a` | 0 of 3 | yes | E | recognised, never claimed |
| P2 | Qualcomm QCA6174, WCN685x | 2016-2026 | common | `168c:003e` `17cb:1103` | no driver | no feature | E | no driver |
| - | Realtek RTL8821CE | 2017-2022 | common | `10ec:c821` `10ec:b821` | 2 of 2 | yes | E | works on the owner's HP laptop on 0.9.3 (28 Oct report, docs/release-evidence/0.9.3/drivers/rtl8821ce/) |
