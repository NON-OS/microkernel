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
# IPC | Memory | Debug = 0x08 | 0x10 | 0x200000 = 0x200018
# No Driver/DeviceEnum/Mmio/Irq/Dma/Pio: this is a class capsule above
# xHCI. Debug carries its [usbnet rndis] lines; frames never reach it.
CAPSULE_REQUIRED_CAPS    := 0x200018

include nonos-mk/capsule.mk
# Builds and is proven on the host, but has no owner certificate, no kernel
# embed and no spawn yet: scripts/hardware_support_evidence.py reports it as
# not in the image instead of as supported hardware. Remove when it ships.
CAPSULE_NOT_IN_IMAGE     := 1
