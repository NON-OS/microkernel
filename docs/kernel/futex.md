# Futex

The two NONOS system calls that let the threads of a [capsule](../overview/glossary.md#capsule) sleep on a 32-bit word in their own memory and wake each other, what they promise, and how Linux programs get their `futex` instead.

## The two calls

| Call | Number | Arguments | Returns |
|---|---|---|---|
| `MkFutexWait` | `0x5754464D`, the tag `MFTW` | `vaddr`, `expected`, `timeout_ms` | 0, or a negative error |
| `MkFutexWake` | `0x4B54464D`, the tag `MFTK` | `vaddr`, `count` | how many waiters were woken |

The numbers are four-byte tags, `SYS_FUTEX_WAIT` and `SYS_FUTEX_WAKE` (`src/syscall/microkernel/numbers.rs:42-43`), dispatched to `sys_futex_wait` and `sys_futex_wake` (`src/syscall/microkernel/dispatch/process.rs:78-79`). Their entries in the ABI file are `desc.MFTK` and `desc.MFTW` (`abi/syscalls.toml:548-557`).

Both need only a valid [capability](../overview/glossary.md#capability) token: `MkFutexWait` and `MkFutexWake` sit with exit, yield and the clock calls that any process may make (`src/syscall/contract/cap_table/mk.rs:20-35`).

| Error | Value | When |
|---|---|---|
| `ERRNO_PERM` | -1 | no current process |
| `ERRNO_INVAL` | -22 | wait only: `vaddr` is zero or not 4-byte aligned |
| `ERRNO_FAULT` | -14 | wait only: the word cannot be read from user memory |

The values are in `src/syscall/microkernel/errnos.rs:22-35`, where `ERRNO_PERM` is first.
