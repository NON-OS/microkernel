# nonos-service

Service registry calls for the native runtime. `lookup` returns the port a name
is registered on and `owner` the pid that registered it; either is `None` when
the call fails or the port or pid is zero. A receiver compares `owner` with the
kernel-recorded sender of a message before trusting it. `register` registers a
name on a port and returns whether the kernel accepted it. See
[the libc and std page](../../docs/handbook/userland/libc-and-std.md).
