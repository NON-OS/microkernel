# Input drivers

How a key press, a mouse movement or a touch on the touchpad reaches an application in NONOS 0.9.2, and which driver handles which device.

## Which driver handles what

| Device | Driver | Service | Page |
|---|---|---|---|
| Keyboard, mouse or touchpad on the i8042 (PS/2) controller | `capsule_driver_ps2_input` | `driver.ps2_kbd0` | [PS/2 keyboard and mouse](ps2.md) |
| HID-over-I2C touchpad on an Intel LPSS or AMD I2C controller | `capsule_driver_i2c_hid` over `capsule_driver_i2c_pci` | `driver.i2c_hid0`, `driver.i2c_pci0` | [I2C-HID touchpads](i2c-hid.md) |
| USB keyboard, mouse or tablet behind an xHCI controller | `capsule_driver_usb_hid` | `driver.usb_hid0` | [USB HID](../usb/hid.md) |

Each driver is its own [capsule](../../overview/glossary.md#capsule) in ring 3, and none of them decides which window gets the input. Only the PS/2 driver holds grants from the [hardware broker](../../overview/glossary.md#hardware-broker) itself. The USB HID driver holds IPC, Memory and InputSource alone and asks `driver.xhci0` for its transfers (`CAPSULE_REQUIRED_CAPS`, `userland/capsule_driver_usb_hid/Capsule.mk:15`); the touchpad driver does the same through `driver.i2c_pci0`.
