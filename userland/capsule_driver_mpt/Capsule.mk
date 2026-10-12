# mpt — the LSI Fusion-MPT SCSI controllers (1000:0030, the 53C1030;
# 1000:0054, the SAS1068): VMware's LSI Logic Parallel and LSI Logic SAS,
# VirtualBox's LsiLogic and LsiLogic SAS, QEMU's mptsas1068. The MPI 1.x
# message interface through one MMIO register window, one coherent DMA
# grant below 4 GiB for the request frame, the reply frames, the sense
# buffer and the data buffer, every interrupt masked and polled (no Irq).
# Serves the disks and CD drives on its targets on the block wire (NBW1)
# through nonos_scsi, to the kernel's client and a StoreWrite holder alone.

CAPSULE_SLUG             := driver-mpt
CAPSULE_HANDLE           := driver.mpt0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_mpt
CAPSULE_BIN_NAME         := driver_mpt
CAPSULE_FEATURE          := nonos-capsule-driver-mpt
CAPSULE_NAMESPACE        := systems.nonos.driver.mpt0
CAPSULE_SERVICE_ENDPOINT := service:4298:driver.mpt0
CAPSULE_REPLY_ENDPOINT   := reply:4299:endpoint.4294967359
# CoreExec|IPC|Memory|Driver|DeviceEnum|Mmio|Dma = 0xB8019. CoreExec for
# its own pid, so a stray reply on its own reply endpoint is dropped rather
# than answered. No Pio: the I/O BAR is never used. No Irq: every interrupt
# is masked and the driver polls the reply FIFO.
CAPSULE_REQUIRED_CAPS    := 0xB8019
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/mpt_capsule

include nonos-mk/capsule.mk
