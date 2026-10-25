# rndis: USB RNDIS class driver (Microsoft Remote NDIS 1.0 over USB).
# Older Android phones' USB tethering and QEMU usb-net's first
# configuration. It finds its device through driver.xhci0, owns RNDIS
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
# IPC | Memory | Crypto = 0x08 | 0x10 | 0x20 = 0x38. The loop drops what
# it sent itself by its own pid, read back through MkServiceLookup (IPC)
# rather than MkGetPid, so no CoreExec.
# No Driver/DeviceEnum/Mmio/Irq/Dma/Pio: this is a class capsule above
# xHCI. No InputSource (0x200000, bit 21), which posts to and drains the
# keystroke ring: a NIC has no use for it. Crypto (0x20) is what the
# CryptoRandom syscall is gated on: the station address is drawn once a
# run, before the first bind, never the adapter's own, and handed to every
# bind; without entropy no device is bound.
CAPSULE_REQUIRED_CAPS    := 0x38
# Debug (0x100, bit 8), granted only by a build that compiles
# `capsule-serial-debug`: it carries the [usbnet rndis] lines, which no
# frame and no station address reaches.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/rndis_capsule

include nonos-mk/capsule.mk
