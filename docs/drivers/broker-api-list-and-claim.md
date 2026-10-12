# Broker API: List and claim

Part of [Broker API](broker-api.md).

## List and claim

`mk_device_list(class, buf, count)` asks for one class, or every record with
class 0, through `list_by_class` (`src/hardware/broker/table/list.rs:29-34`).
`sys_device_list` returns the number of records when `count` is 0 and copies at
most `count` records otherwise (`src/syscall/microkernel/device.rs:40-65`). Most
drivers in the tree pass a buffer of 128 records, as virtio-rng's `MAX_DEVICES`
does; the device list holds ACPI and fabricated records as well as PCI functions
(`userland/capsule_driver_virtio_rng/src/discover/find.rs:22-24`).

`mk_device_claim(device_id)` returns the [claim
epoch](../overview/glossary.md#claim-epoch) or a negative errno, and
`sys_device_claim` answers -19 for an unknown device, -16 when another process
holds it and -1 when the device cannot be confined
(`src/syscall/microkernel/device.rs:67-82`). The broker's `claim` does four
things in order (`src/hardware/broker/claim/claim.rs:23-45`):

1. It refuses a device someone already holds and records the caller with a fresh
   epoch.
2. It attaches a PCI device to the caller's own [IOMMU
   domain](../overview/glossary.md#iommu-domain) through `attach`, which gives a
   record with no PCI address, such as the PS/2 or an ACPI I2C record, no domain
   (`src/hardware/broker/confine/attach.rs:30-101`).
3. It brings a PCI function to power state D0 with `power_on_device`, because
   firmware may leave an LPSS function in D3 with its registers dead
   (`src/hardware/broker/power.rs:20-30`).
4. It clears Enable No Snoop in the PCIe Device Control register with
   `snoop_every_request`, so every DMA request snoops the CPU caches, and logs
   whether the bit stayed off (`src/hardware/broker/claim/no_snoop.rs:33-52`).

A PCI device that no remapping unit in service covers cannot be confined, and
the claim goes ahead without a domain. `attach` asks `translates` first, and
when the answer is no it logs `unconfined: no remapping unit in service covers
it, reaches all memory` and lets the claim through
(`src/hardware/broker/confine/attach.rs:41-48`, `translates`). That is every
device on a machine without VT-d, every device on an AMD-Vi machine, since no
build profile drives AMD-Vi (see [Platform](platform.md#vt-d-and-amd-vi)), and a
device outside every unit's scope on a VT-d machine. If the capsule's domain
then cannot be made, `unconfined_allowed` still lets the claim through for the
four errors that mean no unit is in service
(`src/hardware/broker/confine/posture.rs:32-49`). Any other attach failure
refuses the claim.

Every later call on the device carries the epoch, and calls on a grant name the
grant id. A call with an old epoch fails with -116, `ERRNO_STALE`
(`src/syscall/microkernel/errnos.rs:54`).
