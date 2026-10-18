# PS/2 pointer

What `capsule_driver_ps2_input` does with the bytes of a PS/2 mouse or of a
touchpad on the aux port, and with a mouse that resets. How the mouse is
brought up is in [PS/2 bring-up](ps2-bringup.md).

## Mouse and touchpad

A packet is 3 bytes, or 4 once the wheel knock succeeded. The first byte must
have bit 3 set; a first byte without it is counted as a sync error and dropped,
so the parser waits for the start of the next packet, and a part packet older
than half a second is dropped too (`absorb`,
`userland/capsule_driver_ps2_input/src/mouse/parser.rs:47-69`). The check is
weak: a stray byte with bit 3 set still passes it. An axis the mouse marks as
overflowed is held to 255 in its direction rather than applied as read (`axis`,
`userland/capsule_driver_ps2_input/src/mouse/axis.rs:23-33`).

A PS/2 mouse counts upward motion as positive. The driver negates Y, so that on
the input ring positive Y points down the screen (`parse`,
`userland/capsule_driver_ps2_input/src/mouse/packet.rs:26-62`). It also negates
the wheel step, so a notch away from you counts as +1, as it does for USB and
I2C mice (`publish`,
`userland/capsule_driver_ps2_input/src/mouse/post.rs:24-37`).

When the mouse bring-up failed, bytes from the aux port are still read to keep
the output buffer clear, and then thrown away (`aux_enabled`,
`userland/capsule_driver_ps2_input/src/poll/drain.rs:37-60`).

A touchpad on the aux port gets no special treatment. The driver speaks no
vendor touchpad protocol, so it sees what the touchpad sends in plain PS/2 mouse
mode: relative motion and its buttons.

## A mouse that resets

A mouse that resets (a KVM switch, a re-plug, an EC reset) sends 0xAA 0x00 and
comes back with reporting off, in the plain 3-byte report. The serve loop brings
it up again, the wheel knock included (`reinit_mouse`,
`userland/capsule_driver_ps2_input/src/server/reinit.rs:32-54`). The keyboard
stays in use meanwhile: a key byte that lands while the mouse's reply is awaited
goes through `absorb` like any other, so a key let go then is still let go
(`reenable_mouse`,
`userland/capsule_driver_ps2_input/src/init/enable_mouse/enable.rs:41-58`).

A mouse that resets a fourth time within a minute of three bring-ups, or that
does not come back, is turned off as at start: the aux port off, the keyboard
port on, and, with Debug, one console line saying so (`REINITS`,
`userland/capsule_driver_ps2_input/src/server/reinits.rs:23-24`). The keyboard
keeps working.

## See also

- [PS/2 keys](ps2-input.md)
- [PS/2 keyboard and mouse](ps2.md)
- [Input drivers](README.md)
