# capsule_driver_i2c_hid

## Role

`capsule_driver_i2c_hid` is the HID-over-I2C class capsule for laptop
touchpads. It sits above `driver.i2c_pci0`, finds the pad (at the address ACPI
named, else by a scan), reads its HID and report descriptors, wakes and resets
it, reads its input reports, and posts pointer, wheel and button events to the
kernel input ring with `MkInputEventPost`.

```text
driver.i2c_hid0 -- MkInputEventPost --> kernel input ring --> input_router
        |
        | IPC (the only client driver.i2c_pci0 admits)
        v
driver.i2c_pci0 -- MMIO/IRQ --> I2C controller --> touchpad
```

The capsule is not an I2C controller driver and has no direct hardware grants.

## Microkernel contract

The manifest grants `IPC`, `Memory` and `InputSource`:

```text
CAPSULE_REQUIRED_CAPS = 0x200018
CAPSULE_OPTIONAL_CAPS = 0x100
```

The optional bit is `Debug`, which only a `capsule-serial-debug` build grants.
`InputSource` admits `MkInputEventPost`. The kernel's consumer gate for
`MkInputEventDrain` and `MkInputEventWait` also accepts `InputSource`, so this
capsule could drain the keystroke ring as `input_router` does; nothing in its
code calls those.

The capsule resolves `driver.i2c_pci0` with `MkServiceLookup`, sends bounded IPC
requests with `MkIpcSend`, receives replies with `MkIpcRecv`, and serves callers
with `MkIpcRecvFrom` plus `MkIpcSendToPid`.

No capsule may send to `driver.i2c_hid0`: the kernel holds the endpoint to an
empty list (`src/services/registry/held.rs`). The serving loop answers only a
non-zero sender (`src/server/runner/run.rs`), so at this commit the three ops
below are reachable by nobody; the driver's work is the input it posts.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | found flag, address, probe count |
| `OP_PROBE` | none | refreshed descriptor state |
| `OP_DESCRIPTOR` | none | cached HID descriptor bytes |

Unknown operations reply `E_BAD_OP`. Malformed bodies reply `E_INVAL`.

## Authority

The capsule may talk to the I2C controller capsule over IPC and post events to
the kernel input ring. It has no PCI, MMIO, IRQ, DMA, PIO, filesystem, network,
display, or focus-routing authority.

## Privacy and persistence

The capsule stores no touch events, gestures, keystrokes, sensor samples, or
history. It keeps the I2C address, probe counter, HID and report descriptor
state, the last button mask and the current gesture in volatile memory; each
report is decoded and posted, then dropped.

## Runtime lifecycle

Startup resolves `driver.i2c_pci0`, asks it for the touchpad address the
kernel recovered from ACPI, falls back to probing common HID-over-I2C
addresses, reads the 30-byte HID descriptor (from register `0x0001` on a
scan), validates descriptor length and BCD version, and wakes and resets the
pad. The loop then waits 2 ms for a request, re-probes every 250 cycles while
no pad is found, and reads an input report each cycle. It asks
`driver.i2c_pci0` for a GPIO doorbell (op 8) to pace reads by the pad's
interrupt line, but at this commit the controller driver does not serve that
op (its `doorbell.rs` is not in the module tree), so the reads are timed. Relative reports post pointer, wheel and
button events; absolute touch reports go through a small gesture decoder.

## Failure model

If the controller service is missing, startup fails closed: the lookup is
retried 100 times, 20 ms apart and asleep in between (about two seconds), then
the capsule logs one line and exits `EXIT_ABSENT` (2). The pauses after
power-on (20 ms) and while waiting for the reset report (up to 64 polls, 5 ms
apart) are sleeps too; nothing at start yields in a loop. If no HID descriptor
is found, the capsule still serves health/probe calls but returns `E_NOT_FOUND`
for descriptor reads until a later probe succeeds.

## Current implemented surface

- Runtime service lookup for `driver.i2c_pci0`.
- Bounded HID descriptor reads through `OP_TRANSFER`.
- Probe list covering common ELAN, Synaptics, FocalTech, and alternate HID
  addresses.
- IPC health, reprobe, and descriptor export (unreachable at this commit, see
  above).
- The ACPI address from `driver.i2c_pci0` (`OP_ACPI_HID`), then the scan.
- Power-on, reset and the zero-length reset report, each wait a sleep.
- Report descriptor parse for the touch report, input report reads, and
  relative and absolute decodes posted to the kernel input ring.
- No persistent input history and no direct hardware access.

## Wire format

Requests use the `NHID` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte signed status word. All multi-byte
integers are little-endian.

## State ownership

`driver.i2c_hid0` owns HID-over-I2C class state: descriptor bytes, selected I2C
address, probe counters, and the decode state. `driver.i2c_pci0` owns controller
registers and bus transactions. `input_router` owns focus, event routing, and
policy.

## Operating rules

- Do not map hardware or request Driver/DeviceEnum/Mmio/Irq/Dma/Pio caps.
- Do not persist reports, touches, gestures, or keystrokes.
- Do not route input focus here.
- Keep every transfer bounded by the I2C controller capsule limits.

## Release target

The target chain is:

```text
driver.i2c_pci0 -> driver.i2c_hid0 -> kernel input ring -> input_router -> compositor / apps
```

Report reads and posting are in place; what is left is a boot on real pads.

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

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without an I2C controller the capsule looks `driver.i2c_pci0` up for about two seconds, asleep between tries, then prints `no controller present, not started` and exits `EXIT_ABSENT` (2).
- The pad named by ACPI (or found by the scan) is bound; SET_POWER, RESET and the zero-length reset report complete inside the 320 ms reset budget.
- Absolute touch decodes; tap, drag, two-finger scroll and palm rejection behave.
- Without an I2C controller the driver gives up after about two seconds asleep.

## Explicit non-goals today

This slice does not implement an I2C keyboard, keyboard layout mapping, touch
filtering beyond the gesture decoder, focus routing, or power management
beyond the wake and reset at start.

## Verification

- Build: `make -B nonos-mk-driver-i2c-hid`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-i2c-hid`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Proofs: `(cd userland/i2c_hid_proofs && cargo test --release)`.
- Handbook: [drivers](../../docs/handbook/drivers.md).
