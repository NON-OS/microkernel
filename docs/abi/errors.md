# Errors

What a failed NONOS syscall returns, the codes the handlers use, and every errno name the kernel defines.

## How an error comes back

A call returns a signed 64-bit value in the return register. Zero or more is success. A negative value is an errno with its sign flipped: `SyscallResult::error` stores `-errno` (`src/syscall/types/result.rs:35-37`). The microkernel handlers return negative `i64` constants directly, as the header of their errno file says (`src/syscall/microkernel/errnos.rs:17-20`). So `EINVAL` arrives as -22. `abi/syscalls.toml` publishes `errno_negative` and an `errno_range` of -4095 to -1 (`abi/syscalls.toml:15-19`).

Two refusals come before any handler, and a third before most:

- An unknown number is `ENOSYS`, -38 (`src/arch/x86_64/syscall/manager/entry.rs:38-53`). On x86_64 a [foreign process](../overview/glossary.md#foreign-process) is the exception: its unknown number goes to its supervisor, which chooses the answer.
- A call the caller's [capabilities](../overview/glossary.md#capability) do not admit is `EPERM`, -1, from the contract `dispatch` (`src/syscall/contract/dispatch.rs:31-37`).
- A register too wide for the field it carries is `EINVAL`, -22, where the dispatch reads it with `u32_arg` or one of its siblings and refuses rather than truncates (`src/syscall/microkernel/narrow.rs:17-35`). The IPC calls are the exception: there `resolve_for_recv` and the send path treat an endpoint number too wide for a port as one that names no endpoint, and the call fails as for any unknown endpoint (`src/syscall/microkernel/ipc/inbox_name.rs:28-40`).

The names and numbers are those of Linux on x86_64. `posix.rs` says it holds the standard POSIX errnos 1 to 40 and the aliases `EWOULDBLOCK` and `EDEADLOCK` (`src/syscall/types/errnos/posix.rs:17`). Every value in the kernel's three errno files was compared with the errno table of a Linux x86_64 host and matches it.

## The codes handlers return

The microkernel handlers return these 24 constants, `ERRNO_PERM` to `ERRNO_STALE` (`src/syscall/microkernel/errnos.rs:22-54`). Published says whether `abi/syscalls.toml` lists the code in its `[errors]` table, which runs from `EPERM` to `ENOTEMPTY`; 10 of its 25 entries are not in this set (`abi/syscalls.toml:21-46`). Where the source gives a reason beside a constant, the meaning here is that reason.

| Constant | Name | Value | Published | Meaning |
|---|---|---|---|---|
| `ERRNO_PERM` | `EPERM` | -1 | yes | The caller lacks a capability, does not own the object, or is not its parent. A refusal by the capability check is the same value. |
| `ERRNO_NOENT` | `ENOENT` | -2 | yes | No such [inbox](../overview/glossary.md#inbox), service, file or record. |
| `ERRNO_IO` | `EIO` | -5 | yes | The volume or the disk under it could not do what was asked. |
| `ERRNO_CHILD` | `ECHILD` | -10 | yes | `MkWait` named a process that is not the caller's child. |
| `ERRNO_AGAIN` | `EAGAIN` | -11 | yes | No disk is chosen yet, because the USB driver is still looking, or a supervisor already hosts as many guests as it may; ask again. |
| `ERRNO_NOMEM` | `ENOMEM` | -12 | yes | Out of memory, address space, slots or table room. |
| `ERRNO_ACCES` | `EACCES` | -13 | yes | Access refused, for example a receive on an [endpoint](../overview/glossary.md#endpoint) another process owns. |
| `ERRNO_FAULT` | `EFAULT` | -14 | yes | A user pointer is not mapped, or not writable where the kernel must write; from the store calls, a device fault. |
| `ERRNO_BUSY` | `EBUSY` | -16 | yes | Already claimed or registered, or the target inbox is full. |
| `ERRNO_EXIST` | `EEXIST` | -17 | yes | The endpoint or volume to create already exists. |
| `ERRNO_NODEV` | `ENODEV` | -19 | yes | No such device, or the device or service behind the call is gone. |
| `ERRNO_INVAL` | `EINVAL` | -22 | yes | An argument is out of range, or wider than its field. |
| `ERRNO_NOTTY` | `ENOTTY` | -25 | yes | `MkTtyQuery` asked about a stream that is not on a terminal. |
| `ERRNO_FBIG` | `EFBIG` | -27 | no | A stream fed more bytes than the length named at its start. |
| `ERRNO_NOSPC` | `ENOSPC` | -28 | no | The volume has no room for the bytes still to come. |
| `ERRNO_RANGE` | `ERANGE` | -34 | yes | A DMA map whose device address a 32-bit descriptor cannot carry. |
| `ERRNO_NOSYS` | `ENOSYS` | -38 | yes | Not available here: port I/O and the foreign-process calls on aarch64 and riscv64. |
| `ERRNO_BADMSG` | `EBADMSG` | -74 | no | The file to import is not the one its digest pins. |
| `ERRNO_NOTSUP` | `EOPNOTSUPP` | -95 | no | Unsupported flags or mode, or a battery charge the kernel cannot read. |
| `ERRNO_NETDOWN` | `ENETDOWN` | -100 | no | A network tool was asked for, and the [boot profile](../overview/glossary.md#boot-profile) runs no network: Air-Gapped, Safe Mode or Recovery. |
| `ERRNO_TIMEDOUT` | `ETIMEDOUT` | -110 | no | A receive, call or wait ran out its timeout, or a block device did not answer. |
| `ERRNO_ALREADY` | `EALREADY` | -114 | no | The file was imported and verified before; nothing to feed. |
| `ERRNO_INPROGRESS` | `EINPROGRESS` | -115 | no | A stream finished before every byte came; what came is kept. |
| `ERRNO_STALE` | `ESTALE` | -116 | no | The [claim epoch](../overview/glossary.md#claim-epoch) is not the current one: the device was released and claimed again. |

Some calls map their own failures onto these codes:

- `store_errno` gives `MSWR` and `MSRR` one table: no disk is `ENODEV`, a refused caller `EACCES`, a driver not ready `ETIMEDOUT`, a request outside the disk `EINVAL`, and the rest `EFAULT` (`src/syscall/microkernel/store_errno.rs:29-41`). `kernel_proofs` mounts `store_errno` on the host (`userland/kernel_proofs/src/store_errno/mod.rs:24`); its 388 host tests passed in the flake check run on this commit.
- The [broker](../overview/glossary.md#broker) calls map a missing claim to `EPERM`, a stale claim epoch to `ESTALE`, an unknown device to `ENODEV`, and unsupported flags to `EOPNOTSUPP`; `errno_for` is the DMA map (`src/syscall/microkernel/dma.rs:99-112`). See [Broker](broker.md).
- `map_capsule_error` turns a failure of the crypto [capsule](../overview/glossary.md#capsule) into `EACCES`, `EINVAL`, `EBADMSG` for a failed authentication, `EIO`, `EMSGSIZE`, `EPROTO`, `ENODEV` or `ESTALE` (`src/syscall/dispatch/crypto/error.rs:26-43`).
- `sys_time_rtc` and `sys_time_millis` return -61, `ENODATA`, written as a bare number, while the kernel has no wall-clock time yet (`src/syscall/microkernel/time.rs:33-36`, `src/syscall/microkernel/time.rs:55-58`).
- The surface and input calls keep their own copies of a few values, among them `ESRCH`, 3, when there is no calling process (`src/syscall/dispatch/router/surface_ops.rs:24-29`).
