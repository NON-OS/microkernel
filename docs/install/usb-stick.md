# Write a USB stick

How to put `nonos.img` on a USB stick, check that it arrived whole, and boot from it.

## Before you start

- A stick of 2 GB or more. The image is about 995 MiB ([Requirements](requirements.md#usb-stick)).
- Everything on the stick is destroyed.
- The image, built and sealed: `target/release/<profile>/nonos.img` ([Get an image](get-an-image.md)). Write `nonos.img`, not `nonos.iso`.

Find the stick's device name first, and check its size, so that you do not write over a disk you need. On Linux:

```
lsblk
```

On macOS:

```
diskutil list
```

Not tested in this release.

## With make, on Linux or macOS

The `usb` target writes the sealed image and asks for the disk twice (`Makefile`):

```
make usb
make usb DISK=/dev/sdX
```

Not tested in this release.

- `make usb` alone prints `Image:` and the image it would write. With no sealed image it stops with `no sealed image: make seal`.
- With `DISK=` it prints `About to OVERWRITE`, asks you to type the disk path again, and writes nothing if the two differ: it says `Mismatch; not writing.`
- On Linux it runs `dd` with `bs=4M status=progress conv=fsync`.
- On macOS it unmounts the disk with `diskutil unmountDisk`, writes the raw device (`/dev/rdisk4` for `/dev/disk4`) with `bs=4m`, runs `sync`, and ejects the disk.

When more than one profile is sealed, `make usb` takes the first image it finds under `target/release/`. Name the one you mean: `make usb PROFILE=standard DISK=/dev/sdX`.

## By hand on Linux

Replace `sdX` with the stick's name from `lsblk`: the whole disk, not a partition such as `sdX1`.

```
sudo dd if=target/release/standard/nonos.img of=/dev/sdX bs=4M status=progress conv=fsync
```

Not tested in this release.

## By hand on macOS

Replace `disk4` with the stick's name from `diskutil list`.

```
diskutil unmountDisk /dev/disk4
sudo dd if=target/release/standard/nonos.img of=/dev/rdisk4 bs=4m && sync
diskutil eject /dev/disk4
```

Not tested in this release.

## Windows

This repository documents no tool for writing the stick from Windows. Write it from a Linux or macOS machine.

## Check the write

Compare the stick with the image, byte for byte, over the length of the image. The stick is larger than the image, so `cmp` reads only as many bytes as the image holds. On Linux:

```
sudo cmp -n "$(stat -c %s target/release/standard/nonos.img)" target/release/standard/nonos.img /dev/sdX && echo same
```

Not tested in this release.

On macOS:

```
sudo cmp -n "$(stat -f %z target/release/standard/nonos.img)" target/release/standard/nonos.img /dev/rdisk4 && echo same
```

Not tested in this release.

`same` means the write arrived whole. Check before the first boot. The kernel keeps a boot's state on a USB stick that carries NONOS before any internal disk (`ORDER` in `src/hardware/block_device/select.rs`), so a boot whose setup chose Install keeps its answers in the stick's own package [store](../overview/glossary.md#store), and a used stick no longer matches the image (`userland/capsule_setup_wizard/src/render/screens/mode.rs`).

## Boot from the stick

1. Plug the stick in and turn the machine on.
2. Open the firmware's boot menu and choose the stick. The key for that menu depends on the machine.
3. The NONOS boot menu appears and counts down 10 seconds on its default entry. Pick an entry, or let the countdown start the default ([Boot modes](boot-modes.md)).

If the firmware will not start the stick with Secure Boot on, turn Secure Boot off and try again: the loader is signed only with the NONOS db key ([Requirements](requirements.md#secure-boot-and-the-tpm)). Not tested in this release.
