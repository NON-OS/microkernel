# Update

How to move an installed NONOS to a newer release, what carries over, and what the TPM changes on the way.

## There is no in-place update in 0.9.2

NONOS 0.9.2 has no update service and no A/B system slots. The installer writes a whole disk from the image that is running, and the Marketplace installs programs into the [store](../overview/glossary.md#store), never the system itself. The kernel's boot slots feed the anonymous device proof, and are not an update mechanism.

To update, you install the newer release over the old one.

## The procedure

1. Get the newer image and write it to a stick ([Get an image](get-an-image.md), [Write a USB stick](usb-stick.md)).
2. Boot the stick and choose `Install NØNOS` in the boot menu ([Boot modes](boot-modes.md)).
3. Answer setup. Its answers are the ones the new disk starts with ([First boot](first-boot.md)). If an earlier boot of this stick kept its answers, setup does not run, and those answers go to the disk.
4. On the Disks screen choose the disk that holds the old system. Its row says `NONOS installed`, and the Confirm screen warns `holds NØNOS; its store and data volume are lost`.
5. Type the disk's word, let the installer write and read back, remove the stick and restart ([Install to disk](install-to-disk.md)).

An update from one release to another is not tested in this release.

## What carries over

Nothing from the old disk. The installer carries what the running boot holds, and when you boot the stick that is the stick's store, because the kernel takes a USB stick that carries NONOS before any internal disk. It carries setup's answers, the wallpapers they keep, and the signed programs the stick carries. The old store, the programs you installed there from the Marketplace, anything you kept there, and the old [data volume](../overview/glossary.md#data-volume) are erased with the disk.

NONOS 0.9.2 has no way to copy files off the old disk first: the file service keeps files in memory and in the NONOS store, and mounts no other file system, and the USB storage driver serves sectors, not files.

## The rollback floor

Every signed kernel carries a [rollback index](../overview/glossary.md#rollback-index): the [seal](../overview/glossary.md#seal) signs it in from `rollback_index` in `nonos.toml`, 1 by default, which that file says to raise only for a security release. Each verified boot raises the machine's [TPM](../overview/glossary.md#tpm) [rollback floor](../overview/glossary.md#rollback-floor) to the index of the kernel it just booted, and a kernel whose index is below the floor is refused on every entry, under the title `This kernel is older than allowed`.

So on a machine with a TPM:

- Booting a newer release's stick once is enough to raise the floor, if that release raised its index. From then on the older system on the disk is refused, so install the newer one.
- There is no way back to an older release on that machine once the floor has passed its index, short of clearing the TPM, which also loses every key the TPM derives.

Without a TPM the loader keeps no floor. Hardened and Air-Gapped then refuse to boot; every other entry notes `No TPM: rollback protection is off`, and an older signed kernel boots.

## Firmware changes and the data volume

The data volume's key and the key that seals remembered Wi-Fi networks are derived by the TPM under the boot [PCRs](../overview/glossary.md#pcr) 0, 4, 7 and 9: the firmware code, the boot manager the firmware measured, the Secure Boot policy, and the kernel the loader measured. Nothing of the key is stored, so a change to any of these gives another key:

- after a firmware update, or after turning Secure Boot on or off, the installed data volume stays closed and remembered networks read `Sealed under a different boot state`;
- a newer kernel opens no volume an older one made, by design.

The kernel never formats over a volume it cannot open: it leaves it, and a program that asks for it gets EIO while the serial console says `[DATA] refused with EIO: Unopenable`. The kernel's own line for this goes to its structured log, which nothing installs in this release, so it does not print. Putting the firmware and the Secure Boot setting back the way they were gives the old key back. Clearing the TPM changes the seed these keys come from, and every such key with it, for good.

A reinstall is not affected: the installer clears the key header and the volume's header ring, so the new system makes a new volume on its first boot.

## Where this comes from

- There is no in-place update in 0.9.2
  - The installer writes a disk from the running image: `Install` in `userland/capsule_install/src/install/mod.rs:17-39`.
  - The Marketplace installs programs into the store and writes no disk itself: `capsule_installer` in `userland/capsule_installer/README.md:5-12`.
  - The boot slots feed the device proof: `MkBootSlots` in `userland/boot_slots_proofs/README.md:3-10`.
- What carries over
  - A USB stick that carries NONOS comes first: `ORDER` in `src/hardware/block_device/select.rs:39-48`.
  - Setup's answers, then wallpapers, then signed programs: `gather` in `userland/nonos_disk/src/carry/gather.rs:17-38`.
  - Files in memory and in the store only: `capsule_vfs` in `userland/capsule_vfs/README.md:5-9`.
  - The USB storage driver serves sectors: `capsule_driver_usb_msc` in `userland/capsule_driver_usb_msc/README.md:5-8`.
- The rollback floor
  - The index, `rollback_index`, 1 by default and raised only for a security release: `nonos.toml:27-29` and `tools/nix/config.nix:129`, signed in by `sign_kernel` in `tools/nonos_seal/chain.py:65-70`.
  - Each verified boot raises the floor: `commit_rollback` in `nonos-bootloader/src/boot/crypto/rollback/commit.rs:26-47`.
  - A kernel below the floor is refused: `enforce_floor` in `nonos-bootloader/src/boot/crypto/rollback/floor.rs:28-39`.
  - The title `This kernel is older than allowed`: `PLATFORM` in `nonos-bootloader/src/display/boot/refusal/platform.rs:21-26`.
  - Without a TPM, Hardened and Air-Gapped refused and the rest warned: `Floor::Refuse` and `Floor::Unprotected` in `nonos-bootloader/src/boot/crypto/rollback/floor.rs:41-58`.
- Firmware changes and the data volume
  - PCRs 0, 4, 7 and 9: `BOUND_PCRS` in `src/security/tpm/machine_key/pcrs.rs:21-24`.
  - A newer kernel opens no older volume, by design: the boot tool's note above `serial` in `tools/nonos_qemu/__main__.py:140-142`.
  - Never formatted over: `mount_or_format` in `src/fs/blockfs_volume/mount_or_format.rs:50-76`.
  - EIO and the `[DATA] refused with EIO:` line: `errno` in `src/syscall/microkernel/data/errno.rs:30-53`.
  - Clearing the TPM changes the seed and every key: `TPM2_HMAC` in `src/security/tpm/machine_key/mod.rs:19-31`.

## See also

- [Install to disk](install-to-disk.md)
- [Recovery](recovery.md)
- [Troubleshooting](troubleshooting.md)
- [Rollback protection](../security/rollback-protection.md)
- [Measured boot and the TPM](../security/measured-boot-and-tpm.md)
- [Release notes for 0.9.2](../release/0.9.2.md)
