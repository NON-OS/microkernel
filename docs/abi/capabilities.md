# Capabilities

The 36 capability bits of NONOS 0.9.2: what each one lets a [capsule](../overview/glossary.md#capsule) do, which syscalls it admits, and where else the kernel checks it.

## What a capability is

A [capability](../overview/glossary.md#capability) is one bit in a 64-bit mask. `capability_table!` is the one list of them, with the bit each occupies (`src/capabilities/types/defs.rs:21-83`), and `count` is derived from that list rather than written down (`src/capabilities/types/table.rs:39-43`). A capsule's [capability token](../overview/glossary.md#capability-token) carries the capabilities it holds as `permissions` (`src/capabilities/token/types/defs.rs:22-25`), and the syscall gates ask the token. The kernel installs a capsule's mask when it starts it, with `install_caps` (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:99-100`). On a [boot profile](../overview/glossary.md#boot-profile) that runs no network, `caps` removes `Network` from every capsule's mask (`src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs:48-55`).

`abi/caps.toml` publishes the same bits under upper-case names for toolchains. `scripts/check_caps_abi.py` fails when a published bit disagrees with the kernel, and it passes at this commit.
