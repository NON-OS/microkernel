# rtl8153: Realtek RTL8153 and RTL8153B USB 3.0 gigabit Ethernet, the common
# laptop dongle, in its vendor configuration. It finds its device through
# driver.xhci0, brings the chip up over vendor control transfers, and
# serves Ethernet frames to net.core over NNET.

CAPSULE_SLUG             := driver-rtl8153
CAPSULE_HANDLE           := driver.rtl8153_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_rtl8153
CAPSULE_BIN_NAME         := driver_rtl8153
CAPSULE_FEATURE          := nonos-capsule-driver-rtl8153
CAPSULE_NAMESPACE        := systems.nonos.driver.rtl8153_0
CAPSULE_SERVICE_ENDPOINT := service:4258:driver.rtl8153_0
CAPSULE_REPLY_ENDPOINT   := reply:4259:endpoint.4294967364
# IPC | Memory | Crypto = 0x08 | 0x10 | 0x20 = 0x38, as cdc_ecm. The loop
# drops what it sent itself by its own pid, read back through
# MkServiceLookup (IPC) rather than MkGetPid, so no CoreExec.
# No Driver/DeviceEnum/Mmio/Irq/Dma/Pio: the chip's registers are reached
# by vendor control transfers through driver.xhci0. No InputSource. Crypto
# (0x20) is what the CryptoRandom syscall is gated on: the station address
# is drawn once a run, before the first bind, written into the chip's
# address register, and never the chip's own.
CAPSULE_REQUIRED_CAPS    := 0x38
# Debug (0x100, bit 8), granted only by a build that compiles
# `capsule-serial-debug`: it carries the [usbnet rtl8153] lines, which no
# frame, station address or serial number reaches.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/rtl8153_capsule

include nonos-mk/capsule.mk
