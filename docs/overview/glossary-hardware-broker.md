# Glossary: Hardware broker

Part of [Glossary](glossary.md).

## Hardware broker

<a id="broker"></a>The ring 0 code through which a driver capsule lists devices, claims one and receives grants on it: MMIO windows, DMA buffers, interrupt bindings and, on x86_64 only, port I/O, each behind its own capability. When a VT-d unit in service covers a claimed PCI device, the broker moves it into the capsule's IOMMU domain before powering it, and every grant a process holds is released when it exits. Explained in [The hardware broker](../kernel/hardware-broker.md), with the calls in [Broker ABI](../abi/broker.md). Code: `src/hardware/broker/mod.rs`, `src/hardware/broker/claim/claim.rs`.

## Held endpoint

A service endpoint that only the services named for it may send to, whatever
capabilities a sender holds. The `HELD` table lists fifteen driver endpoints;
among them, a wired network driver takes sends only from `net.core` and
`net.l2`, a Wi-Fi driver from `net.core`, Settings and setup, the xHCI
controller from the USB HID and USB storage drivers, and the keyboard, USB HID,
I2C-HID, USB storage and random-source drivers from no capsule at all, because
the kernel drives them itself. Explained in
[IPC](../kernel/ipc.md#who-may-send-to-whom). Code:
`src/services/registry/held_table.rs`.

## Identity domain

A domain mapping physical memory one to one, which no device is given. Every
function found at boot starts blocked until a driver claims it, but for one
firmware reserved memory for (an RMRR on VT-d, an IVMD on AMD-Vi), which keeps
those ranges alone. Explained in
[IOMMU](../kernel/iommu-when-vt-d.md#when-vt-d-comes-into-service). Code:
`src/arch/x86_64/iommu/unit/bringup/domain.rs`,
`src/arch/x86_64/iommu/amd_vi/reserved.rs`.

## Inbox

A named, bounded message queue in the kernel into which IPC messages are
delivered, such as `proc.<pid>`, where a capsule's requests arrive, and
`stdin.<pid>`, which its parent feeds. An inbox holds 1024 messages by default
and at most 16 MiB. Explained in [IPC](../kernel/ipc.md#limits). Code:
`src/ipc/nonos_inbox/inbox.rs`, `src/ipc/nonos_inbox/budget.rs`.

## IOMMU

The DMA remapping unit, which limits the memory a device can reach and, with
interrupt remapping, the interrupts it can raise. On Intel VT-d and AMD-Vi
machines every image turns both on at boot and confines each claimed PCI device
to its driver capsule's IOMMU domain. A claim on a device no unit in service
can confine and remap is refused, and the boot log says so. Explained in
[IOMMU](../kernel/iommu.md). Code: `src/hardware/broker/confine/decide.rs`.

## IOMMU domain

A set of I/O page tables the remapping unit applies to the devices attached to
it. The hardware broker gives each driver capsule one domain, shared by every
claimed PCI device that a unit in service covers, which maps only the DMA
buffers granted to the capsule, so the device faults on everything else. ACPI
and platform devices get no domain. Explained in
[IOMMU](../kernel/iommu-per-capsule-domains.md#per-capsule-domains). Code:
`src/hardware/broker/confine/attach.rs`.

## Kernel mirror

The kernel module that carries one capsule, named by `CAPSULE_KERNEL_MIRROR` in
its `Capsule.mk`, such as `src/hardware/virtio_rng_capsule` or
`src/userspace/capsule_linux`. It embeds the capsule's four files with
`include_bytes!` and spawns it through the spawn gate with the capabilities it
offers. Explained in [Writing a
driver](../drivers/writing-a-driver.md#11-the-kernel-mirror). Code:
`nonos-mk/capsule.mk`, `src/hardware/virtio_rng_capsule/embed.rs`.
