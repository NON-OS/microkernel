# Input: layouts and limits

The keyboard layouts the input drivers apply themselves, and what the input
path does not do. How an event travels and which driver handles which device is
in [Input drivers](README.md); how it is checked, in [Input: how it is
checked](checks.md).

## Keyboard layouts

The keyboard drivers turn a key into its final character themselves, because
nothing downstream applies Shift or a layout. The PS/2 driver keeps the active
layout in `LAYOUT_INDEX`
(`userland/capsule_driver_ps2_input/src/keymap/active.rs:28`), and the USB HID
driver has the same `cycle`
(`userland/capsule_driver_usb_hid/src/hid/active.rs:43`).

Both use the tables in `nonos_keymap`: US, UK, German, French AZERTY, Spanish
and Italian (`Layout`, `userland/nonos_keymap/src/layout.rs:21-28`). Setup
offers exactly these six (`POLICY_LAYOUTS`,
`userland/nonos_keymap/src/policy.rs:28`). A layout index with no table maps to
nothing, and the driver keeps the layout it has (`from_policy`,
`userland/nonos_keymap/src/policy.rs:32-42`).

The drivers read the layout from the [policy
store](../../overview/glossary-peer-list.md#policy-store) at most once a second (`POLICY`,
`userland/capsule_driver_ps2_input/src/keymap/active.rs:31`). Ctrl+Alt+Space
moves to the next layout in the order above, and the driver consumes that chord,
so no application sees it (`layout_chord`,
`userland/capsule_driver_ps2_input/src/poll/chord.rs:37-48`). [Keyboard
layouts](../../using/keyboard-layouts.md) is the guide for people at the
keyboard.

## What is not there

- Each key gives one character: `resolve` returns one code per key, so there are
  no dead keys, no compose sequences and no input method editor
  (`userland/nonos_keymap/src/resolve.rs:35`).
- Num Lock is not applied. The PS/2 driver lights its LED, on from start, but a
  keypad key always posts its own code, so the keypad always types digits
  (`character`, `userland/capsule_driver_ps2_input/src/keymap/keypad.rs:43-48`).
- I2C touchscreens are left out of the device table the kernel builds from the
  ACPI tables (`register_acpi_i2c`,
  `src/hardware/broker/acpi_i2c/register.rs:36-51`).
- A touchpad on the PS/2 aux port works as a plain PS/2 mouse, without its
  vendor's own protocol. See [PS/2 pointer](ps2-pointer.md).
- An I2C touchpad in touchpad mode gives no right click. See [I2C-HID
  touchpads](i2c-hid.md).
- A USB keyboard's Mute and volume keys are read from the keyboard usage page
  only (`KEYCODE_MUTE`,
  `userland/capsule_driver_usb_hid/src/hid/usage_keycode/map.rs:58-63`). Media
  keys sent on a consumer control interface are not read, since the USB HID
  driver binds the boot keyboard interface alone.

## See also

- [Input drivers](README.md)
- [Input: how it is checked](checks.md)
- [PS/2 keyboard and mouse](ps2.md)
- [I2C-HID touchpads](i2c-hid.md)
- [USB keyboards and mice](../usb/hid.md)
- [Keyboard layouts](../../using/keyboard-layouts.md)
