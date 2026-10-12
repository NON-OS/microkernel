# capsule_driver_gpio

## Role

`capsule_driver_gpio` is the x86 GPIO controller capsule (`driver.gpio0`).
It claims the GPIO controllers the kernel published from ACPI, maps their
windows, arms the `_AEI` pins and the serial-bus devices' `GpioInt` pins as
firmware describes them, and takes the controller's pending pins apart:

```text
GPIO controller (AMD FCH bank; Intel PCH communities held, pins refused)
        |
        v
driver.gpio0 -- _AEI pin --> capsule_acpi (GPIO_EVENT SIGNAL)
             -- device pin --> queued for that device's holder (EVENTS)
```

The engine (register models, router, storm masking) is `nonos_gpio`, proven
in `gpio_proofs` against controller models.

## Microkernel contract

The manifest grants `IPC`, `Memory`, `DeviceEnum`, `Driver`, `Mmio`, and
`Irq`:

```text
CAPSULE_REQUIRED_CAPS = 0x78018
```

The capsule reaches hardware only through `MkDeviceList`, `MkDeviceClaim`,
`MkMmioMap` and `MkIrqBind`/`MkIrqPoll`/`MkIrqWait`/`MkIrqAck`, and names
ACPI devices through `MkAcpiDeviceName` (a look: the controller's `_HID`, a
platform bus controller's path). Each controller's own interrupt is bound,
sensed as its `_CRS` says; a controller is read when its line fires. One
whose line does not bind is read every 5 ms instead.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `EVENTS` (1) | device key u64: the bus controller's PCI function (segment << 16, bus << 8, devfn), or bit 63 set and the broker device id of a platform bus controller | count u8, then (controller `_UID` u16, pin u16) each |
| `WRITE` (2) | controller u16, pin u16, level u8 | status |
| `REARM` (3) | controller u16, pin u16 | status |

Every answer about a device is given only to a sender holding it, asked of
capsule_acpi's HOLDER (21), which is the kernel's claim test. A pin that
storms (more than 500 events in a second) is masked alone and said once; its
device's holder re-arms it.

## Authority

The capsule may enumerate devices, claim the ACPI GPIO controllers, map
their windows and bind their interrupts. It has no DMA, PIO, filesystem, network, display,
credential, or input focus authority. Its sealed trust files come from the
`nonos-capsule-driver-gpio`, which no profile turns on yet.

## Privacy and persistence

The capsule keeps no input history: a device's events wait in a queue of at
most 64 per device, the oldest dropped first, and are gone once its holder
drains them. Nothing is written to storage.

## Runtime lifecycle

At start the capsule lists the broker's devices, claims every ACPI GPIO
controller record and maps its windows, arms the pins firmware names, then
serves one loop: a request answered when one arrives within 5 ms, then every
controller's pending pins taken apart. Claims and maps go back to the kernel
when the capsule exits.

## Failure model

With no GPIO controller in the device list the capsule logs one line and
exits `EXIT_ABSENT` (2). A controller another capsule holds, or whose windows
do not map, is said and passed over, its claim given back. A pin the register
model cannot place, or whose trigger it cannot honour, is said and left
unarmed. A pin that storms is masked alone. When capsule_acpi does not answer,
`_AEI` pins are not armed and holder tests fail closed (refused).

## Current implemented surface

- Claim and map of every ACPI GPIO controller record.
- The AMD FCH bank: arming, masking, pending pins with end of interrupt,
  levels and outputs (`nonos_gpio::amd`).
- Intel PCH controllers held with every pin refused until a published layout
  places their pads (F 46).
- `_AEI` pins listed and signalled through capsule_acpi's GPIO_EVENT, a level
  pin masked while its method runs.
- Serial-bus devices' GpioInt pins queued for the holder of their bus
  controller's PCI function.
- EVENTS, WRITE and REARM, each answered only for the device's holder.

## Wire format

Requests are an op byte and its arguments; replies begin with a 4-byte signed
status word, then the body. All multi-byte integers are little-endian.

## State ownership

`driver.gpio0` owns the GPIO controllers' pad interrupt configuration and the
routing of their pins. capsule_acpi owns the `_AEI` event methods; each
device's driver owns what its pin means.

## Operating rules

- Never guess where a pin is: refuse what the register model cannot place.
- Leave a pad's mode and routing as firmware set them.
- Never write a status bit back as one by accident.
- Give a device's events and outputs only to its holder.

## Release target

```text
GPIO controller -> driver.gpio0 -> capsule_acpi (_AEI) / device holders
```

## Release evidence

Release evidence requires signed capsule spawn on an AMD laptop, the lid or
power `_AEI` event running through capsule_acpi, and a touchpad's GpioInt
delivered to the I2C driver as an event rather than read as a level.

## Release checklist

- Capsule builds with zero warnings; `gpio_proofs` pass.
- HELD lines admit `driver.gpio0` to capsule_acpi and its holders to it.
- The I2C driver gives up its own claim of the controller.

## Explicit non-goals today

Binding the controller's interrupt line (the IOAPIC mode must honour the
`_CRS` polarity first), Intel pad placement, pin multiplexing, and GpioIo
outputs until a GpioIo record names their devices.

## Verification

- Build: `make -B nonos-mk-driver-gpio`
- Proofs: `userland/gpio_proofs`.
- Static gate: `bash nonos-ci/run-static-checks.sh`
