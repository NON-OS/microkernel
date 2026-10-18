# capsule_driver_i2c_hid: finding, waking and reading the pad

How the capsule finds its touchpad, wakes it and reads it, and how each step
fails. The contract is in [README.md](README.md).

## Runtime lifecycle

Startup resolves `driver.i2c_pci0` and asks it, with `OP_ACPI_HID`, for the
address setup bound: the touchpad the firmware declared, or, when the firmware
leaves the address to run time, the first device that answered the
controller's read-only sweep of 0x15, 0x2C and 0x10 on the bus the declaration
names (`userland/capsule_driver_i2c_pci/src/setup/sequence/sweep.rs`). No
other address is ever tried. It reads the 30-byte HID descriptor there (the
declared register, then `0x0001` and `0x0020`; never register 0), validates
it, wakes and resets the device, and walks its report descriptor (usb_hid's
walker, shared as source). A Touch Pad collection or a relative Mouse
collection is driven, a vendor report too long for one read ignored, and a
keyboard that is also a mouse is driven by its mouse collection, its key
reports dropped unread; anything else is put back to sleep and the driver
leaves. The controller then narrows its reach to that address for the session.

The loop waits on its inbox until its next pass is due. While no pad is bound
it asks again every 500 ms. A bound pad is read every 2 ms while reports come
and every 16 ms once it has been quiet a second (`src/drive/pace.rs`); where
the controller senses the pad's interrupt line (op 8) the pad is read only
when the line is asserted. Touch reports go through the gesture decoder, mouse
reports post pointer, wheel and button events, and the read buffer is cleared
after each. There is no suspend, resume or probe op until something exists to
send one.

A pad that resets itself comes back as a mouse. It is set up again, three
times at most with no touch report between (a write the pad refuses is tried
again and counts for none), a second from the end of one set-up to the start
of the next, and what it held is let go. Its signs: the zero-length report its
line rings for, at which the click is let go at once and a set-up is owed
until one takes or a touch report comes; a mouse report while the Input Mode
reads back as other than touchpad mode; and, on a pad that cannot read its
mode back, four or more mouse reports with no touch report among them, 100 ms
from the first, which let go of the gesture's click. Only a layout with no
Input Mode to write is driven as a mouse.

Known limits, on a pad that cannot read its mode back: a click held by a still
finger the pad then reports nothing for, or by a clickpad pressed with no
contact reported (a nail, a glove), is let go after 100 ms of pointing-stick
motion and taken again at the second touch report after, the button being
debounced; and a click a reset left down stays down until the fourth mouse
report, so a slow stick takes long (1.5 s at one report every 500 ms) and one
that stops short of four leaves it down until touch reports come again. One
failed GET on a pad that otherwise reads back arms that rule until its next
set-up. A spurious ring, the line asserted with nothing to read, lets go of a
held click: Linux ignores a zero-length report unless a reset is pending, but
a pad that resets itself sends exactly that unasked, and a button-up too many
is harmless where a click stuck down is not.

## Failure model

If the controller service is missing, startup fails closed: the lookup is
retried 100 times 20 ms apart, asleep between (about two seconds), then the
capsule logs one line and exits `EXIT_ABSENT` (2). The waits after power-on
(60 ms) and for the reset report (polled 5 ms apart for up to 1 s) are sleeps
too. If no pad comes up within 30 s of the first probe, past the controller's
own bring-up, or the device is no touchpad or mouse, the capsule puts a device
that answered to sleep, says why, and exits `EXIT_ABSENT`.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm; the host proofs cover the
parsers, the bring-up against a model, and the bounded retry.

- With no I2C controller the capsule gives up after about two seconds asleep,
  prints `no controller present, not started`, and exits `EXIT_ABSENT` (2).
- The pad named by ACPI (or found by the sweep) is bound; SET_POWER, RESET and
  the zero-length reset report complete within the 60 ms after power-on and
  the 1 s reset wait.
- Absolute touch decodes; tap, drag, two-finger scroll and palm rejection work.
- A touchscreen (a 2-in-1's panel) taps where touched, drags, and releases.
