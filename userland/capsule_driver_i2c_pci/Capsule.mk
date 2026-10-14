# Intel LPSS I2C PCI controller capsule. Hardware-facing only:
# PCI discovery, MMIO register window, controller identity, and
# timing telemetry. HID-over-I2C/input policy stays above this
# driver. The engine polls, so no Irq: the kernel lets an Irq holder
# post input events, and this bus driver has none to post.

CAPSULE_SLUG             := driver-i2c-pci
CAPSULE_HANDLE           := driver.i2c_pci0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_i2c_pci
CAPSULE_BIN_NAME         := driver_i2c_pci
CAPSULE_FEATURE          := nonos-capsule-driver-i2c-pci
CAPSULE_NAMESPACE        := systems.nonos.driver.i2c_pci0
CAPSULE_SERVICE_ENDPOINT := service:4230:driver.i2c_pci0
CAPSULE_REPLY_ENDPOINT   := reply:4231:endpoint.4294967318
# IPC|Memory|DeviceEnum|Driver|Mmio
# = 0x08|0x10|0x8000|0x10000|0x20000 = 0x38018
CAPSULE_REQUIRED_CAPS    := 0x38018
CAPSULE_KERNEL_MIRROR    := src/hardware/i2c_pci_capsule

include nonos-mk/capsule.mk

