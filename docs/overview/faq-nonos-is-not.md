# FAQ: NONOS is not a Linux distribution

Part of [FAQ](faq.md).

## NONOS is not a Linux distribution

NONOS has its own kernel, a capability microkernel written in Rust, and no Linux
kernel at all. Linux programs run on it through the [Linux
personality](glossary-linux-personality.md#linux-personality), a [capsule](glossary-boot-root-record.md#capsule)
that answers their Linux syscalls. See [Architecture](architecture.md).

## Running Linux programs

The Linux personality runs x86_64 Linux programs in processes that hold no NONOS
capabilities. A program read from the store runs only if the proof kept beside
it verifies; the built-in BusyBox, which provides `sh`, is part of the
personality's own signed image. With `linux = true` under `[store]` in
`nonos.toml`, the default, the image's store carries Linux tools, python3,
sqlite3 and john among them. A Linux call the personality does not serve gets
ENOSYS, and the log names the call. A program started from the Terminal's
`linux` command reaches no network. See [Linux
programs](../using/linux-programs.md) and [Linux
personality](../userland/linux-personality.md).

The Terminal also runs seven programs from crates.io, `tokei` and `csview` among
them, each built as its own signed NONOS capsule that needs no Linux
personality. See [Command-line tools](../using/command-line-tools.md).

## Keeping files

By default nothing is kept: every boot is [amnesic](glossary-amnesic-boot.md#amnesic-boot)
until the person chooses to install. Files saved during a boot live in memory,
in the [file store](glossary-dma-pool.md#file-store), and are gone at power off. On a
live stick the [data volume](glossary-claim-epoch.md#data-volume) that holds Qwen models is
in RAM as well. After an install to a disk, a file a program asks to keep goes
to the [package store](glossary.md#store) on that disk, which is not encrypted,
and Qwen models go to an encrypted data volume keyed by the machine's
[TPM](glossary-stark-proof.md#tpm). Files made in Files or Editor are never kept, and a
Linux package is held in memory until restart. The kernel can also key a volume
with a passphrase, but nothing in this release asks for one. See
[Files](../using/files.md) and [Install to disk](../install/install-to-disk.md).

## Hardware NONOS runs on

NONOS 0.9.2 builds images for x86_64 computers, and it runs on every core of the
processor: the kernel starts every CPU the firmware enables, up to 256, and
schedules processes on all of them. Drivers are written for device classes and
chips, and the [support matrix](../hardware/MATRIX.md) lists each class and chip
with its PCI or USB id.

On Intel VT-d machines the [IOMMU](glossary-hardware-broker.md#iommu) confines device DMA: each
PCI device a driver claims reaches only the buffers granted to that driver.
AMD-Vi is not driven and interrupt remapping is off in this release, so on an
AMD machine device DMA is unrestricted, and the boot log says so. See
[IOMMU](../kernel/iommu.md).

One machine has a developer report for these items: Wi-Fi on the Realtek
RTL8821CE (PCI `10ec:c821`) for scan, join, DHCP, DNS and browser traffic; the
local Qwen model offline; the Linux programs sh, python3, sqlite3 and john; the
installer writing to an internal NVMe disk and booting from it; the I2C-HID
touchpad on the Intel LPSS I2C controller (PCI `8086:31ac` to `8086:31ba`, even
device ids only); the PS/2 keyboard with its layouts; Intel HD Audio (PCI class
`0x0403`, or `0x0401` on an Intel controller that is not an SST engine); the
power button and the volume keys. Works on an x86_64 laptop (Intel Gemini Lake,
8 GB), developer hardware report, 6 October 2026; the image commit was not
recorded.

To add a machine of your own, see [Report a machine](../hardware/report.md).
