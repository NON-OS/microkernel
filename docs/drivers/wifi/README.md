# Wi-Fi drivers

NONOS has two Wi-Fi [driver capsules](../../overview/glossary-dma-pool.md#driver-capsule), one for the Realtek RTL8821CE and one for Intel cards; this page lists the chips, follows a scan and a join from the Settings panel to the radio, and states what is refused.

## Chips at a glance

Each Wi-Fi driver is a [capsule](../../overview/glossary-boot-root-record.md#capsule): a signed ring 3 program that reaches its card only through grants from the [hardware broker](../../overview/glossary-hardware-broker.md#hardware-broker).

| Chip | PCI vendor:device | Capsule | State in 0.9.2 | Page |
|---|---|---|---|---|
| Realtek RTL8821CE | 10ec:c821 | `driver.rtl8821ce0` | Works: see the hardware report below the table | [rtl8821ce.md](rtl8821ce.md) |
| Intel AX211, and AX201 modules on the same platforms | 8086:51f0, 51f1, 54f0, 7a70, 7af0, 7f70 | `driver.iwlwifi0` | Partial: boots, scans and joins against a modelled device only | [iwlwifi.md](iwlwifi.md) |
| Intel AX210 discrete, Intel Meteor Lake Wi-Fi | 8086:2725, 2729, 7e40 | `driver.iwlwifi0` | Refused: firmware file not in the tree | [iwlwifi.md](iwlwifi.md) |
| Intel 7260 to 9560, AX200, AX201 on Qu and QuZ platforms, two AX210 family ids without transport values | listed on the iwlwifi page | `driver.iwlwifi0` | Refused: no firmware boot path in this driver | [iwlwifi.md](iwlwifi.md) |
| Intel BE200 and BE201, MediaTek, Qualcomm, Broadcom, other Realtek Wi-Fi | 8086:272b, a840 for the Intel ones | none | Not supported | [not-supported.md](not-supported.md) |

Wi-Fi on Realtek RTL8821CE (scan, join, DHCP, DNS, browser traffic). Works on an x86_64 laptop (Intel Gemini Lake, 8 GB), maintainer hardware report, 6 October 2026; the image commit was not recorded.

That is the only hardware report for Wi-Fi in this release. Every other statement on these pages comes from the code and from host tests in [proof crates](../../overview/glossary-peer-list.md#proof-crate).

## What no Wi-Fi driver does

- No 6 GHz. The RTL8821CE scans 2.4 GHz channels 1 to 13 only; the iwlwifi path keeps the 2.4 and 5 GHz channels the world-safe domain allows (`userland/capsule_driver_iwlwifi/src/firmware/gen3/nvm/layout.rs`, `NVM_CHANNELS`; `userland/nonos_wifi_core/src/regdom/world.rs`, `world`).
- No country setting. Both drivers keep to one world-safe domain: 2.4 GHz channels 1 to 11 may be probed; 12 and 13, and 5 GHz channels 36 to 64, 100 to 144 and 149 to 165, are listened on and joined only after the network's beacon is heard there; 14 is never used.
- No open, TKIP or Enterprise networks. The [security a join accepts](overview/security.md#security-a-join-accepts) is WPA2-Personal or WPA3-Personal with CCMP.
- No access point, mesh or monitor mode, no roaming and no power save.
- No USB Wi-Fi adapters.

## Parts

- [How a scan and a join travel, and the control protocol](overview/travel.md)
- [The security a join accepts, and what a join answers](overview/security.md)
- [Bring-up stages, saved networks, autojoin](overview/stages.md)
- [Images, firmware and its licence, tests](overview/images.md)

## See also

- [Realtek RTL8821CE](rtl8821ce.md)
- [Intel iwlwifi](iwlwifi.md)
- [Wi-Fi chips with no driver](not-supported.md)
- [Ethernet drivers](../ethernet/README.md)
- [The driver model](../README.md)
- [Wi-Fi and networking for users](../../using/wifi-and-networking.md)
- [Hardware support matrix](../../hardware/MATRIX.md)
- [Manifests and capabilities](../../userland/manifests-and-capabilities.md)
