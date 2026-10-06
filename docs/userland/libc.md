# libc and the Rust runtimes

A [capsule](../overview/glossary.md#capsule) reaches the kernel through one of three userland layers; each is described below with what it implements, what it leaves out and which capsules use it.

| Layer | Where | Used by |
|---|---|---|
| `nonos_libc` | `userland/libc/` | almost every capsule |
| the `std` platform layer | `toolchain/nonos-std/`, `toolchain/nonos-rt/` | 17 capsules written against `std` |
| the native runtime and SDK | `userland/nonos_runtime/`, `userland/sdk/` | no capsule in this release |

## nonos_libc

Despite the name, `nonos_libc` is not a C library. It is a `no_std` Rust crate, package `nonos_userland_libc` with library name `nonos_libc`, built as an rlib (`userland/libc/Cargo.toml:12-28`, `nonos_libc`). It has no file descriptors, no `errno` variable and no POSIX calls. What it offers is the NONOS system call interface as plain functions, most of them `extern "C"` and named `mk_*`, for example `mk_ipc_call` (`userland/libc/src/ipc/call.rs:19-28`).

Of the 97 capsules the build includes, 84 name it in their `Cargo.toml`. Of the other thirteen, ten are built from upstream Rust source on the `std` layer and have no `Cargo.toml` in their capsule directory. The last three are `capsule_egui_proof`, which reaches the crate through `app_skeleton`, `capsule_std_proof`, a plain `std` program, and `capsule_nonos_install`, written in assembly with an empty `[dependencies]` table (`userland/capsule_nonos_install/Cargo.toml:18-23`, `nonos_install`).

### How a call reaches the kernel

On x86_64 a naked function moves the call number into `rax` and the six arguments into `rdi`, `rsi`, `rdx`, `r10`, `r8` and `r9`, then executes `syscall` (`userland/libc/src/syscall/raw/x86_asm.rs:17-37`, `raw_syscall_asm`). On aarch64 the number goes in `x8`, the arguments in `x0` to `x5`, and the call is `svc #0` (`userland/libc/src/syscall/raw/aarch64.rs:33-61`, `raw`). Any other architecture gets a stub that returns -38, ENOSYS, without trapping (`userland/libc/src/syscall/raw/fallback.rs:17-27`, `raw`).

Call numbers are four-character tags packed into an integer, so `MICL` is the IPC call (`userland/libc/src/syscall/numbers/ipc.rs:18-25`, `N_MK_IPC_CALL`). A negative return is a negative errno. The numbers and their capability gates are in [ABI: system calls](../abi/syscalls.md).

### What it covers

The crate root re-exports one module per area (`userland/libc/src/lib.rs:64-137`, `mk_ipc_call`):

| Area | Functions |
|---|---|
| IPC and services | `mk_ipc_send`, `mk_ipc_recv`, `mk_ipc_call`, `mk_ipc_call_timeout`, `mk_ipc_recv_from`, `mk_ipc_reply`, `mk_ipc_send_to_pid`, `mk_service_lookup`, `mk_service_register` |
| memory | `mk_mmap`, `mk_munmap`, and a heap (below) |
| processes | `mk_args`, `mk_getpid`, `mk_kill`, `mk_pid_alive`, `mk_wait`, `mk_exit`, `mk_yield` |
| threads | `mk_thread_spawn`, which starts a thread in the same process at an entry and a stack the caller provides |
| time | `mk_time_millis`, `mk_time_rtc`, `mk_uptime_ms`, `mk_time_adjust` |
| cryptography | `crypto_random`, `crypto_hash`, `crypto_encrypt`, `crypto_decrypt`, HKDF, HMAC, Keccak-256, X25519 |
| hardware broker | device claim and release, MMIO, IRQ, DMA and port I/O for driver capsules |
| graphics and input | surface register, attach, share and present, input events |
| capsules and apps | `mk_capsule_load`, `mk_capsule_verify`, `mk_app_install`, `mk_app_launch`, `mk_spawn_instance`, `mk_tool_run` |
| storage | `mk_store_read`, `mk_store_write`, the data volume calls |
| Linux guests | the `mk_foreign_*` calls the [Linux personality](../overview/glossary.md#linux-personality) uses |
| logging | `mk_debug`, one line of at most 256 bytes on the serial log, refused to a capsule without Debug |
| other | reboot and shutdown, attestation queries, capability checks, process statistics, the terminal's standard streams |

The 256-byte bound is `MAX_LEN` in the kernel's handler (`src/syscall/microkernel/debug.rs:37-45`, `MAX_LEN`). Having a function does not mean a capsule may call it. Each call has a gate in the kernel, most of them a capability, and a capsule that fails it gets an error back; see [Manifests and capabilities](manifests-and-capabilities.md).

### Heap and panics

With the default `heap` feature the crate registers a global allocator over `linked_list_allocator` (`userland/libc/Cargo.toml:35-37`, `heap`). The allocator zeroes every block as it is freed, with volatile writes the compiler cannot remove, so copies of secrets do not outlive their drop (`userland/libc/src/heap/zero_on_free.rs:62-77`, `dealloc`). `heap_init` maps 16 MiB (`userland/libc/src/heap/init.rs:22-37`, `INITIAL_HEAP_SIZE`). A capsule that needs more calls `heap_init_sized` first; a region over 1 GiB is mapped in 1 GiB pieces, because the kernel maps at most that much per call (`userland/libc/src/heap/span.rs:26-27`, `PIECE`; `src/syscall/microkernel/memory/consts.rs:19`, `MAX_MMAP_SIZE`).

With the default `panic-handler` feature, a panic writes one `[PANIC]` line with the location and message through `mk_debug` and exits with status 134 (`userland/libc/src/panic.rs:41-47`, `mk_exit`). A capsule without Debug leaves only the exit status. A `std` capsule that links the crate turns both features off and keeps the `std` allocator and panic runtime, as `capsule_mdview` does in its `nonos_libc` dependency (`userland/capsule_mdview/Cargo.toml:21`).

### What it does not do

- No C header. The build does produce a static archive, `libnonos_libc.a` (`mk/20-build.mk:186`, `USERLAND_LIBC`), but no header file ships beside it and no capsule in this release is written in C.
- No file API. Files are reached through the `vfs_pool` service over IPC; see [IPC services](ipc-services.md).
- No signals, `fork` or `exec` for the calling capsule. The `mk_foreign_*` calls act on Linux guests, not on the caller, and a capsule that wants another program started asks the kernel through a launcher call such as `mk_app_launch` or `mk_tool_run`.
- On riscv64 every call returns ENOSYS, through the stub above.
