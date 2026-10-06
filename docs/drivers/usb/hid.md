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
