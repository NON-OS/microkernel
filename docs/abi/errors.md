# Errors

What a failed NONOS syscall returns, the codes the handlers use, and every errno name the kernel defines.

## How an error comes back

A call returns a signed 64-bit value in the return register. Zero or more is success. A negative value is an errno with its sign flipped: `SyscallResult::error` stores `-errno` (`src/syscall/types/result.rs:35-37`). The microkernel handlers return negative `i64` constants directly, as the header of their errno file says (`src/syscall/microkernel/errnos.rs:17-20`). So `EINVAL` arrives as -22. `abi/syscalls.toml` publishes `errno_negative` and an `errno_range` of -4095 to -1 (`abi/syscalls.toml:15-19`).

Two refusals come before any handler, and a third before most:

- An unknown number is `ENOSYS`, -38 (`src/arch/x86_64/syscall/manager/entry.rs:38-53`). On x86_64 a [foreign process](../overview/glossary.md#foreign-process) is the exception: its unknown number goes to its supervisor, which chooses the answer.
- A call the caller's [capabilities](../overview/glossary.md#capability) do not admit is `EPERM`, -1, from the contract `dispatch` (`src/syscall/contract/dispatch.rs:31-37`).
- A register too wide for the field it carries is `EINVAL`, -22, where the dispatch reads it with `u32_arg` or one of its siblings and refuses rather than truncates (`src/syscall/microkernel/narrow.rs:17-35`). The IPC calls are the exception: there `resolve_for_recv` and the send path treat an endpoint number too wide for a port as one that names no endpoint, and the call fails as for any unknown endpoint (`src/syscall/microkernel/ipc/inbox_name.rs:28-40`).

The names and numbers are those of Linux on x86_64. `posix.rs` says it holds the standard POSIX errnos 1 to 40 and the aliases `EWOULDBLOCK` and `EDEADLOCK` (`src/syscall/types/errnos/posix.rs:17`). Every value in the kernel's three errno files was compared with the errno table of a Linux x86_64 host and matches it.
