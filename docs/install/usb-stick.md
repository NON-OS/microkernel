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
