# nonos-cap

The capability bits as `CAP_*` constants, from `CAP_CORE_EXEC` (bit 0) to
`CAP_DEVICE_SECRET` (bit 35), mirroring the kernel's `Capability` enum, plus a
`CapSet` and a process-local record of the word the program says it has:
`set_granted` stores it, `query` reads it, `request` tests a bit.

The record changes nothing in the kernel. The kernel enforces the word it
installed at spawn from the signed manifest. See [the libc and std page](../../docs/handbook/userland/libc-and-std.md)
and [the capabilities page](../../docs/handbook/kernel/capabilities.md).
