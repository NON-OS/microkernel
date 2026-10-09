# i2c_proofs_libc_shim

A host stand-in for `nonos_libc`, used by the I2C proof crates. The package is
`i2c_proofs_libc_shim` and the library is named `nonos_libc`, so the
`capsule_driver_i2c_pci` and `capsule_driver_i2c_hid` sources those crates
include by `#[path]` compile against it unchanged.

There is one shim for both drivers because they talk over IPC: the HID side's
call is answered by the controller side's handler in the same test, and a
dependency graph can hold only one crate named `nonos_libc`.

## What it provides

Every function keeps its state in thread-locals on the calling thread.

- IPC call: `mk_service_lookup` resolves only `driver.i2c_pci0`, and only after
  a test has called `serve(handler)`. It returns `SERVICE_PORT` (7) and
  `SERVICE_PID` (9). `mk_ipc_call_timeout` on that port passes the request to
  the handler and returns what the handler returns. The timeout is ignored.
  The `Served` guard unregisters the handler when dropped.
- IPC reply: `mk_ipc_reply` keeps the last reply and its destination pid, and
  `take_reply` drains it.
- Input: `mk_input_event_post` records each `InputEvent`, and `take_events`
  drains them. The struct and the four `INPUT_KIND_*` constants it exports
  match `userland/libc/src/surface_registry/types.rs`.
- Timing and output: `mk_idle_ms` and `mk_yield` add to counters
  (`slept_ms`, `yields`) without sleeping or yielding. `mk_debug` records lines
  for `take_debug`.

Pointer arguments are turned into slices and values in `src/raw.rs`, on the
same contract the real ABI states: the pointer names `len` live, aligned bytes
for the length of the call. Nothing checks it.

## What it does not do

It is not a libc and does not touch a kernel. It covers only the calls the two
I2C drivers make. Any other `nonos_libc` symbol is absent, and a service name
other than `driver.i2c_pci0` fails lookup with -1.

## Users

`i2c_hid_proofs` and `i2c_transfer_proofs`, each as
`nonos_libc = { package = "i2c_proofs_libc_shim", path = "../i2c_proofs_libc_shim" }`.

## Tests

It has no tests of its own and no `Cargo.lock`, so `nix flake check` does not
build it as a check. It runs inside `proofs-i2c_hid_proofs` and
`proofs-i2c_transfer_proofs`. See [the proofs page](../../docs/handbook/verification/proofs.md)
and [Drivers](../../docs/handbook/drivers.md).
