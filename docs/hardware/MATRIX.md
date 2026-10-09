# Hardware support matrix

Look up your chips here to see whether NONOS 0.9.2 drives them, how far, and what that answer rests on: host tests, a QEMU run or a real machine.

## How to read it

Find your machine's ids first: [Reporting a machine](report.md#read-the-ids-on-another-operating-system) shows how to read them with `lspci` and `lsusb`. Then find the row for each chip. Each row has one of four states:

- Works: the 0.9.2 image carries the whole path for this device.
- Partial: part of the path is there; the row says what is missing.
- Refused: the kernel or the driver finds the device and declines it, for the reason the row gives. Where the refusal reaches the boot log or Settings, the row says so.
- Not supported: no driver for it in the 0.9.2 image. Some of these have driver source and passing [proof crates](../overview/glossary.md#proof-crate) in the tree, but no image builds them: they have no kernel feature and no place in the boot's spawn plan. The USB Ethernet drivers among them could not reach an adapter either, since the [held endpoint](../overview/glossary.md#held-endpoint) `driver.xhci0` takes requests only from the USB HID and storage drivers (`HELD`, `src/services/registry/held_table.rs:20-32`).

How verified:

- Proof crate: host tests that compile the shipping driver source against a model of the device. The number is the test count of its flake check at commit bff12b97, and the check passes unless the row says otherwise.
- QEMU: a QEMU run in the tree attaches this device: `make boot` (`devices`, `tools/nonos_qemu/machine.py:91-100`), CI's boot smoke, or the `mk/10-qemu.mk` and `mk/40-run.mk` targets. No boot log from this commit is recorded here.
- Real hardware: the one report this release has. Works on an x86_64 laptop (Intel Gemini Lake, 8 GB), maintainer hardware report, 6 October 2026; the image commit was not recorded. It covers Wi-Fi on Realtek RTL8821CE (scan, join, DHCP, DNS, browser traffic), the local Qwen model offline, the Linux programs sh, python3, sqlite3 and john, the installer writing to an internal NVMe disk and booting from it, the I2C-HID touchpad on Intel LPSS, the PS/2 keyboard with its layouts, Intel HD Audio, the power button and the volume keys. Nothing else in this matrix has been run on real hardware for this release.

Every row is for x86_64 machines; [Architectures](../architectures/README.md) says what the other ports have. The rows describe the image the Standard [build profile](../overview/glossary.md#build-profile) makes. The Air-Gapped image drops every network driver (`networkFeatures`, `tools/nix/config.nix:92`), and the QEMU image leaves out the drivers only real hardware has (`microkernel-desktop-gui`, `tools/nix/config.nix:95-98`).

PCI ids are vendor:device in hex. USB ids say USB, and ACPI ids are firmware `_HID` strings. Every driver named here is a [capsule](../overview/glossary.md#capsule) unless the row says kernel.

## Platform

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| CPU | x86_64 cores, [SMP](../overview/glossary.md#smp): every core the firmware enables, up to 256 | kernel | Works | QEMU with 8 CPUs, or the host's cores when fewer, under `make boot` and CI's boot smoke; 4 under `nonos-mk-run-smp-serial-log`; 1 to 8 across the boot matrix | 0.9.2 |
| [IOMMU](../overview/glossary.md#iommu) | Intel VT-d DMA remapping | kernel | Works | proof crates `kernel_proofs` (388), `mechanism_proofs` (56); QEMU `intel-iommu` | 0.9.2 |
| IOMMU | Intel VT-d interrupt remapping | kernel | Not supported: the `nonos-iommu-intremap` feature is off until a boot log shows the units acknowledging it | none | 0.9.2 |
| IOMMU | AMD-Vi | kernel | Not supported: the `nonos-iommu-amdvi` feature is off until QEMU `amd-iommu` and an AMD machine show the units in service | none | 0.9.2 |
| [TPM](../overview/glossary.md#tpm) | TPM 2.0, FIFO and CRB interfaces | kernel | Works | proof crates `tpm_key_proofs` (42), `tpm_enroll_proofs` (37); QEMU `tpm-crb` with swtpm | 0.9.2 |
| ACPI | Fixed-feature power button | kernel | Partial: a press reaches the desktop as the Power key, which shows `Power off is not available from the desktop`; the desktop cannot shut down in 0.9.2 | real hardware | 0.9.2 |
| ACPI | Power button through AML (PNP0C0C) or on a hardware-reduced platform | kernel | Refused: NONOS has no AML interpreter to run it, and the desktop has no power-off either | none | 0.9.2 |
| ACPI | Battery charge, PNP0C0A | kernel | Refused: reading the charge runs AML, which NONOS cannot; the top bar shows `Battery status unavailable`, or `No battery` when the firmware declares none | none | 0.9.2 |
| GPIO | Pad level of the touchpad line: Intel Broxton family, Intel chipsets Sunrise Point to Meteor Lake, AMD (AMD0030, AMDI0030, AMDI0031, AMDI0033) | `capsule_driver_i2c_pci` with `nonos_pinctrl` | Partial: read-only, for the touchpad's interrupt line; no general GPIO driver. On other controllers the touchpad driver reads on a timer | proof crates `pinctrl_proofs` (13), `i2c_pci_proofs` (35) | 0.9.2 |
| Entropy | virtio-rng 1af4:1005, 1af4:1044 | `capsule_driver_virtio_rng` | Works | proof crate `virtio_rng_proofs` (12); QEMU | 0.9.2 |

## Display

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Display | UEFI GOP framebuffer | kernel and compositor | Works | QEMU | 0.9.2 |
| Display | virtio-gpu 1af4:1010, 1af4:1050 | `capsule_driver_virtio_gpu` | Works | proof crate `virtio_gpu_proofs` (38); QEMU `virtio-vga` | 0.9.2 |
| Display | Bochs display adapter 1234:1111 | `capsule_driver_bga` | Not supported: parked, since it re-modes the adapter and would destroy the firmware framebuffer the desktop uses | proof crate `bga_proofs` (9) | 0.9.2 |
| Display | Native GPUs: Intel 8086, AMD 1002, NVIDIA 10de, PCI class 03 | none | Not supported: no modeset driver; the desktop draws on the firmware framebuffer ([Display](../drivers/display.md)) | none | 0.9.2 |

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
| USB | EHCI, OHCI and UHCI host controllers, class 0c/03 | none | Not supported: no driver; the [hardware broker](../overview/glossary.md#hardware-broker) lists them as a generic USB host | none | 0.9.2 |
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
| Wi-Fi | Intel 8086:08b1 to 08b4, 095a, 095b, 3165, 3166, 24fb, 24f3 to 24f6, 24fd, 2526, 9df0, a370, 31dc, 30dc, 271b, 271c, 2723, 34f0, 3df0, 4df0, 02f0, 06f0, 43f0, a0f0, 272f, a74f | `capsule_driver_iwlwifi` | Refused: no boot path in this driver. The driver's line naming the card is not printed, since it holds no Debug capability; Settings shows `card not supported yet` | proof crate `iwlwifi_proofs` | 0.9.2 |
| Wi-Fi | Intel BE200 and BE201 8086:272b, a840 | none | Not supported: outside the iwlwifi id table, so the driver leaves them, and Settings says the chip has no NONOS driver | none | 0.9.2 |
| Wi-Fi | Every other Wi-Fi chip | none | Not supported: no driver | none | 0.9.2 |

For a card NONOS cannot run, Settings suggests Ethernet or USB Wi-Fi (`no_driver`, `userland/capsule_settings/src/settings/ui/live_wifi.rs:115-129`; `NoAirPath`, `userland/nonos_wifi_client/src/driver/stage.rs:60`). In 0.9.2 no USB Wi-Fi adapter has a driver, and of the Ethernet drivers in the image only virtio-net, the virtual machine's card, gets received frames through; the table below says why.

## Ethernet

| Class | Chip and id | Driver capsule | State | How verified | Release |
|---|---|---|---|---|---|
| Ethernet | Intel e1000, 28 ids, 8086:100e among them | `capsule_driver_e1000` | Partial: `net.core` drops every frame received through this driver, so DHCP gets no lease ([receive fault](../drivers/ethernet/README.md#the-receive-fault)) | proof crate `e1000_proofs` (20) | 0.9.2 |
| Ethernet | Intel e1000e: 82574, 82583, I217, I218, I219, 64 ids | `capsule_driver_e1000e` | Not supported: not in the image | proof crate `e1000e_proofs` (48) | 0.9.2 |
| Ethernet | Intel I225 and I226 (igc), 16 ids | `capsule_driver_igc` | Not supported: not in the image | proof crate `igc_proofs` (58) | 0.9.2 |
| Ethernet | Realtek RTL8139 10ec:8139 | `capsule_driver_rtl8139` | Partial: the same [receive fault](../drivers/ethernet/README.md#the-receive-fault) as e1000 | proof crate `rtl8139_proofs` (15) | 0.9.2 |
| Ethernet | Realtek RTL8169, RTL8168 and RTL8111, RTL810x, RTL8125: 10ec:8169, 8167, 8161, 8162, 8168, 2502, 2600, 8136, 8125, 3000 | `capsule_driver_rtl8169` | Partial: the same [receive fault](../drivers/ethernet/README.md#the-receive-fault) as e1000 | proof crate `rtl8169_proofs`: its 67 tests pass, then the check fails at this commit on a clippy lint | 0.9.2 |
| Ethernet | Realtek RTL8126 10ec:8126, RTL8127 10ec:8127 | none | Not supported: left out of the RTL8169 driver, since their start differs from the RTL8125's | none | 0.9.2 |
| Ethernet | virtio-net 1af4:1000, 1af4:1041 | `capsule_driver_virtio_net` | Works | proof crate `virtio_net_proofs` (20); QEMU `virtio-net-pci` | 0.9.2 |
| Ethernet | USB CDC-ECM (class 02/06), CDC-NCM (02/0d), RNDIS (02/02/ff, e0/01/03, ef/04/01) | `capsule_driver_cdc_ecm`, `capsule_driver_cdc_ncm`, `capsule_driver_rndis` | Not supported: not in the image | proof crates `cdc_ecm_proofs` (7), `cdc_ncm_proofs` (33), `rndis_proofs` (22) | 0.9.2 |
| Ethernet | ASIX AX88179 and AX88178A, USB 0b95:1790, 0b95:178a and 11 more | `capsule_driver_ax88179` | Not supported: not in the image | proof crate `ax88179_proofs` (25) | 0.9.2 |
| Ethernet | Realtek RTL8153, USB 0bda:8153 and 18 more | `capsule_driver_rtl8153` | Not supported: not in the image | proof crate `rtl8153_proofs` (27) | 0.9.2 |

## Sources

Each row comes from a match table in the driver or the kernel, or from the code that decides its state. All paths are at the commit in the footer.

- CPU: `MAX_CPUS` (`src/smp/constants.rs:17`); the CPUs QEMU gets under `make boot`, `cpus` (`tools/nonos_qemu/machine.py:39-46`).
- PS/2: `register_legacy` publishes the two i8042 records (`src/hardware/broker/platform.rs:40-74`).
- Intel LPSS I2C: `device_info` (`userland/capsule_driver_i2c_pci/src/constants/device_info.rs:29-61`). ACPI I2C controllers: `hid_is_i2c_controller` (`src/arch/x86_64/acpi/aml/controller/hid_match.rs:21-38`). Touchpads and touchscreens: `TOUCHPAD_HIDS` (`src/arch/x86_64/acpi/devices/i2c/hids.rs:19-62`). The broker leaves touchscreens out in `register_acpi_i2c` (`src/hardware/broker/acpi_i2c/register.rs:36-45`) and warns when a touchpad asks for 10-bit addressing (`src/hardware/broker/acpi_i2c/report.rs:62-66`, `ten_bit`).
- GPIO: the Broxton family, `is_bxt_family` (`userland/capsule_driver_i2c_pci/src/constants/device_info.rs:71-74`); Sunrise Point to Meteor Lake, `LAYOUTS` (`userland/nonos_pinctrl/src/tables/index.rs:35-48`); AMD, `AMD_IDS` (`userland/nonos_pinctrl/src/controller.rs:31`).
- USB HID: `CLASS_HID` (`userland/capsule_driver_usb_hid/src/descriptors/types.rs:20-21`); hubs: `CLASS_HUB` (`userland/capsule_driver_usb_hid/src/descriptors/config.rs:22`).
- xHCI and older USB hosts: `USB_HOST_XHCI` in the broker's `classify_pci` (`src/hardware/broker/class.rs:77-81`), matched by `CLASS_USB_HOST_XHCI` (`userland/capsule_driver_xhci/src/constants/pci_class.rs:16`).
- USB mass storage: `CLASS_MASS_STORAGE` (`userland/capsule_driver_usb_msc/src/descriptors/wire.rs:23-24`).
- NVMe: `is_nvme` (`userland/capsule_driver_nvme/src/discover/pci_match.rs:27-33`).
- AHCI and RST: `is_ahci_function` (`userland/capsule_driver_ahci/src/discover/rule.rs:49-58`). VMD: `VMD_DEVICE_IDS`, whose comment says why 28c1 is left out (`src/drivers/pci/vmd/domain/ids.rs:20-29`), which the SATA capsule refuses through `INTEL_VMD_DEVICE_IDS` (`userland/capsule_driver_ahci/src/discover/rule.rs:38-41`).
- eMMC and SD hosts: `INTEL_EMMC` and `INTEL_NOT_EMMC` (`userland/capsule_driver_ahci/src/emmc/pci/ids.rs:25-57`); the kernel's start list, `INTEL_EMMC_DEVICE_IDS` (`src/hardware/inventory/emmc.rs:32-34`).
- Card reader: `LINUX_IDS` (`userland/capsule_driver_rtsx/src/chip/id.rs:25-28`), of which `family` brings up two (`userland/capsule_driver_rtsx/src/chip/id.rs:45-54`).
- virtio: `VIRTIO_BLK_TRANSITIONAL` (`userland/capsule_driver_virtio_blk/src/constants/pci.rs:17-18`), `VIRTIO_NET_TRANSITIONAL` (`userland/capsule_driver_virtio_net/src/constants/pci.rs:23-24`), `VIRTIO_GPU_TRANSITIONAL` (`userland/capsule_driver_virtio_gpu/src/constants/pci.rs:17-18`), `VIRTIO_RNG_TRANSITIONAL` (`userland/capsule_driver_virtio_rng/src/constants/pci.rs:23-24`).
- Display: `classify_display` (`src/hardware/inventory/classify_display.rs:19-28`); the firmware framebuffer, `init_framebuffer` (`src/kernel_core/init/framebuffer/init.rs:25`); native GPUs without a modeset driver, `DisplayNativeIntel` (`src/hardware/inventory/missing.rs:25-27`); the parked Bochs driver, `DisplayBga` (`src/hardware/inventory/driver.rs:33-36`) and `DEVICE_BGA` (`userland/capsule_driver_bga/src/constants.rs:19-20`).
- HD Audio: `hda_controller` (`userland/capsule_driver_hda/src/controller/intel.rs:65-67`), `graphics_audio` (`userland/capsule_driver_hda/src/controller/intel.rs:70-74`), `amd_acp` (`userland/capsule_driver_hda/src/controller/intel.rs:77-79`), `intel_sst` (`userland/capsule_driver_hda/src/controller/sst.rs:33-37`), `eapd_coef` (`userland/capsule_driver_hda/src/controller/codec/realtek.rs:61-104`).
- RTL8821CE: `PCI_DEVICE_RTL8821CE` (`userland/capsule_driver_rtl8821ce/src/constants/mod.rs:26`).
- Intel Wi-Fi: `family_for_device` (`userland/capsule_driver_iwlwifi/src/firmware/family.rs:19-39`); the ids with a boot path, `transport` (`userland/capsule_driver_iwlwifi/src/firmware/gen3/select.rs:121-130`); the cards named without one, `name` (`userland/capsule_driver_iwlwifi/src/firmware/generation.rs:32-65`), in a line `announce` writes with `mk_debug` (`userland/capsule_driver_iwlwifi/src/setup/announce.rs:39-58`).
- e1000: `E1000_DEVICE_IDS` (`userland/capsule_driver_e1000/src/constants/pci.rs:19-23`). e1000e: `I82574` and the lists after it (`userland/capsule_driver_e1000e/src/constants/ids.rs:22-51`). igc: `IGC_DEVICE_IDS` (`userland/capsule_driver_igc/src/constants/pci.rs:24-41`).
- RTL8139: `RTL8139_DEVICE_ID` (`userland/capsule_driver_rtl8139/src/constants/pci.rs:18`). RTL8169 family: `RTL8169_DEVICE_IDS` (`userland/capsule_driver_rtl8169/src/constants/pci.rs:30-31`).
- USB Ethernet: `SUBCLASS_ECM` (`userland/capsule_driver_cdc_ecm/src/ecm/function.rs:26`), `SUBCLASS_NCM` (`userland/capsule_driver_cdc_ncm/src/ncm/function.rs:26`), `CONTROL_CLASSES` (`userland/capsule_driver_rndis/src/rndis/function.rs:30-31`), `PRODUCTS` (`userland/capsule_driver_ax88179/src/ax/products.rs:21-35`), `RTL8153_FAMILY` (`userland/capsule_driver_rtl8153/src/r8153/ids.rs:26-51`).
- Power button: `init` (`src/arch/x86_64/acpi/power_button.rs:62-91`); what the desktop does with the key, `POWER_OFF_UNAVAILABLE` (`userland/capsule_desktop_shell/src/state/system_key.rs:42-47`).
- Battery: `sys_battery_status` (`src/syscall/microkernel/battery.rs:35-44`); the top bar's words, `UNAVAILABLE` (`userland/capsule_desktop_shell/src/state/indicators/battery_text.rs:26-27`).
- TPM: the `crb` and `fifo` transports (`src/security/tpm/mod.rs:25-34`).
- VT-d: `is_enforcing` (`src/arch/x86_64/iommu/mod.rs:17-22`). The IOMMU features are in `Cargo.toml`.
- QEMU devices: under `make boot` and CI's boot smoke, `devices` and `usb_stick` (`tools/nonos_qemu/machine.py:80-100`). The [boot smoke](../build/ci.md#ci-boot-smoke) boots a second time from a USB stick and expects the USB storage driver to bind (`expect`, `.github/workflows/ci-boot-smoke.yml:100-106`). Under the make targets, `QEMU_SMP` (`mk/10-qemu.mk:32`), `QEMU_IOMMU_OPTS` (`mk/10-qemu.mk:45`), `QEMU_GPU` (`mk/10-qemu.mk:96`), `QEMU_USB` (`mk/10-qemu.mk:99-102`), `QEMU_AUDIO` (`mk/10-qemu.mk:109`), `QEMU_TPM` (`mk/10-qemu.mk:122`), `QEMU_NET` (`mk/10-qemu.mk:128`), and the NVMe install target, `INSTALL_TARGET_IMG` (`mk/40-run.mk:452-466`). The boot matrix's USB stick cells are the ones with a `usb_block` (`scripts/bootmatrix/cells.py:59-60`), and `nonos-mk-plan-a-runtime` attaches a `usb-tablet` beside its `QEMU_BLK_IMG` disk (`nonos-ci/plan-a-runtime.sh:60-64`).
- Check results: the flake's `proofs-<crate>` checks (`proofChecks`, `tools/nix/checks.nix:101-102`), each running the crate's tests and then clippy (`tools/nix/checks.nix:85-96`, `clippy`).

## How the matrix is kept current

No tool writes this page. Before the commit in its footer moves, the NONOS team checks every row against the tree by hand:

1. Each row's ids are read again from the tables listed under Sources. A new id, a new driver or a driver that enters or leaves the image becomes a row change.
2. The state follows the code: what the kernel spawns at boot (`src/userspace/init/spawn_plan`), what the driver refuses and why, and what is missing.
3. Proof crate counts and results come from the flake checks for that exact commit. A failing check is shown as failing, with its cause.
4. Real hardware enters the How verified column only with a hardware report made as [Reporting a machine](report.md) describes. A report that cannot name its image commit says so, as the one above does.

A machine is listed by its chips and their ids, never by brand or model.

## See also

- [Reporting a machine](report.md)
- [Drivers](../drivers/README.md)
- [Input drivers](../drivers/input/README.md)
- [Audio](../drivers/audio.md)
- [Installation requirements](../install/requirements.md)
- [Tests and proofs](../contributing/tests-and-proofs.md)
