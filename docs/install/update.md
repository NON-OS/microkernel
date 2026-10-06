# Update

How to move an installed NONOS to a newer release, what carries over, and what the TPM changes on the way.

## There is no in-place update in 0.9.2

NONOS 0.9.2 has no update service and no A/B system slots. The installer writes a whole disk from the image that is running, and the Marketplace installs programs into the store, never the system itself (`userland/capsule_install/src/install/mod.rs`, `userland/capsule_installer/README.md`). The boot slots that `boot_slots_proofs` tests (`MkBootSlots`) feed the anonymous device proof, and are not an update mechanism (`userland/boot_slots_proofs/README.md`).

To update, you install the newer release over the old one.

## The procedure

1. Get the newer image and write it to a stick ([Get an image](get-an-image.md), [Write a USB stick](usb-stick.md)).
2. Boot the stick and choose `Install NØNOS` in the boot menu ([Boot modes](boot-modes.md)).
3. Answer setup. Its answers are the ones the new disk starts with ([First boot](first-boot.md)). If an earlier boot of this stick kept its answers, setup does not run, and those answers go to the disk.
4. On the Disks screen choose the disk that holds the old system. Its row says `NONOS installed`, and the Confirm screen warns `holds NØNOS; its store and data volume are lost`.
5. Type the disk's word, let the installer write and read back, remove the stick and restart ([Install to disk](install-to-disk.md)).

An update from one release to another is not tested in this release.

## What carries over

Nothing from the old disk. The installer carries what the running boot holds, and when you boot the stick that is the stick's store, because the kernel takes a USB stick that carries NONOS before any internal disk (`ORDER` in `src/hardware/block_device/select.rs`). It carries setup's answers, the wallpapers they keep, and the signed programs the stick carries (`userland/nonos_disk/src/carry/gather.rs`). The old store, the programs you installed there from the Marketplace, anything you kept there, and the old [data volume](../overview/glossary.md#data-volume) are erased with the disk.

NONOS 0.9.2 has no way to copy files off the old disk first: the file service keeps files in memory and in the NONOS store, and mounts no other file system (`userland/capsule_vfs/README.md`), and the USB storage driver serves sectors, not files (`userland/capsule_driver_usb_msc/README.md`).

## The rollback floor

Every signed kernel carries a rollback index: the seal signs it in from `rollback_index` in `nonos.toml`, 1 by default, which that file says to raise only for a security release (`sign_kernel` in `tools/nonos_seal/chain.py`, `tools/nix/config.nix`). Each verified boot raises the machine's TPM [rollback floor](../overview/glossary.md#rollback-floor) to the index of the kernel it just booted (`nonos-bootloader/src/boot/crypto/rollback/commit.rs`), and a kernel whose index is below the floor is refused on every entry, under the title `This kernel is older than allowed` (`nonos-bootloader/src/boot/crypto/rollback/floor.rs`, `nonos-bootloader/src/display/boot/refusal/platform.rs`).

So on a machine with a TPM:

- Booting a newer release's stick once is enough to raise the floor, if that release raised its index. From then on the older system on the disk is refused, so install the newer one.
- There is no way back to an older release on that machine once the floor has passed its index.

Without a TPM the loader keeps no floor, notes `No TPM: rollback protection is off`, and an older signed kernel boots (`nonos-bootloader/src/boot/crypto/rollback/floor.rs`).
