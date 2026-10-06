# Storage drivers

Which disks NONOS can read and write, which capsule drives each kind, and how one disk becomes the package store and the data volume.

## What is supported

| Controller | How it is matched | Capsule | State in 0.9.2 |
|---|---|---|---|
| NVMe SSD | PCI class 01h, subclass 08h, prog-if 02h | `driver.nvme0` | Served. See [NVMe](nvme.md). |
| SATA disk on an AHCI controller | PCI class 01h, subclass 06h; Intel class 01h, subclass 04h | `driver.ahci0` | Served, one disk. See [AHCI and Intel RST](ahci-and-rst.md). |
| NVMe behind an Intel VMD | 13 Intel VMD device ids | kernel PCI layer, then `driver.nvme0` | In the kernel, not run on hardware or under QEMU. See [Intel VMD](vmd.md). |
| eMMC on an SD host controller | PCI class 08h, subclass 05h | `driver.ahci0` | Served when no SATA disk comes up, not tested on hardware. See [SD cards and eMMC](sd-and-emmc.md). |
| Realtek PCIe SD card reader | 10ec:5227, 10ec:522a | `driver.rtsx0` | Source and host proofs only, not in the image. See [SD cards and eMMC](sd-and-emmc.md). |
| USB stick or USB disk | interface class 08h, subclass 06h, protocol 50h | `driver.usb_msc0` | Served on root ports. See [USB mass storage](usb-mass-storage.md). |
| virtio-blk under QEMU | 1af4:1001, 1af4:1042 | `driver.virtio_blk0` | Served. |

The virtio-blk ids are `VIRTIO_BLK_TRANSITIONAL` and `VIRTIO_BLK_MODERN` (`userland/capsule_driver_virtio_blk/src/constants/pci.rs:16-18`).

Every storage driver is a [capsule](../../overview/glossary.md#capsule) in ring 3. The PCI drivers take their controller from the [hardware broker](../../overview/glossary.md#hardware-broker); the USB mass-storage driver reaches its device through the xHCI driver. The kernel block layer decides which disk holds NONOS, and the file system above it reads what is on that disk.
