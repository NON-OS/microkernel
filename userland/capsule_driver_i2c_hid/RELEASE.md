# capsule_driver_i2c_hid: surface and release

What the capsule does today and what release still needs. The summary is in
[README.md](README.md).

## Current implemented surface

- Runtime service lookup for `driver.i2c_pci0`.
- Bounded HID descriptor reads through `OP_TRANSFER`.
- The address from `driver.i2c_pci0` (`OP_ACPI_HID`) and no other: no scan;
  given up after 30 s with nothing to drive.
- IPC health and descriptor export (unreachable at this commit).
- Power-on, reset and the zero-length reset report, each wait a sleep.
- The report descriptor walked by usb_hid's walker (shared as source), with
  Push and Pop, per-report offsets, usage ranges and four-byte usages; a
  device that is no touchpad or mouse released asleep.
- Input reads paced by the interrupt line where the controller senses it,
  else every 2 ms while reports come and 16 ms when quiet; a self-reset pad
  set up again and its click let go, read back or not.
- No persistent input history and no direct hardware access.

## Release target

The target chain is `driver.i2c_pci0 -> driver.i2c_hid0 -> kernel input ring ->
input_router -> compositor / apps`. Report reads and posting are in place; what
is left is a boot on real pads
([LIFECYCLE.md](LIFECYCLE.md#real-hardware-bring-up-checklist)).

## Release evidence

Release evidence requires signed spawn of both capsules, successful service
lookup, a bounded descriptor read from a real HID-over-I2C device, and a decoded
descriptor delivered to the input runtime without kernel-resident input policy.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm IPC-only authority and endpoint ownership.
- Kernel profile `microkernel-driver-i2c-hid` resolves signed artifacts.
- Descriptor probe succeeds on supported hardware.
- No report history is persisted.

## Explicit non-goals today

This slice does not drive an I2C keyboard (it is released), post more than a
touchscreen's first finger, keyboard layout mapping, touch filtering beyond the gesture decoder, or focus
routing. Suspend and resume (SET_POWER) are not here: they need a
system-sleep path, in the kernel or a power capsule, to call this driver.

## Verification

- Build: `make -B nonos-mk-driver-i2c-hid`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-i2c-hid`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Proofs: `(cd userland/i2c_hid_proofs && cargo test)`, a debug build.
- Handbook: [drivers](../../docs/handbook/drivers.md).
