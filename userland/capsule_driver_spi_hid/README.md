# capsule_driver_spi_hid

Status: wired, not yet signed. The kernel mirror
(`src/hardware/spi_hid_capsule`) embeds and spawns it under
`nonos-capsule-driver-spi-hid`, after `driver.spi0`. It has no owner
certificate yet, so no image carries it until the owner signs it. It
cannot read the firmware's record until the held table admits it to
capsule_acpi (the lead's line); until then it says so and leaves.

## Role

`driver.spi_hid0` is HID over SPI: a touchpad or keyboard on the bus of
the SPI host `driver.spi0` drives, as the firmware's serial-bus record
names it. It fetches the device's reports through spi0 and posts them to
the kernel input ring with `MkInputEventPost`, decoded by the same code as
HID over I2C (`capsule_driver_i2c_hid`'s report descriptor parser, poll,
gesture and publish files, mounted). Only how a report is fetched is its
own.

```text
driver.spi_hid0 -- MkInputEventPost --> kernel input ring --> input_router
   |          |
   | NSPI     | SERIAL (op 9)
   v          v
driver.spi0  capsule_acpi
   | gpio0 EVENTS, under spi0's key
   v
driver.gpio0
```

## Microkernel contract

`CAPSULE_REQUIRED_CAPS = 0x200018`: IPC, Memory, InputSource.
`CAPSULE_OPTIONAL_CAPS = 0x100`: Debug, in a serial-debug build only. No
hardware capability of any kind: spi0 holds the host. It finds spi0 and
capsule_acpi with `MkServiceLookup`, calls them with `MkIpcCall`, and
never calls `MkIpcRecv`: no capsule may send to it (held `KERNEL_ONLY`),
and it serves nothing.

## Interface contract

None served. It is a client of two wires:

- `driver.spi0`, NSPI: OPEN (its key back), TRANSFER, DOORBELL (op 5, the
  device's GpioInt as `_UID` and pin).
- capsule_acpi, SERIAL (op 9, F 65): `[9, index u32]`, one serial-bus
  record per index, with the SPI connection and the HID-over-SPI _DSM
  facts.

## Wire format

To the device, HID over SPI v1.0 as recalled (`nonos_spi_hid`): every
transaction starts with the firmware's opcode and a 24-bit big-endian
address. An input report is its 4-byte header read at the input header
address, then its body at the input body address. A request is written
at the output address.

## Authority

It may ask spi0 for transfers to its one device and post input events.
It cannot claim, map or bind anything, and reaches no other device.

## Privacy and persistence

Reports are decoded and posted as they come and kept no longer. Nothing
is written anywhere; its console says only bring-up lines.

## Runtime lifecycle

Spawned after spi0. It opens spi0 and learns the key spi0's host goes by,
then asks capsule_acpi's records, index by index, for one that is HID over
SPI on that key's bus. With none, it says why and exits `EXIT_ABSENT` (2),
and spi0 sees its holder go. With one, it asks the device for its device
descriptor, then its report descriptor, and parses them. Then it reads
reports when the device's line rang, by timer until the line has rung
once, and always by timer where the firmware named no GpioInt.

## Failure model

A record it cannot drive is refused, never driven wrong:
- anything but 8-bit frames;
- three-wire, a chip select other than 0, or one active high;
- dual or quad SPI, a phase or polarity past 1, or no speed;
- a missing input header, body or output address, or opcode.

A header without its sync byte is no report. A fragment is read and let
go. A body larger than one spi0 transfer cannot be read. A device that
answers no descriptor is asked again every half second. A reset the
device announces sends it back through bring-up.

## State ownership

The device's descriptors and parsed layouts, and the shared HID driver's
state. The link (spi0's port and the firmware's facts) is set once.

## Operating rules

- Do not post anything that was not decoded from the device's report.
- Do not reach any device but the one the firmware names on spi0's bus.
- Do not change the shared HID code here; change it in i2c_hid.

## Current implemented surface

Setup, bring-up, report reads, the doorbell, and the shared decode and
posting. A precision pad's input mode and surface and button switches are
set after bring-up by i2c_hid's own feature report logic, run over SPI's
get-feature and set-feature requests.

## Release target

```text
driver.spi_hid0 -> kernel input ring -> input_router -> compositor / apps
```

## Release evidence

`spi_hid_proofs` covers the SERIAL record (every field, unknowns, every
truncation, a string past the end) and the refusal of each wiring spi0
cannot drive. It also covers the report laid out for the shared poll, and
the protocol core. `kernel_proofs` covers the held table and the spawn.
`i2c_hid_proofs` covers the feature report logic both drivers share, over
a modelled port. Verification rows: the HID-over-SPI values in
`nonos_spi_hid` (the feature request and response types among them), the
_DSM flags' multi-SPI bits (15:14), and reading with no dummy byte after
the address.

## Release checklist

- The held table admits it to capsule_acpi (the lead's line).
- driver.gpio0 admits driver.spi0 to its EVENTS (F's table).
- Signed by the owner and added to an image's features.
- On a machine with an SPI touchpad: the descriptors read, reports move
  the pointer, and the line rings.

## Explicit non-goals today

No output reports but descriptor and feature requests, no fragmented
reports, and no multi-SPI. Touches become pointer motion,
scroll and taps in the shared HID code, as on I2C; moving that into the
input router is G's.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/spi_hid_proofs && cargo test --release)` and
`(cd userland/kernel_proofs && cargo test --release)`.
