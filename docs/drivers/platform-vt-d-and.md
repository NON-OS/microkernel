# Platform: VT-d and AMD-Vi

Part of [Platform](platform.md).

## VT-d and AMD-Vi

The [IOMMU](../overview/glossary-hardware-broker.md#iommu) is what confines a driver's DMA to
the buffers the [hardware broker](../overview/glossary-hardware-broker.md#hardware-broker) gave
it. Every image is built with `nonos-arch-iommu`, `nonos-iommu-enforce`,
`nonos-iommu-intremap` and `nonos-iommu-amdvi` as part of the core feature
set, so the kernel programs the Intel VT-d units the ACPI DMAR table
describes, or the AMD-Vi units IVRS describes, and turns translation and
interrupt remapping on.

The vendor is read from the ACPI tables, never from CPUID: `IommuVendor` is
AMD-Vi when an IVRS table is present and DMAR describes no unit
(`src/memory/iommu/vendor.rs`). A machine with neither, or whose units did not
come into service, has every PCI claim refused, and the broker says so in the
log on each claim ([Broker API](broker-api-list-and-claim.md#list-and-claim)).
Only a kernel built with a development opt-in, which a release refuses, lets
such a device through, and it can then reach all of memory.

With VT-d in service, `attach` gives each [driver
capsule](../overview/glossary-dma-pool.md#driver-capsule) an [IOMMU
domain](../overview/glossary-hardware-broker.md#iommu-domain) of its own and moves each claimed
device into it. A device that no unit in service covers stays on physical
addresses, as `translates` decides
(`src/hardware/broker/confine/attach.rs:36-44`), and each DMA grant made for it
is counted with `note_unconfined` until it is given back
(`src/hardware/broker/dma/map/record.rs:45-51`). The boot log carries one
posture line from `posture_line`, `[IOMMU] <vendor> present, enforcing=<0 or 1>,
unconfined grants=<n>`, and device DMA is confined only when `enforcing=1` and
the count is zero, as `report_posture` explains
(`src/memory/iommu/posture.rs:17-38`, `src/memory/iommu/posture.rs:69-79`).
Faults are polled from the timer tick with `poll_faults`
(`src/interrupts/timer/clock.rs:54-55`).

[IOMMU](../kernel/iommu.md) covers the tables, the queues and the fault reports.
