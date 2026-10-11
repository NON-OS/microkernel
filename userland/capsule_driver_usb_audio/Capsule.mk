# usb_audio: USB Audio Class 1.0 playback driver. USB speakers, headsets and
# DACs that play 48 kHz stereo 16-bit PCM through an isochronous OUT
# endpoint. It finds its device through driver.xhci0, owns the class binding
# and the stream's packet timing only, and serves audio.server the NHDA PCM
# wire driver.hda0 serves.

CAPSULE_SLUG             := driver-usb-audio
CAPSULE_HANDLE           := driver.usb_audio0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_usb_audio
CAPSULE_BIN_NAME         := driver_usb_audio
CAPSULE_FEATURE          := nonos-capsule-driver-usb-audio
CAPSULE_NAMESPACE        := systems.nonos.driver.usb_audio0
CAPSULE_SERVICE_ENDPOINT := service:4246:driver.usb_audio0
CAPSULE_REPLY_ENDPOINT   := reply:4247:endpoint.4294967327
# CoreExec | IPC | Memory = 0x01 | 0x08 | 0x10 = 0x19. CoreExec is for
# MkGetPid: a reply that found no waiter comes back to the service inbox,
# and the loop drops what it sent itself rather than answer it.
# No Driver/DeviceEnum/Mmio/Irq/Dma/Pio: this is a class capsule above
# xHCI, whose DMA carries the stream.
CAPSULE_REQUIRED_CAPS    := 0x19
# Debug, granted only by a build that compiles `capsule-serial-debug`, for
# the [usbnet usb-audio] lines; PCM never reaches them.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/usb_audio_capsule

include nonos-mk/capsule.mk
