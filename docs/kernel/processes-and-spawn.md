# Processes and capsule spawn

How the NONOS kernel turns a signed [capsule](../overview/glossary.md#capsule) into a running process: the gate it must pass, the ELF loader, the address space and stacks, and what happens when the process ends.

## Processes and address spaces

Every ring 3 program is a process with its own address space. `create_address_space` gives each one a fresh top level page table and copies the kernel half into it (`src/memory/paging/manager/address_space/create.rs:39-48`), and `allocate` records that table for the process (`src/process/address_space/lifecycle/setup.rs:26-49`). A thread shares its process's tables through `inherit` (`src/process/address_space/lifecycle/setup.rs:76-78`) and is held to its process's name and limits when it sends IPC, through `caller_group` (`src/services/registry/peers_check.rs:23-36`).

A process moves through the states of `ProcessState`: `New`, `Ready`, `Running`, `Sleeping`, `Stopped`, then `Zombie` and `Terminated` with an exit code (`src/process/core/types.rs:26-33`). [Memory and paging](memory-and-paging.md) and [Scheduler and SMP](scheduler-and-smp.md) cover the tables and the run queues.

## Where capsules come from

- Built into the image. The kernel embeds a capsule's ELF, identity certificate, [manifest](../overview/glossary.md#manifest) and [attestation trailer](../overview/glossary.md#attestation-trailer) with `include_bytes!`, as `HELLO_ELF` and its neighbours do (`src/userspace/capsule_hello/embed.rs:17-35`). A spawn site gives the name, the service and reply ports and the [capabilities](../overview/glossary.md#capability) it is willing to grant, as `spawn_hello_capsule` does for `app.hello` on port 4810 with reply port 4811 (`src/userspace/capsule_hello/spawn.rs:28-58`). Its `HELLO_ELF` is embedded only in a kernel built with the `nonos-capsule-hello` feature (`src/userspace/capsule_hello/embed.rs:17-23`).
- Installed later. The installer reads the same four artifacts from the store and passes them to `MkCapsuleLoad` by pointer; `load_capsule_from_vfs` takes the service name and both [endpoints](../overview/glossary.md#endpoint) from the signed manifest and limits the requested bits to what the manifest declares (`src/kernel_core/process_spawn/capsule_spawn/from_vfs/load/spawn.rs:34-68`). The call needs `CoreExec`, `IPC` and `Memory` (`src/syscall/contract/cap_table/mk.rs:85-88`).
- On behalf of another process. A caller holding `SpawnBroker` may name a live pid as the new capsule's parent; `attest` is the only way to build that attribution (`src/kernel_core/process_spawn/capsule_spawn/attested_parent.rs:36-46`).
- Linux guests. `MkForeignSpawn` creates a process for the [Linux personality](../overview/glossary.md#linux-personality) that holds no capabilities at all: `install_spawn` gives it an empty word (`src/process/foreign/spawn.rs:65-68`); its system calls go to the supervising capsule. [Linux personality](../userland/linux-personality.md) covers it.
