# Requirements

What a machine needs to boot NONOS 0.9.2 from a USB stick, and to install it on a disk.

| Need | What NONOS 0.9.2 asks for |
|---|---|
| [Processor](#processor-and-firmware) | x86_64, with NX and at least 36 bits of physical address |
| [Firmware](#processor-and-firmware) | UEFI; there is no legacy BIOS path |
| [Random source](#processor-and-firmware) | RDRAND or a TPM, or virtio-rng in a virtual machine |
| [Secure Boot and a TPM](#secure-boot-and-the-tpm) | neither, for Standard on a `standard` image; both, for Hardened |
| [Memory](#memory) | no set minimum; the amount decides what runs |
| [USB stick](#usb-stick) | 2 GB or more |
| [Install disk](#a-disk-to-install-to) | NVMe, SATA or Intel eMMC, 2177 MiB or more, with 512-byte blocks; the whole disk |
| [Network](#network) | optional |

## Processor and firmware

- An x86_64 processor, with one core or many. NONOS starts every core the firmware enables, up to 256, and uses them all. The image carries one loader, `EFI/BOOT/BOOTX64.EFI`, and an x86_64 kernel.
- UEFI firmware. The stick is a GPT disk whose only boot path is its EFI system partition, the [ESP](../overview/glossary.md#esp). There is no legacy BIOS path.
- The NX bit, and at least 36 bits of physical address. The loader stops on a processor without them, and its reason reads `This CPU has no NX bit` or `This CPU addresses under 36 bits of physical memory`.
- A hardware random source. Every boot entry needs one for the boot's keys, and the menu lists `HARDWARE RNG` as missing when none answers. The loader's own advice is to turn on RDRAND or the TPM in the firmware, or to give a virtual machine a virtio-rng device.

## Secure Boot and the TPM

What the firmware must have on depends on the boot entry you pick and on how the image was built.

| Entry | Secure Boot | TPM 2.0 |
|---|---|---|
| Standard, Safe Mode, Recovery, Install | not needed | optional |
| Air-Gapped | not needed | needed, for the [rollback floor](../overview/glossary.md#rollback-floor) |
| Hardened | on, with a platform key (PK) and a signature database (db) | needed |

These are the floors of an image built with the `standard`, `qemu` or `core` [build profile](../overview/glossary.md#build-profile). An image built `hardened` or `airgapped` uses the loader's production policy: every entry then needs [Secure Boot](../overview/glossary.md#secure-boot) and a [TPM](../overview/glossary.md#tpm), because the menu can raise the build's floor and never lower it. The menu shows that floor as `BUILD FLOOR`, beside the Secure Boot and TPM state, before you choose.

Unless the NONOS db certificate is enrolled in your firmware, turn Secure Boot off and boot Standard. Firmware with Secure Boot on starts a loader only when its own db trusts the signature, and the [seal](../overview/glossary.md#seal) signs `BOOTX64.EFI` with the NONOS db key and no other. Without that key the loader is not signed for Secure Boot at all, and a `--release` seal of a `hardened` or `airgapped` image stops. With Secure Boot off the firmware starts any loader; [Boot chain and signatures](../security/boot-chain-and-signatures.md#secure-boot) says what still holds. A `hardened` or `airgapped` image cannot boot that way, because every entry on it needs Secure Boot. This release documents no procedure for the enrollment, and booting with Secure Boot on is not tested in this release.

A Standard boot runs without a TPM, with these things missing:

- No rollback floor. The menu shows `ROLLBACK` with `NO COUNTER`, the loader notes `No TPM: rollback protection is off`, and an older signed kernel would boot.
- No measured check of the loader. The kernel checks the loader file the loader handed over, on the loader's own word, and its log says `self-reported, not measured`.
- No key for an installed disk's [data volume](../overview/glossary.md#data-volume). The kernel derives that key from the TPM, and without one it leaves the volume closed.
- No remembered Wi-Fi networks. A remembered passphrase is sealed with ChaCha20-Poly1305 under a key the TPM derives, and is never written in the clear. Without a TPM, remembering a network fails with `No TPM to seal the passphrase with`.

## Memory

This release sets no minimum amount of memory: the loader's hardware check names only NX and the physical address width. What memory decides is what runs:

- On a boot from the stick, the session's data volume is held in RAM, and only when at least 256 MiB is free beyond what the kernel keeps for the system, which is the larger of 1 GiB and a quarter of memory. Below that, that boot has no data volume: the serial console says `[DATA] live boot:` with the memory free and `no volume in memory`, and a Qwen install stops with `Too little free memory to hold models on this live session`.
- The Qwen model step in setup offers only the tiers that fit this machine's memory ([Local AI](../using/local-ai.md)).
- The QEMU boots give the virtual machine 8 GiB by default (`--mem 8G`), and the boot tool's help says Qwen, Linux programs and a second window need more than 2G.

## USB stick

The sealed image `nonos.img` is 1,043,148,800 bytes, about 995 MiB:

| Part | Size |
|---|---|
| Partition table, [package store](../overview/glossary.md#store), [disk plan](../overview/glossary.md#disk-plan) and ESP | 384 MiB |
| The Qwen3 0.6B model file, laid past the ESP | 639,446,688 bytes, rounded up to whole sectors |
| Room for the backup partition table | 1 MiB |

Use a stick of 2 GB or more. Writing the image erases everything on the stick.

## A disk to install to

- An NVMe disk, a SATA disk on an AHCI controller, or an Intel eMMC host, which the SATA driver also serves. Under QEMU, a virtio disk. The installer lists disks from the NVMe, SATA and virtio-blk drivers only, so a USB disk is never a target.
- At least 2177 MiB, which a drive label calls 2.3 GB.
- 512-byte logical blocks. A disk with 4096-byte blocks is listed and refused.
- Preferably, the firmware's storage mode set to AHCI. With Intel RST in RAID mode the SATA driver still takes the controller, and with VMD on the kernel brings up the drives behind it, but that bring-up has not run on a machine with a VMD ([Intel VMD](../drivers/storage/vmd.md)). When an RST or VMD controller is on the bus and no disk shows, the installer says so; set the mode to AHCI then.
- The whole disk. The installer replaces everything on it, whatever it holds.

Which controllers and chips have drivers, and what has been seen working, is in the [hardware support matrix](../hardware/MATRIX.md).

## Network

A network is optional: setup's default is no network at all.

- Wi-Fi goes through two drivers ([Wi-Fi drivers](../drivers/wifi/README.md)). The Realtek RTL8821CE driver takes PCI id 10ec:c821. The Intel iwlwifi driver takes more Intel cards than it can join on, and on those it reports `card not supported yet` ([iwlwifi](../drivers/wifi/iwlwifi.md)). NONOS 0.9.2 has no USB Wi-Fi driver.
- A wired card served by the e1000, RTL8169, RTL8139 or virtio-net driver is used as soon as a cable is plugged in. Read from the code, only virtio-net, in a virtual machine, gets an address that way: `net.core` drops every frame received through the other three, so DHCP gets no lease through them ([the receive fault](../drivers/ethernet/README.md#the-receive-fault)).

## Reported on real hardware

These items were reported working on one machine: Wi-Fi on the Realtek RTL8821CE (scan, join, DHCP, DNS, browser traffic), the local Qwen model offline, the Linux programs sh, python3, sqlite3 and john, the installer writing to an internal NVMe disk and booting from it, the I2C-HID touchpad on Intel LPSS, the PS/2 keyboard with its layouts, Intel HD Audio, the power button and the volume keys. Works on an x86_64 laptop (Intel Gemini Lake, 8 GB), maintainer hardware report, 6 October 2026; the image commit was not recorded.

## Where this comes from

- Processor and firmware
  - One loader and the kernel on the ESP: `esp` in `tools/nonos_seal/media.py:50-56`.
  - The NX and physical address refusals: `verify_hardware_requirements` in `nonos-bootloader/src/boot/security/hardware.rs:27-35`.
  - `HARDWARE RNG` listed as missing: `missing` in `nonos-bootloader/src/bootmenu/ready.rs:51-58`.
  - The advice on RDRAND, the TPM and virtio-rng: `PLATFORM` in `nonos-bootloader/src/display/boot/refusal/platform.rs:21-36`.
- Secure Boot and the TPM
  - The menu raises the build's floor and never lowers it: `policy_of` in `nonos-bootloader/src/bootmenu/ready.rs:40-49`.
  - `hardened` and `airgapped` use the production loader policy: `loader` in `tools/nix/config.nix:78-91`.
  - `BUILD FLOOR` beside the Secure Boot and TPM state: `facts` in `nonos-bootloader/src/bootmenu/platform.rs:31-48`.
  - The loader signed with the db key alone, and the `--release` stop: `secure_boot` in `tools/nonos_seal/chain.py:97-104` and `tools/nonos_seal/__main__.py:150`.
  - `ROLLBACK` with `NO COUNTER`: `tpm_counter_ok` in `nonos-bootloader/src/bootmenu/platform.rs:42-46`.
  - No floor without a TPM, and its note: `Floor::Unprotected` in `nonos-bootloader/src/boot/crypto/rollback/floor.rs:52-58`.
  - The loader's word only: `decide` in `src/security/boot/loader_check/run.rs:34-55`, and its log line, `SelfReported` in `src/security/boot/loader_check/log.rs:30-31`.
  - The data volume key from the TPM: `derive_for_kernel` in `src/fs/blockfs_volume/open_machine.rs:73-77`.
  - Wi-Fi passphrases sealed with ChaCha20-Poly1305: `seal_file` in `userland/nonos_wifi_client/src/saved/file.rs:6-29`.
  - The sealing key from the TPM: `with_key` in `userland/nonos_wifi_client/src/saved/key.rs:21-25`, and its refusal, `NoTpm` in `userland/nonos_wifi_client/src/saved/error.rs:35`.
- Memory
  - No minimum beyond NX and the address width: `verify_hardware_requirements` in `nonos-bootloader/src/boot/security/hardware.rs:27-35`.
  - 256 MiB free for a volume in RAM, and the `[DATA] live boot:` line: `LEAST` in `src/fs/blockfs_volume/session.rs:34-44`.
  - The system's share, the larger of 1 GiB and a quarter: `reserve` in `src/fs/cryptoblock/ram.rs:55-58`.
  - The Qwen install refusal: `WHY_NO_MEMORY` in `userland/market_proto/src/reason.rs:202-206`.
  - 8G by default, and the help on 2G: `mem` in `tools/nonos_qemu/__main__.py:64`.
- USB stick
  - 384 MiB for the table, store, plan and ESP: `USB_MB` in `tools/nonos_seal/media.py:35`.
  - The Qwen3 0.6B tier laid past the ESP: `STICK_TIER` in `tools/nonos_seal/media.py:41`.
  - 1 MiB kept for the backup table: `GPT_TAIL` in `tools/nonos_seal/media.py:44`.
  - The model file's size: `QWEN3` in `userland/capsule_linux/src/linux/file/models/pinned_qwen3.rs:27-31`.
- A disk to install to
  - The Intel eMMC host on the SATA driver: `Emmc` in `userland/nonos_blk_client/src/driver/pci.rs:35-37`.
  - Disks from the NVMe, SATA and virtio-blk drivers only: `ALL` in `userland/nonos_blk_client/src/driver/table.rs:31`.
  - 2177 MiB: `MIN_DISK_SECTORS` in `userland/nonos_disk/src/layout/sizes.rs:34-38`.
  - 4096-byte blocks refused: `refusal` in `userland/nonos_blk_client/src/disks/describe.rs:48-56`.
  - RST in RAID mode taken by the SATA driver: `is_ahci_function` in `userland/capsule_driver_ahci/src/discover/rule.rs:46-55`.
  - The installer's word on RST or VMD: `raid_hides_disks` in `userland/nonos_blk_client/src/disks/scan.rs:67-73`.
- Network
  - The two Wi-Fi drivers: `SERVICES` in `userland/nonos_wifi_client/src/driver/services.rs:32-34`.
  - PCI id 10ec:c821: `PCI_DEVICE_RTL8821CE` in `userland/capsule_driver_rtl8821ce/src/constants/mod.rs:25-26`.
  - `card not supported yet`: `NoAirPath` in `userland/nonos_wifi_client/src/driver/stage.rs:60`.
  - The wired drivers, used once a cable is in: `WIRED` in `userland/capsule_setup_wizard/src/network/wired.rs:3-14`.

## See also

- [Get an image](get-an-image.md)
- [Boot modes](boot-modes.md)
- [Install to disk](install-to-disk.md)
- [Hardware support matrix](../hardware/MATRIX.md)
- [Measured boot and the TPM](../security/measured-boot-and-tpm.md)
- [Rollback protection](../security/rollback-protection.md)
