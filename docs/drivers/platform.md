# Platform

What NONOS does with the processor cores, the IOMMU, the TPM, the ACPI buttons and the GPIO controllers under every driver, and what it does not do in this release.

## Processor cores

NONOS 0.9.2 targets x86_64 processors from Intel and AMD; AMD machines are not tested in this release, and the other targets are in [../architectures/README.md](../architectures/README.md). The image build adds the `nonos-smp` feature when `smp` is true, which is the default (`nonos.toml:19-20`, `tools/nix/config.nix:127-150`). With it, `start_secondary_cpus` starts the application processors at boot and logs `[SMP-PROOF] cpu_count=` with the number running (`src/kernel_core/init/start_secondary.rs:17-48`).

`start_aps` walks every secondary in `get_ap_list`, the list the platform reported, and stops at `MAX_CPUS` (256) (`src/smp/init/ap_start.rs:22-62`, `src/smp/topology/detection/state.rs:43-47`, `src/smp/constants.rs:17`). It logs how many answered and how many `cpus_online` reports, which can differ when an AP comes up late (`src/smp/init/ap_start.rs:65-83`). The same bring-up path serves Intel and AMD; a few other parts of the kernel read the vendor, for example `vendor_from_leaf0` for TSC calibration (`src/arch/x86_64/time/tsc/calibration/math.rs:28-39`).

Two details keep the cores in step:

- Where CPUID reports the MSR, each AP sets its `IA32_TSC_ADJUST` to the boot CPU's value in `align`, so every core reads the same uptime (`src/smp/ap/tsc_adjust.rs:17-24`, `src/smp/ap/tsc_adjust.rs:55-72`).
- On Intel hybrid parts each core reads its kind from CPUID leaf 0x1A, and `wake_pass` offers a woken task to an idle performance core before an efficiency core (`src/smp/topology/core_kind.rs:17-70`).

The scheduler side is in [../kernel/scheduler-and-smp.md](../kernel/scheduler-and-smp.md).
