# nonos_userland_libc

The userland libc: the crate almost every capsule links. The package is
`nonos_userland_libc`, imported as `nonos_libc`. It is `no_std` and exposes the
kernel's syscall ABI as `extern "C"` functions named `mk_*`, plus the crypto
calls (`crypto_random`, `crypto_encrypt`, `crypto_hash` and the rest). 95
`Cargo.toml` files under `userland/` depend on it.

## What is in it

| Area | Modules |
|---|---|
| the trap | `syscall/raw`: a naked `syscall` trampoline on x86_64, a trap for aarch64, and a fallback that returns ENOSYS on anything else |
| syscall numbers | `syscall/numbers`: four ASCII bytes packed little-endian by `tag4`, so `MEXT` is exit |
| process, time, memory | `unistd`, `process`, `time`, `mem` (`mk_mmap`, `mk_munmap`) |
| IPC and services | `ipc`: `mk_ipc_send`, `mk_ipc_recv`, `mk_ipc_call`, `mk_ipc_reply`, `mk_service_lookup`, `mk_service_register` |
| devices | `broker`: claims, MMIO, IRQ, DMA, port I/O, PCI config |
| driver start | `bringup`: `start_driver` and `bring_up`, a bounded bring-up of 7 attempts with a doubling sleep capped at 3.2 s, then exit code `EXIT_ABSENT` (2) or `EXIT_GAVE_UP` (6) |
| surfaces and input | `graphics`, `surface_registry` |
| the Linux personality | `foreign*` |
| trust | `attest`, `capsule_verify`, `capsule_load`, `install_source`, `boot_slots`, `device_secret`, `enroll` |
| storage | `store`, `data` |

## Features

Both are on by default.

- `heap` registers a `#[global_allocator]`: a `linked_list_allocator` heap wrapped
  so that every freed block is zeroed with volatile stores before it is reused.
  `heap_init` maps 16 MiB once; `heap_init_sized` asks for another size before
  that. The heap does not grow.
- `panic-handler` formats the panic into a 240-byte line, sends it with
  `mk_debug` and exits with 134. The kernel drops the line for a capsule without
  the Debug capability.

A capsule built against real `std` turns both off (`default-features = false`).

## Building

The crate is an `rlib`. Each capsule's own build compiles it with `-Zbuild-std`
for `userland/x86_64-nonos-user.json` (`nonos-mk/capsule.mk`, and
`tools/nix/capsules.nix` under `nix build`).

See [the libc and std page](../../docs/handbook/userland/libc-and-std.md).
