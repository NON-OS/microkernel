# IOMMU: The posture line

Part of [IOMMU](iommu.md).

## The posture line

`report_posture` prints the state once DMA protection has been set up, right
after paging, and prints the first line again whenever the count changes
(`src/memory/iommu/posture.rs:38-79`):

```
[IOMMU] <none|intel-vt-d|amd-vi> present, enforcing=<0|1>, unconfined grants=<n>, identity devices=<n>
[IOMMU] capabilities aw=<bits> ir=<0|1> snoop=<0|1> pages=<mask> domains=<n>
```

`enforcing=1` together with `unconfined grants=0` means that every DMA buffer
the broker has granted is confined, because `unconfined_grants` counts every
grant made without a domain (`src/memory/iommu/posture.rs:17-28`). It does not
mean every device is confined: a device firmware reserved memory for keeps that
memory while no capsule has claimed it. `identity devices` counts the devices
the unit still lets reach all of memory, through an identity map on VT-d or a
passthrough device table entry on AMD-Vi; bring-up leaves none, as every device
starts blocked or in a domain of its reserved memory, and the line is printed
again when a claim or a release changes the count
(`src/memory/iommu/posture_line.rs`). Only `enforcing=1`, `unconfined grants=0`
and `identity devices=0` together mean that no device reaches memory outside a
capsule's grants and its own reserved memory. On a machine with no remapping
hardware the boot prints `[IOMMU] no DMAR remapping unit and no IVRS table;
IOMMU domains refused; DMA is unrestricted` and selects `IommuVendor::Absent`
(`src/arch/x86_64/iommu/backend/select.rs:62-65`).

## AMD-Vi

`init` for AMD-Vi first takes every unit back from firmware, because a unit left
enabled would translate every device through tables built for firmware's own
drivers (`src/arch/x86_64/iommu/amd_vi/init.rs`). `microkernel-core` builds
`nonos-iommu-amdvi`, so the kernel then drives the units through `bringup`:
one shared device table in which every requester id starts blocked, IV set
with no table so its interrupts are refused, a device with IVMD ranges keeping
those, per capsule domains, and a remapping table per device that binds an
interrupt (`src/arch/x86_64/iommu/amd_vi/intr.rs`). The boot prints
`[AMD-VI] IOMMU translation enabled, units=<n> reserved=<n>`.

## Trying it in QEMU

The build has a boot with an emulated Intel IOMMU.
`nonos-mk-run-iommu-serial-log` builds the desktop image and runs QEMU under
TCG, without hardware acceleration, so it is slow; it writes the serial log to
`QEMU_IOMMU_SERIAL_LOG`, the path it prints (`mk/40-run.mk:489-501`):

```
make nonos-mk-run-iommu-serial-log
```

No run of this target is recorded for this release, so no log from it is quoted
here.
