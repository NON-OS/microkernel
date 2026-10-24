# usb_mic — the USB Audio capture driver. Finds a microphone on any USB
# host's ports, root or virtual, and streams it only to a peer the kernel says
# holds MicrophoneCapture, asked at every open and start, while the
# compositor's indicator shows it. It touches no hardware: every request
# goes through the host wire.

CAPSULE_SLUG             := driver-usb-mic
CAPSULE_HANDLE           := driver.usb_mic0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_usb_mic
CAPSULE_BIN_NAME         := driver_usb_mic
CAPSULE_FEATURE          := nonos-capsule-driver-usb-mic
CAPSULE_NAMESPACE        := systems.nonos.driver.usb_mic0
CAPSULE_SERVICE_ENDPOINT := service:4520:driver.usb_mic0
CAPSULE_REPLY_ENDPOINT   := reply:4521:endpoint.4294967481
# IPC and Memory: the microphone is reached through a host's wire, so no
# Driver/DeviceEnum/Mmio/Irq/Dma/Pio. It never holds MicrophoneCapture.
CAPSULE_REQUIRED_CAPS    := 0x18
# Debug, granted only by a build that compiles `capsule-serial-debug`, for
# the [usb-mic] lines.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/usb_mic_capsule

include nonos-mk/capsule.mk
