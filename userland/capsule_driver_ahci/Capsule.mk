# AHCI: SATA host controller capsule. PCI MMIO + DMA; the INTx line
# is bound when it can be, but every command completion is polled.
# Owns controller discovery, broker claim, ABAR mapping, AHCI-mode
# enable, port identity, command DMA, and read/write/flush block
# operations.

CAPSULE_SLUG             := driver-ahci
CAPSULE_HANDLE           := driver.ahci0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_ahci
CAPSULE_BIN_NAME         := driver_ahci
CAPSULE_FEATURE          := nonos-capsule-driver-ahci
CAPSULE_NAMESPACE        := systems.nonos.driver.ahci0
CAPSULE_SERVICE_ENDPOINT := service:4216:driver.ahci0
CAPSULE_REPLY_ENDPOINT   := reply:4217:endpoint.4294967311
# IPC|Memory|Driver|DeviceEnum|Mmio|Irq|Dma = 0xf8019
CAPSULE_REQUIRED_CAPS    := 0xf8018
# Debug, granted only by a build that compiles `capsule-serial-debug` (the
# development images): the kernel mirror folds it in through
# serial_debug_cap(), so the driver's own lines reach the console there and
# nowhere else.
CAPSULE_OPTIONAL_CAPS    := 0x100

include nonos-mk/capsule.mk
