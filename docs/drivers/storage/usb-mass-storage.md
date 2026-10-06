# USB mass storage

How NONOS reads and writes USB sticks and USB disks through `driver.usb_msc0`, and what it does not support.

## What it binds

`driver.usb_msc0` binds a USB interface of class 08h (mass storage), subclass 06h (SCSI transparent) and protocol 50h, the Bulk-Only Transport, with one bulk IN and one bulk OUT endpoint (`userland/capsule_driver_usb_msc/src/descriptors/visitor.rs:43-52`, `PROTOCOL_BULK_ONLY`). USB Attached SCSI (UAS) is not implemented in this release, so a device that offers only UAS is not served. The driver reads the SuperSpeed endpoint companion, so a USB 3 device's burst size reaches the controller (`userland/capsule_driver_usb_msc/src/descriptors/visitor.rs:71-93`, `visit_companion`).
