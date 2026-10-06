# capsule_driver_i2c_pci

## Role

`capsule_driver_i2c_pci` is the Intel LPSS DesignWare I2C controller capsule.
It owns PCI discovery, BAR0 mapping, interrupt binding, controller identity,
clock metadata, and safe register telemetry for `driver.i2c_pci0`.

```text
driver.i2c_hid0 (the only sender the kernel admits)
        |
        v
driver.i2c_pci0 -- brokered MMIO/IRQ --> Intel LPSS I2C controller
```

The capsule is not a HID parser, touchpad driver, sensor hub, ACPI policy
engine, or input router. Those layers stay above the controller driver.

## Microkernel contract

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, and `Irq`:

```text
CAPSULE_REQUIRED_CAPS = 0x78018
```

The capsule reaches hardware only through `MkDeviceList`, `MkDeviceClaim`,
`MkMmioMap`, and `MkIrqBind`. The kernel validates the signed manifest, brokers
grants, routes IPC, and revokes all grants on capsule exit.

Only `driver.i2c_hid0` may send to `driver.i2c_pci0`: the kernel holds the
endpoint to it (`src/services/registry/held.rs`), by name and by pid, because
`OP_TRANSFER` is a raw bus transaction to any device on the bus.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | status |
| `OP_CONTROLLER_INFO` | none | PCI id, clock, MMIO/IRQ grants |
| `OP_REGISTER_SNAPSHOT` | none | DesignWare status/FIFO/config registers |
| `OP_TIMING_INFO` | none | standard/fast-mode SCL count registers |
| `OP_TRANSFER` | address, write bytes, read length | status, abort source, read bytes |
| `OP_PROBE` | 7-bit address | present / absent |
| `OP_ACPI_HID` | none | the touchpad the kernel registered from ACPI: I2C address, HID descriptor register, GPIO pin; empty when firmware declared none |

`src/server/handlers/doorbell.rs` holds a GPIO doorbell handler that
`driver.i2c_hid0` asks for as op 8, but it is not in the module tree at this
commit, so op 8 is answered `E_BAD_OP`.

Unknown operations reply `E_BAD_OP`. Non-empty bodies on fixed-width requests
reply `E_INVAL`.

## Authority

The capsule may enumerate PCI devices, claim one Intel LPSS I2C function, map
BAR0, and bind the device IRQ. It has no DMA, PIO, filesystem, network, display,
credential, or input focus authority.

## Privacy and persistence

The capsule stores no touch events, gestures, sensor readings, device names, HID
reports, or ACPI tables. Runtime state is limited to grant ids, PCI identity,
controller clock metadata, and side-effect-free register snapshots.

## Runtime lifecycle

Startup discovers every Intel LPSS controller, on PCI or as an ACPI serial-bus
record whose source clock the broker carries in the BAR's `aux` word. For each
one it tries, it claims it, maps BAR0, binds the IRQ, verifies the DesignWare
component type when exposed, masks controller interrupts and clears pending
interrupt state. It keeps the first controller whose bus answers one of the
touchpad addresses ACPI declared, else one a candidate's `_CRS` names, else
the first that comes up, releases the others, and serves IPC
(`src/setup/sequence/run.rs`). Process teardown and
broker revocation release every hardware grant.

## Failure model

Unsupported PCI IDs, missing BAR0, missing IRQ, failed grants, or an unreadable
controller window prevent the capsule from serving. With no LPSS I2C controller
in the device list the capsule logs one line and exits `EXIT_ABSENT` (2)
before claiming anything; when controllers are listed but none comes up, the
pass over them is retried on the shared bounded schedule
(`nonos_libc::bring_up`: seven tries, sleeping between them, every controller
not kept released in between), and running out exits `EXIT_GAVE_UP` (6).
Transfers are bounded to
small controller-local buffers, carry an explicit timeout, and surface
DesignWare abort state so higher layers can distinguish NACK from bus failure.

## Current implemented surface

- Intel LPSS I2C PCI discovery across Skylake through Meteor Lake-era ids, and
  ACPI-only controllers from the broker's ACPI records.
- The ACPI touchpad record handed to the HID driver (`OP_ACPI_HID`).
- Brokered device claim, BAR0 MMIO map, and IRQ bind.
- DesignWare component-type read, enable/status/timing/FIFO register telemetry.
- Interrupt mask and clear during setup.
- Bounded master write/read transaction engine with timeout and TX-abort
  reporting.
- IPC health, controller info, register snapshot, timing info, probe,
  transfer, and ACPI HID operations.
- Static gates for broker-only access and endpoint ownership.

## Wire format

Requests use the `NI2C` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte signed status word. All multi-byte
integers are little-endian.

## State ownership

`driver.i2c_pci0` owns only controller-facing I2C state: PCI identity, MMIO
grant, IRQ grant, component id, controller enable/status values, FIFO levels,
and timing registers. HID-over-I2C owns descriptors and reports; the input
router owns focus, routing, and policy.

## Operating rules

- Do not parse HID reports in this capsule.
- Do not persist touch, gesture, or sensor data.
- Do not import kernel driver, memory, paging, or hardware internals.
- Do not add inline architecture assembly or raw PIO.
- Keep bus transfers bounded and fail closed on timeout or controller abort.

## Release target

The target chain is:

```text
driver.i2c_pci0 -> driver.i2c_hid0 -> kernel input ring -> input_router -> compositor / apps
```

The next runtime slice is IRQ-aware completion and the GPIO doorbell above the
same bounded transfer primitive.

## Release evidence

Release evidence requires signed capsule spawn on Intel LPSS hardware, confirmed
controller identity, stable register snapshot IPC, successful bounded write-read
against an I2C HID descriptor register, and delivery of a decoded HID descriptor
to the higher-level HID runtime without kernel-resident input policy.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm brokered MMIO/IRQ authority and endpoint ownership.
- Kernel profile `microkernel-driver-i2c-pci` resolves signed artifacts.
- Controller identity and timing registers are readable on supported hardware.
- Bounded write-read transaction validation passes without storing input history.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller present, not started` line needs the Debug capability, which this manifest does not grant, so the kernel prints `[EXIT] <service> status 2: no device present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- Every LPSS I2C controller is found (PCI or ACPI); the one whose bus answers the touchpad's address is bound.
- The SCL counts give a fast-mode bus within spec on the real input clock.
- When no controller comes up, the pass is retried and then given up on, with every controller released in between.

## Explicit non-goals today

This slice does not implement touchpad gestures, interrupt-driven transfers,
DMA, SMBus, sensor fusion, or input focus.

## Verification

- Build: `make -B nonos-mk-driver-i2c-pci`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-i2c-pci`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Proofs: `userland/i2c_pci_proofs`, `userland/i2c_transfer_proofs`.
- Handbook: [drivers](../../docs/handbook/drivers.md).
