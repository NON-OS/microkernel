# nonos-alloc

The native runtime's heap. `init` maps 16 MiB with `mmap`, refuses a result that
is null, negative or above the user address limit, and hands the region to a
`linked_list_allocator` wrapped in `ZeroOnFree`, which zeroes each block with
volatile stores before it goes back on the free list. That wrapper is the same
code as `nonos_libc`'s, kept identical on purpose; `mechanism_proofs` tests the
libc copy.

`init` runs once; a second call returns `AllocError::AlreadyInitialized`. The
heap does not grow. `nonos_runtime::boot` calls `init`. See
[the libc and std page](../../docs/handbook/userland/libc-and-std.md).
