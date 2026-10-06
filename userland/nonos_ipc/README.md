# nonos-ipc

Slice wrappers over the IPC syscalls for the native runtime: `send`, `call`,
`recv` (with a timeout in milliseconds), `recv_from` (which also writes the
kernel-recorded sender pid) and `reply`. Each returns the kernel's `i64` result
unchanged. The kernel checks the caller's capabilities against the endpoint on
every send. See [the libc and std page](../../docs/handbook/userland/libc-and-std.md) and
[the IPC page](../../docs/handbook/kernel/ipc.md).
