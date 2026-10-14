# capsule_driver_i2c_hid: surface and release

What the capsule does today and what release still needs. The summary is in
[README.md](README.md).

## Current implemented surface

- Runtime service lookup for `driver.i2c_pci0`.
- Bounded HID descriptor reads through `OP_TRANSFER`.
- The address from `driver.i2c_pci0` (`OP_ACPI_HID`), then a probe list covering
  common ELAN, Synaptics, FocalTech, and alternate HID addresses, within the
  controller's reach.
- IPC health, reprobe, and descriptor export (unreachable at this commit).
- Power-on, reset and the zero-length reset report, each wait a sleep.
- Report descriptor parse for the touch report, input report reads, and relative
  and absolute decodes posted to the kernel input ring.
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

This slice does not implement an I2C keyboard, keyboard layout mapping, touch
filtering beyond the gesture decoder, focus routing, or power management beyond
the wake and reset at start.

## Verification

- Build: `make -B nonos-mk-driver-i2c-hid`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-i2c-hid`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Proofs: `(cd userland/i2c_hid_proofs && cargo test --release)`.
- Handbook: [drivers](../../docs/handbook/drivers.md).
