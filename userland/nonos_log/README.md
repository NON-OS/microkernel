# nonos-log

Console output for the native runtime. `log`, `stdout` and `stderr` all send the
line, cut to 256 bytes, with `MDBG` (MkDebug), and return -22 for an empty
line. The kernel admits `MDBG` only for a capsule holding the Debug capability,
and a kernel built without the `capsule-serial-debug` feature grants Debug to
no capsule, so on a hardened image these calls print nothing. See
[the libc and std page](../../docs/handbook/userland/libc-and-std.md).
