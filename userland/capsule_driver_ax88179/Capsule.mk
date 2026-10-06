# ax88179: ASIX AX88179 / AX88178A USB 3.0 and 2.0 Gigabit Ethernet, the
# chip in most USB-C and USB-A Gigabit dongles (Linux ax88179_178a). It
# finds its device through driver.xhci0, owns the chip's bring-up, link
# and vendor framing only, and serves Ethernet frames to net.core over NNET.

CAPSULE_SLUG             := driver-ax88179
CAPSULE_HANDLE           := driver.ax88179_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_ax88179
CAPSULE_BIN_NAME         := driver_ax88179
CAPSULE_FEATURE          := nonos-capsule-driver-ax88179
CAPSULE_NAMESPACE        := systems.nonos.driver.ax88179_0
CAPSULE_SERVICE_ENDPOINT := service:4256:driver.ax88179_0
CAPSULE_REPLY_ENDPOINT   := reply:4257:endpoint.4294967363
# IPC | Memory | Debug = 0x08 | 0x10 | 0x200000 = 0x200018
# No Driver/DeviceEnum/Mmio/Irq/Dma/Pio: this is a class capsule above
# xHCI. No Crypto either, so a chip without a station address is not
# given a random one. Debug carries its [usbnet ax88179] lines.
CAPSULE_REQUIRED_CAPS    := 0x200018

include nonos-mk/capsule.mk
# Builds and is proven on the host, but has no owner certificate, no kernel
# embed and no spawn yet: scripts/hardware_support_evidence.py reports it as
# not in the image instead of as supported hardware. Remove when it ships.
CAPSULE_NOT_IN_IMAGE     := 1
