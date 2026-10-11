# virtio-input — QEMU's virtio-tablet, virtio-keyboard and virtio-mouse
# (1af4:1052), every function found, over the modern virtio transport. PCI
# MMIO + one DMA page per function, polled. Events go onto the kernel input
# ring; nothing sends to this capsule.

CAPSULE_SLUG             := driver-virtio-input
CAPSULE_HANDLE           := driver.virtio_input0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_virtio_input
CAPSULE_BIN_NAME         := driver_virtio_input
CAPSULE_FEATURE          := nonos-capsule-driver-virtio-input
CAPSULE_NAMESPACE        := systems.nonos.driver.virtio_input0
CAPSULE_SERVICE_ENDPOINT := service:4278:driver.virtio_input0
CAPSULE_REPLY_ENDPOINT   := reply:4279:endpoint.4294967350
# IPC|Memory|Driver|DeviceEnum|Mmio|Dma|InputSource = 0x2B8018. No Irq: the
# driver polls and its available ring asks for no interrupt. IPC also reads
# the keyboard layout from the policy store, as the PS/2 driver does.
CAPSULE_REQUIRED_CAPS    := 0x2B8018
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100

include nonos-mk/capsule.mk
