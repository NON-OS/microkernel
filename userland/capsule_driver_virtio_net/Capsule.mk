# virtio_net: virtio network device. PCI MMIO or PIO + DMA, polled
# (INTx turned off), with separate RX and TX rings (four DMA grants
# total). Frame-level transport over IPC; no socket or routing policy.
# `Network` is absent: that authority belongs to the stack capsules
# above this driver (net.core, net.l2), the only ones the kernel lets
# send to it.

CAPSULE_SLUG             := driver-virtio-net
CAPSULE_HANDLE           := driver.virtio_net0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_virtio_net
CAPSULE_BIN_NAME         := driver_virtio_net
CAPSULE_FEATURE          := nonos-capsule-driver-virtio-net
CAPSULE_NAMESPACE        := systems.nonos.driver.virtio_net0
CAPSULE_SERVICE_ENDPOINT := service:4204:driver.virtio_net0
CAPSULE_REPLY_ENDPOINT   := reply:4205:endpoint.4294967305
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Dma|Pio = 0x1B8038
# Crypto (0x20) is what the CryptoRandom syscall is gated on: the station
# address is drawn per boot rather than read off the device, as e1000
# does, and the draw fails closed without it.
# No Irq: the driver polls its rings and makes no MkIrq* call (and posts
# no input), the only calls Irq admits. A driver moved to interrupts
# takes the bit back.
CAPSULE_REQUIRED_CAPS    := 0x1B8038
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/virtio_net_capsule

include nonos-mk/capsule.mk
