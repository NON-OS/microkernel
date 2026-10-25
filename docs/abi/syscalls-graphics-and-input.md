# Syscalls: Graphics and input structures

Part of [Syscalls](syscalls.md).

## Graphics and input structures

`MSRG` reads a `SurfaceDescriptor` from the caller, and `MSAT` writes one back
(`src/syscall/dispatch/router/surface_handlers/register.rs:35-38`,
`do_attach` in `src/syscall/dispatch/router/surface_handlers/share.rs`). `do_register`
refuses a `byte_len` of 0 or over `MAX_SURFACE_BYTES`, 64 MiB, and a `base_va`
that is not page aligned
(`src/syscall/dispatch/router/surface_handlers/frames.rs:34-37`). `MIEP` reads one
`InputEvent` (`src/syscall/dispatch/router/input_ops/do_post.rs:23-27`). Both
layouts match `abi/wire.toml`.

### `SurfaceDescriptor`, 48 bytes

| Field | Offset | Type | Meaning |
|---|---|---|---|
| `width` | 0 | `u32` | Width in pixels |
| `height` | 4 | `u32` | Height in pixels |
| `stride` | 8 | `u32` | Row stride |
| `format` | 12 | `u32` | Pixel format; `FMT_ARGB8888`, 1, is the only one |
| `byte_len` | 16 | `u64` | Bytes of backing memory |
| `base_va` | 24 | `u64` | Page-aligned start of the caller's own writable memory |
| `flags` | 32 | `u64` | Flags word |
| `owner_pid` | 40 | `u32` | Written by `MSAT`: the process that registered the surface (`descriptor` in `src/kernel_core/surface_registry/share/descriptor.rs`) |
| `writable` | 44 | `u32` | Written by `MSAT`: 1 when the caller's view is writable, 0 when read-only (`attach_surface` in `src/kernel_core/surface_registry/share/attach_surface.rs`) |

### `InputEvent`, 32 bytes

| Field | Offset | Type | Meaning |
|---|---|---|---|
| `kind` | 0 | `u16` | 0 key down, 1 key up, 2 relative pointer, 3 absolute pointer, 4 wheel, 5 button down, 6 button up, 7 touch |
| `flags` | 2 | `u16` | Bits 0 to 7, the modifiers the poster set; bits 8 to 13, the source the kernel writes over the poster's; bits 14 and 15, a Bluetooth bond's marks the Bluetooth capsule sets; input_router clears bits 8 to 15 before an app sees the event |
| `code` | 4 | `u32` | Key or button code |
| `x` | 8 | `i32` | Position |
| `y` | 12 | `i32` | Position |
| `delta_x` | 16 | `i32` | Motion or wheel delta |
| `delta_y` | 20 | `i32` | Motion or wheel delta |
| `timestamp_ns` | 24 | `u64` | Time of the event in nanoseconds |

`FMT_ARGB8888` is defined in `src/kernel_core/surface_registry/types.rs`, and
the kinds are the `INPUT_KIND_KEY_DOWN` to `INPUT_KIND_TOUCH` constants of
`nonos_abi` (`userland/nonos_abi/src/input.rs:20-27`).

## Limits

- Arguments are not listed here. `abi/syscalls.toml` lists them, and several
  lists are wrong; see [The NONOS
  ABI](abi-stability-in-092.md#stability-in-092). The broker and IPC arguments
  in [Broker](broker.md) and [IPC](ipc.md) are read from the handlers.
- When no microkernel group handles a number, `route_tail` returns -1, which
  reads as `EPERM` (`src/syscall/microkernel/dispatch/route.rs:52-68`). The
  router's own default, `util::errno` with 38, is `ENOSYS`
  (`src/syscall/dispatch/router/dispatch_fn.rs:58`). Neither is reached by a
  number in `REGISTRY` at this commit; `scripts/check_syscall_abi.py` checks
  that every published call reaches a handler.
- `AbiStatus::Unavailable` is declared and never used: every entry is routed
  (`src/syscall/abi/status.rs:17-21`).
- `handle_syscall` in `src/syscall/entry.rs` is a second entry point that no
  architecture calls (`src/syscall/entry.rs:25-35`).

## See also

- [The NONOS ABI](README.md)
- [Capabilities](capabilities.md)
- [Errors](errors.md)
- [Broker](broker.md)
- [IPC](ipc.md)
- [Kernel syscalls](../kernel/syscalls.md)
- [Processes and capsule spawn](../kernel/processes-and-spawn.md)
- [The Linux personality](../userland/linux-personality.md)
