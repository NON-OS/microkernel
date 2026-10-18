# Hardware support matrix: Storage; USB; Input

Part of [Hardware support matrix](MATRIX.md).

## Storage

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Storage | NVMe, PCI class 01/08 prog-if 02 | `capsule_driver_nvme` | Works | proof crate `nvme_proofs` (81); QEMU `nvme` install target; real hardware (install to an internal NVMe disk and boot from it) | 0.9.2 |
| Storage | AHCI SATA, class 01/06; Intel RST in RAID mode, 8086 class 01/04 | `capsule_driver_ahci` | Works | proof crate `ahci_link_proofs` (99); QEMU q35 | 0.9.2 |
| Storage | Intel eMMC on SDHCI, class 08/05: 8086:0f14, 0f50, 2294, 0acc, 1aa8, 5acc, 31cc, 9d2b, 9dc4, 34c4, 18db, 4b47, 4dc4 | `capsule_driver_ahci` | Works, except 8086:9d2b, which is Partial: the kernel's start list `INTEL_EMMC_DEVICE_IDS` leaves it out, so the capsule starts for it only on a machine that also has a SATA controller ([SD cards and eMMC](../drivers/storage/sd-and-emmc.md)) | proof crate `emmc_proofs` (83) | 0.9.2 |
| Storage | Intel SD card and SDIO hosts, 14 ids such as 8086:31ca, 5aca | none | Not supported: no SD card driver in the image; these are never taken for the internal disk | none | 0.9.2 |
| Storage | Intel VMD, 13 ids such as 8086:9a0b, 467f, a77f | kernel, then `capsule_driver_nvme` for the drives behind it | Partial: the kernel brings each VMD domain up and lists the drives behind it, but 8086:28c1 is left out, and the bring-up has never run on a machine with a VMD ([Intel VMD](../drivers/storage/vmd.md)) | proof crate `kernel_proofs` (388), against a simulated bus | 0.9.2 |
| Storage | Realtek PCIe card reader 10ec:5227, 10ec:522a | `capsule_driver_rtsx` | Not supported: not in the image | proof crate `rtsx_proofs` (26) | 0.9.2 |
| Storage | USB mass storage, class 08 subclass 06, bulk-only | `capsule_driver_usb_msc` | Works | proof crate `usb_msc_proofs`: the check fails at this commit on a clippy lint; QEMU `usb-storage` on `qemu-xhci`, booted from by CI's boot smoke and by two `nonos-mk-boot-matrix` cells | 0.9.2 |
| Storage | virtio-blk 1af4:1001, 1af4:1042 | `capsule_driver_virtio_blk` | Works | proof crate `virtio_blk_proofs` (13); QEMU | 0.9.2 |

## USB

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| USB | xHCI host controller, class 0c/03 prog-if 30 | `capsule_driver_xhci` | Works | proof crate `xhci_proofs`: the check fails at this commit on clippy lints; QEMU `qemu-xhci` | 0.9.2 |
| USB | EHCI, OHCI and UHCI host controllers, class 0c/03 | none | Not supported: no driver; the [hardware broker](../overview/glossary-hardware-broker.md#hardware-broker) lists them as a generic USB host | none | 0.9.2 |
| USB | Hubs, class 09 | `capsule_driver_usb_hid` | Partial: the HID driver brings a hub up and powers its ports, but a device behind a hub is not reached, because `capsule_driver_xhci` refuses the hub ops 0x20 and 0x21 ([USB hubs](../drivers/usb/hubs.md)) | proof crate `usb_proofs` (83) | 0.9.2 |

## Input

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Input | i8042 PS/2 keyboard, platform record 0001:0303 | `capsule_driver_ps2_input` | Works | proof crates `ps2_input_proofs` (38), `input_proofs` (96); QEMU i8042; real hardware (keyboard with its layouts, volume keys) | 0.9.2 |
| Input | PS/2 mouse or touchpad on the aux port, platform record 0001:0304 | `capsule_driver_ps2_input` | Partial: mice work with wheel; a touchpad is a plain relative mouse, with no vendor protocol | proof crates `ps2_input_proofs`, `input_proofs`; QEMU i8042 | 0.9.2 |
| Input | I2C-HID touchpad on Intel LPSS, PCI ids in [I2C-HID touchpads](../drivers/input/i2c-hid.md) | `capsule_driver_i2c_pci`, `capsule_driver_i2c_hid` | Works | proof crates `i2c_hid_proofs` (51), `i2c_pci_proofs` (35), `i2c_transfer_proofs` (26); real hardware | 0.9.2 |
| Input | I2C-HID touchpad on an ACPI-declared controller: AMDI0010, AMDI0510, AMD0010, INT33C2, INT33C3, INT3432, INT3433, INT3442 to INT3447, 80860F41, 808622C1 | `capsule_driver_i2c_pci`, `capsule_driver_i2c_hid` | Works | proof crates as above; not run on such a machine | 0.9.2 |
| Input | I2C-HID touchscreens | none | Refused: the kernel leaves them out of the device table | none | 0.9.2 |
| Input | I2C-HID touchpad with a 10-bit address | none | Refused: 10-bit I2C addressing is not supported; the boot log warns | none | 0.9.2 |
| Input | USB HID keyboard, mouse and tablet, class 03 | `capsule_driver_usb_hid` | Works | proof crate `usb_proofs` (83); QEMU `usb-tablet` under `nonos-mk-plan-a-runtime` | 0.9.2 |
