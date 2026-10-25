# piix-ide — the Intel PIIX3/PIIX4 IDE controller (8086:7010, 8086:7111):
# QEMU's pc machine, VirtualBox's default, Hyper-V Generation 1. Port I/O
# for both channels and the bus master window, one coherent DMA grant for
# the descriptor table and the transfer buffer, polled (no Irq). Serves the
# block wire (NBW1), one unit per drive slot, to the kernel's client and a
# StoreWrite holder alone.

CAPSULE_SLUG             := driver-piix-ide
CAPSULE_HANDLE           := driver.piix_ide0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_piix_ide
CAPSULE_BIN_NAME         := driver_piix_ide
CAPSULE_FEATURE          := nonos-capsule-driver-piix-ide
CAPSULE_NAMESPACE        := systems.nonos.driver.piix_ide0
CAPSULE_SERVICE_ENDPOINT := service:4288:driver.piix_ide0
CAPSULE_REPLY_ENDPOINT   := reply:4289:endpoint.4294967355
# CoreExec|IPC|Memory|Driver|DeviceEnum|Dma|Pio = 0x198019. CoreExec for
# its own pid, so a stray reply on its own reply endpoint is dropped rather
# than answered. No Mmio: every window is I/O ports. No Irq: the driver
# polls the engine and the disk.
CAPSULE_REQUIRED_CAPS    := 0x198019
# Debug, granted only by a build that compiles `capsule-serial-debug`: the
# kernel mirror folds it in through serial_debug_cap().
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_KERNEL_MIRROR    := src/hardware/piix_ide_capsule

include nonos-mk/capsule.mk
