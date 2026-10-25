# Input: routing and privacy

Where `input_router` sends each event it drains from the kernel input ring,
the keys no focused window ever sees, and what no input driver keeps. How an
event gets to the ring is in [Input drivers](README.md).

## Routing

For a pointer event the router moves the cursor, sends the compositor the new
position, and asks the window manager `wm` which window is under the pointer. A
key press goes to the focused app, the window `wm` names as having focus. Its
release goes to the window that got the press, so a focus change while a key is
held cannot strand it (`route_keyboard`,
`userland/capsule_input_router/src/route/keyboard.rs:25-56`).

A process receives an event only when it has subscribed to that kind of event,
and the router keeps at most 64 subscriptions (`MAX_SUBSCRIBERS`,
`userland/capsule_input_router/src/state/subscriptions/types.rs:21`). Five
services may grab chosen kinds of event, so that those events go to them alone:
the boot splash, the setup wizard, the input probe, `desktop_shell` and the
installer. Any other sender is refused (`GRABBERS`,
`userland/capsule_input_router/src/server/handlers/grab_request.rs:28-29`).

Three groups of keys never reach the focused window:

- Ctrl+Alt+Esc goes to `desktop_shell`, which brings Process Manager forward to
  end a window that misbehaves (`is_reserved_chord`,
  `userland/capsule_input_router/src/route/chord.rs:34-36`).
- Ctrl+Alt+T goes to `desktop_shell` over focus, which brings the Terminal
  forward or opens it (`is_terminal_chord` in the same file). It is not
  reserved: a grab or the trusted hold takes it like any key.
- Mute, Volume Down, Volume Up and Power go to `desktop_shell` whatever has
  focus (`is_shell_key`,
  `userland/capsule_input_router/src/route/shell_keys.rs:32-34`).
  [Audio](../audio.md#volume-and-the-volume-keys) says what the volume keys do.
  The Power key, from a keyboard or from the [ACPI power
  button](../platform-acpi-power-button.md#the-power-button), asks `Power off and
  wipe memory?`, and Power Off sends `power.off` to capsule_power, which runs the
  wiped shutdown (`userland/capsule_desktop_shell/src/server/handlers/power_off.rs:36`,
  `userland/capsule_power/src/off/mod.rs:38`).

## Privacy

No input driver writes a key to disk or to a log. The one exception for pointer
data is the touchpad driver: when it holds the Debug capability it writes the
first 16 bytes of each of its first six raw reports to the console, so a decode
can be checked against the wire (`frame_dumps`,
`userland/capsule_driver_i2c_hid/src/input/poll/read_frame.rs:55-58`).
`input_router` is granted IPC, Memory and InputConsumer and nothing else, so it
cannot write to the serial console (`CAPSULE_REQUIRED_CAPS`,
`userland/capsule_input_router/Capsule.mk:14-17`).

The kernel makes `driver.ps2_kbd0`, `driver.usb_hid0` and `driver.i2c_hid0`
[held endpoints](../../overview/glossary-hardware-broker.md#held-endpoint) with an empty sender
list, so no capsule can ask a keyboard driver for keys (`KERNEL_ONLY`,
`src/services/registry/held_table.rs:32-34`).

The capability rules above close two gaps an earlier rule left. `Irq` no
longer admits a post, so the AHCI, HD Audio, Intel Wi-Fi, NVMe, virtio-blk and
xHCI drivers, whose manifests ask for it, cannot forge input; the I2C
controller driver no longer asks for it, as its engine polls
(`CAPSULE_REQUIRED_CAPS`, `userland/capsule_driver_i2c_pci/Capsule.mk:18`).
And the three input drivers, which hold `InputSource`, cannot drain the ring:
that takes `InputConsumer`, which `input_router` alone holds, with no
`InputSource`, and the static checks and the seal refuse it in any other
capsule (`userland/capsule_input_router/Capsule.mk:14-17`; `can_input_consumer`,
`src/capabilities/token/types/authority_broker.rs:63-66`;
`scripts/check_input_drain_cap.py`).

## See also

- [Input drivers](README.md)
- [Input: layouts and limits](layouts.md)
- [Audio](../audio.md)
- [Platform: the power button](../platform-acpi-power-button.md#the-power-button)
