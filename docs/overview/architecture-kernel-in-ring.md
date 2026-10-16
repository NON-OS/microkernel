# Architecture: Kernel in ring 0

Part of [Architecture](architecture.md).

## Kernel in ring 0

The kernel is a Rust microkernel of 300,321 lines in 5,761 Rust files under
`src/` (see [Counting the tree](architecture-counting-the-tree.md#counting-the-tree)). It runs on every core: it
starts every CPU the firmware enables, up to 256, and every one of them takes
processes from one run queue shared by the whole machine. That holds for every
image built from the shipped `nonos.toml`, which sets `smp` to true
(`nonos.toml:20`); [Scheduler and SMP](../kernel/scheduler-and-smp.md) says what
a build without it does. It keeps address spaces and page tables
(`src/memory/`), the scheduler and the other CPUs (`src/process/scheduler/`,
`src/smp/`), IPC (`src/ipc/`), capabilities (`src/capabilities/`), the syscall
boundary (`src/syscall/`), process creation and capsule spawn
(`src/kernel_core/process_spawn/`) and init, which starts the capsules
(`src/userspace/init/`). In this release it also keeps the TPM driver
(`src/security/tpm/`) and the encrypted [data volume](glossary-claim-epoch.md#data-volume)
(`src/fs/`). For its own boot it keeps PCI enumeration (`init_pci` in
`src/drivers/mod.rs:17-32`), and it drives no RNG device.

Every syscall number the kernel knows goes through `dispatch`, which resolves
the caller's capability for that call and refuses with EPERM when it does not
resolve (`src/syscall/contract/dispatch.rs:25-40`). A number it does not know is
parked for the caller's supervisor, as for a Linux program, or else answered
with ENOSYS (`syscall_handler` in
`src/arch/x86_64/syscall/manager/entry.rs:24-53`). The kernel defines 130
syscalls (`SyscallNumber` in `src/syscall/numbers/defs.rs:19-150`) and 36
capability bits (`capability_table` in `src/capabilities/types/defs.rs:21-83`).
Messages between capsules wait in kernel [inboxes](glossary-hardware-broker.md#inbox): all of
them together hold at most 96 MiB, one inbox at most 16 MiB, and one sender at
most half of any one inbox (`TOTAL_BYTES_MAX`, `INBOX_BYTES_MAX` and
`SHARE_BYTES_MAX` in `src/ipc/nonos_inbox/budget.rs:41-43`).

[Kernel](../kernel/README.md) is the entry point. [Memory and
paging](../kernel/memory-and-paging.md), [Scheduler and
SMP](../kernel/scheduler-and-smp.md), [IPC](../kernel/ipc.md),
[Capabilities](../kernel/capabilities.md), [Syscalls](../kernel/syscalls.md) and
[Processes and spawn](../kernel/processes-and-spawn.md) go deeper.
