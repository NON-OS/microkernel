# capsule_driver_i2c_hid

## Role
The HID-over-I2C class capsule for laptop touchpads. Above `driver.i2c_pci0` it
finds the pad, reads its descriptors, wakes it, reads its reports and posts
pointer, wheel and button events with `MkInputEventPost`; it holds no hardware.

```text
driver.i2c_hid0 -- MkInputEventPost --> kernel input ring --> input_router
        | IPC (the only client driver.i2c_pci0 admits)
driver.i2c_pci0 -- MMIO --> I2C controller --> touchpad
```

## Microkernel contract
`CAPSULE_REQUIRED_CAPS = 0x200018` (IPC, Memory, InputSource), Debug optional.
`MkServiceLookup` finds `driver.i2c_pci0`, `MkIpcSend` and `MkIpcRecv` call it
([CONTRACT.md](CONTRACT.md#microkernel-contract)).

## Interface contract
`OP_HEALTHCHECK`, `OP_PROBE` and `OP_DESCRIPTOR`, which no capsule may send yet
([CONTRACT.md](CONTRACT.md#interface-contract)).

## Authority
IPC to the controller and posts to the input ring, nothing else; it reaches only
the addresses the controller has in reach
([CONTRACT.md](CONTRACT.md#authority)).

## Privacy and persistence
No touch, gesture, keystroke or history is kept; each report is decoded, posted
and dropped ([CONTRACT.md](CONTRACT.md#privacy-and-persistence)).

## Runtime lifecycle
Look the controller up, bind the pad, wake and reset it, then serve and read
reports ([LIFECYCLE.md](LIFECYCLE.md#runtime-lifecycle)).

## Failure model
No controller service: `EXIT_ABSENT` (2) after two seconds asleep; a silent pad
leaves it reprobing ([LIFECYCLE.md](LIFECYCLE.md#failure-model)).

## Current implemented surface
Lookup, bounded descriptor reads, the address from `OP_ACPI_HID` then a scan,
wake, report decode and posting ([RELEASE.md](RELEASE.md)).

## Wire format
The `NHID` header, version `1`, in the 20-byte driver envelope; replies start
with a 4-byte signed status word; integers are little-endian.

## State ownership
Descriptors, the address, probe counters and decode state; the controller owns
the bus and the reach ([CONTRACT.md](CONTRACT.md#state-ownership)).

## Operating rules
No hardware caps, no stored input, no focus routing, every transfer bounded
([CONTRACT.md](CONTRACT.md#operating-rules)).

## Release target
`driver.i2c_pci0 -> driver.i2c_hid0 -> kernel input ring -> input_router`; a
boot on real pads is what is left ([RELEASE.md](RELEASE.md#release-target)).

## Release evidence
Both capsules spawned signed and a real pad's descriptor decoded and delivered
([RELEASE.md](RELEASE.md#release-evidence)).

## Release checklist
Zero warnings, static gates, signed artifacts, a probe on supported hardware
([RELEASE.md](RELEASE.md#release-checklist)).

## Explicit non-goals today
An I2C keyboard, layout mapping, touch filtering beyond the gesture decoder,
focus routing, power management beyond the wake at start.

## Verification
`make -B nonos-mk-driver-i2c-hid`, the static checks, and `i2c_hid_proofs`
([RELEASE.md](RELEASE.md#verification)).
