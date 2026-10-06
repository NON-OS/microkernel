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
