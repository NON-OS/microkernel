# e1000e: Intel 82574/82583 and the I217/I218/I219 PHYs on Intel PCH MACs.
# PCI MMIO + DMA, polled, with separate RX and TX rings (four DMA grants).
# Frame-level transport over IPC; no socket or routing policy. `Network` is
# absent: that authority belongs to the stack capsules above this driver
# (net.core, net.l2). Crypto draws the per-boot station address. Wire shape
# matches `driver-e1000` so one net-stack client drives either backend.
#
# Not included from mk/ yet: wiring it into the kernel and the stack is a
# separate step, because including it makes the seal need new publisher keys.

CAPSULE_SLUG             := driver-e1000e
CAPSULE_HANDLE           := driver.e1000e_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_e1000e
CAPSULE_BIN_NAME         := driver_e1000e
CAPSULE_FEATURE          := nonos-capsule-driver-e1000e
CAPSULE_NAMESPACE        := systems.nonos.driver.e1000e_0
CAPSULE_SERVICE_ENDPOINT := service:4270:driver.e1000e_0
CAPSULE_REPLY_ENDPOINT   := reply:4271:endpoint.4294967390
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Dma = 0xB8038. No Irq: the driver
# polls and binds no line. MkPciConfigRead and MkPciConfigWrite (bus master
# on, the I219 ring status read) are gated on Driver alone
# (src/syscall/contract/cap_table/mk.rs), which this already holds.
# Crypto (0x20) is what the CryptoRandom syscall is gated on; the station
# address is drawn and that draw fails closed.
CAPSULE_REQUIRED_CAPS    := 0xB8038
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap(). MkDebug is gated on
# it, and without it every `e1000e:` step line is dropped before the log.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/e1000e_capsule

include nonos-mk/capsule.mk
# Builds and is proven on the host, but has no owner certificate, no kernel
# embed and no spawn yet: scripts/hardware_support_evidence.py reports it as
# not in the image instead of as supported hardware. Remove when it ships.
CAPSULE_NOT_IN_IMAGE     := 1
