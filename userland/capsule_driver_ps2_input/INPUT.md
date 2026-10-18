# capsule_driver_ps2_input: keys, lock LEDs and mouse packets

What the serve loop does with each byte the i8042 delivers. The contract is in
[README.md](README.md).

## Keyboard

Each byte from the keyboard is framed by the prefixes before it
(`src/poll/prefix.rs`), decoded through the active layout and posted at once;
none is kept. Pause (E1 1D 45 E1 9D C5) is one press and one release of
Pause, never Ctrl+Num Lock; Print Screen (E0 37, or SysRq 54) and Break
(E0 46) are keys of their own, and the fake shifts around them post nothing.
The keyboard's ACK and RESEND are answers to a command, not keys, wherever
they land, even between the bytes of Pause.

## Lock LEDs

Caps Lock, Num Lock and Scroll Lock toggle the driver's lock state, and the
serve loop sends it to the keyboard (0xED and the LED bitmap, each
acknowledged within 200 ms, `src/init/leds.rs`), so the lights and the case
agree; a keyboard that does not acknowledge is not asked again until the next
lock key. The first pass sends it whatever firmware left lit: Num Lock on, as
the keypad always types its digits, and Caps and Scroll Lock off
(`src/poll/drainer.rs`).

## Mouse packets

The parser requires bit 3 in the first packet byte for synchronization. Bad
alignment increments `mouse_sync_errors` and never enters the queue. A part
packet older than half a second is dropped, so a lost byte cannot take the
next packet's header.

A mouse that resets (a KVM switch, a re-plug, an EC reset) announces it with
0xAA 0x00 at a packet boundary; that is never read as a packet, and the serve
loop brings the mouse up again, the wheel knock included
(`src/server/reinit.rs`). The keyboard stays in use meanwhile: a key byte that
lands while the mouse's reply is awaited goes through the keyboard decode and is
posted, so a key let go then is not lost (`src/init/read_port.rs`). A mouse that
does not come back, or resets again after three bring-ups within a minute
(`src/server/reinits.rs`), is turned off for the session, the aux port off and
the keyboard port on, and the console says so once with Debug.
