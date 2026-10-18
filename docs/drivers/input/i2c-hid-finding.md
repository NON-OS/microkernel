# I2C-HID touchpads: finding the pad

How the touchpads ACPI declares are registered, which controller and which
addresses `driver.i2c_pci0` settles on, and how `driver.i2c_hid0` finds the pad.
Which controllers are driven: [controllers](i2c-hid-controllers.md).

## Finding the touchpad

The kernel walks the ACPI tables for HID-over-I2C devices: a `_HID` or `_CID` of
PNP0C50 or ACPI0C50, or a `_HID` that starts with a touchpad vendor's prefix
(`parse_hid_devices`, `src/arch/x86_64/acpi/devices/i2c/parse.rs:35`). It
registers touchpads first and leaves touchscreens out (`register_acpi_i2c`,
`src/hardware/broker/acpi_i2c/register.rs:36-51`). Known touchpad and
touchscreen ids are listed in `TOUCHPAD_HIDS` and `TOUCHSCREEN_HIDS`
(`src/arch/x86_64/acpi/devices/i2c/hids.rs:19-62`). For each touchpad the kernel
writes one line to the boot console with its id, bus, address, speed and
interrupt, and a warning when the pad uses 10-bit addressing, which NONOS does
not support (`report`, `src/hardware/broker/acpi_i2c/report.rs:32-74`).

`driver.i2c_pci0` then picks the controller and the reach (`run`,
`userland/capsule_driver_i2c_pci/src/setup/sequence/run.rs:33-72`):

1. Controllers that a declared touchpad names are tried first (`probe_order`,
   `userland/capsule_driver_i2c_pci/src/setup/sequence/order.rs:31-47`).
2. On each, every declared 7-bit address is asked for its HID descriptor, at the
   declared register, then 0x0001 and 0x0020 (`probe_hid`,
   `userland/capsule_driver_i2c_pci/src/transaction/engine/probe_hid.rs:43`).
   The first controller whose bus returns one is kept, with that address alone
   in reach.
3. Failing that, a controller where a declared device only acknowledged, then
   the controller the firmware named, then the first that comes up. Until a
   device returns its descriptor, every pad declared on that controller, or on
   none, stays in reach, so the pad multi-SKU firmware declares second is
   reachable once it wakes; an address declared twice carries the register of
   the declaration bound, which `OP_ACPI_HID` answers (`settle`,
   `userland/capsule_driver_i2c_pci/src/setup/sequence/settle.rs:50-75`). On
   the last resort, the first controller that comes up when none the firmware
   named does, nothing is bound and every declared address is in reach,
   whichever controller its declaration names (`declared`,
   `userland/capsule_driver_i2c_pci/src/setup/sequence/order.rs:25-27`;
   `userland/capsule_driver_i2c_pci/src/setup/sequence/run.rs:65-68`). The
   first time a client then writes exactly the HID descriptor register declared
   for one of those addresses and reads a valid HID descriptor back
   (wHIDDescLength 30, bcdVersion 1.00) through `OP_TRANSFER`, the reach becomes
   that address alone for the rest of the session; it never widens again
   (`after_read`,
   `userland/capsule_driver_i2c_pci/src/transaction/types/narrow.rs:34-43`).
4. A declaration with address 0, as Intel's reference firmware writes its
   TPD0, names its controller and nothing else. There, and only there, the
   sweep reads 0x15, 0x2C and 0x10, those three alone, one byte each, and binds
   the first device that answers even if it is not a pad. Once the kernel
   evaluates `_CRS` for the pad, the sweep goes: [a pad with no
   address](i2c-hid-sweep.md).
5. With nothing declared, nothing on the bus is probed and nothing is in reach.

The bus runs in fast mode unless a device on it declared a lower speed
(`standard_mode`,
`userland/capsule_driver_i2c_pci/src/setup/sequence/order.rs:51-55`).

`driver.i2c_hid0` asks for the address setup bound (`OP_ACPI_HID`, `handle`,
`userland/capsule_driver_i2c_pci/src/server/handlers/acpi_hid.rs:36-51`) and
probes it (`reprobe`, `userland/capsule_driver_i2c_hid/src/setup.rs:21-29`).
When there is none, or no descriptor answers there, it scans the addresses in
reach ([a pad with no address](i2c-hid-sweep.md#the-hid-drivers-scan)).

## See also

- [I2C-HID touchpads](i2c-hid.md)
- [I2C-HID touchpads: controllers](i2c-hid-controllers.md)
- [I2C-HID touchpads: a pad with no address](i2c-hid-sweep.md)
- [I2C-HID touchpads: the pad](i2c-hid-pad.md)
