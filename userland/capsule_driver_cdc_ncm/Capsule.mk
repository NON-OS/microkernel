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
# IPC | Memory | Debug = 0x08 | 0x10 | 0x200000 = 0x200018
# No Driver/DeviceEnum/Mmio/Irq/Dma/Pio: this is a class capsule above
# xHCI. Debug carries its [usbnet cdc-ncm] lines; frames never reach it.
CAPSULE_REQUIRED_CAPS    := 0x200018

include nonos-mk/capsule.mk
# Builds and is proven on the host, but has no owner certificate, no kernel
# embed and no spawn yet: scripts/hardware_support_evidence.py reports it as
# not in the image instead of as supported hardware. Remove when it ships.
CAPSULE_NOT_IN_IMAGE     := 1
