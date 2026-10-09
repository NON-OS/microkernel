# nonos-abi

The bottom of the native runtime: a naked `syscall` trampoline (`raw_syscall`)
and the syscall number tags the runtime crates use. `syscall` takes a number and
six arguments; `syscall_diverging` is the form for calls that do not return. On
an architecture other than x86_64 the trampoline returns -38 (ENOSYS).

It also carries `mmap`, `munmap` and `input_drain` with the `InputEvent` layout,
and the tags `MISD`, `MIRC`, `MICL`, `MIRF`, `MIRY`, `MISP`, `MSVL`, `MSVR`,
`MMAP`, `MUMP`, `MEXT`, `MYLD`, `MTMS`, `CRND`, `MDBG`, `GDIM`, the surface calls
and `MIED`. It does not depend on `nonos_libc`.

Used by `nonos_alloc`, `nonos_ipc`, `nonos_service`, `nonos_log`, `nonos_panic`,
`nonos_surface`, `nonos_runtime` and several SDK crates. No capsule in the
build depends on it at this time. See [the libc and std page](../../docs/handbook/userland/libc-and-std.md).
