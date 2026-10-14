# I2C-HID touchpads: a pad with no address

What happens when the firmware does not say where the touchpad is: the
controller's sweep, and the HID driver's scan, which reaches only what the
controller put in reach. The rest of the search is in [finding the
pad](i2c-hid-finding.md).

## The controller's sweep

Intel's reference firmware declares its pad as TPD0 with
I2cSerialBusV2(0x0000, ...) and patches the address in from NVS in its `_INI`
method, which the kernel cannot run, so the record names the controller and
no more. On that controller alone, `driver.i2c_pci0` reads the three
addresses touchpads commonly ship at, 0x15 (ELAN), 0x2C (Synaptics) and 0x10
(ALPS), one byte each and in that order, which writes nothing to the device,
not even a register pointer (`sweep`,
`userland/capsule_driver_i2c_pci/src/setup/sequence/sweep.rs:38-51`). A bus
that faults ends the sweep there.

The first device that answers is bound and joins the reach whether it is a
pad or not: one byte cannot tell, `OP_ACPI_HID` names it, and
`driver.i2c_hid0` writes its descriptor register and reads it back on every
probe until a pad answers. No address after it is touched, and the sweep
tries none where boards put something else: GPIO expanders of the PCA9555
kind (0x20, 0x24), whose interrupt latch a read clears and whose lines a
write can turn into outputs; thermal sensors of the LM75 kind (0x4B, 0x4C),
whose configuration register the HID register pointer 01 00 would write; and
0x38, where FocalTech touchscreens sit with their address declared.

A pad at any other address, or one that does not acknowledge a plain read
(the descriptor probe's notes say some ELAN parts do not), is not found this
way. Once the kernel evaluates the pad's `_CRS` I2cSerialBus resource (K13 in
the plan), the declaration carries the real address and
the sweep goes. `sweep_tests` and `sweep_neighbour_tests` in
`i2c_transfer_proofs`, and `bystander_tests` in `i2c_hid_proofs`, prove it.

## The HID driver's scan

When `OP_ACPI_HID` names no address, or no descriptor answers there,
`driver.i2c_hid0` scans 0x10, 0x15, 0x2C, 0x38, 0x4B, 0x4C, 0x20 and 0x24 at
descriptor registers 0x0001 and 0x0020 (`CANDIDATE_ADDRS`,
`userland/capsule_driver_i2c_hid/src/hid/probe/scan.rs:20-23`); the
controller refuses every address of the scan that is not in reach, so it
only finds a pad there. Until a pad answers it probes again every 250 turns
of its loop (`REPROBE_EVERY`,
`userland/capsule_driver_i2c_hid/src/server/runner/run.rs:31`). After 20
probes with no answer it writes one console line, when it holds Debug,
saying that no touchpad answers on the bus (`UNANSWERED_PROBES`,
`userland/capsule_driver_i2c_hid/src/setup.rs:61-68`).

## See also

- [I2C-HID touchpads](i2c-hid.md)
- [I2C-HID touchpads: finding the pad](i2c-hid-finding.md)
- [I2C-HID touchpads: authority, limits and checks](i2c-hid-limits.md)
