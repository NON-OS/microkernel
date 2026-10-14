# capsule_driver_i2c_hid: finding, waking and reading the pad

How the capsule finds its touchpad, wakes it and reads it, and how each step
fails. The contract is in [README.md](README.md).

## Runtime lifecycle

Startup resolves `driver.i2c_pci0` and asks it, with `OP_ACPI_HID`, for the
address setup bound: the touchpad the firmware declared, or, when the
firmware leaves the address to be filled in at run time, the device that
answered the controller's read-only sweep of 0x15, 0x2C and 0x10 first, on
the bus the declaration names, pad or not; the sweep stops there, so only that
device joins the reach
(`userland/capsule_driver_i2c_pci/src/setup/sequence/sweep.rs`). When there
is none, or no descriptor answers there, it falls back to probing common
HID-over-I2C addresses; the controller refuses every one it has not put in
reach before it reaches the bus, so the scan writes to nothing the firmware
did not declare or the sweep did not find. It reads the 30-byte HID
descriptor (from register `0x0001` on a scan), validates descriptor length
and BCD version, and wakes and resets the pad. Once it has read that
descriptor at the register the controller has declared for the address, the
controller narrows its reach to that address for the rest of the session.

The loop then waits 2 ms for a request, re-probes every 250 cycles while no
pad is found, and reads an input report each cycle. It asks
`driver.i2c_pci0` for a GPIO doorbell (op 8) to pace reads by the pad's
interrupt line, and trusts it once it has rung; when the controller has no
doorbell to offer, the reads are timed. Relative reports post pointer, wheel
and button events; absolute touch reports go through a small gesture
decoder.

## Failure model

If the controller service is missing, startup fails closed: the lookup is
retried 100 times, 20 ms apart and asleep in between (about two seconds),
then the capsule logs one line and exits `EXIT_ABSENT` (2). The pauses after
power-on (20 ms) and while waiting for the reset report (up to 64 polls, 5 ms
apart) are sleeps too; nothing at start yields in a loop. If no HID
descriptor is found, the capsule still serves health/probe calls but returns
`E_NOT_FOUND` for descriptor reads until a later probe succeeds.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the
parsers, the bring-up sequence against a model, and the bounded retry; these
do not.

- On a machine without an I2C controller the capsule looks `driver.i2c_pci0`
  up for about two seconds, asleep between tries, then prints
  `no controller present, not started` and exits `EXIT_ABSENT` (2).
- The pad named by ACPI (or found by the controller's sweep, or by the scan)
  is bound; SET_POWER, RESET and the zero-length reset report complete
  inside the 320 ms reset budget.
- Absolute touch decodes; tap, drag, two-finger scroll and palm rejection
  behave.
- Without an I2C controller the driver gives up after about two seconds
  asleep.
