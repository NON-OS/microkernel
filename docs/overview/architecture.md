# Architecture

The whole of NONOS in one diagram, then one section for each part, with links to the pages that cover it in depth.

## The system in one diagram

```mermaid
flowchart TB
    fw["UEFI firmware"] --> loader["NONOS loader"]
    loader --> kernel["Kernel in ring 0"]
    apps["Apps"] -->|syscalls| kernel
    services["System services"] -->|syscalls| kernel
    drivers["Driver capsules"] -->|syscalls| kernel
    linux["Linux personality"] -->|syscalls| kernel
    guests["Linux programs"] -->|Linux syscalls| kernel
    kernel -->|foreign frames| linux
    apps -->|IPC| services
    services -->|IPC| drivers
    kernel --> broker["Hardware broker"]
    broker --> iommu["IOMMU"]
    iommu --> devices["Devices"]
```

Read it from the top. UEFI firmware starts the NONOS loader, which starts the kernel after the checks its boot menu names. Apps, System services, Driver capsules and the [Linux personality](glossary.md#linux-personality) are [capsules](glossary.md#capsule) in ring 3: they reach the kernel only through syscalls, and each other only through IPC. Linux programs make Linux syscalls, which the kernel does not answer itself: it parks each one as a foreign frame for the Linux personality. Driver capsules reach Devices only through the Hardware broker, and a PCI device's DMA passes the IOMMU where a unit in service covers it.

## UEFI firmware and the NONOS loader

The loader in `nonos-bootloader/` is a UEFI application. Its menu has seven entries: Standard, Hardened, Safe Mode, Air-Gapped, Recovery, `Install NØNOS` and Shut down (`ENTRIES` in `nonos-bootloader/src/bootmenu/entries.rs:33-56`). Under every entry that boots, the menu names what the loader checks before the kernel runs: Ed25519, ML-DSA-65, STARK, rollback and RNG, with Secure Boot and TPM 2.0 added under Hardened (`STD` in `nonos-bootloader/src/bootmenu/entries.rs:40-59`). The loader's verification code is described on the security pages. The loader then hands the kernel a boot record (`BootHandoffV1` in `src/boot/handoff/types/handoff.rs:28`). [Boot chain and signatures](../security/boot-chain-and-signatures.md), [Rollback protection](../security/rollback-protection.md) and [Measured boot and the TPM](../security/measured-boot-and-tpm.md) cover the checks, [Boot handoff](../kernel/boot-handoff.md) covers the record, and [Boot modes](../install/boot-modes.md) covers the menu.

## Kernel in ring 0

The kernel is a Rust microkernel of 300,321 lines in 5,761 files under `src/` (see [Counting the tree](#counting-the-tree)). It keeps address spaces and page tables (`src/memory/`), the scheduler and the other CPUs (`src/sched/`, `src/smp/`), IPC (`src/ipc/`), capabilities (`src/capabilities/`), the syscall boundary (`src/syscall/`), process creation and capsule spawn (`src/kernel_core/process_spawn/`) and init, which starts the capsules (`src/userspace/init/`). In this release it also keeps the TPM driver (`src/security/tpm/`) and the encrypted data volume (`src/fs/`). For its own boot it keeps PCI enumeration and a virtio-rng entropy probe (`init_pci` and `init_virtio_rng` in `src/drivers/mod.rs:17-35`).

Every syscall number the kernel knows goes through `dispatch`, which resolves the caller's capability for that call and refuses with EPERM when it does not resolve (`src/syscall/contract/dispatch.rs:25-40`). A number it does not know is parked for the caller's supervisor, as for a Linux program, or else answered with ENOSYS (`syscall_handler` in `src/arch/x86_64/syscall/manager/entry.rs:24-53`). The kernel defines 130 syscalls (`SyscallNumber` in `src/syscall/numbers/defs.rs:19-150`) and 36 capability bits (`capability_table` in `src/capabilities/types/defs.rs:21-83`). Messages between capsules wait in kernel inboxes: all of them together hold at most 96 MiB, one inbox at most 16 MiB, and one sender at most half of any one inbox (`TOTAL_BYTES_MAX`, `INBOX_BYTES_MAX` and `SHARE_BYTES_MAX` in `src/ipc/nonos_inbox/budget.rs:41-43`).

[Kernel](../kernel/README.md) is the entry point. [Memory and paging](../kernel/memory-and-paging.md), [Scheduler and SMP](../kernel/scheduler-and-smp.md), [IPC](../kernel/ipc.md), [Capabilities](../kernel/capabilities.md), [Syscalls](../kernel/syscalls.md) and [Processes and spawn](../kernel/processes-and-spawn.md) go deeper.
