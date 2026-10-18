# buslogic — the BusLogic MultiMaster SCSI host adapter (104b:1040, the
# BT-958): VirtualBox's and VMware's BusLogic controllers. Port I/O for its
# four registers, one coherent DMA grant below 4 GiB for the 32-bit
# mailboxes, the CCB, the sense buffer and the data buffer, polled (no
# Irq). Serves the disks and CD drives on its targets on the block wire
# (NBW1) through nonos_scsi, to the kernel's client and a StoreWrite holder
# alone.

CAPSULE_SLUG             := driver-buslogic
CAPSULE_HANDLE           := driver.buslogic0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_buslogic
CAPSULE_BIN_NAME         := driver_buslogic
CAPSULE_FEATURE          := nonos-capsule-driver-buslogic
CAPSULE_NAMESPACE        := systems.nonos.driver.buslogic0
CAPSULE_SERVICE_ENDPOINT := service:4296:driver.buslogic0
CAPSULE_REPLY_ENDPOINT   := reply:4297:endpoint.4294967358
# CoreExec|IPC|Memory|Driver|DeviceEnum|Dma|Pio = 0x198019. CoreExec for
# its own pid, so a stray reply on its own reply endpoint is dropped rather
# than answered. No Mmio: the registers are ports. No Irq: the adapter's
# interrupt is off and the driver polls the incoming mailbox.
CAPSULE_REQUIRED_CAPS    := 0x198019
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/buslogic_capsule

include nonos-mk/capsule.mk
