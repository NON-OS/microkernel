# virtio_gpu: virtio display controller. PCI MMIO/PIO + DMA, polled.
# The capsule owns device initialization and the control queue; UI,
# compositor policy, surfaces, and focus stay outside the driver.

CAPSULE_SLUG             := driver-virtio-gpu
CAPSULE_HANDLE           := driver.virtio_gpu0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_virtio_gpu
CAPSULE_BIN_NAME         := driver_virtio_gpu
CAPSULE_FEATURE          := nonos-capsule-driver-virtio-gpu
CAPSULE_NAMESPACE        := systems.nonos.driver.virtio_gpu0
CAPSULE_SERVICE_ENDPOINT := service:4226:driver.virtio_gpu0
CAPSULE_REPLY_ENDPOINT   := reply:4227:endpoint.4294967316
# IPC | Memory | GraphicsSurfaceCreate | DeviceEnum | Driver | Mmio | Dma | Pio
# = 0x08 | 0x10 | 0x1000 | 0x8000 | 0x10000 | 0x20000 | 0x80000 | 0x100000 = 0x1B9018
# No Irq: the driver polls its rings and makes no MkIrq* call (and posts
# no input), the only calls Irq admits. A driver moved to interrupts
# takes the bit back.
# Debug must stay in the manifest, as an optional bit, because the kernel
# grants it through serial_debug_cap() under the capsule-serial-debug build:
# a grant outside the manifest once made the spawn gate reject this driver,
# leaving the compositor with no gpu backend.
CAPSULE_REQUIRED_CAPS    := 0x1B9018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/virtio_gpu_capsule

include nonos-mk/capsule.mk
