# PS/2 keyboard and mouse

`capsule_driver_ps2_input` drives the i8042 controller: a laptop's built-in keyboard when it sits behind an i8042 (often one an embedded controller emulates), a PS/2 mouse, and a touchpad wired to the i8042's aux port.

## What works

- The keys of scan code set 1 in the driver's tables. A keyboard powers up in set 2, so the driver turns on the controller's translation bit when it writes the configuration byte (`keyboard_config`, `userland/capsule_driver_ps2_input/src/init/enable_keyboard/config.rs:31-33`).
- The E0-prefixed keys: arrows, Home, End, Page Up, Page Down, Insert, Delete, keypad Enter and Divide, right Ctrl, right Alt and both Windows keys (`keycode_for`, `userland/capsule_driver_ps2_input/src/keymap/set1_e0.rs:21-50`).
- F1 to F12, the numeric keypad, and the extra key left of Z on ISO keyboards (`KEY_ISO`, `userland/capsule_driver_ps2_input/src/keymap/set1/function.rs:51`).
- The Fn volume keys of a laptop that sends E0 20, E0 2E and E0 30 for Mute, Volume Down and Volume Up (`KEYCODE_MUTE`, `userland/capsule_driver_ps2_input/src/keymap/set1_e0.rs:28-30`), and a keyboard power key on E0 5E (`KEYCODE_POWER`, `userland/capsule_driver_ps2_input/src/keymap/set1_e0.rs:46`).
- Both Shift keys, both Ctrl keys, left Alt, both Windows keys as Meta, and right Alt as AltGr, never as Alt; the US layout maps nothing to AltGr (`modifier_bit`, `userland/capsule_driver_ps2_input/src/keymap/modifiers.rs:32-43`). Caps Lock is a toggle.
- A PS/2 mouse with left, right and middle buttons, and its scroll wheel when it answers the IntelliMouse knock (`detect_wheel`, `userland/capsule_driver_ps2_input/src/init/enable_mouse/detect_wheel.rs:29-39`).
- A touchpad on the aux port, as a plain relative mouse.
