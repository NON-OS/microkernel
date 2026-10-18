# usb_proofs

Host-runnable proofs for the USB HID driver over the bytes a device chooses:
the configuration descriptor, the report descriptor, every interrupt-IN
report, and a hub's port status. The shipping driver source is included
through `#[path]` and run on the host. `nonos_libc` is a shim
(`libc_shim/`) whose input ring is a list the tests read back and whose
clock moves only when a test moves it; `driver.xhci0` is played in
`src/xhci/` (a scripted hub, a report descriptor, queued interrupt reports),
and its own endpoint Interval rule is included to check the poll pace
against. Only the poll loop itself and the layout policy lookup make system
calls and stay out.

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
(8 bytes for a boot keyboard, 3 for a boot mouse, 1 for an interface in
report protocol, whose report descriptor says the rest) and is no larger
than the 1024 bytes USB allows an interrupt endpoint. The HID descriptor
after an interface gives its report descriptor's length. A boot interface
of no known protocol binds nothing, and the binding count stops at the cap
however many interfaces follow. Every HID interface is offered: a
keyboard's media key interface is no longer dropped unread.

## Report descriptor walk

`report/`: the walk (HID 1.11, 6.2.2) reads short items and steps over long
ones, and refuses a descriptor whole when an item is cut, it passes 512
items, 8 nested collections, 4 pushed globals, 8 report IDs, a Report ID of
0, a size times count or a report past 2048 bits, or leaves a collection
open. A descriptor longer than one 512-byte read, or given shorter than its
length, is refused unread (`read_tests`). The layout keeps the first
keyboard (modifiers and key slots), consumer and system control keys (Mute,
Volume Up and Down, Power), and pointer (five buttons, X, Y, wheel; tip and
in range for a pen or touchscreen) it finds, each under its report ID. An
interface binds only through the driver's own `device_for`: when the layout
names one of those and every input report fits one 8-byte read. A gamepad
or a UPS binds nothing; QEMU's tablet, a report ID mouse, a touchscreen and
a keyboard's media keys do. Reads through a layout never go past the report
and a short report is nothing. Real descriptors (QEMU's tablet, the HID
appendix keyboard, a DragonRise gamepad, a UPS, a touchscreen) and 200,000
seeded hostile ones run through it.

## Interrupt reads

A read asks the controller driver for one packet but never more than its
8-byte report buffer; that limit is included from `capsule_driver_xhci`, which
refuses a longer read. `poll/`: an endpoint is asked only when due, as often
as the controller serves it (checked against `capsule_driver_xhci`'s own
Interval rule for every speed and bInterval), every 8 ms once quiet for a
second, and at most 16 reports at a time; the loop sleeps on its inbox until
the next is due, 1 to 16 ms; the ports are rescanned 64 ms after the last
report and hub ports mid bring-up read every 10 ms. Each report goes to the
state of its own interface: a second keyboard's idle reports release
nothing on the first, and a second mouse ends no drag.

## Boot keyboard reports

A report presses exactly the keys it names that were up and releases exactly
the keys it no longer names, presses first in slot order and releases in held
order. A usage named in several slots is one key. Only usages the HID
keyboard page gives a key (0x04 to 0xA4 and 0xB0 to 0xDD) make events; empty
slots, reserved usages and the modifier usages 0xE0 to 0xE7 do not, checked
over all 256 values. A rollover report (an error code 0x01 to 0x03 in any
slot) makes no event and keeps the held keys held, with its modifier byte
still taken. A report that is not exactly eight bytes changes nothing.

## Boot mouse and absolute pointer reports

Every motion and wheel byte is sign extended. A mouse report under three
bytes is nothing; bytes past the wheel are never read. Only the five mouse
button bits, or the buttons a layout declares, are buttons, and every pair
of button bytes makes events for exactly the bits that changed, numbered
from 1, in the direction they went. Every absolute position is clamped to
its logical range and scaled onto 0 to 0x7FFF, the range the input router
scales onto the screen, and the router's divisor is pinned to that number.

## Unplug, retention and the service wire

A device that leaves, or a hub above it, releases every key with the code
it went down with and every button, and its repeat ends. A keyboard or
pointer holds no event log (C8): every key typed is posted once down and
once up and nothing of it kept. No op feeds a report in or reads a key back
out; every frame a client can send is decoded or refused with a whole reply
(`request/`).

## Hub ports

`hub/`, `enumerate/hub_flap_tests`: a device on a hub port is debounced
100 ms, reset (read no sooner than 10 ms after the request, failed at
800 ms), given 10 ms to recover, then addressed, every wait a deadline
looked at by the loop, never a sleep; a new hub's ports wait for their
power to be good the same way; a port is tried three times a minute however
often it reconnects. A hub flapping all fifteen ports for a minute never
holds the loop.

## Hostile device fuzz and named edges

A seeded xorshift generator plays a hostile device for 200,000 rounds each:
structured and arbitrary configuration descriptors, a keyboard report stream,
and mouse and tablet reports. Nothing may panic. Every walk must agree with a
second statement of the binding rules written in the test from the USB and
HID specifications. Every key event must name a defined key, press only a key
that is up and release only one that is down, and the keys down must equal
the keys the last whole report named. Motion, position and button events stay
inside their defined ranges. `boundary/` lists each edge by name with the
one outcome its rule allows.

Every rule above was checked by breaking it in the driver source: each break
fails at least one test here.

## What changed in the driver

The report descriptor parser (`descriptors/report/`), the layout decode
(`hid/report/`), per-interface state (`hid/device.rs`), unplug release, the
stepped hub bring-up (`hub/bringup.rs`, `stage.rs`, `budget.rs`) and the
poll pace (`orchestrator/poll/pace.rs`, `period.rs`, `rescan.rs`) are new;
the key and mouse event queues and the feed and poll ops are gone, and
`hid/tablet_report.rs`, QEMU's tablet layout taken for every non-boot
interface, is replaced by the report descriptor.

The report decodes moved out of the feeds into pure files
(`hid/keyboard/boot_report.rs`, `key_changes.rs`, `is_error_code.rs`,
`hid/mouse_report.rs`, `hid/button_changes.rs`,
`descriptors/packet_size.rs`, `orchestrator/poll/read_len.rs`). The proofs
found and the driver now fixes: an endpoint after a cut interface record
bound to the interface before it; endpoint 0 and packet sizes of 0, too small
for a report, or past 1024 were bound; a usage in two slots pressed twice; a
rollover report released every held key and the next report typed them
again; reserved and modifier usages were posted as keys; a short interrupt
report was zero padded and released every held key; absolute positions
above 0x7FFF passed through; and a read asked for the whole packet, which the
controller driver refuses past 8 bytes, so larger endpoints were never read.

## Run

```sh
cd userland/usb_proofs
cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
