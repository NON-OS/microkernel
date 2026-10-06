# capsule_tokio_smoke

Runtime gate for the NONOS tokio stack (mio backend and socket2 shim). It runs
a current-thread tokio runtime that races a timer against an idle socket
accept; the timer must keep firing while the accept is parked, which proves
the mio backend's waker self-wake and the time driver. It emits
`[TOKIO-SMOKE]` lines.

- The binary is built outside the capsule rules and taken as
  `CAPSULE_PREBUILT_BIN := target/upstream-tokio-smoke/tokio-smoke`.
- `CAPSULE_REQUIRED_CAPS = 0x1d`: CoreExec, Network, IPC and Memory. Debug
  (`0x100`) is optional and only a `capsule-serial-debug` build grants it; the
  ceiling is `0x11d`.
- Service `service:4504:tokio_smoke`, reply
  `reply:4505:endpoint.tokio_smoke.reply`.
- It never finishes on purpose, so it lives only in
  `microkernel-desktop-gui-async-gate`, the desktop plus this test, not in the
  desktop images. The terminal treats `tokio-smoke` as a store tool name, but
  the store disk (`NONOS_STORE_DEMO_ENTRIES` in `mk/40-run.mk`) does not carry it.

The std layer is described in
[docs/handbook/userland/libc-and-std.md](../../docs/handbook/userland/libc-and-std.md).
