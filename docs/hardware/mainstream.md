# Mainstream hardware

What a laptop, desktop or mini PC sold from 2018 to 2026 most likely carries, part by part, and how much of it NONOS drives today. It answers one question for a person with a USB stick: on the machine I already own, do I get a picture, a keyboard and touchpad, a disk, a network and sound. The [support matrix](MATRIX.md) says what has been shown to work; the [coverage map](coverage.md) lists every gap with its owner; the controller ids of each Intel and AMD platform are in [Platform controller ids](platform-ids.md).

Updated 27 October 2026 (S4, drivers). Sources were fetched that day; each figure links to one.

## How the shares are counted

The shares are linux-hardware.org's December 2025 Trends, the share of probed machines that month, split into notebooks ([Notebook][trnb]) and desktops ([Desktop][trdt]). Only machines whose owners ran `hw-probe` under Linux are counted, so the figures lean to Linux-friendly and slightly older hardware, and they rank parts rather than measure the market. A share is the part's share of its class: of the network controllers, of the sound controllers.

The NONOS column says what the code in this tree does, in four words:

- **Booted**: run on that part, on a real machine or the QEMU model of it, with a log in the tree.
- **Host-proven**: the driver is in the Standard image and its proof crate drives its real source against a model of the part; never run on the part.
- **Written**: a driver exists but no release image carries it.
- **None**: no driver.

Only one real machine has run NONOS 0.9.x so far, an Intel Gemini Lake laptop ([MATRIX.md](MATRIX.md)). Read "host-proven" as "should work, unknown until someone boots it".

## Wired Ethernet

| Part | Id | Share (Dec 2025) | NONOS |
|---|---|---|---|
| Realtek RTL8111/8168 | 10ec:8168 | 15.92% of notebooks, 24.61% of desktops | Host-proven (`capsule_driver_rtl8169`); QEMU has no model |
| Realtek RTL8125 2.5 GbE | 10ec:8125 | 10.71% of desktops | Host-proven, same driver |
| Realtek RTL8126 5 GbE | 10ec:8126 | 1.72% of desktops | None: its start differs from the RTL8125's |
| Intel I219 (e1000e) | 15bb/15bc, 15fb/15fc, 1a1e/1a1f, 0dc5-0dc8, 550a/550b, 57a0/57a1 | 2.02% of desktops, 1.27% of notebooks; the business-laptop NIC | Host-proven; booted on QEMU's 82574 with a DHCP lease ([evidence](../release-evidence/0.9.3/drivers/e1000e/)) |
| Intel I225-V, I226-V/LM (igc) | 8086:15f3, 125c, 125b | 2.41% and 1.87% of desktops; the ASUS NUC 15 Pro's | Host-proven, in Standard since 27 October 2026; no QEMU model |
| Realtek RTL8153 USB dongle | USB 0bda:8153 | 2.64% of notebooks | None: the driver the matrix named never existed in the tree |

Sources: [trnb], [trdt]. NONOS covers, host-proven, the parts behind about 26% of notebooks' and 45% of desktops' network controllers above; booted, none yet.

## Wi-Fi

| Part | Id | Share | NONOS |
|---|---|---|---|
| Intel AX211, AX201 on an SO platform (Alder and Raptor Lake CNVi) | 8086:51f0, 51f1, 54f0, 7a70, 7af0, 7f70 | 1.78% and 1.25% of notebooks | Host-proven (`capsule_driver_iwlwifi`), firmware bundled |
| Intel AX210 discrete, AX211 on Meteor Lake | 8086:2725, 7e40 | 2.32% of desktops | Refused by name: driver path present, firmware not bundled |
| Intel AX200, AX201 on Comet, Ice and Tiger Lake | 8086:2723, 02f0, 06f0, a0f0 | 1.78% and 1.81% of notebooks | None: the 22000-family boot is not written |
| Intel BE200 discrete | 8086:272b | no Trends row yet | In progress (Bz/Gl port, S4), refused until it lands |
| Intel BE201/BE202 CNVi (Arrow and Lunar Lake) | 8086:7740, a840 | no Trends row yet; the ASUS NUC 15 Pro's | In progress, refused until it lands |
| MediaTek MT7922 / RZ616, MT7921 / RZ608 | 14c3:0616, 7922, 7961, 0608 | 1.71% and 1.45% of notebooks | None. Firmware licence forbids building it into a GPL-covered work: needs a separate firmware package |
| Realtek RTL8821CE | 10ec:c821 | 1.75% of notebooks | Booted on one real laptop (scan, join, DHCP, browser) |
| Realtek RTL8822CE, RTL8852BE/CE | 10ec:c822, b852, c852 | 1.57% and 1.27% of notebooks | None |
| Qualcomm QCA6174, QCA9377, WCN6855, WCN7850 | 168c:003e, 0042, 17cb:1103, 1107 | about 1% each | None |

Sources: [trnb], [trdt], Linux's iwlwifi device table ([iwldrv]). NONOS drives the Wi-Fi of roughly 5% of notebooks today, counted generously.

## Storage

| Part | NONOS |
|---|---|
| NVMe, any vendor (class 01/08/02): half of notebook drives by connector | Booted on the one real laptop (installed to and booted from NVMe, [MATRIX.md](MATRIX.md)) |
| AHCI SATA (class 01/06) | Spawned on QEMU's q35 AHCI, the boot disk (`[DRIVER-AHCI] capsule spawned`, [rehearsal serial log](../build/evidence/2026-10-26/rehearsal-standard-8bf24f59b/serial.log)) |
| Intel VMD/RST in front of NVMe: about 4.3% of Linux notebooks, roughly half of the Tiger and Alder Lake laptops that reached Linux | The kernel brings up the VMD domain itself, never run on a VMD machine; the installer names every VMD and tells the person to switch to AHCI/NVMe ([Intel VMD](../drivers/storage/vmd.md)) |
| eMMC (Intel SDHCI), SD readers | eMMC host-proven; QEMU lanes `emmc`, `sd` boot pending. Realtek rtsx readers: written, not in an image |

Sources: [trnb], [trdt], [DevicePopulation][pop].

## USB

Every machine since 2018 has xHCI, matched by class (0c/03/30), so every PCH, SoC, AMD FCH and Thunderbolt/USB4 xHCI is claimed. The xHCI, USB HID, USB storage and hub drivers start on QEMU's xHCI (the [rehearsal serial log](../build/evidence/2026-10-26/rehearsal-standard-8bf24f59b/serial.log)). Hubs and docks: `capsule_driver_usb_hub` is in Standard, host-proven, with no QEMU lane run yet. USB4 and Thunderbolt tunnels (a dock's own PCIe devices) are not driven.

## Input

| Part | NONOS |
|---|---|
| PS/2 keyboard behind the embedded controller (most laptops) | Booted on QEMU and on the real laptop |
| I2C-HID touchpad on Intel LPSS I2C | Booted on the real laptop (Gemini Lake); the LPSS ids run to Lunar Lake since 27 October 2026, host-proven |
| I2C-HID touchpad on AMD DesignWare I2C (ACPI AMDI0010) | Host-proven, never run on an AMD laptop |
| Touchpad behind Intel's Touch Host Controller (Lunar Lake, a848/a84a) | None |
| USB keyboard and mouse | Booted on QEMU |

## Audio

| Path | Where | NONOS |
|---|---|---|
| HD Audio controller and codec (class 04/03): desktops, older laptops, headphone jacks | AMD 1022:15e3 is the single most common sound controller of notebooks (11.74%) | Booted on QEMU and on the real laptop |
| Intel SOF DSP (class 04/01, 04/03/80): the internal microphones and often speakers of most Intel laptops since 2019 | Tiger Lake Smart Sound 3.05%, Raptor Lake cAVS 2.41% of notebooks | None. HDA serves the codec where the firmware exposes it; the DSP path is not written |
| AMD ACP (1022:15e2): the internal microphones of Ryzen laptops | needs no firmware on consumer laptops | None |
| USB headsets and USB codecs on desktop boards | | `capsule_driver_usb_audio` in Standard, QEMU lane `usb-audio` boot pending |

## Display

NONOS draws through the firmware's GOP framebuffer on every Intel, AMD and NVIDIA GPU. The native Intel driver (Haswell to Raptor Lake) is held out of images until it runs on real hardware; native AMD and NVIDIA drivers are out of scope for this release. See [GPUs](gpu.md).

## What the reference machines get

The four machines the owner tests on, their parts from the vendors' spec sheets, and what NONOS gives each today. One-page bring-up plans are in [Bring-up plans](bring-up/README.md).

| Machine | Picture | Keyboard, touchpad | Disk | Network | Sound |
|---|---|---|---|---|---|
| Intel laptop: ThinkPad T14 Gen 5 Intel (Meteor Lake) | GOP | PS/2; I2C-HID on LPSS 7e78 (host-proven) | NVMe | I219-LM (host-proven); AX211 on 7e40 refused (no firmware), BE200 in progress | HDA codec; the array microphone needs SOF (none) |
| AMD laptop: ThinkPad T14 Gen 4 AMD (Phoenix) | GOP | PS/2; I2C-HID on AMDI0010 (host-proven) | NVMe | RTL8111 (host-proven); MT7922 (none) | HDA codec; microphone needs ACP (none) |
| Desktop board: ASRock B650M Pro RS WiFi, ROG STRIX B760-F | GOP | USB | NVMe, SATA | RTL8125 or Intel 2.5 GbE (host-proven) | HDA ALC897; the B760-F's ALC4080 may be USB audio (unverified) |
| Mini PC: ASUS NUC 15 Pro (Arrow Lake-H) | GOP | USB | NVMe | I226-V (host-proven, in Standard); BE201 on 7740 in progress | HDA 8086:7728, codec unlisted |

Sources for the machines: Lenovo PSREF ([T14 Gen 5 Intel][t14g5i], [T14 Gen 4 AMD][t14g4a]), [ASRock B650M Pro RS WiFi manual][asrock], [ROG STRIX B760-F][rogb760], [ASUS NUC 15 Pro][nuc15].

## Firmware licences

| Firmware | Licence | In a NONOS image |
|---|---|---|
| Intel iwlwifi (linux-firmware `LICENCE.iwlwifi_firmware`) | Binary redistribution without modification allowed; the patent grant covers use with an OSI-approved open-source OS, which AGPL NONOS is | Yes: bundled in the driver, pinned in `scripts/baselines/upstream.sha256` |
| Realtek rtw88/rtw89 (`LICENCE.rtlwifi_firmware.txt`) | Same terms as Intel's | Yes for RTL8821CE |
| MediaTek MT7921/MT7922 (`LICENCE.mediatek`) | Redistribution allowed, "but not as part of the Linux kernel or in any other form which would require these files themselves to be covered by the terms of the GNU GPL or LGPL" | Not as the Intel firmware is shipped. A driver would need the firmware as a separate package outside the AGPL image; the owner decides |
| Qualcomm ath10k/ath11k/ath12k (`LICENSE.QualcommAtheros_ath10k`) | Binary redistribution allowed for use with Qualcomm chipsets only | Allowed with the notice; no driver yet |
| Intel SOF (`sof-bin`, `LICENCE.Intel`) | Redistributable; Intel DSPs run Intel-signed images | No driver yet |
| AMD ACP | None needed on consumer laptops | No driver yet |

Quoted clauses and links: [WHENCE], [Intel licence][liciwl], [MediaTek licence][licmtk], [Qualcomm licence][licqca], [SOF licence][soflic].

[trnb]: https://github.com/linuxhw/Trends/blob/master/Notebook/README.md
[trdt]: https://github.com/linuxhw/Trends/blob/master/Desktop/README.md
[pop]: https://github.com/linuxhw/DevicePopulation
[iwldrv]: https://github.com/torvalds/linux/blob/master/drivers/net/wireless/intel/iwlwifi/pcie/drv.c
[WHENCE]: https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git/tree/WHENCE
[liciwl]: https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git/tree/LICENSES/LICENCE.iwlwifi_firmware
[licmtk]: https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git/tree/LICENSES/LICENCE.mediatek
[licqca]: https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git/tree/LICENSES/LICENSE.QualcommAtheros_ath10k
[soflic]: https://github.com/thesofproject/sof-bin/blob/main/LICENCE.Intel
[t14g5i]: https://psref.lenovo.com/syspool/Sys/PDF/ThinkPad/ThinkPad_T14_Gen_5_Intel/ThinkPad_T14_Gen_5_Intel_Spec.pdf
[t14g4a]: https://psref.lenovo.com/syspool/Sys/PDF/ThinkPad/ThinkPad_T14_Gen_4_AMD/ThinkPad_T14_Gen_4_AMD_Spec.pdf
[asrock]: https://download.asrock.com/Manual/B650M%20Pro%20RS%20WiFi.pdf
[rogb760]: https://rog.asus.com/motherboards/rog-strix/rog-strix-b760-f-gaming-wifi-model/spec/
[nuc15]: https://www.asus.com/displays-desktops/nucs/nuc-mini-pcs/asus-nuc-15-pro/techspec/
