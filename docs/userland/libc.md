# libc and the Rust runtimes

A [capsule](../overview/glossary.md#capsule) reaches the kernel through one of three userland layers; each is described below with what it implements, what it leaves out and which capsules use it.

| Layer | Where | Used by |
|---|---|---|
| `nonos_libc` | `userland/libc/` | almost every capsule |
| the `std` platform layer | `toolchain/nonos-std/`, `toolchain/nonos-rt/` | 17 capsules written against `std` |
| the native runtime and SDK | `userland/nonos_runtime/`, `userland/sdk/` | no capsule in this release |
