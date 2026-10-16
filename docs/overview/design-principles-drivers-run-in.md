# Design principles: Drivers run in ring 3

Part of [Design principles](design-principles.md).

## Drivers run in ring 3

A device driver is a capsule like any other, with no more reach than its bits
allow. It touches its device only through the [hardware
broker](glossary-hardware-broker.md#hardware-broker): claiming a device needs Driver, mapping
its registers needs Mmio, and a DMA buffer needs Dma (`MkDeviceClaim`,
`MkMmioMap` and `MkDmaMap` in
`src/syscall/contract/cap_table/mk_device.rs:25-32`). On Intel VT-d machines the
[IOMMU](glossary-hardware-broker.md#iommu) confines device DMA: each PCI device a driver claims
moves into that driver capsule's own domain, which maps only the buffers granted
to it, and a claim the unit in service cannot confine is refused (`attach` in
`src/hardware/broker/confine/attach.rs:30-101`). A driver that crashes ends as a
process, and the process teardown releases its claims and grants on the way out
(`release_all_for_pid` in `src/process/exit/teardown.rs:47-51`).

Of device code, the kernel keeps PCI enumeration and a virtio-rng entropy probe
for its own boot (`init_pci` and `init_virtio_rng` in
`src/drivers/mod.rs:17-35`), plus the interrupt controllers, the timers and the
TPM.

The limits: every device a capsule claims shares its one domain, so each can
reach the buffers granted for the others (`map` in
`src/hardware/broker/confine/map.rs`). With no remapping unit in service, or
none that remaps interrupts, a claim is refused, unless the kernel is built with
a development opt-in that a release refuses (`verdict` in
`src/hardware/broker/confine/decide.rs`). A device found at boot that no driver
claims is blocked, but for the memory firmware reserved for it (the [identity
domain](glossary-hardware-broker.md#identity-domain) entry says which), and a device with no
PCI requester id gets no domain (`pci_address` in
`src/hardware/broker/confine/table.rs`).
