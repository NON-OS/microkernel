# Processes and capsule spawn

How the NONOS kernel turns a signed [capsule](../overview/glossary-boot-root-record.md#capsule) into a running process: the gate it must pass, the ELF loader, the address space and stacks, and what happens when the process ends.

## Parts

- [Processes and where capsules come from](processes-and-spawn-sources.md)
- [The spawn gate](processes-and-spawn-gate.md)
- [Install, the ELF loader and stacks](processes-and-spawn-install.md)
- [Exit and reaping](processes-and-spawn-exit.md)

## Processes and address spaces

On [the processes and where capsules come from page](processes-and-spawn-sources.md#processes-and-address-spaces).

## Where capsules come from

On [the processes and where capsules come from page](processes-and-spawn-sources.md#where-capsules-come-from).

## The spawn gate

On [the spawn gate page](processes-and-spawn-gate.md#the-spawn-gate).

## Install

On [the install, the ELF loader and stacks page](processes-and-spawn-install.md#install).

## Loading the ELF

On [the install, the ELF loader and stacks page](processes-and-spawn-install.md#loading-the-elf).

## Stacks

On [the install, the ELF loader and stacks page](processes-and-spawn-install.md#stacks).

## Exit and reaping

On [the exit and reaping page](processes-and-spawn-exit.md#exit-and-reaping).

## See also

- [IPC](ipc.md): the inboxes and endpoints spawn creates and exit removes.
- [Capabilities](capabilities.md): the token `install_spawn` sets.
- [Memory and paging](memory-and-paging.md): address spaces and page tables.
- [Capsule model](../userland/README.md): what a capsule is from the author's side.
- [Manifests and capabilities](../userland/manifests-and-capabilities.md): what the manifest declares.
- [Capsule isolation](../security/capsule-isolation.md): what the spawn gate and the address space protect.
