# ccid — the USB CCID smart-card reader driver. Finds an APDU-level reader
# on any USB host's ports, root or virtual, and passes one client at a time
# its cards' APDUs. Its endpoint is held to no one until the owner names
# that client. It touches no hardware: every request goes through the host
# wire.

CAPSULE_SLUG             := driver-ccid
CAPSULE_HANDLE           := driver.ccid0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_ccid
CAPSULE_BIN_NAME         := driver_ccid
CAPSULE_FEATURE          := nonos-capsule-driver-ccid
CAPSULE_NAMESPACE        := systems.nonos.driver.ccid0
CAPSULE_SERVICE_ENDPOINT := service:4524:driver.ccid0
CAPSULE_REPLY_ENDPOINT   := reply:4525:endpoint.4294967483
# IPC and Memory: the reader is reached through a host's wire, so no
# Driver/DeviceEnum/Mmio/Irq/Dma/Pio.
CAPSULE_REQUIRED_CAPS    := 0x18
# Debug, granted only by a build that compiles `capsule-serial-debug`, for
# the [ccid] lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/ccid_capsule

include nonos-mk/capsule.mk
