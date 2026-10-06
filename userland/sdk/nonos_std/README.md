# nonos-std

A `no_std` library shaped like `std`, built on `nonos_libc`. A crate is ported
to it by replacing `std` with `nonos_std` in its source. It re-exports `alloc`
and `core` modules and adds `collections` (on `hashbrown`), `env`, `fs`, `io`,
`net`, `path`, `process`, `sync` and `time`. It has no threads.

- `fs` talks to `vfs_pool`, found with `mk_service_lookup`.
- `io` standard output goes through `mk_debug`, so it needs the Debug capability.
- `net` follows the system's chosen network, read through `nonos_route_link`
  for each connection (`src/net/way.rs`). On Direct a stream is a direct
  `net.sockets` socket and a name is looked up with `net.dns`. On Nym it is a
  mixnet socket on `net.sockets`, which takes an address only and refuses a
  name. On Anyone it is one stream through `net.anon` at a time per app. A
  chosen network that is not running gives an error, not another network. Name
  lookups, UDP and listening sockets are refused unless Direct is chosen.

The only user is `userland/capsule_gui_proof`, which has no `Capsule.mk`. This
is a different library from `toolchain/nonos-std`, which patches real `std`.
See [the libc and std page](../../../docs/handbook/userland/libc-and-std.md).
