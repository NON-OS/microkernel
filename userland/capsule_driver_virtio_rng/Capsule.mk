# virtio_rng: virtio entropy source. PCI MMIO or PIO + DMA, polled
# (Interrupt Disable set, no line bound). Only entropy bytes flow out
# over IPC; no crypto runs inside the driver. Reply inbox name is the kernel-side IPC routing label
# (see `src/hardware/virtio_rng_capsule/client/transport.rs`).

CAPSULE_SLUG             := driver-virtio-rng
CAPSULE_HANDLE           := driver.virtio_rng
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_virtio_rng
CAPSULE_BIN_NAME         := driver_virtio_rng
CAPSULE_FEATURE          := nonos-capsule-driver-virtio-rng
CAPSULE_NAMESPACE        := systems.nonos.driver.virtio_rng
CAPSULE_SERVICE_ENDPOINT := service:4200:driver.virtio_rng
CAPSULE_REPLY_ENDPOINT   := reply:4201:endpoint.4294967302
# IPC|Memory|Driver|DeviceEnum|Mmio|Dma|Pio
# = 0x08|0x10|0x10000|0x8000|0x20000|0x80000|0x100000 = 0x1B8018
# No Irq: the driver polls its rings and makes no MkIrq* call (and posts
# no input), the only calls Irq admits. A driver moved to interrupts
# takes the bit back.
CAPSULE_REQUIRED_CAPS    := 0x1B8018
CAPSULE_KERNEL_MIRROR    := src/hardware/virtio_rng_capsule

include nonos-mk/capsule.mk
