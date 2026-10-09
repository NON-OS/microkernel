# nonos-runtime

A small native runtime beside `nonos_libc`, with no dependency on it. It ties
together `nonos_abi`, `nonos_alloc`, `nonos_ipc`, `nonos_service`, `nonos_cap`,
`nonos_log`, `nonos_panic` and `nonos_surface`, and re-exports them in
`prelude`.

- `nonos_main!(CAPS, entry)` emits `_start`, which calls `run`.
- `run` calls `boot`, then the entry function, then the cleanup hooks in reverse
  order of `on_cleanup`, then exits with 0. A failed `boot` exits with 1.
- `boot` initialises the heap and records the capability word with
  `nonos_cap::set_granted`. That record is what the program says it has; the
  kernel enforces the word from the signed manifest.
- `entropy`, `time_millis`, `yield_now` and `exit` are single syscalls.

No capsule in the build uses it at this time: its users are
`userland/nonos_examples` and the SDK in `userland/sdk`, and no `Capsule.mk`
builds either. See [the libc and std page](../../docs/handbook/userland/libc-and-std.md).
