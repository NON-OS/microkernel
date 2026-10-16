# spi_hid — HID over SPI. Fetches input reports through driver.spi0, the
# only client spi0 admits, from the device the firmware's serial-bus record
# names (capsule_acpi's SERIAL), and posts them as HID over I2C does. It
# holds no hardware.

CAPSULE_SLUG             := driver-spi-hid
CAPSULE_HANDLE           := driver.spi_hid0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_spi_hid
CAPSULE_BIN_NAME         := driver_spi_hid
CAPSULE_FEATURE          := nonos-capsule-driver-spi-hid
CAPSULE_NAMESPACE        := systems.nonos.driver.spi_hid0
CAPSULE_SERVICE_ENDPOINT := service:4534:driver.spi_hid0
CAPSULE_REPLY_ENDPOINT   := reply:4535:endpoint.4294967490
# IPC, Memory and InputSource: talks to spi0 and capsule_acpi, and posts
# input events.
CAPSULE_REQUIRED_CAPS    := 0x200018
# Debug, granted only by a build that compiles `capsule-serial-debug`.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/spi_hid_capsule

include nonos-mk/capsule.mk
