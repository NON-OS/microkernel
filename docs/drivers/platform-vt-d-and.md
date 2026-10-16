# Platform: VT-d and AMD-Vi

Part of [Platform](platform.md).

## VT-d and AMD-Vi

The [IOMMU](../overview/glossary.md#iommu) is what confines a driver's DMA to
the buffers the [hardware broker](../overview/glossary.md#hardware-broker) gave
it. Every image is built with `nonos-arch-iommu` and `nonos-iommu-enforce` as
part of the core feature set, so the kernel programs the Intel VT-d remapping
units the ACPI DMAR table describes and turns translation on
(`Cargo.toml:240-249`, `Cargo.toml:867-877`). Two parts are off in every build
profile in `tools/nix/config.nix`:

- Interrupt remapping, the `nonos-iommu-intremap` feature
  (`Cargo.toml:879-884`).
- The AMD-Vi backend, the `nonos-iommu-amdvi` feature (`Cargo.toml:886-891`).

The vendor is read from the ACPI tables, never from CPUID: `IommuVendor` is
AMD-Vi when an IVRS table is present and DMAR describes no unit
(`src/memory/iommu/vendor.rs:19-31`). In every build profile, such a machine
gets every domain call refused by `route` with `AmdViNotDriven`
(`src/arch/x86_64/iommu/backend/route.rs:38-48`,
`src/arch/x86_64/iommu/backend/refuse.rs:22-31`). `translates` then answers no
for every device, so the broker lets each claim through without a domain
(`src/arch/x86_64/iommu/backend/dispatch_device.rs:39-46`, `translates`). On
such a machine, and on any machine without VT-d, a claimed device can reach all
of memory. The broker says so in the log on each claim ([Broker
API](broker-api-list-and-claim.md#list-and-claim)).

With VT-d in service, `attach` gives each [driver
capsule](../overview/glossary.md#driver-capsule) an [IOMMU
domain](../overview/glossary.md#iommu-domain) of its own and moves each claimed
device into it. A device that no unit in service covers stays on physical
addresses, as `translates` decides
(`src/hardware/broker/confine/attach.rs:30-48`), and each DMA grant made for it
is counted with `note_unconfined` until it is given back
(`src/hardware/broker/dma/map/record.rs:45-51`). The boot log carries one
posture line from `posture_line`, `[IOMMU] <vendor> present, enforcing=<0 or 1>,
unconfined grants=<n>`, and device DMA is confined only when `enforcing=1` and
the count is zero, as `report_posture` explains
(`src/memory/iommu/posture.rs:17-38`, `src/memory/iommu/posture.rs:69-79`).
Faults are polled from the timer tick with `poll_faults`
(`src/interrupts/timer/clock.rs:54-55`).

[IOMMU](../kernel/iommu.md) covers the tables, the queues and the fault reports.
