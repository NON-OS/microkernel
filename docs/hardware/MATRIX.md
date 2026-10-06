# Hardware support matrix

Every class of device NONOS 0.9.2 knows about, the chips and ids its drivers match, what state each is in, and what that state rests on.

## How to read it

State:

- Works: the 0.9.2 image carries the whole path for this device.
- Partial: part of the path is there; the row says what is missing.
- Refused (reason): the kernel or the driver finds the device and declines it, for the reason given. Where the refusal reaches the boot console or Settings, the row says so.
- Not supported: no driver for it in the 0.9.2 image. Some of these have driver source and passing [proof crates](../overview/glossary.md#proof-crate) in the tree but are not built into any image: no kernel feature, no place in the boot's spawn plan, and `driver.xhci0` admits only the USB HID and storage drivers (`HELD`, `src/services/registry/held_table.rs:20-32`).

How verified:

- Proof crate: host tests that compile the shipping driver source against a model of the device. The number is the test count of its flake check at commit bff12b97, and the check passes unless the row says otherwise.
- QEMU: the build's QEMU run targets attach this device (`mk/10-qemu.mk`, `mk/40-run.mk`). No boot log from this commit is recorded here.
- Real hardware: the one report this release has. Works on an x86_64 laptop (Intel Gemini Lake, 8 GB), maintainer hardware report, 6 October 2026; the image commit was not recorded. It covers Wi-Fi on Realtek RTL8821CE (scan, join, DHCP, DNS, browser traffic), the local Qwen model offline, the Linux programs sh, python3, sqlite3 and john, the installer writing to an internal NVMe disk and booting from it, the I2C-HID touchpad on Intel LPSS, the PS/2 keyboard with its layouts, Intel HD Audio, the power button and the volume keys. Nothing else in this matrix has been run on real hardware for this release.

Every row is for x86_64 machines; [Architectures](../architectures/README.md) says what the other ports have. The rows describe the Standard image. The Air-Gapped image drops every network driver (`networkFeatures`, `tools/nix/config.nix:92`), and the QEMU image leaves out the drivers only real hardware has (`microkernel-desktop-gui`, `tools/nix/config.nix:95-98`).

PCI ids are vendor:device in hex. USB ids say USB, and ACPI ids are firmware `_HID` strings. Every driver named here is a [capsule](../overview/glossary.md#capsule) unless the row says kernel.

## Platform

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| CPU | x86_64 cores, SMP | kernel | Works | QEMU with 4 cores | 0.9.2 |
| IOMMU | Intel VT-d DMA remapping | kernel | Works | proof crates `kernel_proofs` (388), `mechanism_proofs` (56); QEMU `intel-iommu` | 0.9.2 |
| IOMMU | Intel VT-d interrupt remapping | kernel | Not supported: the `nonos-iommu-intremap` feature is off until a boot log shows the units acknowledging it | none | 0.9.2 |
| IOMMU | AMD-Vi | kernel | Not supported: the `nonos-iommu-amdvi` feature is off until QEMU `amd-iommu` and an AMD machine show the units in service | none | 0.9.2 |
| TPM | TPM 2.0, FIFO and CRB interfaces | kernel | Works | proof crates `tpm_key_proofs` (42), `tpm_enroll_proofs` (37); QEMU `tpm-crb` with swtpm | 0.9.2 |
| ACPI | Fixed-feature power button | kernel | Partial: a press reaches the desktop as the Power key, which shows `Power off is not available from the desktop`; the desktop cannot shut down in 0.9.2 | real hardware | 0.9.2 |
| ACPI | Power button through AML (PNP0C0C) or on a hardware-reduced platform | kernel | Refused: NONOS has no AML interpreter to run it, and the desktop has no power-off either | none | 0.9.2 |
| GPIO | Pad level of the touchpad line: Intel Broxton family, Intel chipsets Sunrise Point to Meteor Lake, AMD (AMD0030, AMDI0030, AMDI0031, AMDI0033) | `capsule_driver_i2c_pci` with `nonos_pinctrl` | Partial: read-only, for the touchpad's interrupt line; no general GPIO driver | proof crates `pinctrl_proofs` (13), `i2c_pci_proofs` (35) | 0.9.2 |
| Entropy | virtio-rng 1af4:1005, 1af4:1044 | `capsule_driver_virtio_rng` | Works | proof crate `virtio_rng_proofs` (12); QEMU | 0.9.2 |

## Display

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Display | UEFI GOP framebuffer | kernel and compositor | Works | QEMU | 0.9.2 |
| Display | virtio-gpu 1af4:1010, 1af4:1050 | `capsule_driver_virtio_gpu` | Works | proof crate `virtio_gpu_proofs` (38); QEMU `virtio-vga` | 0.9.2 |
| Display | Bochs display adapter 1234:1111 | `capsule_driver_bga` | Not supported: parked, since it re-modes the adapter and would destroy the firmware framebuffer the desktop uses | proof crate `bga_proofs` (9) | 0.9.2 |
| Display | Native GPUs: Intel 8086, AMD 1002, NVIDIA 10de, PCI class 03 | none | Not supported: no modeset driver; the desktop draws on the firmware framebuffer | none | 0.9.2 |

## Storage

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Storage | NVMe, PCI class 01/08 prog-if 02 | `capsule_driver_nvme` | Works | proof crate `nvme_proofs` (81); QEMU `nvme` install target; real hardware (install to an internal NVMe disk and boot from it) | 0.9.2 |
| Storage | AHCI SATA, class 01/06; Intel RST in RAID mode, 8086 class 01/04 | `capsule_driver_ahci` | Works | proof crate `ahci_link_proofs` (99); QEMU q35 | 0.9.2 |
| Storage | Intel eMMC on SDHCI, class 08/05: 8086:0f14, 0f50, 2294, 0acc, 1aa8, 5acc, 31cc, 9d2b, 9dc4, 34c4, 18db, 4b47, 4dc4 | `capsule_driver_ahci` | Works | proof crate `emmc_proofs` (83) | 0.9.2 |
| Storage | Intel SD card and SDIO hosts, 14 ids such as 8086:31ca, 5aca | none | Not supported: no SD card driver in the image; these are never taken for the internal disk | none | 0.9.2 |
| Storage | Intel VMD, 13 ids such as 8086:9a0b, 467f, a77f | none | Not supported: listed by the kernel, never driven, so the disks behind it stay hidden | none | 0.9.2 |
| Storage | Realtek PCIe card reader 10ec:5227, 10ec:522a | `capsule_driver_rtsx` | Not supported: not in the image | proof crate `rtsx_proofs` (26) | 0.9.2 |
| Storage | USB mass storage, class 08 subclass 06, bulk-only | `capsule_driver_usb_msc` | Works | proof crate `usb_msc_proofs`: the check fails at this commit on a clippy lint | 0.9.2 |
| Storage | virtio-blk 1af4:1001, 1af4:1042 | `capsule_driver_virtio_blk` | Works | proof crate `virtio_blk_proofs` (13); QEMU | 0.9.2 |

## USB

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| USB | xHCI host controller, class 0c/03 prog-if 30 | `capsule_driver_xhci` | Works | proof crate `xhci_proofs`: the check fails at this commit on clippy lints; QEMU `qemu-xhci` | 0.9.2 |
| USB | EHCI, OHCI and UHCI host controllers, class 0c/03 | none | Not supported: no driver; the broker lists them as a generic USB host | none | 0.9.2 |
| USB | Hubs, class 09 | `capsule_driver_usb_hid` | Partial: the HID driver brings hubs up to reach keyboards and mice below them | proof crate `usb_proofs` (83) | 0.9.2 |

## Input

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Input | i8042 PS/2 keyboard, platform record 0001:0303 | `capsule_driver_ps2_input` | Works | proof crates `ps2_input_proofs` (38), `input_proofs` (96); QEMU i8042; real hardware (keyboard with its layouts, volume keys) | 0.9.2 |
| Input | PS/2 mouse or touchpad on the aux port, platform record 0001:0304 | `capsule_driver_ps2_input` | Partial: mice work with wheel; a touchpad is a plain relative mouse, with no vendor protocol | proof crates `ps2_input_proofs`, `input_proofs`; QEMU i8042 | 0.9.2 |
| Input | I2C-HID touchpad on Intel LPSS, PCI ids in [I2C-HID touchpads](../drivers/input/i2c-hid.md) | `capsule_driver_i2c_pci`, `capsule_driver_i2c_hid` | Works | proof crates `i2c_hid_proofs` (51), `i2c_pci_proofs` (35), `i2c_transfer_proofs` (26); real hardware | 0.9.2 |
| Input | I2C-HID touchpad on an ACPI-declared controller: AMDI0010, AMDI0510, AMD0010, INT33C2, INT33C3, INT3432, INT3433, INT3442 to INT3447, 80860F41, 808622C1 | `capsule_driver_i2c_pci`, `capsule_driver_i2c_hid` | Works | proof crates as above; not run on such a machine | 0.9.2 |
| Input | I2C-HID touchscreens | none | Refused: the kernel leaves them out of the device table | none | 0.9.2 |
| Input | I2C-HID touchpad with a 10-bit address | none | Refused: 10-bit I2C addressing is not supported; the boot log warns | none | 0.9.2 |
| Input | USB HID keyboard, mouse and tablet, class 03 | `capsule_driver_usb_hid` | Works | proof crate `usb_proofs` (83) | 0.9.2 |

## Audio

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Audio | HD Audio controller, class 04/03 any vendor; Intel class 04/01 | `capsule_driver_hda` | Works | proof crate `hda_proofs` (116); QEMU `intel-hda`; real hardware (Intel HD Audio) | 0.9.2 |
| Audio | Realtek ALC codecs, 10ec vendor, with Linux's EAPD coefficients and ALC256 setup | `capsule_driver_hda` | Works | proof crate `hda_proofs` with ALC236 and ALC269 models | 0.9.2 |
| Audio | Other HD Audio codecs with an analog output | `capsule_driver_hda` | Works | proof crate `hda_proofs` with QEMU's duplex codec model; QEMU `hda-duplex` | 0.9.2 |
| Audio | HDMI and DisplayPort audio; graphics controllers 1002, 10de, 8086:490d, 4f90, 4f91, 4f92, e2f7 | `capsule_driver_hda` | Refused: NONOS plays through speakers, headphones and line out only; Settings and the player say so | proof crate `hda_proofs` | 0.9.2 |
| Audio | Intel SST engines 8086:9c36, 9cb6, 0f28, 22a8, 119a, and Intel DSP-only laptops | `capsule_driver_hda` | Refused: needs Intel SOF, which NONOS does not have; Settings and the player say so | proof crate `hda_proofs` | 0.9.2 |
| Audio | AMD audio coprocessor (ACP), 1022 class 04/80 | `capsule_driver_hda` | Refused: needs an ACP driver, which NONOS does not have; Settings and the player say so | proof crate `hda_proofs` | 0.9.2 |
| Audio | Microphones and recording | none | Not supported: no input stream is opened | none | 0.9.2 |

## Wi-Fi

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Wi-Fi | Realtek RTL8821CE 10ec:c821 | `capsule_driver_rtl8821ce` | Works | proof crate `rtl8821ce_proofs` (171); real hardware (scan, join, DHCP, DNS, browser traffic) | 0.9.2 |
| Wi-Fi | Intel AX210 family on SO platforms 8086:51f0, 51f1, 54f0, 7a70, 7af0, 7f70 | `capsule_driver_iwlwifi` | Partial: firmware start, scan and WPA2 and WPA3 join run against a modelled device; not run on the air in this release | proof crate `iwlwifi_proofs` (205) | 0.9.2 |
| Wi-Fi | Intel 8086:2725, 7e40, 2729 | `capsule_driver_iwlwifi` | Refused: their firmware image is not in the tree | proof crate `iwlwifi_proofs` | 0.9.2 |
| Wi-Fi | Intel 8086:08b1 to 08b4, 095a, 095b, 3165, 3166, 24fb, 24f3 to 24f6, 24fd, 2526, 9df0, a370, 31dc, 30dc, 271b, 271c, 2723, 34f0, 3df0, 4df0, 02f0, 06f0, 43f0, a0f0, 272f, a74f, 272b, a840 | `capsule_driver_iwlwifi` | Refused: no boot path in this driver. Its line naming the card is not printed, since the driver holds no Debug capability | proof crate `iwlwifi_proofs` | 0.9.2 |
| Wi-Fi | Every other Wi-Fi chip | none | Not supported: no driver | none | 0.9.2 |

## Ethernet

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Ethernet | Intel e1000, 28 ids, 8086:100e among them | `capsule_driver_e1000` | Works | proof crate `e1000_proofs` (20) | 0.9.2 |
| Ethernet | Intel e1000e: 82574, 82583, I217, I218, I219, 64 ids | `capsule_driver_e1000e` | Not supported: not in the image | proof crate `e1000e_proofs` (48) | 0.9.2 |
| Ethernet | Intel I225 and I226 (igc), 16 ids | `capsule_driver_igc` | Not supported: not in the image | proof crate `igc_proofs` (58) | 0.9.2 |
| Ethernet | Realtek RTL8139 10ec:8139 | `capsule_driver_rtl8139` | Works | proof crate `rtl8139_proofs` (15) | 0.9.2 |
| Ethernet | Realtek RTL8169, RTL8168 and RTL8111, RTL810x, RTL8125: 10ec:8169, 8167, 8161, 8162, 8168, 2502, 2600, 8136, 8125, 3000 | `capsule_driver_rtl8169` | Works | proof crate `rtl8169_proofs`: the check fails at this commit on a clippy lint after its tests ran | 0.9.2 |
| Ethernet | Realtek RTL8126 10ec:8126, RTL8127 10ec:8127 | none | Not supported: left out of the RTL8169 driver, since their start differs from the RTL8125's | none | 0.9.2 |
| Ethernet | virtio-net 1af4:1000, 1af4:1041 | `capsule_driver_virtio_net` | Works | proof crate `virtio_net_proofs` (20); QEMU `virtio-net-pci` | 0.9.2 |
| Ethernet | USB CDC-ECM (class 02/06), CDC-NCM (02/0d), RNDIS (02/02/ff, e0/01/03, ef/04/01) | `capsule_driver_cdc_ecm`, `capsule_driver_cdc_ncm`, `capsule_driver_rndis` | Not supported: not in the image | proof crates `cdc_ecm_proofs` (7), `cdc_ncm_proofs` (33), `rndis_proofs` (22) | 0.9.2 |
| Ethernet | ASIX AX88179 and AX88178A, USB 0b95:1790, 0b95:178a and 11 more | `capsule_driver_ax88179` | Not supported: not in the image | proof crate `ax88179_proofs` (25) | 0.9.2 |
| Ethernet | Realtek RTL8153, USB 0bda:8153 and 18 more | `capsule_driver_rtl8153` | Not supported: not in the image | proof crate `rtl8153_proofs` (27) | 0.9.2 |
