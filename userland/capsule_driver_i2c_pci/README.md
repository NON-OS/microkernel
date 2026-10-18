# capsule_driver_i2c_pci

## Role
The Intel LPSS DesignWare I2C controller capsule behind `driver.i2c_pci0`:
discovery, BAR0 mapping, identity, clocks, register telemetry and the bounded
transfer engine. HID parsing, touchpad, ACPI and input policy stay above it.

```text
driver.i2c_hid0 --> driver.i2c_pci0 -- brokered MMIO --> LPSS I2C controller
```

## Microkernel contract
`CAPSULE_REQUIRED_CAPS = 0x38018`: IPC, Memory, Driver, DeviceEnum, Mmio, no
Irq. Hardware only through `MkDeviceList`, `MkDeviceClaim`, `MkPciConfigWrite`
and `MkMmioMap`; its one client and that client's reach:
[CONTRACT.md](CONTRACT.md).

## Interface contract
Eight operations; `OP_TRANSFER` and `OP_PROBE` reach only the devices in reach,
which narrows to the pad alone once a client has read its HID descriptor
([PROTOCOL.md](PROTOCOL.md)). Unknown operations reply `E_BAD_OP`.

## Authority
PCI enumeration, one claimed LPSS I2C function, its BAR0; no DMA, IRQ, PIO or
input authority, bus mastering off ([CONTRACT.md](CONTRACT.md#authority)).

## Privacy and persistence
No touch, report, gesture, sensor reading or ACPI table is stored
([CONTRACT.md](CONTRACT.md#privacy-and-persistence)).

## Runtime lifecycle
Bring up the controllers found, keep the touchpad's, release the rest, serve
([SETUP.md](SETUP.md#runtime-lifecycle)).

## Failure model
No controller: `EXIT_ABSENT` (2); none comes up: `EXIT_GAVE_UP` (6). Waits are
bounded, bus faults reset the controller ([SETUP.md](SETUP.md#failure-model)).

## Current implemented surface
PCI and ACPI discovery, bring-up, the bounded engine, eight operations, the
reach ([RELEASE.md](RELEASE.md#current-implemented-surface)).

## Wire format
The `NI2C` header, version `1`, in the 20-byte driver envelope; replies start
with a 4-byte signed status word; integers are little-endian.

## State ownership
Controller-facing I2C state, the reach and the bus state; descriptors, reports
and focus belong above ([CONTRACT.md](CONTRACT.md#state-ownership)).

## Operating rules
No HID parsing, no stored input, no kernel internals, no raw PIO, every transfer
bounded ([CONTRACT.md](CONTRACT.md#operating-rules)).

## Release target
`driver.i2c_pci0 -> driver.i2c_hid0 -> kernel input ring -> input_router`, then
IRQ-aware completion ([RELEASE.md](RELEASE.md#release-target)).

## Release evidence
Signed spawn on LPSS hardware and a decoded descriptor delivered upward
([RELEASE.md](RELEASE.md#release-evidence)).

## Release checklist
Zero warnings, static gates, signed artifacts, a bounded write-read on hardware
([RELEASE.md](RELEASE.md#release-checklist)).

## Explicit non-goals today
Touchpad gestures, interrupt-driven transfers, DMA, SMBus, sensor fusion and
input focus.

## Verification
`make -B nonos-mk-driver-i2c-pci`, the static checks, and three proof crates
([RELEASE.md](RELEASE.md#verification)).
