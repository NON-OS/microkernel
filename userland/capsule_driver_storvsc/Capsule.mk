# storvsc — Hyper-V's synthetic SCSI controller, on the VMBus channel the
# VMBus core gives it (its record: 1414:5343 on the virtual bus, the relid in
# irq_source). One coherent DMA grant for the channel's two rings and the
# data buffer, polled (no Irq). Serves the disks and CD drives on its
# targets on the block wire (NBW1) through nonos_scsi, to the kernel's
# client and a StoreWrite holder alone.

CAPSULE_SLUG             := driver-storvsc
CAPSULE_HANDLE           := driver.storvsc0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_storvsc
CAPSULE_BIN_NAME         := driver_storvsc
CAPSULE_FEATURE          := nonos-capsule-driver-storvsc
CAPSULE_NAMESPACE        := systems.nonos.driver.storvsc0
CAPSULE_SERVICE_ENDPOINT := service:4304:driver.storvsc0
CAPSULE_REPLY_ENDPOINT   := reply:4305:endpoint.4294967394
# CoreExec|IPC|Memory|Driver|DeviceEnum|Dma = 0x98019. CoreExec for its own
# pid, so a stray reply on its own reply endpoint is dropped rather than
# answered. Driver for the channel record's claim and its MkHv* calls. No
# Mmio or Pio: a channel has neither. No Irq: the driver polls the host's
# ring.
CAPSULE_REQUIRED_CAPS    := 0x98019
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/storvsc_capsule

include nonos-mk/capsule.mk
