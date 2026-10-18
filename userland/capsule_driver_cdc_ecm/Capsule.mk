# cdc_ecm: USB CDC Ethernet Control Model class driver (CDC ECM 1.2). USB
# Ethernet adapters and phones that tether over ECM. It finds its device
# through driver.xhci0, owns ECM binding and framing only, and serves
# Ethernet frames to net.core over NNET.

CAPSULE_SLUG             := driver-cdc-ecm
CAPSULE_HANDLE           := driver.cdc_ecm0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_cdc_ecm
CAPSULE_BIN_NAME         := driver_cdc_ecm
CAPSULE_FEATURE          := nonos-capsule-driver-cdc-ecm
CAPSULE_NAMESPACE        := systems.nonos.driver.cdc_ecm0
CAPSULE_SERVICE_ENDPOINT := service:4250:driver.cdc_ecm0
CAPSULE_REPLY_ENDPOINT   := reply:4251:endpoint.4294967360
# IPC | Memory | Crypto = 0x08 | 0x10 | 0x20 = 0x38
# No Driver/DeviceEnum/Mmio/Irq/Dma/Pio: this is a class capsule above
# xHCI. No InputSource (0x200000, bit 21), which posts to and drains the
# keystroke ring: a NIC has no use for it. Crypto (0x20) is what the
# CryptoRandom syscall is gated on: the station address is drawn once a
# run, before the first bind, never the adapter's own, and handed to every
# bind; without entropy no device is bound.
CAPSULE_REQUIRED_CAPS    := 0x38
# Debug (0x100, bit 8), granted only by a build that compiles
# `capsule-serial-debug`: it carries the [usbnet cdc-ecm] lines, none of
# which holds a station address.
CAPSULE_OPTIONAL_CAPS    := 0x100

include nonos-mk/capsule.mk
# Builds and is proven on the host, but has no owner certificate, no kernel
# embed and no spawn yet: scripts/hardware_support_evidence.py reports it as
# not in the image instead of as supported hardware. Remove when it ships.
CAPSULE_NOT_IN_IMAGE     := 1
