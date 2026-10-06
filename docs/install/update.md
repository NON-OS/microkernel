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
