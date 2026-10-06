# usb_proofs

Host-runnable proofs for the USB HID driver over the bytes a device chooses:
the configuration descriptor and every interrupt-IN report. The shipping
driver source is included through `#[path]` and run on the host. The parts
of the driver that post input events or read the layout policy make system
calls and stay out; every decision about descriptor or report bytes lives in
the included files.

A malicious or malfunctioning USB device is inside the threat model for any
machine with an exposed port, so all of this runs on data the system does not
control.

## Configuration descriptor walk

The walk refuses a record whose bLength is 0 or 1 (the descriptor-walk loop
is an error, not a spin), refuses a record that runs past wTotalLength, and
refuses a wTotalLength below the 9-byte header or past the buffer. Every
wTotalLength from 0 to 0xFFFF is decided against one buffer without reading
past either, and bytes past wTotalLength are never walked. A cut endpoint
record is stepped over; a cut interface record ends the interface before it,
so its endpoints never bind to an earlier one.

Only a HID class interface binds, only through an interrupt IN endpoint
numbered 1 to 15, and only when one packet holds a whole report of its kind
(8 bytes for a boot keyboard, 3 for a boot mouse, 5 for a tablet) and is no
larger than the 1024 bytes USB allows an interrupt endpoint. A boot interface
of no known protocol binds nothing, and the binding count stops at the cap
however many interfaces follow.

## Interrupt reads

A read asks the controller driver for one packet but never more than its
8-byte report buffer; that limit is included from `capsule_driver_xhci`, which
refuses a longer read.

## Boot keyboard reports

A report presses exactly the keys it names that were up and releases exactly
the keys it no longer names, presses first in slot order and releases in held
order. A usage named in several slots is one key. Only usages the HID
keyboard page gives a key (0x04 to 0xA4 and 0xB0 to 0xDD) make events; empty
slots, reserved usages and the modifier usages 0xE0 to 0xE7 do not, checked
over all 256 values. A rollover report (an error code 0x01 to 0x03 in any
slot) makes no event and keeps the held keys held, with its modifier byte
still taken. A report that is not exactly eight bytes changes nothing.

## Boot mouse and tablet reports

Every motion and wheel byte is sign extended. A mouse report under three
bytes, or a tablet report under five, is nothing; bytes past the wheel are
never read. Only the five mouse or three tablet button bits are buttons, and
every pair of button bytes makes events for exactly the bits that changed,
numbered from 1, in the direction they went. Every tablet position is clamped
to 0 to 0x7FFF, the range the input router scales onto the screen, and the
router's divisor is pinned to that number.

## Hostile device fuzz and named edges

A seeded xorshift generator plays a hostile device for 200,000 rounds each:
structured and arbitrary configuration descriptors, a keyboard report stream,
and mouse and tablet reports. Nothing may panic. Every walk must agree with a
second statement of the binding rules written in the test from the USB and
HID specifications. Every key event must name a defined key, press only a key
that is up and release only one that is down, and the keys down must equal
the keys the last whole report named. Motion, position and button events stay
inside their defined ranges. `boundary_tests` lists each edge by name with
the one outcome its rule allows.

Every rule above was checked by breaking it in the driver source: each break
fails at least one test here.

## What changed in the driver

The report decodes moved out of the feeds into pure files
(`hid/keyboard/boot_report.rs`, `key_changes.rs`, `is_error_code.rs`,
`hid/mouse_report.rs`, `hid/tablet_report.rs`, `hid/button_changes.rs`,
`descriptors/packet_size.rs`, `orchestrator/poll/read_len.rs`). The proofs
found and the driver now fixes: an endpoint after a cut interface record
bound to the interface before it; endpoint 0 and packet sizes of 0, too small
for a report, or past 1024 were bound; a usage in two slots pressed twice; a
rollover report released every held key and the next report typed them
again; reserved and modifier usages were posted as keys; a short interrupt
report was zero padded and released every held key; tablet positions above
0x7FFF passed through; and a read asked for the whole packet, which the
controller driver refuses past 8 bytes, so larger endpoints were never read.

## Run

```sh
cd userland/usb_proofs
cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
