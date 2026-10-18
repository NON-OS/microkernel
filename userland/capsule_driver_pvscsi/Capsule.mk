# pvscsi — VMware's paravirtual SCSI adapter (15ad:07c0): VMware's PVSCSI
# and QEMU's pvscsi. One MMIO register window, one coherent DMA grant for
# the rings state, the request and completion rings, the sense buffer and
# the data buffer, every interrupt masked and polled (no Irq). Serves the
# disks and CD drives on its targets on the block wire (NBW1) through
# nonos_scsi, to the kernel's client and a StoreWrite holder alone.

CAPSULE_SLUG             := driver-pvscsi
CAPSULE_HANDLE           := driver.pvscsi0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_pvscsi
CAPSULE_BIN_NAME         := driver_pvscsi
CAPSULE_FEATURE          := nonos-capsule-driver-pvscsi
CAPSULE_NAMESPACE        := systems.nonos.driver.pvscsi0
CAPSULE_SERVICE_ENDPOINT := service:4294:driver.pvscsi0
CAPSULE_REPLY_ENDPOINT   := reply:4295:endpoint.4294967357
# CoreExec|IPC|Memory|Driver|DeviceEnum|Mmio|Dma = 0xB8019. CoreExec for
# its own pid, so a stray reply on its own reply endpoint is dropped rather
# than answered. No Pio: VMware's I/O BAR is never used. No Irq: every
# interrupt is masked and the driver polls the completion ring.
CAPSULE_REQUIRED_CAPS    := 0xB8019
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/pvscsi_capsule

include nonos-mk/capsule.mk
