# vmmdev — VirtualBox's guest device (80ee:cafe): mouse integration. Port
# I/O for the request port, MMIO for the event page, one DMA page for the
# requests, polled (no Irq). Posts absolute positions onto the kernel input
# ring and serves nothing; the kernel holds the endpoint to no capsule.

CAPSULE_SLUG             := driver-vmmdev
CAPSULE_HANDLE           := driver.vmmdev0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_vmmdev
CAPSULE_BIN_NAME         := driver_vmmdev
CAPSULE_FEATURE          := nonos-capsule-driver-vmmdev
CAPSULE_NAMESPACE        := systems.nonos.driver.vmmdev0
CAPSULE_SERVICE_ENDPOINT := service:4284:driver.vmmdev0
CAPSULE_REPLY_ENDPOINT   := reply:4285:endpoint.4294967353
# IPC|Memory|Driver|DeviceEnum|Mmio|Dma|Pio|InputSource = 0x3B8018. No Irq:
# the driver polls the event page.
CAPSULE_REQUIRED_CAPS    := 0x3B8018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/vmmdev_capsule

include nonos-mk/capsule.mk
