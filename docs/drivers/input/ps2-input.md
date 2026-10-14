# PS/2 keys

What `capsule_driver_ps2_input` does with each byte from the keyboard once it
serves, and how it keeps the lock LEDs. Mouse and touchpad packets are in [PS/2
pointer](ps2-pointer.md); how the driver finds the controller, in [PS/2
keyboard and mouse](ps2.md).

## Keys

Each byte from the keyboard goes through `absorb`: E0 and E1 prefixes become
flags, and bit 7 marks a release (`absorb`,
`userland/capsule_driver_ps2_input/src/poll/absorb.rs:26-58`). The byte is then
translated to a key, resolved through the active layout with Shift, Caps Lock
and AltGr, and posted to the kernel input ring (`publish`,
`userland/capsule_driver_ps2_input/src/keymap/post.rs:25-48`). Nothing of it is
kept once it is posted but the code of a key still held.

- Pause (E1 1D 45 E1 9D C5) is one press and one release of Pause, never
  Ctrl+Num Lock. The keyboard's ACK (0xFA) and RESEND (0xFE) are answers to a
  command wherever they land, even between the bytes of Pause, and never keys
  (`take`, `userland/capsule_driver_ps2_input/src/poll/prefix.rs:53-56`).
- A release carries the code its press went down with. Shift+A released after
  Shift is still an A release, so the router can pair it with its press
  (`HeldKeys`, `userland/nonos_keymap/src/held.rs:31-34`).
- A held key repeats as more presses. Mute and Power act once per press and
  their repeats are dropped (`acts_once`,
  `userland/capsule_driver_ps2_input/src/keymap/once.rs:27-29`).
- Ctrl+Alt+Space switches to the next keyboard layout and is not passed on
  (`layout_chord`, `userland/capsule_driver_ps2_input/src/poll/chord.rs:37-48`).
  [Keyboard layouts](../../using/keyboard-layouts.md) lists the six layouts.

## Lock keys and LEDs

Caps Lock, Num Lock and Scroll Lock toggle the driver's lock state; Caps Lock
also decides the case of every letter. Between two waits the serve loop sends
the state to the keyboard when it changed, 0xED and then the bitmap, each
acknowledged within 200 ms (`set_leds`,
`userland/capsule_driver_ps2_input/src/init/leds.rs:34-36`). A keyboard that
does not acknowledge is not asked again until the next lock key (`sync_leds`,
`userland/capsule_driver_ps2_input/src/server/upkeep.rs:28-40`). The first pass
sends it whatever firmware left lit: Num Lock on, as the keypad types digits,
and the other two off (`new`,
`userland/capsule_driver_ps2_input/src/poll/drainer.rs:35-39`).

## See also

- [PS/2 pointer](ps2-pointer.md)
- [PS/2 keyboard and mouse](ps2.md)
- [Input drivers](README.md)
- [Keyboard layouts](../../using/keyboard-layouts.md)
