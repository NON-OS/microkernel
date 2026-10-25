# usb_hub — the USB hub walker. Finds hubs on every USB host's root ports
# and below them, addresses each child through the host, and parks it with
# the host as a virtual port for the class drivers to claim. It touches no
# hardware: every request goes through the host wire.

CAPSULE_SLUG             := driver-usb-hub
CAPSULE_HANDLE           := driver.usb_hub0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_usb_hub
CAPSULE_BIN_NAME         := driver_usb_hub
CAPSULE_FEATURE          := nonos-capsule-driver-usb-hub
CAPSULE_NAMESPACE        := systems.nonos.driver.usb_hub0
CAPSULE_SERVICE_ENDPOINT := service:4516:driver.usb_hub0
CAPSULE_REPLY_ENDPOINT   := reply:4517:endpoint.4294967399
# IPC and Memory: every device is reached through a host's wire, so no
# Driver/DeviceEnum/Mmio/Irq/Dma/Pio.
CAPSULE_REQUIRED_CAPS    := 0x18
# Debug, granted only by a build that compiles `capsule-serial-debug`, for
# the [usb-hub] lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/usb_hub_capsule

include nonos-mk/capsule.mk
