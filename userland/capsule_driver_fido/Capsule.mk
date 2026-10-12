# fido — the USB FIDO2 security key driver. Finds a CTAPHID key on any USB
# host's ports, root or virtual, and passes one client at a time its CTAP2
# requests and answers, unread. Its endpoint is held to no one until the
# owner names who may ask a key to sign. It touches no hardware: every
# request goes through the host wire.

CAPSULE_SLUG             := driver-fido
CAPSULE_HANDLE           := driver.fido0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_fido
CAPSULE_BIN_NAME         := driver_fido
CAPSULE_FEATURE          := nonos-capsule-driver-fido
CAPSULE_NAMESPACE        := systems.nonos.driver.fido0
CAPSULE_SERVICE_ENDPOINT := service:4526:driver.fido0
CAPSULE_REPLY_ENDPOINT   := reply:4527:endpoint.4294967484
# IPC and Memory: the key is reached through a host's wire, so no
# Driver/DeviceEnum/Mmio/Irq/Dma/Pio.
CAPSULE_REQUIRED_CAPS    := 0x18
# Debug, granted only by a build that compiles `capsule-serial-debug`, for
# the [fido] lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/fido_capsule

include nonos-mk/capsule.mk
