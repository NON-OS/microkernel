# capsule_wayland_proofs

The Linux personality's Wayland server (`userland/capsule_linux/src/linux/wayland/`), the
whole of it, compiled on the host through `#[path]` at the path it has in the capsule,
`crate::linux::wayland`. Nothing in it is copied or changed: every request is decoded,
dispatched and answered by the shipped files.

What the server reaches outside itself is a host double:

- `libc_shim/` stands in for `nonos_libc`: debug lines kept, surfaces registered with their
  pixels' address, the inbox fed by the host instead of the input router.
- `skeleton_shim/` stands in for `nonos_app_skeleton`: one desktop per thread (a display of a
  set size, a window manager, a compositor, a clipboard) that keeps every call, with the
  shipped chrome rule and input event mounted as they are.
- `src/linux/` stands in for the rest of the personality: the guest (its descriptor table
  and the memory its pools are read from), the clocks, the console. A pool is the client's
  descriptor mapped into this process, so the server reads the pixels the client draws.

## The host server

`nonos-wayland-host <socket> [control-socket]` serves real clients on a Unix socket, one
guest each. The control socket takes a command a line (`src/host/control.rs`): typed text,
keys with modifiers, pointer motion, clicks and wheel, the display's size, and `dump` of the
window's pixels as a PPM. With `NONOS_WAYLAND_RECORD=<path>` each client's session is
recorded to `<path>.<n>`: every read the server took, with the length of each descriptor it
carried, and every input frame delivered.

## Tests

`tests/replay.rs` plays recorded sessions of real clients (`tests/sessions/`) back into a
fresh server and holds each to: no request unserved, no protocol error, and what the
desktop was asked (the window opened, full screen and back). A descriptor in a record is
played back as a zeroed memfd of its length.

    cargo +nightly-2026-01-16 test
