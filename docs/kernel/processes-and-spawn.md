# Processes and capsule spawn

How the NONOS kernel turns a signed [capsule](../overview/glossary.md#capsule) into a running process: the gate it must pass, the ELF loader, the address space and stacks, and what happens when the process ends.

## Processes and address spaces

Every ring 3 program is a process with its own address space. `create_address_space` gives each one a fresh top level page table and copies the kernel half into it (`src/memory/paging/manager/address_space/create.rs:39-48`), and `allocate` records that table for the process (`src/process/address_space/lifecycle/setup.rs:26-49`). A thread shares its process's tables through `inherit` (`src/process/address_space/lifecycle/setup.rs:76-78`) and is held to its process's name and limits when it sends IPC, through `caller_group` (`src/services/registry/peers_check.rs:23-36`).

A process moves through the states of `ProcessState`: `New`, `Ready`, `Running`, `Sleeping`, `Stopped`, then `Zombie` and `Terminated` with an exit code (`src/process/core/types.rs:26-33`). [Memory and paging](memory-and-paging.md) and [Scheduler and SMP](scheduler-and-smp.md) cover the tables and the run queues.
