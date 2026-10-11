# rndis: USB RNDIS class driver (Microsoft Remote NDIS 1.0 over USB, as
# Linux rndis_host). Older Android phones' USB tethering and QEMU usb-net's
# first configuration. It finds its device through driver.xhci0, owns RNDIS
# binding and framing only, and serves Ethernet frames to net.core over NNET.

CAPSULE_SLUG             := driver-rndis
CAPSULE_HANDLE           := driver.rndis0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_rndis
CAPSULE_BIN_NAME         := driver_rndis
CAPSULE_FEATURE          := nonos-capsule-driver-rndis
CAPSULE_NAMESPACE        := systems.nonos.driver.rndis0
CAPSULE_SERVICE_ENDPOINT := service:4254:driver.rndis0
CAPSULE_REPLY_ENDPOINT   := reply:4255:endpoint.4294967362
# IPC | Memory = 0x08 | 0x10 = 0x18. The loop drops what it sent itself by
# its own pid, read back through MkServiceLookup (IPC) rather than
# MkGetPid, so no CoreExec.
# No Driver/DeviceEnum/Mmio/Irq/Dma/Pio: this is a class capsule above
# xHCI.
CAPSULE_REQUIRED_CAPS    := 0x18
# Debug, granted only by a build that compiles `capsule-serial-debug`, for
# the [usbnet rndis] lines; frames never reach them.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/rndis_capsule

include nonos-mk/capsule.mk
