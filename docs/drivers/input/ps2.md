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

## How the driver finds the controller

The i8042 cannot be enumerated, so the [hardware broker](../../overview/glossary.md#hardware-broker) publishes two records for it on every machine: the keyboard record with PNP vendor 0x0001, device 0x0303, ports 0x60 to 0x64 and IRQ 1, and the aux record with device 0x0304 and IRQ 12 (`register_legacy`, `src/hardware/broker/platform.rs:40-74`). The driver looks for both by these ids (`PNP_DEVICE_PS2_KBD`, `userland/capsule_driver_ps2_input/src/constants/pnp.rs:16-19`).

A record on every machine proves nothing. Before it starts, the driver claims the keyboard record, takes the port grant, empties the output buffer and gives the claim back, whatever it found (`controller_answers`, `userland/capsule_driver_ps2_input/src/setup/probe.rs:31-42`). On a machine whose keyboard and pointer are USB or I2C the ports float, the buffer never empties, and the driver exits with status 2, `EXIT_ABSENT`, holding nothing (`_start`, `userland/capsule_driver_ps2_input/src/main.rs:46-57`).
