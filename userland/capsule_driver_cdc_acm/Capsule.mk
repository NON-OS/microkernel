# cdc_acm — the USB CDC ACM serial driver. Finds a serial line or a
# modem's AT port on any USB host's ports, root or virtual, and serves one
# client at a time its bytes, settings and line state. Its endpoint is held
# to the terminal. It touches no hardware: every request goes through the
# host wire.

CAPSULE_SLUG             := driver-cdc-acm
CAPSULE_HANDLE           := driver.cdc_acm0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_cdc_acm
CAPSULE_BIN_NAME         := driver_cdc_acm
CAPSULE_FEATURE          := nonos-capsule-driver-cdc-acm
CAPSULE_NAMESPACE        := systems.nonos.driver.cdc_acm0
CAPSULE_SERVICE_ENDPOINT := service:4522:driver.cdc_acm0
CAPSULE_REPLY_ENDPOINT   := reply:4523:endpoint.4294967482
# IPC and Memory: the line is reached through a host's wire, so no
# Driver/DeviceEnum/Mmio/Irq/Dma/Pio.
CAPSULE_REQUIRED_CAPS    := 0x18
# Debug, granted only by a build that compiles `capsule-serial-debug`, for
# the [cdc-acm] lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/cdc_acm_capsule

include nonos-mk/capsule.mk
