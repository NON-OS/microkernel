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
