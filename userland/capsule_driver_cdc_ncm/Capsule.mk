# cdc_ncm: USB CDC Network Control Model class driver (USB NCM 1.0). Phones
# that tether over NCM and NCM USB Ethernet adapters. It finds its device
# through driver.xhci0, owns NCM binding and NTB framing only, and serves
# Ethernet frames to net.core over NNET.

CAPSULE_SLUG             := driver-cdc-ncm
CAPSULE_HANDLE           := driver.cdc_ncm0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_cdc_ncm
CAPSULE_BIN_NAME         := driver_cdc_ncm
CAPSULE_FEATURE          := nonos-capsule-driver-cdc-ncm
CAPSULE_NAMESPACE        := systems.nonos.driver.cdc_ncm0
CAPSULE_SERVICE_ENDPOINT := service:4252:driver.cdc_ncm0
CAPSULE_REPLY_ENDPOINT   := reply:4253:endpoint.4294967361
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
# `capsule-serial-debug`: it carries the [usbnet cdc-ncm] lines, which no
# frame and no station address reaches.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/cdc_ncm_capsule

include nonos-mk/capsule.mk
