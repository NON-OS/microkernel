# rtl8153: Realtek RTL8153 and RTL8153B USB 3.0 Gigabit Ethernet in vendor
# mode (Linux r8152), the chip of most USB-C Gigabit dongles and of dock
# Ethernet. It finds its device through driver.xhci0, owns the chip's
# register bring-up and framing only, and serves Ethernet frames to
# net.core over NNET.

CAPSULE_SLUG             := driver-rtl8153
CAPSULE_HANDLE           := driver.rtl8153_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_rtl8153
CAPSULE_BIN_NAME         := driver_rtl8153
CAPSULE_FEATURE          := nonos-capsule-driver-rtl8153
CAPSULE_NAMESPACE        := systems.nonos.driver.rtl8153_0
CAPSULE_SERVICE_ENDPOINT := service:4258:driver.rtl8153_0
CAPSULE_REPLY_ENDPOINT   := reply:4259:endpoint.4294967364
# IPC | Memory | Debug = 0x08 | 0x10 | 0x200000 = 0x200018
# No Driver/DeviceEnum/Mmio/Irq/Dma/Pio: this is a class capsule above
# xHCI. Debug carries its [usbnet rtl8153] lines; frames never reach it.
CAPSULE_REQUIRED_CAPS    := 0x200018

include nonos-mk/capsule.mk
# Builds and is proven on the host, but has no owner certificate, no kernel
# embed and no spawn yet: scripts/hardware_support_evidence.py reports it as
# not in the image instead of as supported hardware. Remove when it ships.
CAPSULE_NOT_IN_IMAGE     := 1
