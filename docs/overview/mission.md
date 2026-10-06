# Mission

What NONOS is for, who it serves, and what it does not try to be.

## What NONOS is

NONOS is an operating system for x86_64 computers. Its kernel is a capability microkernel written in Rust. Most of what a person would call the operating system runs above it in ring 3, each part as a [capsule](glossary.md#capsule): the drivers, the network stack, the desktop, the apps, and a [Linux personality](glossary.md#linux-personality) that runs Linux programs. Each capsule holds the capabilities its signed manifest grants and nothing more, and the kernel checks them on every syscall. The kernel itself keeps memory, scheduling, IPC, capabilities and capsule spawn, and in this release also the TPM driver and the encrypted data volume. [Architecture](architecture.md) shows how the parts fit together.
