# Platform

What NONOS does with the processor cores, the IOMMU, the TPM, the ACPI buttons and the GPIO controllers under every driver, and what it does not do in this release.

## Processor cores

NONOS 0.9.2 targets x86_64 processors from Intel and AMD; AMD machines are not tested in this release, and the other targets are in [../architectures/README.md](../architectures/README.md). The image build adds the `nonos-smp` feature when `smp` is true, which is the default (`nonos.toml:19-20`, `tools/nix/config.nix:127-150`). With it, `start_secondary_cpus` starts the application processors at boot and logs `[SMP-PROOF] cpu_count=` with the number running (`src/kernel_core/init/start_secondary.rs:17-48`).

`start_aps` walks every secondary in `get_ap_list`, the list the platform reported, and stops at `MAX_CPUS` (256) (`src/smp/init/ap_start.rs:22-62`, `src/smp/topology/detection/state.rs:43-47`, `src/smp/constants.rs:17`). It logs how many answered and how many `cpus_online` reports, which can differ when an AP comes up late (`src/smp/init/ap_start.rs:65-83`). The same bring-up path serves Intel and AMD; a few other parts of the kernel read the vendor, for example `vendor_from_leaf0` for TSC calibration (`src/arch/x86_64/time/tsc/calibration/math.rs:28-39`).

Two details keep the cores in step:

- Where CPUID reports the MSR, each AP sets its `IA32_TSC_ADJUST` to the boot CPU's value in `align`, so every core reads the same uptime (`src/smp/ap/tsc_adjust.rs:17-24`, `src/smp/ap/tsc_adjust.rs:55-72`).
- On Intel hybrid parts each core reads its kind from CPUID leaf 0x1A, and `wake_pass` offers a woken task to an idle performance core before an efficiency core (`src/smp/topology/core_kind.rs:17-70`).

The scheduler side is in [../kernel/scheduler-and-smp.md](../kernel/scheduler-and-smp.md).

## VT-d and AMD-Vi

The IOMMU is what confines a driver's DMA to the buffers the broker gave it. Every image is built with `nonos-arch-iommu` and `nonos-iommu-enforce` as part of the core feature set, so the kernel programs the Intel VT-d remapping units the ACPI DMAR table describes and turns translation on (`Cargo.toml:240-249`, `Cargo.toml:867-877`). Two parts are off by default:

- Interrupt remapping, the `nonos-iommu-intremap` feature (`Cargo.toml:879-884`).
- The AMD-Vi backend, the `nonos-iommu-amdvi` feature (`Cargo.toml:886-891`).

The vendor is read from the ACPI tables, never from CPUID: `IommuVendor` is AMD-Vi when an IVRS table is present and DMAR describes no unit (`src/memory/iommu/vendor.rs:19-31`). In the default build, such a machine gets every domain call refused by `route` with `AmdViNotDriven` (`src/memory/iommu/backend_x86_64/route.rs:38-48`, `src/memory/iommu/backend_x86_64/refuse.rs:22-31`). The broker then lets each claim through unconfined, because `unconfined_allowed` treats that error as no unit in service (`src/hardware/broker/confine/posture.rs:32-49`). On such a machine, and on any machine without VT-d, a claimed device can reach all of memory. The broker says so in the log on each claim.

With VT-d in service, `attach` gives each [driver capsule](../overview/glossary.md#driver-capsule) an [IOMMU domain](../overview/glossary.md#iommu-domain) of its own and moves each claimed device into it. A device that no unit in service `translates` stays on physical addresses (`src/hardware/broker/confine/attach.rs:30-48`), and each DMA grant made for it is counted with `note_unconfined` until it is given back (`src/hardware/broker/dma/map/record.rs:45-51`). The boot log carries one posture line from `posture_line`, `[IOMMU] <vendor> present, enforcing=<0 or 1>, unconfined grants=<n>`, and device DMA is confined only when `enforcing=1` and the count is zero, as `report_posture` explains (`src/memory/iommu/posture.rs:17-38`, `src/memory/iommu/posture.rs:69-79`). Faults are polled from the timer tick with `poll_faults` (`src/interrupts/timer/clock.rs:54-55`).

[../kernel/iommu.md](../kernel/iommu.md) covers the tables, the queues and the fault reports.
