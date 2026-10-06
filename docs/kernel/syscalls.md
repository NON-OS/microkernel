# System calls

How a system call travels from a [capsule](../overview/glossary.md#capsule) into the NONOS kernel and back: the numbers, the registers, the entry code, the dispatch and the [capability](../overview/glossary.md#capability) check, and how many calls there are.

## How many there are

NONOS 0.9.2 has 130 system calls. The count was taken three ways at this commit, and all three agree:

- `abi/syscalls.toml` lists 130 tags under `[numbers]`, starting at `APPS` (`abi/syscalls.toml:54-55`), and 130 `[desc.TAG]` blocks, one per call.
- The kernel enum `SyscallNumber` has 130 variants (`src/syscall/numbers/defs.rs:17-150`). Sorted, its tags are the same set as the ones in the toml file.
- The kernel's `REGISTRY` joins four tables: 114 microkernel, 12 crypto, 3 admin and 1 graphics entry (`src/syscall/abi/registry/mod.rs:24-28`).

`scripts/check_syscall_abi.py` goes further and checks that every published call reaches a handler:

```
$ python3 scripts/check_syscall_abi.py
syscall-abi: 130 published syscalls reach a handler
```

| Family | Calls | Tags | Routed to |
|---|---:|---|---|
| Microkernel | 114 | four letters starting with `M` | `microkernel_ops`, `surface_ops`, `input_ops` |
| Crypto | 12 | `CRND` to `CMKY` | the crypto router |
| Admin | 3 | `ARBT`, `ASDN`, `APPS` | the admin router |
| Graphics | 1 | `GDIM` | `graphics_backend` |

The full list, with each call's arguments and the capability it needs, is on [Syscall ABI](../abi/syscalls.md).

## Numbers are tags

A system call number is four ASCII letters packed into a `u64` by `tag4`, first letter in the lowest byte (`src/syscall/abi/tag.rs:17-22`). These are the [syscall tags](../overview/glossary.md#syscall-tag). `MkIpcSend` is `MISD`, which is `0x4453494D` (`abi/syscalls.toml:150`), and a memory dump of the number reads `MISD`.

`SyscallNumber::from_u64` calls `lookup_id`, which searches the registry (`src/syscall/abi/mod.rs:31-40`). A number that is not there gets `ENOSYS`, -38, unless the caller is a Linux guest, described below.
