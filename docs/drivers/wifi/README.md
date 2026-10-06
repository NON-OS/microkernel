# Wi-Fi drivers

NONOS has two ring 3 Wi-Fi driver [capsules](../../overview/glossary.md#capsule), one for the Realtek RTL8821CE and one for Intel cards; this page lists the chips, follows a scan and a join from the Settings panel to the radio, and states what is refused.

## Chips at a glance

Each Wi-Fi driver is a capsule: a signed ring 3 program that reaches its card only through grants from the [hardware broker](../../overview/glossary.md#hardware-broker).

| Chip | PCI vendor:device | Capsule | State in 0.9.2 | Page |
|---|---|---|---|---|
| Realtek RTL8821CE | 10ec:c821 | `driver.rtl8821ce0` | Works: see the hardware report below the table | [rtl8821ce.md](rtl8821ce.md) |
| Intel AX211, and AX201 modules on the same platforms | 8086:51f0, 51f1, 54f0, 7a70, 7af0, 7f70 | `driver.iwlwifi0` | Partial: boots, scans and joins against a modelled device only | [iwlwifi.md](iwlwifi.md) |
| Intel AX210 discrete, Intel Meteor Lake Wi-Fi | 8086:2725, 2729, 7e40 | `driver.iwlwifi0` | Refused: firmware file not in the tree | [iwlwifi.md](iwlwifi.md) |
| Intel 7260 to 9560, AX200, AX201 on Qu and QuZ platforms, two AX210 family ids without transport values | listed on the iwlwifi page | `driver.iwlwifi0` | Refused: no firmware boot path in this driver | [iwlwifi.md](iwlwifi.md) |
| MediaTek, Qualcomm, Broadcom, other Realtek Wi-Fi | none | none | Not supported | [not-supported.md](not-supported.md) |

Wi-Fi on Realtek RTL8821CE (scan, join, DHCP, DNS, browser traffic). Works on an x86_64 laptop (Intel Gemini Lake, 8 GB), maintainer hardware report, 6 October 2026; the image commit was not recorded.

That is the only hardware report for Wi-Fi in this release. Every other statement on these pages comes from the code and from host tests in [proof crates](../../overview/glossary.md#proof-crate).
