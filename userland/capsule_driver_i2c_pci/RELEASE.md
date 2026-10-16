# capsule_driver_i2c_pci: surface and release

What the capsule does today, what release still needs, and how it is checked.
The summary is in [README.md](README.md).

## Current implemented surface

- Intel LPSS I2C PCI discovery across Skylake through Meteor Lake-era ids, and
  ACPI-only controllers from the broker's ACPI records.
- The touchpads in reach named to the HID driver in turn, the bound one first
  (`OP_ACPI_HID`).
- Brokered device claim and BAR0 MMIO map.
- DesignWare component-type read, enable/status/timing/FIFO register telemetry.
- Interrupt mask and clear during setup.
- Bounded master write/read transaction engine with timeout and TX-abort
  reporting.
- IPC health, controller info, register snapshot, timing info, probe, transfer,
  ACPI HID and GPIO doorbell operations.
- The reach: the devices a client may address, narrowed once the pad has
  answered.
- Static gates for broker-only access and endpoint ownership.

## Release target

The target chain is `driver.i2c_pci0 -> driver.i2c_hid0 -> kernel input ring ->
input_router -> compositor / apps`. The next runtime slice is IRQ-aware
completion above the same bounded transfer primitive, which brings `Irq` back
into the manifest.

## Release evidence

Release evidence requires signed capsule spawn on Intel LPSS hardware, confirmed
controller identity, stable register snapshot IPC, successful bounded write-read
against an I2C HID descriptor register, and delivery of a decoded HID descriptor
to the higher-level HID runtime without kernel-resident input policy.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm brokered MMIO authority and endpoint ownership.
- Kernel profile `microkernel-driver-i2c-pci` resolves signed artifacts.
- Controller identity and timing registers are readable on supported hardware.
- Bounded write-read transaction validation passes without storing input
  history.
- What only a boot on real silicon can confirm:
  [HARDWARE.md](HARDWARE.md).

## Explicit non-goals today

This slice does not implement touchpad gestures, interrupt-driven transfers,
DMA, SMBus, sensor fusion, or input focus.

## Verification

- Build: `make -B nonos-mk-driver-i2c-pci`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-i2c-pci`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Proofs: `userland/i2c_pci_proofs`, `userland/i2c_transfer_proofs`,
  `userland/i2c_hid_proofs`.
- Handbook: [drivers](../../docs/handbook/drivers.md).
