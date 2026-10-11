# virtio-scsi — virtio's SCSI host (1af4:1048, and 1af4:1004 with the
# modern capabilities): QEMU's virtio-scsi-pci. The modern transport's
# regions through MMIO, one coherent DMA grant for the ring, the request
# and the buffer, polled (no Irq). Serves the disks and CD drives on its
# targets on the block wire (NBW1) through nonos_scsi, to the kernel's
# client and a StoreWrite holder alone.

CAPSULE_SLUG             := driver-virtio-scsi
CAPSULE_HANDLE           := driver.virtio_scsi0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_virtio_scsi
CAPSULE_BIN_NAME         := driver_virtio_scsi
CAPSULE_FEATURE          := nonos-capsule-driver-virtio-scsi
CAPSULE_NAMESPACE        := systems.nonos.driver.virtio_scsi0
CAPSULE_SERVICE_ENDPOINT := service:4292:driver.virtio_scsi0
CAPSULE_REPLY_ENDPOINT   := reply:4293:endpoint.4294967356
# CoreExec|IPC|Memory|Driver|DeviceEnum|Mmio|Dma = 0xB8019. CoreExec for
# its own pid, so a stray reply on its own reply endpoint is dropped rather
# than answered. No Pio: the legacy window is never used. No Irq: the
# driver polls the used ring.
CAPSULE_REQUIRED_CAPS    := 0xB8019
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/virtio_scsi_capsule

include nonos-mk/capsule.mk
