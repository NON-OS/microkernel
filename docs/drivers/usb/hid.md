# USB keyboards and mice

What `driver.usb_hid0` binds, how it reads USB keyboards and mice, and where their events go.

## What it binds

`driver.usb_hid0` reads each device's configuration and binds its HID interfaces, class 03h, through an interrupt IN endpoint other than endpoint 0 (`userland/capsule_driver_usb_hid/src/descriptors/binding.rs:32-66`, `from_pair`):

| Interface | Bound as |
|---|---|
| boot subclass 01h, protocol 01h | keyboard |
| boot subclass 01h, protocol 02h | mouse |
| a HID interface outside the boot subclass, on a device with no boot keyboard or mouse | absolute pointer |

A device with a boot keyboard or boot mouse interface keeps only those. Its other HID interfaces, such as media and system controls, are not read (`userland/capsule_driver_usb_hid/src/descriptors/parse.rs:52-62`, `keep_boot_interfaces`). Up to 8 interfaces are bound per device (`userland/capsule_driver_usb_hid/src/protocol/limits.rs:21`, `MAX_HID_BINDINGS`), and the endpoint's packet must hold a whole report (`userland/capsule_driver_usb_hid/src/descriptors/packet_size.rs:23-37`, `holds_a_report`).

## Boot protocol only

The driver puts keyboards and mice in the boot protocol with SET_PROTOCOL, and asks keyboards with SET_IDLE(0) to report only on change (`userland/capsule_driver_usb_hid/src/orchestrator/binding.rs:24-47`, `SET_PROTOCOL`). It does not read HID report descriptors. The reports it decodes have fixed layouts:

- Keyboard: exactly 8 bytes, the modifiers, a reserved byte and six key slots. A report of any other length changes nothing (`userland/capsule_driver_usb_hid/src/hid/keyboard/boot_report.rs:29-40`, `BootReport::parse`).
- Mouse: at least 3 bytes, five buttons, X and Y motion, and the wheel in a fourth byte when there is one; bytes past it are not read (`userland/capsule_driver_usb_hid/src/hid/mouse_report.rs:24-46`, `mouse_event`).
- Absolute pointer: at least 5 bytes, three buttons, then X and Y as 16-bit positions from 0 to 0x7FFF, then an optional wheel, the layout QEMU's `usb-tablet` sends (`userland/capsule_driver_usb_hid/src/hid/tablet_report.rs:17-52`, `tablet_report`).

So every absolute pointer, a touch screen or a pen tablet among them, is decoded in the `usb-tablet` layout whatever layout it really sends, and media keys sent on a separate consumer-control interface are not read (`userland/capsule_driver_usb_hid/src/hid/usage_keycode/map.rs:58-61`, `KEYCODE_MUTE`). A keyboard or mouse that refuses SET_PROTOCOL is still bound, and its reports are read as boot reports whatever their layout.
