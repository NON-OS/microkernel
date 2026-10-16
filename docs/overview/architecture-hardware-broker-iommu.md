# Architecture: Hardware broker, IOMMU and devices

Part of [Architecture](architecture.md).

## Hardware broker, IOMMU and devices

No driver capsule touches a device on its own. It claims the device from the
hardware broker in the kernel (`src/hardware/broker/`), then asks for grants on
it: MMIO windows, DMA buffers, interrupt bindings and, on x86_64 only, port I/O.
Each kind of call needs its own bit: claiming needs Driver, an MMIO window Mmio,
an interrupt binding Irq, a DMA buffer Dma and port I/O Pio (`MkDeviceClaim`,
`MkMmioMap`, `MkIrqBind`, `MkDmaMap` and `MkPioGrant` in
`src/syscall/contract/cap_table/mk_device.rs:25-37`).

The broker gives each driver capsule its own [IOMMU
domain](glossary-hardware-broker.md#iommu-domain), which every device that capsule claims shares
(`attach` in `src/hardware/broker/confine/attach.rs`). This kernel drives Intel
VT-d and AMD-Vi, and every image turns translation and interrupt remapping on at
boot (`nonos-iommu-enforce`, `nonos-iommu-intremap` and `nonos-iommu-amdvi` in
`microkernel-core`). When a claim
is released, the broker turns the device's bus mastering off, then detaches it
from its domain (`release` in `src/hardware/broker/claim/release.rs:24-36`).

Confinement does not reach every device. A device found at boot that no driver
claims is blocked, but for the memory firmware reserved for it (see [identity
domain](glossary-hardware-broker.md#identity-domain)), and a device with no PCI requester id,
such as a controller found through ACPI, gets no domain (`pci_address` in
`src/hardware/broker/confine/table.rs`). When no remapping unit is in service,
because the firmware describes none or the one found did not come up, or when
interrupts are not remapped, a claim is refused, unless the kernel is built
with a development opt-in that a release build refuses. The serial log names
each such claim (`verdict` in `src/hardware/broker/confine/decide.rs`, `admit`
in `src/hardware/broker/confine/interrupts.rs`).

[Hardware broker](../kernel/hardware-broker.md), [IOMMU](../kernel/iommu.md),
[PCI and ACPI](../kernel/pci-and-acpi.md) and the driver side, [Broker
API](../drivers/broker-api.md), go deeper.

## Driver capsules

Each driver is a capsule in ring 3, in a directory named
`userland/capsule_driver_<name>`. Of the 26 such directories with a
`Capsule.mk`, the build catalogue `tools/nix/capsules.json` lists 18, and those
are the drivers this release builds: `ahci`, `e1000`, `hda`, `i2c_hid`,
`i2c_pci`, `iwlwifi`, `nvme`, `ps2_input`, `rtl8139`, `rtl8169`, `rtl8821ce`,
`usb_hid`, `usb_msc`, `virtio_blk`, `virtio_gpu`, `virtio_net`, `virtio_rng` and
`xhci`. The other eight, `ax88179`, `cdc_ecm`, `cdc_ncm`, `e1000e`, `igc`,
`rndis`, `rtl8153` and `rtsx`, are in the tree with [proof
crates](glossary-peer-list.md#proof-crate) but not in this release's catalogue, so no
image carries them. A 27th directory, `capsule_driver_bga`, has no `Capsule.mk`.

When a driver ends, by exit or by fault, the process teardown gives back every
claim and grant it held (`release_all_for_pid` in
`src/process/exit/teardown.rs:47-51`). The NVMe, AHCI and virtio-blk drivers
serve raw sectors only to the kernel's own client and to holders of StoreWrite,
and ask the kernel on every request (`permits` in
`userland/capsule_driver_nvme/src/server/medium.rs:17-30`).

[Drivers](../drivers/README.md) describes the driver model, [Writing a
driver](../drivers/writing-a-driver.md) builds one, and the [support
matrix](../hardware/MATRIX.md) lists each device class and chip with its id.
