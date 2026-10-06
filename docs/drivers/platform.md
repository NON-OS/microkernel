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

## TPM 2.0

The kernel has a runtime TPM 2.0 driver in `src/security/tpm`, for quotes against a fresh nonce, attestation keys, enrolment, the machine key and the device secret. The bootloader has a separate one for measuring the boot chain, as the module comment says above `transact` (`src/security/tpm/mod.rs:17-37`).

`detect` decides which register file is behind the window at `TPM_MMIO_BASE`, 0xFED40000, from two sources (`src/security/tpm/transport/detect.rs:29-79`, `src/security/tpm/mmio/map.rs:25`):

- The ACPI TPM2 table's start method: 6 is `START_FIFO` (TIS), 7 and 8 are CRB (`src/security/tpm/transport/acpi.rs:22-28`). A table whose length or checksum is wrong is ignored by `table` (`src/security/tpm/transport/acpi.rs:56-74`).
- The part's own interface id register.

When the two disagree the TPM is refused, because the two register files overlap. A CRB control area outside the window, as an AMD firmware TPM has, is taken from the table. Start method 2, `START_ACPI`, is refused: its doorbell is an ACPI `_DSM` call and the kernel has no AML interpreter (`src/security/tpm/transport/detect.rs:35-61`). The driver uses locality 0, and `announce` logs one `[TPM] interface` line (`src/security/tpm/transport/detect.rs:81-92`).

Two host proof crates cover part of this driver. The key crate takes the kernel's FIFO (TIS) protocol in by `#[path]` and drives it against a modelled register file (`userland/tpm_key_proofs/src/security/tpm/fifo/mod.rs:17-19`). Both `tpm_key_proofs` and `tpm_enroll_proofs` run live tests against a software TPM, and the flake fails when a live test is skipped, through `needsTpm` (`tools/nix/checks.nix:32-34`). At this commit they pass with 42 and 37 tests. The CRB path and `detect` have no host test. What the TPM measures and seals is in [../security/measured-boot-and-tpm.md](../security/measured-boot-and-tpm.md).
