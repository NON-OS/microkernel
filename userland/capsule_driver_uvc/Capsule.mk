# uvc — the USB Video Class camera driver. Finds a camera on any USB host's
# ports, root or virtual, and streams it only to a peer the kernel says
# holds CameraCapture, asked at every open and start, while the
# compositor's indicator shows it. It touches no hardware: every request
# goes through the host wire.

CAPSULE_SLUG             := driver-uvc
CAPSULE_HANDLE           := driver.uvc0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_uvc
CAPSULE_BIN_NAME         := driver_uvc
CAPSULE_FEATURE          := nonos-capsule-driver-uvc
CAPSULE_NAMESPACE        := systems.nonos.driver.uvc0
CAPSULE_SERVICE_ENDPOINT := service:4518:driver.uvc0
CAPSULE_REPLY_ENDPOINT   := reply:4519:endpoint.4294967480
# IPC and Memory: the camera is reached through a host's wire, so no
# Driver/DeviceEnum/Mmio/Irq/Dma/Pio. It never holds CameraCapture.
CAPSULE_REQUIRED_CAPS    := 0x18
# Debug, granted only by a build that compiles `capsule-serial-debug`, for
# the [uvc] lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/uvc_capsule

include nonos-mk/capsule.mk
