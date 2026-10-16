# IOMMU: Per capsule domains

Part of [IOMMU](iommu.md).

## Per capsule domains

The [hardware broker](hardware-broker.md) gives each driver capsule one domain,
which maps nothing until `MkDmaMap` grants the capsule a buffer
(`src/hardware/broker/confine/mod.rs:17-21`). `attach` runs inside
`MkDeviceClaim` and decides what the device can reach; when it refuses, the
claim fails with `EPERM`.

```mermaid
flowchart TD
    C[MkDeviceClaim] --> R{PCI requester id}
    R -->|none, ACPI device| N[claim granted, no DMA grant ever made]
    R -->|yes| T{unit in service covers it, interrupts remapped}
    T -->|no| X
    T -->|yes| S{id held by another capsule}
    S -->|yes| X[claim refused, EPERM]
    S -->|no| A{attach_device}
    A -->|fails| X
    A -->|ok| D[device in the capsule's domain]
```

- If `translates` says no unit in service covers the device, the claim is
  refused (`verdict` in `src/hardware/broker/confine/decide.rs`). `translates`
  decides coverage: a unit must be enforcing and its scope must hold the device
  (`src/arch/x86_64/iommu/backend/device.rs`). Only a kernel built with
  `nonos-dev-unconfined-dma`, which a release refuses, lets it through on
  physical addresses, with a serial line saying it reaches all memory.
- A claim is refused too where interrupts are not remapped, but under
  `nonos-dev-unremapped-interrupts` (`admit` in
  `src/hardware/broker/confine/interrupts.rs`).
- If a unit covers it, the device leaves the blocked domain, or its reserved
  one, and `attach_device` puts it in the capsule's domain
  (`src/hardware/broker/confine/join.rs`).
- Drives behind one VMD share a requester id; one capsule may hold several of
  them, and a second capsule is refused the shared id with `Refused`
  (`src/hardware/broker/confine/attach.rs:72-82`).
- A device with no PCI requester id, such as a controller found through ACPI, is
  no bus master: `pci_address` returns nothing for it and the claim goes ahead
  with no DMA grant ever made (`src/hardware/broker/confine/table.rs`), but
  under the development opt-in.

A DMA [grant](../overview/glossary-dma-pool.md#grant) in a confined domain is mapped with
`IommuProtection::READ_WRITE` at an I/O virtual address from the capsule's own
range (`src/hardware/broker/confine/map.rs:22-50`). When the capsule releases
the device, `detach` puts it back to denied, not to the identity domain, and the
domain goes with the capsule's last device
(`src/hardware/broker/confine/detach.rs:21-46`).

## When remapping is not in service

Every call that claims to confine a device asks first whether a unit is
translating. `require` refuses with `NotInitialized` before bring-up has
succeeded, so the kernel never writes entries into tables no hardware walks, and
never hands a device an I/O virtual address it would take as physical
(`src/arch/x86_64/iommu/backend/enforced.rs:17-36`). On an AMD-Vi machine the
calls are refused with `AmdViNotDriven` in a build without the AMD-Vi backend,
and with no IOMMU with `NoIommu`, each with a serial line from `amd_vi` or
`absent`
(`src/arch/x86_64/iommu/backend/refuse.rs:22-38`).

Under the development opt-in the device then reaches all of memory, and the
kernel counts it instead of hiding it. Each DMA grant made without a confining domain adds one to the count
through `note_unconfined`, and its release takes one off
(`src/memory/iommu/unconfined.rs:35-51`). The kernel's own virtio-rng entropy
driver counts its buffers the same way, with `note_unconfined`
(`src/drivers/virtio_rng/device/core.rs:44-45`).
