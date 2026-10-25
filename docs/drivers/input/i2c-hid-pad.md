# I2C-HID touchpads: the pad

What `driver.i2c_hid0` does once it has found a touchpad's HID descriptor: wakes
the pad, reads its reports, and turns them into pointer events. How it finds
the pad: [I2C-HID touchpads: finding the pad](i2c-hid-finding.md).

## Waking the pad

Once the 30-byte HID descriptor reads, the driver (`reprobe`,
`userland/capsule_driver_i2c_hid/src/setup.rs:21-64`):

1. Sends SET_POWER ON, once more 1 ms later if the pad does not acknowledge it,
   then waits 60 ms (`POWER_ON_MS`,
   `userland/capsule_driver_i2c_hid/src/hid/power/settle.rs:22-25`). Then it
   sends RESET and reads the input register every 5 ms for up to one second for
   the empty report a reset device sends, holding an early answer to at least
   100 ms (`RESET_TIMEOUT_MS`,
   `userland/capsule_driver_i2c_hid/src/hid/power/await_reset.rs:29-31`).
2. Reads the report descriptor, at most 1024 bytes, three times 20 ms apart if
   it has to (`REPORT_DESC_ATTEMPTS`,
   `userland/capsule_driver_i2c_hid/src/setup.rs:112`).
3. Sets the Precision Touchpad input mode to 3 and the surface and button
   switches on, writing a feature report only when a bit differs
   (`INPUT_MODE_TOUCHPAD`,
   `userland/capsule_driver_i2c_hid/src/hid/input_mode/configure.rs:28-54`).

## Reading reports

The transfer engine polls the controller, and the touchpad's own interrupt line
is routed to neither capsule. A touchpad holds that line active while a report
waits, so `driver.i2c_pci0` reads the level of that GPIO pad and answers
`OP_GPIO_DOORBELL` with it (`handle`,
`userland/capsule_driver_i2c_pci/src/server/handlers/doorbell.rs:37-52`). It
finds the pad's register:

- on Broxton, Apollo Lake and Gemini Lake, from the community the firmware
  names, at the pin's index (`is_bxt_family`,
  `userland/capsule_driver_i2c_pci/src/constants/device_info.rs:71-74`);
- on AMD, through `nonos_pinctrl` (`locate`,
  `userland/nonos_pinctrl/src/locate.rs`). On Intel chipsets from Sunrise Point
  on no pad layout is kept since the originality audit (F 46), so the driver
  reads on a timer there.

`driver.i2c_hid0` trusts the doorbell once it has rung. It then reads only when
the line is active, and falls back to timed reads after 2500 quiet turns, or for
good when the controller has no doorbell to offer (`DOORBELL_TRUST_CYCLES`,
`userland/capsule_driver_i2c_hid/src/server/runner/run.rs:35-37`). Its loop
waits 2 ms for a request each turn (`RECV_TIMEOUT_MS`,
`userland/capsule_driver_i2c_hid/src/server/runner/run.rs:26`).

A pad in touchpad mode sends absolute contacts, which go through the gesture
decoder. A pad that only sends a mouse report is decoded as a relative mouse
with up to five buttons (`publish`,
`userland/capsule_driver_i2c_hid/src/input/publish.rs:25-49`).

## Gestures

| Gesture | What it does | Source |
|---|---|---|
| One finger moves | Relative cursor motion with speed-dependent gain, capped at 36 units per report | `MOTION_CAP`, `userland/capsule_driver_i2c_hid/src/input/gesture/types.rs:31` |
| Tap | Left click when the finger lifts within 40 reports and moved less than a sixteenth of the pad | `TAP_MAX_FRAMES`, `userland/capsule_driver_i2c_hid/src/input/gesture/types.rs:40-41` |
| Clickpad press | Left button, after two reports agree | `on_touch`, `userland/capsule_driver_i2c_hid/src/input/gesture/on_touch.rs:26-48` |
| Two fingers move | Scroll, one wheel notch per 1/32 of the pad's height, traditional direction | `SCROLL_NOTCHES_PER_PAD`, `userland/capsule_driver_i2c_hid/src/input/gesture/types.rs:46-50` |
| Palm | A contact the pad marks not confident stops all motion until every finger lifts | `palm`, `userland/capsule_driver_i2c_hid/src/input/gesture/on_touch.rs:62-74` |

Both a tap and a clickpad press post button 1 (`publish_touch`,
`userland/capsule_driver_i2c_hid/src/input/publish_touch.rs:29-46`). In touchpad
mode there is no right click, no three-finger gesture and no pinch. Natural
scrolling is a constant in the source, off, and not a setting (`NATURAL_SCROLL`,
`userland/capsule_driver_i2c_hid/src/input/gesture/types.rs:50`).

## See also

- [I2C-HID touchpads](i2c-hid.md)
- [Input drivers](README.md)
- [Platform: GPIO and pinctrl](../platform.md)
