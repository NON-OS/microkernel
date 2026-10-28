# ax88179: ASIX AX88179 and AX88178A USB 3.0 gigabit Ethernet driver, and
# the adapters sold under other names around them. It finds its device
# through the USB hosts, brings the chip up through its vendor register
# access, and serves Ethernet frames to net.core over NNET.

CAPSULE_SLUG             := driver-ax88179
CAPSULE_HANDLE           := driver.ax88179_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_ax88179
CAPSULE_BIN_NAME         := driver_ax88179
CAPSULE_FEATURE          := nonos-capsule-driver-ax88179
CAPSULE_NAMESPACE        := systems.nonos.driver.ax88179_0
CAPSULE_SERVICE_ENDPOINT := service:4256:driver.ax88179_0
CAPSULE_REPLY_ENDPOINT   := reply:4257:endpoint.4294967363
# IPC | Memory | Crypto = 0x08 | 0x10 | 0x20 = 0x38. The loop drops what
# it sent itself by its own pid, read back through MkServiceLookup (IPC)
# rather than MkGetPid, so no CoreExec.
# No Driver/DeviceEnum/Mmio/Irq/Dma/Pio: the chip is reached only through
# the USB host's control and bulk transfers. No InputSource (0x200000, bit
# 21), which posts to and drains the keystroke ring: a NIC has no use for
# it. Crypto (0x20) is what the CryptoRandom syscall is gated on: the
# station address is drawn once a run, before the first bind, written into
# the chip's station register, never the chip's own; without entropy no
# device is bound.
CAPSULE_REQUIRED_CAPS    := 0x38
# Debug (0x100, bit 8), granted only by a build that compiles
# `capsule-serial-debug`: it carries the [usbnet ax88179] lines, which no
# frame and no station address reaches.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/ax88179_capsule

include nonos-mk/capsule.mk
